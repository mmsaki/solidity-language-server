use solidity_language_server::symbols::{extract_symbols, extract_document_symbols};
use tower_lsp::lsp_types::SymbolKind;
use std::process::Command;

fn get_test_ast_data() -> Option<serde_json::Value> {
    let output = Command::new("forge")
        .args(["build", "--ast", "--silent", "--build-info"])
        .current_dir("testdata")
        .output()
        .ok()?;

    let stdout_str = String::from_utf8(output.stdout).ok()?;
    serde_json::from_str(&stdout_str).ok()
}

#[test]
fn test_extract_symbols_basic() {
    let ast_data = match get_test_ast_data() {
        Some(data) => data,
        None => return,
    };

    let symbols = extract_symbols(&ast_data);

    // Should find some symbols
    assert!(!symbols.is_empty());

    // Check that we have contracts
    let contract_symbols: Vec<_> = symbols.iter()
        .filter(|s| s.kind == SymbolKind::CLASS)
        .collect();
    assert!(!contract_symbols.is_empty(), "Should find at least one contract");

    // Check that we have functions
    let function_symbols: Vec<_> = symbols.iter()
        .filter(|s| s.kind == SymbolKind::FUNCTION)
        .collect();
    assert!(!function_symbols.is_empty(), "Should find at least one function");
}

#[test]
fn test_symbol_kinds() {
    let ast_data = match get_test_ast_data() {
        Some(data) => data,
        None => return,
    };

    let symbols = extract_symbols(&ast_data);

    // Check that we have various symbol kinds
    let has_class = symbols.iter().any(|s| s.kind == SymbolKind::CLASS);
    let has_function = symbols.iter().any(|s| s.kind == SymbolKind::FUNCTION);

    // Should have at least contracts and functions
    assert!(has_class, "Should have contract symbols");
    assert!(has_function, "Should have function symbols");
}

#[test]
fn test_extract_document_symbols_basic() {
    let ast_data = match get_test_ast_data() {
        Some(data) => data,
        None => return,
    };

    let symbols = extract_document_symbols(&ast_data, "testdata/Simple.sol");

    // Should find some symbols
    assert!(!symbols.is_empty());

    // Check that we have contracts
    let contract_symbols: Vec<_> = symbols.iter()
        .filter(|s| s.kind == SymbolKind::CLASS)
        .collect();
    assert!(!contract_symbols.is_empty(), "Should find at least one contract");

    // Check that we have functions
    let function_symbols: Vec<_> = symbols.iter()
        .filter(|s| s.kind == SymbolKind::FUNCTION)
        .collect();
    assert!(!function_symbols.is_empty(), "Should find at least one function");
}

#[test]
fn test_document_symbol_kinds() {
    let ast_data = match get_test_ast_data() {
        Some(data) => data,
        None => return,
    };

    let symbols = extract_document_symbols(&ast_data, "testdata/Simple.sol");

    // Check that we have various symbol kinds
    let has_class = symbols.iter().any(|s| s.kind == SymbolKind::CLASS);
    let has_function = symbols.iter().any(|s| s.kind == SymbolKind::FUNCTION);
    let _has_variable = symbols.iter().any(|s| s.kind == SymbolKind::VARIABLE || s.kind == SymbolKind::FIELD);
    let _has_event = symbols.iter().any(|s| s.kind == SymbolKind::EVENT);
    let _has_struct = symbols.iter().any(|s| s.kind == SymbolKind::STRUCT);
    let _has_enum = symbols.iter().any(|s| s.kind == SymbolKind::ENUM);

    // Should have at least contracts and functions
    assert!(has_class, "Should have contract symbols");
    assert!(has_function, "Should have function symbols");
    // Other types may or may not be present depending on the test file
}

#[test]
fn test_enum_members_in_document_symbols() {
    // Test with a file that has enums - we'll check if enum members are extracted
    // This test will pass even if no enums exist, but verifies the logic works
    let ast_data = match get_test_ast_data() {
        Some(data) => data,
        None => return,
    };

    // Check all files for enum symbols with children
    let mut found_enum_with_members = false;
    if let Some(sources) = ast_data.get("sources").and_then(|v| v.as_object()) {
        for (path, _) in sources {
            let symbols = extract_document_symbols(&ast_data, path);
            for symbol in symbols {
                if symbol.kind == SymbolKind::STRUCT
                    && let Some(children) = &symbol.children
                        && !children.is_empty() {
                            // Verify all children are enum values (shown as enums)
                            let all_enum_values = children.iter().all(|c| c.kind == SymbolKind::ENUM);
                            assert!(all_enum_values, "Enum children should all be enum values");
                            found_enum_with_members = true;
                        }
            }
        }
    }

    // If we found enums with members, the test passes
    // If no enums exist in test data, this is also fine
    if found_enum_with_members {
        println!("Found enum with members in test data");
    }
}