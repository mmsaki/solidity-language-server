//! Direct `solc --standard-json` runner for fast AST generation.
//!
//! The output is normalized into the same shape that `forge build --json --ast`
//! produces, so all downstream consumers (goto, hover, completions, etc.) work
//! unchanged.

use crate::config::FoundryConfig;
use crate::runner::RunnerError;
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use tokio::process::Command;

/// Cached list of installed solc versions. Populated on first access,
/// invalidated after a successful `svm install`.
static INSTALLED_VERSIONS: OnceLock<Mutex<Vec<SemVer>>> = OnceLock::new();

fn get_installed_versions() -> Vec<SemVer> {
    let mutex = INSTALLED_VERSIONS.get_or_init(|| Mutex::new(scan_installed_versions()));
    mutex.lock().unwrap().clone()
}

fn invalidate_installed_versions() {
    if let Some(mutex) = INSTALLED_VERSIONS.get() {
        *mutex.lock().unwrap() = scan_installed_versions();
    }
}

/// Resolve the path to the solc binary.
///
/// Resolution order:
/// 1. If `foundry.toml` specifies a solc version, use it (project config wins).
/// 2. Parse `pragma solidity` from the source file and find a matching
///    installed version via svm/solc-select directories.
/// 3. If no match is installed, auto-install via `svm install`.
/// 4. Fall back to whatever `solc` is on `$PATH`.
pub async fn resolve_solc_binary(
    config: &FoundryConfig,
    file_source: Option<&str>,
    client: Option<&tower_lsp::Client>,
) -> PathBuf {
    // 1. foundry.toml solc version takes precedence — skip pragma detection
    if let Some(ref version) = config.solc_version
        && let Some(path) = find_solc_binary(version)
    {
        if let Some(c) = client {
            c.log_message(
                tower_lsp::lsp_types::MessageType::INFO,
                format!(
                    "solc: using foundry.toml version {version} → {}",
                    path.display()
                ),
            )
            .await;
        }
        return path;
    }

    // 2. Try pragma from the file being compiled
    if let Some(source) = file_source
        && let Some(constraint) = parse_pragma(source)
    {
        let installed = get_installed_versions();
        if let Some(version) = find_matching_version(&constraint, &installed)
            && let Some(path) = find_solc_binary(&version.to_string())
        {
            if let Some(c) = client {
                c.log_message(
                    tower_lsp::lsp_types::MessageType::INFO,
                    format!(
                        "solc: pragma {constraint:?} → {version} → {}",
                        path.display()
                    ),
                )
                .await;
            }
            return path;
        }

        // 3. No matching version installed — try auto-install via svm
        let install_version = version_to_install(&constraint);
        if let Some(ref ver_str) = install_version {
            if let Some(c) = client {
                c.show_message(
                    tower_lsp::lsp_types::MessageType::INFO,
                    format!("Installing solc {ver_str}..."),
                )
                .await;
            }

            if svm_install(ver_str).await {
                // Refresh the cached version list after install
                invalidate_installed_versions();

                if let Some(c) = client {
                    c.show_message(
                        tower_lsp::lsp_types::MessageType::INFO,
                        format!("Installed solc {ver_str}"),
                    )
                    .await;
                }
                if let Some(path) = find_solc_binary(ver_str) {
                    return path;
                }
            } else if let Some(c) = client {
                c.show_message(
                    tower_lsp::lsp_types::MessageType::WARNING,
                    format!(
                        "Failed to install solc {ver_str}. \
                             Install it manually: svm install {ver_str}"
                    ),
                )
                .await;
            }
        }
    }

    // 4. Fall back to system solc
    if let Some(c) = client {
        c.log_message(
            tower_lsp::lsp_types::MessageType::INFO,
            "solc: no pragma match, falling back to system solc",
        )
        .await;
    }
    PathBuf::from("solc")
}

