#![allow(deprecated)]

use serde_json::Value;
use tower_lsp::lsp_types::{DocumentSymbol, Location, Range, SymbolInformation, SymbolKind, Url, Position};
use crate::utils::byte_offset_to_position;

pub fn extract_symbols(ast_data: &Value) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    let mut seen = std::collections::HashSet::new();

    if let Some(sources) = ast_data.get("sources")
        && let Some(sources_obj) = sources.as_object() {
            for (path, contents) in sources_obj {
                if let Some(contents_array) = contents.as_array()
                    && let Some(first_content) = contents_array.first()
                        && let Some(source_file) = first_content.get("source_file")
                            && let Some(ast) = source_file.get("ast") {
                                let file_symbols = extract_symbols_from_ast(ast, path);
                                for symbol in file_symbols {
                                    // Deduplicate based on location (URI + range)
                                    let key = format!("{}:{:?}:{:?}",
                                        symbol.location.uri,
                                        symbol.location.range.start,
                                        symbol.location.range.end
                                    );
                                     if seen.insert(key) {
                                         symbols.push(symbol);
                                     }
                                 }
                             }
             }
         }

     symbols
}

pub fn extract_document_symbols(ast_data: &Value, file_path: &str) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();

    if let Some(sources) = ast_data.get("sources")
        && let Some(sources_obj) = sources.as_object() {
            for (path, contents) in sources_obj {
                if (path == file_path || path.ends_with(&format!("/{}", file_path)) || path.ends_with(file_path))
                    && let Some(contents_array) = contents.as_array()
                    && let Some(first_content) = contents_array.first()
                    && let Some(source_file) = first_content.get("source_file")
                    && let Some(ast) = source_file.get("ast") {
                        let file_symbols = extract_document_symbols_from_ast(ast, file_path);
                        symbols.extend(file_symbols);
                    }
            }
        }

    symbols
}

