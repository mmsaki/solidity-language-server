use solidity_language_server::build::{build_output_to_diagnostics, ignored_error_code_warning};
use solidity_language_server::runner::{ForgeRunner, Runner};
use solidity_language_server::utils::byte_offset_to_position;
use std::fs;
use tower_lsp::lsp_types::{DiagnosticSeverity, NumberOrString};

static CONTRACT: &str = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

contract A {
    using B for string;

    function() internal c;

    function add_num(uint256 a) public pure returns (uint256) {
        bool fad;
        return a + 4;
    }
}"#;

fn setup(contents: &str) -> (tempfile::TempDir, std::path::PathBuf, ForgeRunner) {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");

    // Create src directory
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).expect("failed to create src dir");

    // Write foundry.toml
    let foundry_toml = r#"[profile.default]
src = "src"
out = "out"
libs = ["lib"]
"#;
    fs::write(temp_dir.path().join("foundry.toml"), foundry_toml)
        .expect("failed to write foundry.toml");

    let contract_path = src_dir.join("Contract.sol");
    fs::write(&contract_path, contents).expect("failed to write contract");

    let compiler = ForgeRunner;
    (temp_dir, contract_path, compiler)
}

#[tokio::test]
async fn test_build_success() {
    let (temp_dir, _contract_path, compiler) = setup(CONTRACT);
    let file_path = temp_dir.path().to_string_lossy().to_string();

    let result = compiler.build(&file_path).await;
    assert!(result.is_ok(), "Expected build to succeed");
}

#[tokio::test]
async fn test_build_has_errors_array() {
    let (temp_dir, _contract_path, compiler) = setup(CONTRACT);
    let file_path = temp_dir.path().to_string_lossy().to_string();

    let json = compiler.build(&file_path).await.unwrap();
    assert!(
        json.get("errors").is_some(),
        "Expected 'errors' array in build output"
    );
}

#[tokio::test]
async fn test_build_error_formatting() {
    let (temp_dir, _contract_path, compiler) = setup(CONTRACT);
    let file_path = temp_dir.path().to_string_lossy().to_string();

    let json = compiler.build(&file_path).await.unwrap();
    if let Some(errors) = json.get("errors")
        && let Some(first) = errors.get(0)
    {
        assert!(
            first.get("message").is_some(),
            "Expected error object to have a message"
        );
    }
}

#[tokio::test]
async fn test_diagnostic_offsets_match_source() {
    let (temp_dir, contract_path, compiler) = setup(CONTRACT);
    let file_path = temp_dir.path().to_string_lossy().to_string();
    let source_code = tokio::fs::read_to_string(&contract_path)
        .await
        .expect("read source");
    let build_output = compiler.build(&file_path).await.expect("build failed");
    let expected_start_byte = 81;
    let expected_end_byte = 82;
    let expected_start_pos = byte_offset_to_position(&source_code, expected_start_byte);
    let expected_end_pos = byte_offset_to_position(&source_code, expected_end_byte);
    let filename = std::path::Path::new(&contract_path)
        .file_name()
        .and_then(|f| f.to_str())
        .expect("filename");
    let diagnostics = build_output_to_diagnostics(&build_output, filename, &source_code);
    assert!(!diagnostics.is_empty(), "no diagnostics found");

    let diag = &diagnostics[0];
    assert_eq!(diag.range.start.line, expected_start_pos.0);
    assert_eq!(diag.range.start.character, expected_start_pos.1);
    assert_eq!(diag.range.end.line, expected_end_pos.0);
    assert_eq!(diag.range.end.character, expected_end_pos.1);
}

#[tokio::test]
async fn test_build_output_to_diagnostics_from_file() {
    let (temp_dir, contract_path, compiler) = setup(CONTRACT);
    let file_path = temp_dir.path().to_string_lossy().to_string();
    let source_code = tokio::fs::read_to_string(&contract_path)
        .await
        .expect("Failed to read source file");
    let build_output = compiler
        .build(&file_path)
        .await
        .expect("Compiler build failed");
    let filename = std::path::Path::new(&contract_path)
        .file_name()
        .and_then(|f| f.to_str())
        .expect("Failed to get filename");

    let diagnostics = build_output_to_diagnostics(&build_output, filename, &source_code);
    assert!(!diagnostics.is_empty(), "Expected at least one diagnostic");

    let diag = &diagnostics[0];
    assert_eq!(diag.severity, Some(DiagnosticSeverity::ERROR));
    assert!(diag.message.contains("Identifier is not a library name"));
    assert_eq!(diag.code, Some(NumberOrString::String("9589".to_string())));
    assert!(diag.range.start.line > 0);
    assert!(diag.range.start.character > 0);
}

#[tokio::test]
async fn test_ignored_code_for_tests() {
    let error_json = serde_json::json!({
        "errorCode": "5574",
        "sourceLocation": {
            "file": "test/ERC6909Claims.t.sol"
        }
    });
    assert!(ignored_error_code_warning(&error_json));

    let error_json_non_test = serde_json::json!({
        "errorCode": "5574",
        "sourceLocation": {
            "file": "contracts/ERC6909Claims.sol"
        }
    });
    // These codes are now ignored for all .sol files, not just test files
    assert!(ignored_error_code_warning(&error_json_non_test));

    let error_json_other_code = serde_json::json!({
        "errorCode": "1234",
        "sourceLocation": {
            "file": "test/ERC6909Claims.t.sol"
        }
    });
    assert!(!ignored_error_code_warning(&error_json_other_code));
}
