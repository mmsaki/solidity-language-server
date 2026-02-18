//! Direct `solc --standard-json` runner for fast AST generation.
//!
//! The output is normalized into the same shape that `forge build --json --ast`
//! produces, so all downstream consumers (goto, hover, completions, etc.) work
//! unchanged.

use crate::config::FoundryConfig;
use crate::runner::RunnerError;
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};
use tokio::process::Command;

/// Resolve the path to the solc binary.
///
/// Resolution order:
/// 1. If `foundry.toml` specifies a solc version, look for it in solc-select
///    artifacts (`~/.solc-select/artifacts/solc-<version>/solc-<version>`).
/// 2. Fall back to whatever `solc` is on `$PATH`.
pub fn resolve_solc_binary(config: &FoundryConfig) -> PathBuf {
    if let Some(ref version) = config.solc_version {
        // Try solc-select artifacts
        if let Some(home) = dirs_path() {
            let artifact = home
                .join(".solc-select")
                .join("artifacts")
                .join(format!("solc-{version}"))
                .join(format!("solc-{version}"));
            if artifact.is_file() {
                return artifact;
            }
        }
    }
    // Fall back to system solc
    PathBuf::from("solc")
}

/// Try to get the user's home directory.
fn dirs_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// Fetch remappings by running `forge remappings` in the project root.
///
/// Falls back to config remappings, then to an empty list.
pub async fn resolve_remappings(config: &FoundryConfig) -> Vec<String> {
    // Try `forge remappings` first — it merges all sources (foundry.toml,
    // remappings.txt, auto-detected libs).
    let output = Command::new("forge")
        .arg("remappings")
        .current_dir(&config.root)
        .env("FOUNDRY_DISABLE_NIGHTLY_WARNING", "1")
        .output()
        .await;

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let remappings: Vec<String> = stdout
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();
            if !remappings.is_empty() {
                return remappings;
            }
        }
    }

    // Fall back to remappings from foundry.toml
    if !config.remappings.is_empty() {
        return config.remappings.clone();
    }

    // Fall back to remappings.txt at project root
    let remappings_txt = config.root.join("remappings.txt");
    if let Ok(content) = std::fs::read_to_string(&remappings_txt) {
        return content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.to_string())
            .collect();
    }

    Vec::new()
}

/// Build the `--standard-json` input for solc.
pub fn build_standard_json_input(file_path: &str, remappings: &[String]) -> Value {
    json!({
        "language": "Solidity",
        "sources": {
            file_path: {
                "urls": [file_path]
            }
        },
        "settings": {
            "remappings": remappings,
            "outputSelection": {
                "*": {
                    "*": [
                        "abi",
                        "devdoc",
                        "userdoc",
                        "evm.methodIdentifiers",
                        "evm.gasEstimates"
                    ],
                    "": ["ast"]
                }
            }
        }
    })
}

/// Run `solc --standard-json` and return the parsed output.
pub async fn run_solc(
    solc_binary: &Path,
    input: &Value,
    project_root: &Path,
) -> Result<Value, RunnerError> {
    let input_str = serde_json::to_string(input)?;

    let mut child = Command::new(solc_binary)
        .arg("--standard-json")
        .current_dir(project_root)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    // Write the standard-json input to solc's stdin.
    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin
            .write_all(input_str.as_bytes())
            .await
            .map_err(RunnerError::CommandError)?;
        // Drop stdin to close it, signaling EOF to solc.
    }

    let output = child
        .wait_with_output()
        .await
        .map_err(RunnerError::CommandError)?;

    // solc writes JSON to stdout even on errors (errors are in the JSON)
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(RunnerError::CommandError(std::io::Error::other(format!(
            "solc produced no output, stderr: {stderr}"
        ))));
    }

    let parsed: Value = serde_json::from_str(&stdout)?;
    Ok(parsed)
}

