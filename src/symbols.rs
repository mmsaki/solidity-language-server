#![allow(deprecated)]

use crate::utils::byte_offset_to_position;
use serde_json::Value;
use tower_lsp::lsp_types::{DocumentSymbol, Location, Range, SymbolInformation, SymbolKind, Url};

pub fn extract_symbols(ast_data: &Value) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    let mut seen = std::collections::HashSet::new();

    if let Some(sources) = ast_data.get("sources")
        && let Some(sources_obj) = sources.as_object()
    {
        for (path, contents) in sources_obj {
            if let Some(contents_array) = contents.as_array()
                && let Some(first_content) = contents_array.first()
                && let Some(source_file) = first_content.get("source_file")
                && let Some(ast) = source_file.get("ast")
            {
                let file_symbols = extract_symbols_from_ast(ast, path);
                for symbol in file_symbols {
                    // Deduplicate based on location (URI + range)
                    let key = format!(
                        "{}:{:?}:{:?}",
                        symbol.location.uri, symbol.location.range.start, symbol.location.range.end
                    );
                    if seen.insert(key) {
                        symbols.push(symbol);
                    }
                }
            }
        }
    }

    // Set container_name for fields
    let containers: Vec<_> = symbols
        .iter()
        .filter(|s| {
            s.kind == tower_lsp::lsp_types::SymbolKind::STRUCT
                || s.kind == tower_lsp::lsp_types::SymbolKind::CLASS
        })
        .cloned()
        .collect();
    for symbol in &mut symbols {
        if symbol.kind == tower_lsp::lsp_types::SymbolKind::FIELD {
            let mut best_container = None;
            let mut min_size = i32::MAX;
            for container in &containers {
                if container.location.uri == symbol.location.uri
                    && container.location.range.start.line <= symbol.location.range.start.line
                    && container.location.range.end.line >= symbol.location.range.end.line
                {
                    let size = container.location.range.end.line as i32
                        - container.location.range.start.line as i32;
                    if size < min_size {
                        min_size = size;
                        best_container = Some(container);
                    }
                }
            }
            if let Some(container) = best_container {
                symbol.container_name = Some(container.name.clone());
            }
        }
    }

    // For functions, prefer symbols with containers over symbols without containers
    // when they have the same location (to keep library functions properly contained)
    let mut deduped_symbols = Vec::new();
    let mut seen_locations = std::collections::HashSet::new();

    for symbol in symbols {
        let key = format!(
            "{}:{:?}:{:?}",
            symbol.location.uri, symbol.location.range.start, symbol.location.range.end
        );

        if seen_locations.insert(key) {
            deduped_symbols.push(symbol);
        } else {
            // If we already have a symbol at this location, prefer one with a container
            if let Some(existing_index) = deduped_symbols.iter().position(|s| {
                s.location.uri == symbol.location.uri
                    && s.location.range.start == symbol.location.range.start
                    && s.location.range.end == symbol.location.range.end
            }) {
                let existing = &deduped_symbols[existing_index];
                if existing.container_name.is_none() && symbol.container_name.is_some() {
                    // Replace with the one that has a container
                    deduped_symbols[existing_index] = symbol;
                }
                // Otherwise keep the existing one
            }
        }
    }

    symbols = deduped_symbols;

    symbols
}

