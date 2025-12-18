use solidity_language_server::lint::lint_output_to_diagnostics;
use solidity_language_server::runner::{ForgeRunner, Runner};
use std::fs;

static CONTRACT: &str = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

contract A {
    function add_num(uint256 a) public pure returns (uint256) {
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
async fn test_lint_valid_file() {
    let (_temp_dir, contract_path, compiler) = setup(CONTRACT);
    let file_path = contract_path.to_string_lossy().to_string();

    let result = compiler.lint(&file_path).await;
    assert!(result.is_ok(), "Expected lint to succeed");

    let json_value = result.unwrap();
    assert!(json_value.is_array(), "Expected lint output to be an array");
}

#[tokio::test]
async fn test_lint_diagnosis_output() {
    let (_temp_dir, contract_path, compiler) = setup(CONTRACT);
    let file_path = contract_path.to_string_lossy().to_string();

    let result = compiler.lint(&file_path).await;
    assert!(result.is_ok());

    let json_value = result.unwrap();
    let diagnostics = lint_output_to_diagnostics(&json_value, &file_path);
    assert!(!diagnostics.is_empty(), "Expected diagnostics");
}

#[tokio::test]
async fn test_lint_to_lsp_diagnostics() {
    let (_temp_dir, contract_path, compiler) = setup(CONTRACT);
    let file_path = contract_path.to_string_lossy().to_string();

    let result = compiler.lint(&file_path).await;
    assert!(result.is_ok(), "Expected lint to succeed");

    let json_value = result.unwrap();
    let diagnostics = lint_output_to_diagnostics(&json_value, &file_path);
    assert!(!diagnostics.is_empty(), "Expected at least one diagnostic");

    let first_diag = &diagnostics[0];
    assert_eq!(first_diag.source, Some("forge-lint".to_string()));
    assert_eq!(
        first_diag.message,
        "[forge lint] function names should use mixedCase"
    );
    assert_eq!(
        first_diag.severity,
        Some(tower_lsp::lsp_types::DiagnosticSeverity::INFORMATION)
    );
    assert_eq!(first_diag.range.start.line, 4);
    assert_eq!(first_diag.range.start.character, 13);
}