/// Determine which version to install for a pragma constraint.
///
/// - Exact: install that version
/// - Caret `^0.8.20`: install `0.8.20` (minimum satisfying)
/// - Gte `>=0.8.0`: install `0.8.0` (minimum satisfying)
/// - Range `>=0.6.2 <0.9.0`: install `0.6.2` (minimum satisfying)
fn version_to_install(constraint: &PragmaConstraint) -> Option<String> {
    match constraint {
        PragmaConstraint::Exact(v) => Some(v.to_string()),
        PragmaConstraint::Caret(v) => Some(v.to_string()),
        PragmaConstraint::Gte(v) => Some(v.to_string()),
        PragmaConstraint::Range(lower, _) => Some(lower.to_string()),
    }
}

/// Run `svm install {version}` to auto-install a solc version.
///
/// Returns `true` if the install succeeded.
async fn svm_install(version: &str) -> bool {
    let result = Command::new("svm")
        .arg("install")
        .arg(version)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .status()
        .await;

    matches!(result, Ok(status) if status.success())
}

/// All directories where solc versions may be installed.
///
/// Each entry is `(dir, binary_name_fn)` where `dir` contains version
/// subdirectories and `binary_name_fn` produces the binary filename.
///
/// Locations checked:
/// - svm-rs (forge): `{data_dir}/svm/{ver}/solc-{ver}` (platform-specific)
///   - macOS: `~/Library/Application Support/svm/`
///   - Linux: `~/.svm/`
///   - Windows: `%APPDATA%\svm\`
/// - solc-select: `~/.solc-select/artifacts/solc-{ver}/solc-{ver}`
fn svm_data_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(home) = home_dir() {
        // svm-rs: platform-specific data directory
        #[cfg(target_os = "macos")]
        dirs.push(home.join("Library/Application Support/svm"));

        #[cfg(target_os = "linux")]
        dirs.push(home.join(".svm"));

        #[cfg(target_os = "windows")]
        if let Some(appdata) = std::env::var_os("APPDATA") {
            dirs.push(PathBuf::from(appdata).join("svm"));
        }

        // Also check ~/.svm on all platforms (some setups use this)
        let dot_svm = home.join(".svm");
        if !dirs.contains(&dot_svm) {
            dirs.push(dot_svm);
        }

        // solc-select (pip-based, common on all Unix)
        dirs.push(home.join(".solc-select").join("artifacts"));
    }

    dirs
}

/// Look up a solc binary by version, checking all known install locations.
fn find_solc_binary(version: &str) -> Option<PathBuf> {
    for dir in svm_data_dirs() {
        // svm-rs layout: {dir}/{version}/solc-{version}
        let svm_path = dir.join(version).join(format!("solc-{version}"));
        if svm_path.is_file() {
            return Some(svm_path);
        }

        // solc-select layout: {dir}/solc-{version}/solc-{version}
        let select_path = dir
            .join(format!("solc-{version}"))
            .join(format!("solc-{version}"));
        if select_path.is_file() {
            return Some(select_path);
        }
    }

    None
}

/// Try to get the user's home directory (cross-platform).
fn home_dir() -> Option<PathBuf> {
    // $HOME works on macOS/Linux, USERPROFILE on Windows
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

// ── Pragma parsing ────────────────────────────────────────────────────────

/// A parsed semver version (major.minor.patch).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    fn parse(s: &str) -> Option<SemVer> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(SemVer {
            major: parts[0].parse().ok()?,
            minor: parts[1].parse().ok()?,
            patch: parts[2].parse().ok()?,
        })
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A version constraint from `pragma solidity`.
#[derive(Debug, Clone, PartialEq)]
pub enum PragmaConstraint {
    /// `0.8.26` — exact match
    Exact(SemVer),
    /// `^0.8.0` — same major.minor, patch >= specified
    /// Actually in Solidity: `^0.8.0` means `>=0.8.0 <0.9.0`
    Caret(SemVer),
    /// `>=0.8.0` — at least this version
    Gte(SemVer),
    /// `>=0.6.2 <0.9.0` — range
    Range(SemVer, SemVer),
}

