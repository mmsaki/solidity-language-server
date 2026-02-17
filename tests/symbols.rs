use solidity_language_server::symbols::{extract_document_symbols, extract_workspace_symbols};
use tower_lsp::lsp_types::{SymbolKind, Url};

#[test]
fn test_extract_symbols_basic() {
    let source = r#"
pragma solidity ^0.8.0;

contract Counter {
    uint256 public count;

    function increment() public {
        count += 1;
    }

    function getCount() public view returns (uint256) {
        return count;
    }
}
"#;
    let uri = Url::parse("file:///test/Counter.sol").unwrap();
    let symbols = extract_workspace_symbols(&[(uri, source.to_string())]);

    assert!(!symbols.is_empty());

    // Check that no symbols have empty names
    let symbols_with_names: Vec<_> = symbols.iter().filter(|s| !s.name.is_empty()).collect();
    assert!(
        !symbols_with_names.is_empty(),
        "Should have symbols with non-empty names"
    );

    // Check that function symbols exist
    let function_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| s.kind == SymbolKind::FUNCTION)
        .collect();
    assert!(!function_symbols.is_empty(), "Should have function symbols");

    // Check that we have contracts
    let contract_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| s.kind == SymbolKind::CLASS)
        .collect();
    assert!(
        !contract_symbols.is_empty(),
        "Should find at least one contract"
    );
}

#[test]
fn test_symbol_kinds() {
    let source = r#"
pragma solidity ^0.8.0;

contract Token {
    uint256 public totalSupply;
    event Transfer(address from, address to, uint256 amount);
    function transfer(address to, uint256 amount) external {}
}
"#;
    let uri = Url::parse("file:///test/Token.sol").unwrap();
    let symbols = extract_workspace_symbols(&[(uri, source.to_string())]);

    let has_class = symbols.iter().any(|s| s.kind == SymbolKind::CLASS);
    let has_function = symbols.iter().any(|s| s.kind == SymbolKind::FUNCTION);

    assert!(has_class, "Should have contract symbols");
    assert!(has_function, "Should have function symbols");
}

#[test]
fn test_extract_document_symbols_basic() {
    let source = r#"
pragma solidity ^0.8.0;

contract Simple {
    uint256 public value;

    function setValue(uint256 _value) public {
        value = _value;
    }

    function getValue() public view returns (uint256) {
        return value;
    }
}
"#;
    let symbols = extract_document_symbols(source);

    assert!(!symbols.is_empty());

    let contract_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| s.kind == SymbolKind::CLASS)
        .collect();
    assert!(
        !contract_symbols.is_empty(),
        "Should find at least one contract"
    );

    let contract = &contract_symbols[0];
    let children = contract
        .children
        .as_ref()
        .expect("contract should have children");

    let function_children: Vec<_> = children
        .iter()
        .filter(|c| c.kind == SymbolKind::FUNCTION)
        .collect();
    assert!(
        !function_children.is_empty(),
        "Should find at least one function"
    );
}

#[test]
fn test_document_symbol_kinds() {
    let source = r#"
pragma solidity ^0.8.0;

contract Mixed {
    uint256 public x;
    event Logged(uint256 val);
    error BadValue();
    struct Info { string name; uint256 id; }
    enum Status { Active, Paused }

    function doSomething() public {}
}
"#;
    let symbols = extract_document_symbols(source);

    let contract = symbols
        .iter()
        .find(|s| s.kind == SymbolKind::CLASS)
        .expect("Should have contract");
    let children = contract.children.as_ref().unwrap();

    let has_function = children.iter().any(|c| c.kind == SymbolKind::FUNCTION);
    let has_field = children.iter().any(|c| c.kind == SymbolKind::FIELD);
    let has_event = children.iter().any(|c| c.kind == SymbolKind::EVENT);
    let has_struct = children.iter().any(|c| c.kind == SymbolKind::STRUCT);
    let has_enum = children.iter().any(|c| c.kind == SymbolKind::ENUM);

    assert!(has_function, "Should have function symbols");
    assert!(has_field, "Should have field symbols");
    assert!(has_event, "Should have event symbols");
    assert!(has_struct, "Should have struct symbols");
    assert!(has_enum, "Should have enum symbols");
}

#[test]
fn test_enum_members_in_document_symbols() {
    let source = r#"
contract Foo {
    enum Color { Red, Green, Blue }
}
"#;
    let symbols = extract_document_symbols(source);
    let contract = &symbols[0];
    let children = contract.children.as_ref().unwrap();

    let enum_sym = children
        .iter()
        .find(|c| c.kind == SymbolKind::ENUM)
        .expect("Should have enum");
    assert_eq!(enum_sym.name, "Color");

    let members = enum_sym
        .children
        .as_ref()
        .expect("Enum should have members");
    assert_eq!(members.len(), 3);
    assert!(members.iter().all(|m| m.kind == SymbolKind::ENUM_MEMBER));
    assert!(members.iter().any(|m| m.name == "Red"));
    assert!(members.iter().any(|m| m.name == "Green"));
    assert!(members.iter().any(|m| m.name == "Blue"));
}

#[test]
fn test_container_names() {
    let source = r#"
contract MyContract {
    uint256 public x;
    function foo() public {}
    struct Bar { uint256 val; }
}
"#;
    let uri = Url::parse("file:///test.sol").unwrap();
    let symbols = extract_workspace_symbols(&[(uri, source.to_string())]);

    let symbols_with_container: Vec<_> = symbols
        .iter()
        .filter(|s| s.container_name.is_some())
        .collect();

    assert!(
        !symbols_with_container.is_empty(),
        "Should find symbols with container_name set"
    );

    // Function and field should have MyContract as container
    assert!(
        symbols
            .iter()
            .any(|s| s.name == "foo" && s.container_name.as_deref() == Some("MyContract"))
    );
    assert!(
        symbols
            .iter()
            .any(|s| s.name == "x" && s.container_name.as_deref() == Some("MyContract"))
    );

    // Struct members should have Bar as container
    let struct_members: Vec<_> = symbols
        .iter()
        .filter(|s| s.kind == SymbolKind::FIELD && s.container_name.as_deref() == Some("Bar"))
        .collect();
    assert!(
        !struct_members.is_empty(),
        "Struct members should have container name"
    );
}