/// Normalize raw solc `--standard-json` output into the canonical shape.
///
/// Solc's native shape is already close to canonical:
/// - `sources[path] = { id, ast }` — kept as-is
/// - `contracts[path][name] = { abi, evm, ... }` — kept as-is
/// - `errors` — kept as-is (defaults to `[]` if absent)
///
/// Constructs `source_id_to_path` from source IDs for cross-file resolution.
///
/// Takes ownership and uses `Value::take()` to move AST nodes in-place,
/// avoiding expensive clones of multi-MB AST data.
pub fn normalize_solc_output(mut solc_output: Value) -> Value {
    let mut result = Map::new();

    // Move errors out (defaults to [] if absent)
    let errors = solc_output
        .get_mut("errors")
        .map(Value::take)
        .unwrap_or_else(|| json!([]));
    result.insert("errors".to_string(), errors);

    // Sources: keep solc's native { path: { id, ast } } shape.
    // Also build source_id_to_path for cross-file resolution.
    let mut source_id_to_path = Map::new();

    if let Some(sources) = solc_output
        .get_mut("sources")
        .and_then(|s| s.as_object_mut())
    {
        for (path, source_data) in sources.iter() {
            if let Some(id) = source_data.get("id") {
                source_id_to_path.insert(id.to_string(), json!(path));
            }
        }
    }

    // Move sources as-is
    let sources = solc_output
        .get_mut("sources")
        .map(Value::take)
        .unwrap_or_else(|| json!({}));
    result.insert("sources".to_string(), sources);

    // Contracts: keep solc's native { path: { name: { abi, evm, ... } } } shape
    let contracts = solc_output
        .get_mut("contracts")
        .map(Value::take)
        .unwrap_or_else(|| json!({}));
    result.insert("contracts".to_string(), contracts);

    // Construct source_id_to_path for cross-file resolution
    result.insert(
        "source_id_to_path".to_string(),
        Value::Object(source_id_to_path),
    );

    Value::Object(result)
}

/// Normalize forge `build --json --ast` output into the canonical shape.
///
/// Forge wraps data in arrays with metadata:
/// - `sources[path] = [{ source_file: { id, ast }, build_id, profile, version }]`
/// - `contracts[path][name] = [{ contract: { abi, evm, ... }, build_id, profile, version }]`
/// - `build_infos = [{ source_id_to_path: { ... } }]`
///
/// This unwraps to the canonical flat shape:
/// - `sources[path] = { id, ast }`
/// - `contracts[path][name] = { abi, evm, ... }`
/// - `source_id_to_path = { ... }`
pub fn normalize_forge_output(mut forge_output: Value) -> Value {
    let mut result = Map::new();

    // Move errors out
    let errors = forge_output
        .get_mut("errors")
        .map(Value::take)
        .unwrap_or_else(|| json!([]));
    result.insert("errors".to_string(), errors);

    // Unwrap sources: [{ source_file: { id, ast } }] → { id, ast }
    let mut normalized_sources = Map::new();
    if let Some(sources) = forge_output
        .get_mut("sources")
        .and_then(|s| s.as_object_mut())
    {
        for (path, entries) in sources.iter_mut() {
            if let Some(arr) = entries.as_array_mut() {
                if let Some(first) = arr.first_mut() {
                    if let Some(sf) = first.get_mut("source_file") {
                        normalized_sources.insert(path.clone(), sf.take());
                    }
                }
            }
        }
    }
    result.insert("sources".to_string(), Value::Object(normalized_sources));

    // Unwrap contracts: [{ contract: { ... } }] → { ... }
    let mut normalized_contracts = Map::new();
    if let Some(contracts) = forge_output
        .get_mut("contracts")
        .and_then(|c| c.as_object_mut())
    {
        for (path, names) in contracts.iter_mut() {
            let mut path_contracts = Map::new();
            if let Some(names_obj) = names.as_object_mut() {
                for (name, entries) in names_obj.iter_mut() {
                    if let Some(arr) = entries.as_array_mut() {
                        if let Some(first) = arr.first_mut() {
                            if let Some(contract) = first.get_mut("contract") {
                                path_contracts.insert(name.clone(), contract.take());
                            }
                        }
                    }
                }
            }
            normalized_contracts.insert(path.clone(), Value::Object(path_contracts));
        }
    }
    result.insert("contracts".to_string(), Value::Object(normalized_contracts));

    // Extract source_id_to_path from build_infos
    let source_id_to_path = forge_output
        .get_mut("build_infos")
        .and_then(|bi| bi.as_array_mut())
        .and_then(|arr| arr.first_mut())
        .and_then(|info| info.get_mut("source_id_to_path"))
        .map(Value::take)
        .unwrap_or_else(|| json!({}));
    result.insert("source_id_to_path".to_string(), source_id_to_path);

    Value::Object(result)
}

/// Run solc for a file and return normalized output.
///
/// This is the main entry point used by the LSP.
pub async fn solc_ast(file_path: &str, config: &FoundryConfig) -> Result<Value, RunnerError> {
    let solc_binary = resolve_solc_binary(config);
    let remappings = resolve_remappings(config).await;
    let input = build_standard_json_input(file_path, &remappings);
    let raw_output = run_solc(&solc_binary, &input, &config.root).await?;
    Ok(normalize_solc_output(raw_output))
}