fn extract_document_symbols_from_ast(ast: &Value, file_path: &str) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();

    // First, find all top-level nodes (contracts, interfaces, libraries, etc.)
    if let Some(nodes) = ast.get("nodes").and_then(|v| v.as_array()) {
        for node in nodes {
            if let Some(node_type) = node.get("nodeType").and_then(|v| v.as_str()) {
                match node_type {
                    "ContractDefinition" | "InterfaceDefinition" | "LibraryDefinition" => {
                        if let Some(symbol) = create_contract_document_symbol_with_children(node, file_path) {
                            symbols.push(symbol);
                        }
                    }
                    "UsingForDirective" => {
                        if let Some(symbol) = create_using_for_document_symbol(node, file_path) {
                            symbols.push(symbol);
                        }
                    }
                    "ImportDirective" => {
                        if let Some(symbol) = create_import_document_symbol(node, file_path) {
                            symbols.push(symbol);
                        }
                    }
                    "PragmaDirective" => {
                        if let Some(symbol) = create_pragma_document_symbol(node, file_path) {
                            symbols.push(symbol);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    symbols
}

fn create_contract_document_symbol_with_children(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let mut children = Vec::new();

    // Process contract members
    if let Some(nodes) = node.get("nodes").and_then(|v| v.as_array()) {
        for member_node in nodes {
            if let Some(node_type) = member_node.get("nodeType").and_then(|v| v.as_str()) {
                match node_type {
                    "FunctionDefinition" => {
                        if let Some(symbol) = create_function_document_symbol_with_children(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "VariableDeclaration" => {
                        if let Some(symbol) = create_variable_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "EventDefinition" => {
                        if let Some(symbol) = create_event_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "ModifierDefinition" => {
                        if let Some(symbol) = create_modifier_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "StructDefinition" => {
                        if let Some(symbol) = create_struct_document_symbol_with_children(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "EnumDefinition" => {
                        if let Some(symbol) = create_enum_document_symbol_with_children(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "ConstructorDefinition" => {
                        if let Some(symbol) = create_constructor_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "ErrorDefinition" => {
                        if let Some(symbol) = create_error_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "UsingForDirective" => {
                        if let Some(symbol) = create_using_for_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "FallbackFunctionDefinition" => {
                        if let Some(symbol) = create_fallback_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "ReceiveFunctionDefinition" => {
                        if let Some(symbol) = create_receive_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::CLASS,
        range,
        selection_range: range,
        children: if children.is_empty() { None } else { Some(children) },
        tags: None,
        deprecated: None,
    })
}



fn create_function_document_symbol_with_children(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let range = get_node_range(node, file_path)?;
    let is_constructor = node.get("kind").and_then(|v| v.as_str()) == Some("constructor");

    let name = if is_constructor {
        "constructor".to_string()
    } else {
        match node.get("name").and_then(|v| v.as_str()) {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => return None, // Skip functions with no name
        }
    };

    let kind = if is_constructor {
        SymbolKind::CONSTRUCTOR
    } else {
        SymbolKind::FUNCTION
    };

    // Extract parameters as children
    let mut children = Vec::new();

    // Try different AST structures for parameters
    let param_array = node.get("parameters").and_then(|p| p.get("parameters")).and_then(|p| p.as_array())
        .or_else(|| node.get("parameters").and_then(|p| p.as_array()));

    if let Some(parameters) = param_array {
        for param in parameters {
            if let Some(param_symbol) = create_parameter_document_symbol(param, file_path) {
                children.push(param_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name,
        detail: None,
        kind,
        range,
        selection_range: range,
        children: if children.is_empty() { None } else { Some(children) },
        tags: None,
        deprecated: None,
    })
}

fn create_variable_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    // Determine if this is a state variable or local variable
    let kind = if is_state_variable(node) {
        SymbolKind::FIELD
    } else {
        SymbolKind::VARIABLE
    };

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_event_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::EVENT,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_modifier_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::METHOD, // Modifiers are represented as methods
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_struct_document_symbol_with_children(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    // Extract struct members as children
    let mut children = Vec::new();
    if let Some(members) = node.get("members").and_then(|v| v.as_array()) {
        for member in members {
            if let Some(member_symbol) = create_struct_member_document_symbol(member, file_path) {
                children.push(member_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::STRUCT,
        range,
        selection_range: range,
        children: if children.is_empty() { None } else { Some(children) },
        tags: None,
        deprecated: None,
    })
}

fn create_struct_member_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::FIELD,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_enum_document_symbol_with_children(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    // Extract enum members as children
    let mut children = Vec::new();
    if let Some(members) = node.get("members").and_then(|v| v.as_array()) {
        for member in members {
            if let Some(member_symbol) = create_enum_member_document_symbol(member, file_path) {
                children.push(member_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::STRUCT,
        range,
        selection_range: range,
        children: if children.is_empty() { None } else { Some(children) },
        tags: None,
        deprecated: None,
    })
}

fn create_enum_member_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::ENUM,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_constructor_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: "constructor".to_string(),
        detail: None,
        kind: SymbolKind::CONSTRUCTOR,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_error_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::EVENT, // Errors are similar to events in Solidity
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_fallback_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: "fallback".to_string(),
        detail: None,
        kind: SymbolKind::FUNCTION,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_receive_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: "receive".to_string(),
        detail: None,
        kind: SymbolKind::FUNCTION,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_parameter_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    // Skip unnamed parameters
    if name.is_empty() {
        return None;
    }

    let range = get_node_range(node, file_path)?;

    Some(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::VARIABLE,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}



fn create_using_for_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let range = get_node_range(node, file_path)?;

    // Build the name from the AST data
    let mut name_parts = Vec::new();
    name_parts.push("using".to_string());

    // Add library name if present
    if let Some(library_name) = node.get("libraryName")
        && let Some(id) = library_name.get("name").and_then(|v| v.as_str()) {
            name_parts.push(id.to_string());
        }

    name_parts.push("for".to_string());

    // Add type name if present
    if let Some(type_name) = node.get("typeName")
        && let Some(name_str) = extract_type_name(type_name) {
            name_parts.push(name_str);
        }

    let name = name_parts.join(" ");

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: SymbolKind::PROPERTY, // Using directives are properties/attributes
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn extract_type_name(type_node: &Value) -> Option<String> {
    if let Some(node_type) = type_node.get("nodeType").and_then(|v| v.as_str()) {
        match node_type {
            "ElementaryTypeName" => {
                type_node.get("name").and_then(|v| v.as_str()).map(|s| s.to_string())
            }
            "UserDefinedTypeName" => {
                type_node.get("name").and_then(|v| v.as_str()).map(|s| s.to_string())
            }
            "Mapping" => {
                Some("mapping".to_string())
            }
            _ => None
        }
    } else {
        None
    }
}

fn create_import_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let range = get_node_range(node, file_path)?;

    // Try to get the file name being imported
    let name = if let Some(file) = node.get("file").and_then(|v| v.as_str()) {
        format!("import {}", file)
    } else {
        "import".to_string()
    };

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: SymbolKind::MODULE,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_pragma_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let range = get_node_range(node, file_path)?;

    // Extract a clean pragma name
    let name = if let Some(literals) = node.get("literals").and_then(|v| v.as_array()) {
        let parts: Vec<String> = literals.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.trim().to_string()) // Trim spaces from each part
            .collect();

        if parts.len() >= 2 && parts[0] == "solidity" {
            // For solidity pragmas, join all version parts without spaces
            // e.g., ["solidity", "^0.8.0"] -> "solidity ^0.8.0"
            // or ["solidity", ">=", "0.8.0", "<", "0.9.0"] -> "solidity >=0.8.0<0.9.0"
            let version_parts: Vec<String> = parts[1..].to_vec();
            format!("{} {}", parts[0], version_parts.join(""))
        } else {
            // For other pragmas, show the joined text
            format!("pragma{}", parts.join(""))
        }
    } else {
        "pragma".to_string()
    };

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: SymbolKind::STRING, // Pragma directives are like string literals
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn extract_symbols_from_ast(ast: &Value, file_path: &str) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    let mut stack = vec![ast];

    while let Some(node) = stack.pop() {
        if let Some(node_type) = node.get("nodeType").and_then(|v| v.as_str()) {
            match node_type {
                "ContractDefinition" => {
                    if let Some(symbol) = create_contract_symbol_info(node, file_path) {
                        symbols.push(symbol);
                    }
                }
                "FunctionDefinition" => {
                    if let Some(symbol) = create_function_symbol_info(node, file_path) {
                        symbols.push(symbol);
                    }
                }
                "VariableDeclaration" => {
                    if let Some(symbol) = create_variable_symbol_info(node, file_path) {
                        symbols.push(symbol);
                    }
                }
                "EventDefinition" => {
                    if let Some(symbol) = create_event_symbol_info(node, file_path) {
                        symbols.push(symbol);
                    }
                }
                "ModifierDefinition" => {
                    if let Some(symbol) = create_modifier_symbol_info(node, file_path) {
                        symbols.push(symbol);
                    }
                }
                "StructDefinition" => {
                    if let Some(symbol) = create_struct_symbol_info(node, file_path) {
                        symbols.push(symbol);
                    }
                }
                "EnumDefinition" => {
                    if let Some(symbol) = create_enum_symbol_info(node, file_path) {
                        symbols.push(symbol);
                    }
                }
                _ => {}
            }
        }

        // Add child nodes to stack
        push_child_nodes(node, &mut stack);
    }

    symbols
}

fn create_contract_symbol_info(node: &Value, file_path: &str) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind: SymbolKind::CLASS,
        tags: None,
        deprecated: None,
        location,
        container_name: None,
    })
}

fn create_function_symbol_info(node: &Value, file_path: &str) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    // Skip constructors (they have empty name in some AST versions)
    if name.is_empty() {
        return None;
    }

    let kind = if node.get("kind").and_then(|v| v.as_str()) == Some("constructor") {
        SymbolKind::CONSTRUCTOR
    } else {
        SymbolKind::FUNCTION
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind,
        tags: None,
        deprecated: None,
        location,
        container_name: None,
    })
}

fn create_variable_symbol_info(node: &Value, file_path: &str) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    // Determine if this is a state variable or local variable
    let kind = if is_state_variable(node) {
        SymbolKind::FIELD
    } else {
        SymbolKind::VARIABLE
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind,
        tags: None,
        deprecated: None,
        location,
        container_name: None,
    })
}

fn create_event_symbol_info(node: &Value, file_path: &str) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind: SymbolKind::EVENT,
        tags: None,
        deprecated: None,
        location,
        container_name: None,
    })
}

fn create_modifier_symbol_info(node: &Value, file_path: &str) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind: SymbolKind::METHOD, // Modifiers are represented as methods
        tags: None,
        deprecated: None,
        location,
        container_name: None,
    })
}

fn create_struct_symbol_info(node: &Value, file_path: &str) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind: SymbolKind::STRUCT,
        tags: None,
        deprecated: None,
        location,
        container_name: None,
    })
}

fn create_enum_symbol_info(node: &Value, file_path: &str) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind: SymbolKind::STRUCT,
        tags: None,
        deprecated: None,
        location,
        container_name: None,
    })
}

fn get_node_range(node: &Value, file_path: &str) -> Option<Range> {
    let src = node.get("src").and_then(|v| v.as_str())?;
    let parts: Vec<&str> = src.split(':').collect();
    if parts.len() < 3 {
        return None;
    }

    let start_offset: usize = parts[0].parse().ok()?;
    let length: usize = parts[1].parse().ok()?;

    // Read the file content to convert byte offsets to positions
    let content = std::fs::read_to_string(file_path).ok()?;
    let (start_line, start_col) = byte_offset_to_position(&content, start_offset);
    let (end_line, end_col) = byte_offset_to_position(&content, start_offset + length);

    Some(Range {
        start: Position { line: start_line, character: start_col },
        end: Position { line: end_line, character: end_col },
    })
}

fn push_child_nodes<'a>(node: &'a Value, stack: &mut Vec<&'a Value>) {
    if let Some(children) = node.as_object() {
        for value in children.values() {
            match value {
                Value::Array(arr) => {
                    for item in arr {
                        stack.push(item);
                    }
                }
                Value::Object(_) => {
                    stack.push(value);
                }
                _ => {}
            }
        }
    }
}

fn is_state_variable(node: &Value) -> bool {
    // A variable is a state variable if it's not inside a function body
    // We can check this by walking up the AST to see if we're inside a function
    let mut current = node;
    while let Some(parent) = current.get("parent") {
        if let Some(node_type) = parent.get("nodeType").and_then(|v| v.as_str())
            && (node_type == "FunctionDefinition" || node_type == "ModifierDefinition") {
                return false; // Inside a function or modifier, so it's local
            }
        current = parent;
    }
    true // Not inside a function, so it's a state variable
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