/// Parse `pragma solidity <constraint>;` from Solidity source.
///
/// Handles:
/// - `pragma solidity 0.8.26;` → Exact
/// - `pragma solidity ^0.8.0;` → Caret
/// - `pragma solidity >=0.8.0;` → Gte
/// - `pragma solidity >=0.6.2 <0.9.0;` → Range
pub fn parse_pragma(source: &str) -> Option<PragmaConstraint> {
    // Find the pragma line — only scan the first ~20 lines for performance
    let pragma_line = source
        .lines()
        .take(20)
        .find(|line| line.trim_start().starts_with("pragma solidity"))?;

    // Extract the constraint string between "pragma solidity" and ";"
    let after_keyword = pragma_line
        .trim_start()
        .strip_prefix("pragma solidity")?
        .trim();
    let constraint_str = after_keyword
        .strip_suffix(';')
        .unwrap_or(after_keyword)
        .trim();

    if constraint_str.is_empty() {
        return None;
    }

    // Range: >=X.Y.Z <A.B.C
    if let Some(rest) = constraint_str.strip_prefix(">=") {
        let rest = rest.trim();
        if let Some(space_idx) = rest.find(|c: char| c.is_whitespace() || c == '<') {
            let lower_str = rest[..space_idx].trim();
            let upper_part = rest[space_idx..].trim();
            if let Some(upper_str) = upper_part.strip_prefix('<') {
                let upper_str = upper_str.trim();
                if let (Some(lower), Some(upper)) =
                    (SemVer::parse(lower_str), SemVer::parse(upper_str))
                {
                    return Some(PragmaConstraint::Range(lower, upper));
                }
            }
        }
        // Just >=X.Y.Z
        if let Some(ver) = SemVer::parse(rest) {
            return Some(PragmaConstraint::Gte(ver));
        }
    }

    // Caret: ^X.Y.Z
    if let Some(rest) = constraint_str.strip_prefix('^')
        && let Some(ver) = SemVer::parse(rest.trim())
    {
        return Some(PragmaConstraint::Caret(ver));
    }

    // Exact: X.Y.Z
    if let Some(ver) = SemVer::parse(constraint_str) {
        return Some(PragmaConstraint::Exact(ver));
    }

    None
}

/// List installed solc versions (cached — use `get_installed_versions()` internally).
pub fn list_installed_versions() -> Vec<SemVer> {
    get_installed_versions()
}

/// Scan the filesystem for installed solc versions from all known locations.
///
/// Returns sorted, deduplicated versions (ascending).
fn scan_installed_versions() -> Vec<SemVer> {
    let mut versions = Vec::new();

    for dir in svm_data_dirs() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = match entry.file_name().into_string() {
                    Ok(n) => n,
                    Err(_) => continue,
                };
                // svm-rs: directory named "0.8.26"
                // solc-select: directory named "solc-0.8.26"
                let version_str = name.strip_prefix("solc-").unwrap_or(&name);
                if let Some(ver) = SemVer::parse(version_str) {
                    versions.push(ver);
                }
            }
        }
    }

    versions.sort();
    versions.dedup();
    versions
}

/// Find the best matching installed version for a pragma constraint.
///
/// For all constraint types, picks the **latest** installed version that
/// satisfies the constraint.
pub fn find_matching_version(
    constraint: &PragmaConstraint,
    installed: &[SemVer],
) -> Option<SemVer> {
    let candidates: Vec<&SemVer> = installed
        .iter()
        .filter(|v| version_satisfies(v, constraint))
        .collect();

    // Pick the latest (last, since installed is sorted ascending)
    candidates.last().cloned().cloned()
}