/// Run solc for build diagnostics (same output, just used for error extraction).
pub async fn solc_build(file_path: &str, config: &FoundryConfig) -> Result<Value, RunnerError> {
    solc_ast(file_path, config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_solc_sources() {
        let solc_output = json!({
            "sources": {
                "src/Foo.sol": {
                    "id": 0,
                    "ast": {
                        "nodeType": "SourceUnit",
                        "absolutePath": "src/Foo.sol",
                        "id": 100
                    }
                },
                "src/Bar.sol": {
                    "id": 1,
                    "ast": {
                        "nodeType": "SourceUnit",
                        "absolutePath": "src/Bar.sol",
                        "id": 200
                    }
                }
            },
            "contracts": {},
            "errors": []
        });

        let normalized = normalize_solc_output(solc_output);

        // Sources kept in solc-native shape: path -> { id, ast }
        let sources = normalized.get("sources").unwrap().as_object().unwrap();
        assert_eq!(sources.len(), 2);

        let foo = sources.get("src/Foo.sol").unwrap();
        assert_eq!(foo.get("id").unwrap(), 0);
        assert_eq!(
            foo.get("ast")
                .unwrap()
                .get("nodeType")
                .unwrap()
                .as_str()
                .unwrap(),
            "SourceUnit"
        );

        // Check source_id_to_path constructed
        let id_to_path = normalized
            .get("source_id_to_path")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(id_to_path.len(), 2);
    }

    #[test]
    fn test_normalize_solc_contracts() {
        let solc_output = json!({
            "sources": {},
            "contracts": {
                "src/Foo.sol": {
                    "Foo": {
                        "abi": [{"type": "function", "name": "bar"}],
                        "evm": {
                            "methodIdentifiers": {
                                "bar(uint256)": "abcd1234"
                            },
                            "gasEstimates": {
                                "external": {"bar(uint256)": "200"}
                            }
                        }
                    }
                }
            },
            "errors": []
        });

        let normalized = normalize_solc_output(solc_output);

        // Contracts kept in solc-native shape: path -> name -> { abi, evm, ... }
        let contracts = normalized.get("contracts").unwrap().as_object().unwrap();
        let foo_contracts = contracts.get("src/Foo.sol").unwrap().as_object().unwrap();
        let foo = foo_contracts.get("Foo").unwrap();

        let method_ids = foo
            .get("evm")
            .unwrap()
            .get("methodIdentifiers")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            method_ids.get("bar(uint256)").unwrap().as_str().unwrap(),
            "abcd1234"
        );
    }

    #[test]
    fn test_normalize_solc_errors_passthrough() {
        let solc_output = json!({
            "sources": {},
            "contracts": {},
            "errors": [{
                "sourceLocation": {"file": "src/Foo.sol", "start": 0, "end": 10},
                "type": "Warning",
                "component": "general",
                "severity": "warning",
                "errorCode": "2394",
                "message": "test warning",
                "formattedMessage": "Warning: test warning"
            }]
        });

        let normalized = normalize_solc_output(solc_output);

        let errors = normalized.get("errors").unwrap().as_array().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].get("errorCode").unwrap().as_str().unwrap(),
            "2394"
        );
    }

    #[test]
    fn test_normalize_empty_solc_output() {
        let solc_output = json!({
            "sources": {},
            "contracts": {}
        });

        let normalized = normalize_solc_output(solc_output);

        assert!(
            normalized
                .get("sources")
                .unwrap()
                .as_object()
                .unwrap()
                .is_empty()
        );
        assert!(
            normalized
                .get("contracts")
                .unwrap()
                .as_object()
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            normalized.get("errors").unwrap().as_array().unwrap().len(),
            0
        );
        assert!(
            normalized
                .get("source_id_to_path")
                .unwrap()
                .as_object()
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn test_build_standard_json_input() {
        let input = build_standard_json_input(
            "/path/to/Foo.sol",
            &[
                "ds-test/=lib/forge-std/lib/ds-test/src/".to_string(),
                "forge-std/=lib/forge-std/src/".to_string(),
            ],
        );

        let sources = input.get("sources").unwrap().as_object().unwrap();
        assert!(sources.contains_key("/path/to/Foo.sol"));

        let settings = input.get("settings").unwrap();
        let remappings = settings.get("remappings").unwrap().as_array().unwrap();
        assert_eq!(remappings.len(), 2);

        let output_sel = settings.get("outputSelection").unwrap();
        assert!(output_sel.get("*").is_some());
    }

    #[test]
    fn test_resolve_solc_binary_default() {
        let config = FoundryConfig::default();
        let binary = resolve_solc_binary(&config);
        assert_eq!(binary, PathBuf::from("solc"));
    }
}