pub fn extract_document_symbols(ast_data: &Value, file_path: &str) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();

    if let Some(sources) = ast_data.get("sources")
        && let Some(sources_obj) = sources.as_object()
    {
        for (path, contents) in sources_obj {
            if (path == file_path
                || path.ends_with(&format!("/{}", file_path))
                || path.ends_with(file_path))
                && let Some(contents_array) = contents.as_array()
                && let Some(first_content) = contents_array.first()
                && let Some(source_file) = first_content.get("source_file")
                && let Some(ast) = source_file.get("ast")
            {
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
                        if let Some(symbol) =
                            create_contract_document_symbol_with_children(node, file_path)
                        {
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

fn create_contract_document_symbol_with_children(
    node: &Value,
    file_path: &str,
) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
    let range = get_node_range(node, file_path)?;
    let mut children = Vec::new();

    // Process contract members
    if let Some(nodes) = node.get("nodes").and_then(|v| v.as_array()) {
        for member_node in nodes {
            if let Some(node_type) = member_node.get("nodeType").and_then(|v| v.as_str()) {
                match node_type {
                    "FunctionDefinition" => {
                        if let Some(symbol) =
                            create_function_document_symbol_with_children(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "VariableDeclaration" => {
                        if let Some(symbol) =
                            create_variable_document_symbol(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "EventDefinition" => {
                        if let Some(symbol) = create_event_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "ModifierDefinition" => {
                        if let Some(symbol) =
                            create_modifier_document_symbol(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "StructDefinition" => {
                        if let Some(symbol) =
                            create_struct_document_symbol_with_children(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "EnumDefinition" => {
                        if let Some(symbol) =
                            create_enum_document_symbol_with_children(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "ConstructorDefinition" => {
                        if let Some(symbol) =
                            create_constructor_document_symbol(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "ErrorDefinition" => {
                        if let Some(symbol) = create_error_document_symbol(member_node, file_path) {
                            children.push(symbol);
                        }
                    }
                    "UsingForDirective" => {
                        if let Some(symbol) =
                            create_using_for_document_symbol(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "FallbackFunctionDefinition" => {
                        if let Some(symbol) =
                            create_fallback_document_symbol(member_node, file_path)
                        {
                            children.push(symbol);
                        }
                    }
                    "ReceiveFunctionDefinition" => {
                        if let Some(symbol) = create_receive_document_symbol(member_node, file_path)
                        {
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
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
        tags: None,
        deprecated: None,
    })
}

fn create_function_document_symbol_with_children(
    node: &Value,
    file_path: &str,
) -> Option<DocumentSymbol> {
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

    // Build function signature detail
    let mut signature_parts = Vec::new();

    // Add parameters to signature
    let param_array = node
        .get("parameters")
        .and_then(|p| p.get("parameters"))
        .and_then(|p| p.as_array())
        .or_else(|| node.get("parameters").and_then(|p| p.as_array()));

    let mut param_strings = Vec::new();
    if let Some(parameters) = param_array {
        for param in parameters {
            if let Some(param_name) = param.get("name").and_then(|v| v.as_str())
                && !param_name.is_empty()
                && let Some(type_node) = param.get("typeName")
                && let Some(type_name) = extract_type_name(type_node)
            {
                param_strings.push(format!("{} {}", type_name, param_name));
            }
        }
    }
    signature_parts.push(format!("({})", param_strings.join(", ")));

    // Add return types to signature
    if !is_constructor {
        let return_array = node
            .get("returnParameters")
            .and_then(|p| p.get("parameters"))
            .and_then(|p| p.as_array())
            .or_else(|| node.get("returnParameters").and_then(|p| p.as_array()));

        let mut return_strings = Vec::new();
        if let Some(returns) = return_array {
            for ret in returns {
                if let Some(type_node) = ret.get("typeName")
                    && let Some(type_name) = extract_type_name(type_node)
                {
                    return_strings.push(type_name);
                }
            }
        }
        if !return_strings.is_empty() {
            signature_parts.push(format!(" returns ({})", return_strings.join(", ")));
        }
    }

    let detail = if signature_parts.len() > 1 {
        Some(signature_parts.join(""))
    } else {
        None
    };

    // Extract parameters, return parameters, and local variables as children
    let mut children = Vec::new();

    // Add input parameters as children
    if let Some(parameters) = param_array {
        for param in parameters {
            if let Some(param_symbol) = create_parameter_document_symbol(param, file_path) {
                children.push(param_symbol);
            }
        }
    }

    // Add return parameters as children
    if !is_constructor
        && let Some(returns) = node
            .get("returnParameters")
            .and_then(|p| p.get("parameters"))
            .and_then(|p| p.as_array())
            .or_else(|| node.get("returnParameters").and_then(|p| p.as_array()))
    {
        for ret in returns {
            if let Some(ret_symbol) = create_return_parameter_document_symbol(ret, file_path) {
                children.push(ret_symbol);
            }
        }
    }

    // Extract local variables from the function body
    if let Some(body) = node.get("body") {
        extract_local_variables_from_block(body, file_path, &mut children);
    }

    Some(DocumentSymbol {
        name,
        detail,
        kind,
        range,
        selection_range: range,
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
        tags: None,
        deprecated: None,
    })
}

fn create_variable_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
    let range = get_node_range(node, file_path)?;
    let selection_range = get_symbol_range(node, file_path).unwrap_or(range);

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
        selection_range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_event_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
    if name.is_empty() {
        return None;
    }
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

fn create_struct_document_symbol_with_children(
    node: &Value,
    file_path: &str,
) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
        tags: None,
        deprecated: None,
    })
}

fn create_struct_member_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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

fn create_enum_document_symbol_with_children(
    node: &Value,
    file_path: &str,
) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
        tags: None,
        deprecated: None,
    })
}

fn create_enum_member_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
    if name.is_empty() {
        return None;
    }
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

fn extract_local_variables_from_block(
    node: &Value,
    file_path: &str,
    children: &mut Vec<DocumentSymbol>,
) {
    if let Some(statements) = node.get("statements").and_then(|v| v.as_array()) {
        for stmt in statements {
            if let Some(node_type) = stmt.get("nodeType").and_then(|v| v.as_str()) {
                match node_type {
                    "VariableDeclarationStatement" => {
                        if let Some(declarations) =
                            stmt.get("declarations").and_then(|v| v.as_array())
                        {
                            for decl in declarations {
                                if let Some(var_symbol) =
                                    create_variable_document_symbol(decl, file_path)
                                {
                                    children.push(var_symbol);
                                }
                            }
                        }
                    }
                    "Block" => {
                        // Recurse into nested blocks
                        extract_local_variables_from_block(stmt, file_path, children);
                    }
                    "IfStatement" => {
                        // Recurse into true and false bodies
                        if let Some(true_body) = stmt.get("trueBody") {
                            extract_local_variables_from_block(true_body, file_path, children);
                        }
                        if let Some(false_body) = stmt.get("falseBody") {
                            extract_local_variables_from_block(false_body, file_path, children);
                        }
                    }
                    "ForStatement" => {
                        if let Some(body) = stmt.get("body") {
                            extract_local_variables_from_block(body, file_path, children);
                        }
                    }
                    "WhileStatement" => {
                        if let Some(body) = stmt.get("body") {
                            extract_local_variables_from_block(body, file_path, children);
                        }
                    }
                    // Add other control structures as needed
                    _ => {}
                }
            }
        }
    }
}

fn create_parameter_document_symbol(node: &Value, file_path: &str) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str()).unwrap_or(""); // Allow empty names for return parameters
    if name.is_empty() {
        return None; // Skip unnamed parameters
    }
    let range = get_node_range(node, file_path)?;

    let detail = if let Some(type_node) = node.get("typeName") {
        extract_type_name(type_node).map(|type_name| format!("{} {}", type_name, name))
    } else {
        None
    };

    Some(DocumentSymbol {
        name: name.to_string(),
        detail,
        kind: SymbolKind::VARIABLE,
        range,
        selection_range: range,
        children: None,
        tags: None,
        deprecated: None,
    })
}

fn create_return_parameter_document_symbol(
    node: &Value,
    file_path: &str,
) -> Option<DocumentSymbol> {
    let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let range = get_node_range(node, file_path)?;

    let display_name = if name.is_empty() {
        if let Some(type_node) = node.get("typeName") {
            if let Some(type_name) = extract_type_name(type_node) {
                format!("<{}>", type_name)
            } else {
                "<return>".to_string()
            }
        } else {
            "<return>".to_string()
        }
    } else {
        name.to_string()
    };

    let detail = if let Some(type_node) = node.get("typeName") {
        extract_type_name(type_node).map(|type_name| format!("{} {}", type_name, name))
    } else {
        None
    };

    Some(DocumentSymbol {
        name: display_name,
        detail,
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
        && let Some(id) = library_name.get("name").and_then(|v| v.as_str())
    {
        name_parts.push(id.to_string());
    }

    name_parts.push("for".to_string());

    // Add type name if present
    if let Some(type_name) = node.get("typeName")
        && let Some(name_str) = extract_type_name(type_name)
    {
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

pub fn extract_type_name(type_node: &Value) -> Option<String> {
    if let Some(node_type) = type_node.get("nodeType").and_then(|v| v.as_str()) {
        match node_type {
            "ElementaryTypeName" => type_node
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            "UserDefinedTypeName" => type_node
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            "Mapping" => {
                let mut mapping_str = "mapping(".to_string();
                if let Some(key_type) = type_node.get("keyType")
                    && let Some(key_name) = extract_type_name(key_type)
                {
                    mapping_str.push_str(&key_name);
                }
                mapping_str.push_str(" => ");
                if let Some(value_type) = type_node.get("valueType")
                    && let Some(value_name) = extract_type_name(value_type)
                {
                    mapping_str.push_str(&value_name);
                }
                mapping_str.push(')');
                Some(mapping_str)
            }
            "ArrayTypeName" => {
                if let Some(base_type) = type_node.get("baseType") {
                    extract_type_name(base_type).map(|base_name| format!("{}[]", base_name))
                } else {
                    None
                }
            }
            "FunctionTypeName" => Some("function".to_string()),
            _ => None,
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
        let parts: Vec<String> = literals
            .iter()
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
    extract_symbols_from_ast_with_container(ast, file_path, None)
}

fn extract_symbols_from_ast_with_container(
    ast: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    let mut stack = vec![(ast, container_name)];

    while let Some((node, current_container)) = stack.pop() {
        if let Some(node_type) = node.get("nodeType").and_then(|v| v.as_str()) {
            match node_type {
                "ContractDefinition" => {
                    let contract_name = node
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    if let Some(symbol) =
                        create_contract_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                    // Push members with contract as container
                    if let Some(nodes) = node.get("nodes").and_then(|v| v.as_array()) {
                        for member in nodes {
                            stack.push((member, contract_name.clone()));
                        }
                    }
                }
                "LibraryDefinition" => {
                    let library_name = node
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    if let Some(symbol) =
                        create_library_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                    // Push members with library as container
                    if let Some(nodes) = node.get("nodes").and_then(|v| v.as_array()) {
                        for member in nodes {
                            stack.push((member, library_name.clone()));
                        }
                    }
                }
                "FunctionDefinition" => {
                    if let Some(symbol) =
                        create_function_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                    // Push parameters with function as container
                    if let Some(params) = node
                        .get("parameters")
                        .and_then(|p| p.get("parameters"))
                        .and_then(|p| p.as_array())
                    {
                        let func_name = node
                            .get("name")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        for param in params {
                            stack.push((param, func_name.clone()));
                        }
                    }
                }
                "VariableDeclaration" => {
                    if let Some(symbol) =
                        create_variable_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                }
                "EventDefinition" => {
                    if let Some(symbol) =
                        create_event_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                }
                "ModifierDefinition" => {
                    if let Some(symbol) =
                        create_modifier_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                }
                "StructDefinition" => {
                    let struct_name = node
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    if let Some(symbol) =
                        create_struct_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                    // Push members with struct as container
                    if let Some(members) = node.get("members").and_then(|v| v.as_array()) {
                        for member in members {
                            stack.push((member, struct_name.clone()));
                        }
                    }
                }
                "EnumDefinition" => {
                    let enum_name = node
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    if let Some(symbol) =
                        create_enum_symbol_info(node, file_path, current_container.clone())
                    {
                        symbols.push(symbol);
                    }
                    // Push members with enum as container
                    if let Some(members) = node.get("members").and_then(|v| v.as_array()) {
                        for member in members {
                            stack.push((member, enum_name.clone()));
                        }
                    }
                }
                _ => {}
            }
        }

        // Add other child nodes to stack with current container
        push_child_nodes_with_container(node, &mut stack, current_container);
    }

    symbols
}

fn create_contract_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        container_name,
    })
}

fn create_library_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
    let range = get_node_range(node, file_path)?;
    let location = Location {
        uri: Url::from_file_path(file_path).ok()?,
        range,
    };

    Some(SymbolInformation {
        name: name.to_string(),
        kind: SymbolKind::CLASS, // Libraries are similar to classes
        tags: None,
        deprecated: None,
        location,
        container_name,
    })
}

fn create_function_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
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
        container_name,
    })
}

fn create_variable_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        container_name,
    })
}

fn create_event_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        container_name,
    })
}

fn create_modifier_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        container_name,
    })
}

fn create_struct_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        container_name,
    })
}

fn create_enum_symbol_info(
    node: &Value,
    file_path: &str,
    container_name: Option<String>,
) -> Option<SymbolInformation> {
    let name = node.get("name").and_then(|v| v.as_str())?;
    if name.is_empty() {
        return None;
    }
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
        container_name,
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
    let start = byte_offset_to_position(&content, start_offset);
    let end = byte_offset_to_position(&content, start_offset + length);

    Some(Range { start, end })
}

fn get_symbol_range(node: &Value, file_path: &str) -> Option<Range> {
    // Try to use nameLocation if available
    if let Some(name_loc) = node.get("nameLocation").and_then(|v| v.as_str()) {
        let parts: Vec<&str> = name_loc.split(':').collect();
        if parts.len() >= 3 {
            let start_offset: usize = parts[0].parse().ok()?;
            let length: usize = parts[1].parse().ok()?;
            let content = std::fs::read_to_string(file_path).ok()?;
            let start = byte_offset_to_position(&content, start_offset);
            let end = byte_offset_to_position(&content, start_offset + length);
            return Some(Range { start, end });
        }
    }
    // Fallback to node range
    get_node_range(node, file_path)
}

fn push_child_nodes_with_container<'a>(
    node: &'a Value,
    stack: &mut Vec<(&'a Value, Option<String>)>,
    container: Option<String>,
) {
    if let Some(children) = node.as_object() {
        for value in children.values() {
            match value {
                Value::Array(arr) => {
                    for item in arr {
                        stack.push((item, container.clone()));
                    }
                }
                Value::Object(_) => {
                    stack.push((value, container.clone()));
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
            && (node_type == "FunctionDefinition" || node_type == "ModifierDefinition")
        {
            return false; // Inside a function or modifier, so it's local
        }
        current = parent;
    }
    true // Not inside a function, so it's a state variable
}