/// Check if a version satisfies a pragma constraint.
pub fn version_satisfies(version: &SemVer, constraint: &PragmaConstraint) -> bool {
    match constraint {
        PragmaConstraint::Exact(v) => version == v,
        PragmaConstraint::Caret(v) => {
            // Solidity caret: ^0.8.0 means >=0.8.0 <0.9.0
            // i.e. same major, next minor is the ceiling
            version.major == v.major && version >= v && version.minor < v.minor + 1
        }
        PragmaConstraint::Gte(v) => version >= v,
        PragmaConstraint::Range(lower, upper) => version >= lower && version < upper,
    }
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

    if let Ok(output) = output
        && output.status.success()
    {
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
            if let Some(arr) = entries.as_array_mut()
                && let Some(first) = arr.first_mut()
                && let Some(sf) = first.get_mut("source_file")
            {
                normalized_sources.insert(path.clone(), sf.take());
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
                    if let Some(arr) = entries.as_array_mut()
                        && let Some(first) = arr.first_mut()
                        && let Some(contract) = first.get_mut("contract")
                    {
                        path_contracts.insert(name.clone(), contract.take());
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
/// This is the main entry point used by the LSP. Reads the file source
/// to detect the pragma version and resolve the correct solc binary.
pub async fn solc_ast(
    file_path: &str,
    config: &FoundryConfig,
    client: Option<&tower_lsp::Client>,
) -> Result<Value, RunnerError> {
    // Read source to detect pragma version
    let file_source = std::fs::read_to_string(file_path).ok();
    let solc_binary = resolve_solc_binary(config, file_source.as_deref(), client).await;
    let remappings = resolve_remappings(config).await;
    let input = build_standard_json_input(file_path, &remappings);
    let raw_output = run_solc(&solc_binary, &input, &config.root).await?;
    Ok(normalize_solc_output(raw_output))
}

/// Run solc for build diagnostics (same output, just used for error extraction).
pub async fn solc_build(
    file_path: &str,
    config: &FoundryConfig,
    client: Option<&tower_lsp::Client>,
) -> Result<Value, RunnerError> {
    solc_ast(file_path, config, client).await
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

    #[tokio::test]
    async fn test_resolve_solc_binary_default() {
        let config = FoundryConfig::default();
        let binary = resolve_solc_binary(&config, None, None).await;
        assert_eq!(binary, PathBuf::from("solc"));
    }

    #[test]
    fn test_parse_pragma_exact() {
        let source = "// SPDX\npragma solidity 0.8.26;\n";
        assert_eq!(
            parse_pragma(source),
            Some(PragmaConstraint::Exact(SemVer {
                major: 0,
                minor: 8,
                patch: 26
            }))
        );
    }

    #[test]
    fn test_parse_pragma_caret() {
        let source = "pragma solidity ^0.8.0;\n";
        assert_eq!(
            parse_pragma(source),
            Some(PragmaConstraint::Caret(SemVer {
                major: 0,
                minor: 8,
                patch: 0
            }))
        );
    }

    #[test]
    fn test_parse_pragma_gte() {
        let source = "pragma solidity >=0.8.0;\n";
        assert_eq!(
            parse_pragma(source),
            Some(PragmaConstraint::Gte(SemVer {
                major: 0,
                minor: 8,
                patch: 0
            }))
        );
    }

    #[test]
    fn test_parse_pragma_range() {
        let source = "pragma solidity >=0.6.2 <0.9.0;\n";
        assert_eq!(
            parse_pragma(source),
            Some(PragmaConstraint::Range(
                SemVer {
                    major: 0,
                    minor: 6,
                    patch: 2
                },
                SemVer {
                    major: 0,
                    minor: 9,
                    patch: 0
                },
            ))
        );
    }

    #[test]
    fn test_parse_pragma_none() {
        let source = "contract Foo {}\n";
        assert_eq!(parse_pragma(source), None);
    }

    #[test]
    fn test_version_satisfies_exact() {
        let v = SemVer {
            major: 0,
            minor: 8,
            patch: 26,
        };
        assert!(version_satisfies(&v, &PragmaConstraint::Exact(v.clone())));
        assert!(!version_satisfies(
            &SemVer {
                major: 0,
                minor: 8,
                patch: 25
            },
            &PragmaConstraint::Exact(v)
        ));
    }

    #[test]
    fn test_version_satisfies_caret() {
        let constraint = PragmaConstraint::Caret(SemVer {
            major: 0,
            minor: 8,
            patch: 0,
        });
        assert!(version_satisfies(
            &SemVer {
                major: 0,
                minor: 8,
                patch: 0
            },
            &constraint
        ));
        assert!(version_satisfies(
            &SemVer {
                major: 0,
                minor: 8,
                patch: 26
            },
            &constraint
        ));
        // 0.9.0 is outside ^0.8.0
        assert!(!version_satisfies(
            &SemVer {
                major: 0,
                minor: 9,
                patch: 0
            },
            &constraint
        ));
        // 0.7.0 is below
        assert!(!version_satisfies(
            &SemVer {
                major: 0,
                minor: 7,
                patch: 0
            },
            &constraint
        ));
    }

    #[test]
    fn test_version_satisfies_gte() {
        let constraint = PragmaConstraint::Gte(SemVer {
            major: 0,
            minor: 8,
            patch: 0,
        });
        assert!(version_satisfies(
            &SemVer {
                major: 0,
                minor: 8,
                patch: 0
            },
            &constraint
        ));
        assert!(version_satisfies(
            &SemVer {
                major: 0,
                minor: 9,
                patch: 0
            },
            &constraint
        ));
        assert!(!version_satisfies(
            &SemVer {
                major: 0,
                minor: 7,
                patch: 0
            },
            &constraint
        ));
    }

    #[test]
    fn test_version_satisfies_range() {
        let constraint = PragmaConstraint::Range(
            SemVer {
                major: 0,
                minor: 6,
                patch: 2,
            },
            SemVer {
                major: 0,
                minor: 9,
                patch: 0,
            },
        );
        assert!(version_satisfies(
            &SemVer {
                major: 0,
                minor: 6,
                patch: 2
            },
            &constraint
        ));
        assert!(version_satisfies(
            &SemVer {
                major: 0,
                minor: 8,
                patch: 26
            },
            &constraint
        ));
        // 0.9.0 is the upper bound (exclusive)
        assert!(!version_satisfies(
            &SemVer {
                major: 0,
                minor: 9,
                patch: 0
            },
            &constraint
        ));
        assert!(!version_satisfies(
            &SemVer {
                major: 0,
                minor: 6,
                patch: 1
            },
            &constraint
        ));
    }

    #[test]
    fn test_find_matching_version() {
        let installed = vec![
            SemVer {
                major: 0,
                minor: 8,
                patch: 0,
            },
            SemVer {
                major: 0,
                minor: 8,
                patch: 20,
            },
            SemVer {
                major: 0,
                minor: 8,
                patch: 26,
            },
            SemVer {
                major: 0,
                minor: 8,
                patch: 33,
            },
        ];
        // ^0.8.20 should pick latest: 0.8.33
        let constraint = PragmaConstraint::Caret(SemVer {
            major: 0,
            minor: 8,
            patch: 20,
        });
        let matched = find_matching_version(&constraint, &installed);
        assert_eq!(
            matched,
            Some(SemVer {
                major: 0,
                minor: 8,
                patch: 33
            })
        );

        // exact 0.8.20
        let constraint = PragmaConstraint::Exact(SemVer {
            major: 0,
            minor: 8,
            patch: 20,
        });
        let matched = find_matching_version(&constraint, &installed);
        assert_eq!(
            matched,
            Some(SemVer {
                major: 0,
                minor: 8,
                patch: 20
            })
        );

        // exact 0.8.15 — not installed
        let constraint = PragmaConstraint::Exact(SemVer {
            major: 0,
            minor: 8,
            patch: 15,
        });
        let matched = find_matching_version(&constraint, &installed);
        assert_eq!(matched, None);
    }

    #[test]
    fn test_list_installed_versions() {
        // Just verify it doesn't panic — actual versions depend on system
        let versions = list_installed_versions();
        // Versions should be sorted
        for w in versions.windows(2) {
            assert!(w[0] <= w[1]);
        }
    }
}
