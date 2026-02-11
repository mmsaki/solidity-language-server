use solidity_language_server::lint::lint_output_to_diagnostics;

/// Inline forge lint JSON output for the contract:
/// ```solidity
/// // SPDX-License-Identifier: MIT
/// pragma solidity ^0.8.29;
///
/// contract A {
///     function add_num(uint256 a) public pure returns (uint256) {
///         return a + 4;
///     }
/// }
/// ```
///
/// Produced by: `forge lint src/Contract.sol --json 2>&1`
static LINT_OUTPUT: &str = r#"{"$message_type":"diagnostic","message":"function names should use mixedCase","code":{"code":"mixed-case-function","explanation":null},"level":"note","spans":[{"file_name":"src/Contract.sol","byte_start":84,"byte_end":91,"line_start":5,"line_end":5,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    function add_num(uint256 a) public pure returns (uint256) {","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null}],"children":[{"message":"https://book.getfoundry.sh/reference/forge/forge-lint#mixed-case-function","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"src/Contract.sol","byte_start":84,"byte_end":91,"line_start":5,"line_end":5,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    function add_num(uint256 a) public pure returns (uint256) {","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":"addNum"}],"children":[],"rendered":null}],"rendered":"note[mixed-case-function]: function names should use mixedCase\n --> src/Contract.sol:5:14\n  |\n5 |     function add_num(uint256 a) public pure returns (uint256) {\n  |              ^^^^^^^ help: consider using: `addNum`\n  |\n  = help: https://book.getfoundry.sh/reference/forge/forge-lint#mixed-case-function\n\n"}"#;

fn load_lint_output() -> serde_json::Value {
    let diag: serde_json::Value = serde_json::from_str(LINT_OUTPUT).unwrap();
    serde_json::Value::Array(vec![diag])
}

#[test]
fn test_lint_output_parses_as_array() {
    let json_value = load_lint_output();
    assert!(json_value.is_array(), "Expected lint output to be an array");
    assert_eq!(json_value.as_array().unwrap().len(), 1);
}

#[test]
fn test_lint_diagnosis_output() {
    let json_value = load_lint_output();
    let diagnostics = lint_output_to_diagnostics(&json_value, "src/Contract.sol");
    assert!(!diagnostics.is_empty(), "Expected diagnostics");
}

#[test]
fn test_lint_to_lsp_diagnostics() {
    let json_value = load_lint_output();
    let diagnostics = lint_output_to_diagnostics(&json_value, "src/Contract.sol");
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
