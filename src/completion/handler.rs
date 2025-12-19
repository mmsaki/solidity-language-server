use serde_json::Value;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionParams, Position, InsertTextFormat};

use super::{elementary_type, global_symbol, keyword, member_access};

fn extract_query(text: &str, position: Position) -> Option<String> {
    let byte_offset = crate::utils::position_to_byte_offset(text, position.line, position.character);
    let before_cursor = &text[..byte_offset];
    if before_cursor.is_empty() {
        return None;
    }

    // Find the last word/identifier
    let last_word = before_cursor
        .trim_end()
        .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
        .next()
        .unwrap_or("")
        .to_string();

    if last_word.is_empty() || !last_word.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
        None
    } else {
        Some(last_word)
    }
}

pub fn get_completions(
    text: &str,
    ast_data: &Value,
    position: Position,
    params: &CompletionParams,
) -> (Vec<CompletionItem>, Option<String>) {
    let mut completions = Vec::new();

    // Extract the query (identifier before cursor)
    let query = extract_query(text, position);

    // Check if this is a dot completion
    let is_dot_completion = params
        .context
        .as_ref()
        .and_then(|ctx| ctx.trigger_character.as_ref())
        .map(|t| t == ".")
        .unwrap_or(false)
        || query.as_ref().map(|q| q.ends_with('.')).unwrap_or(false);

    if is_dot_completion {
        // Dot completion - need to determine type and return appropriate completions
        return get_dot_completions(text, ast_data, position);
    }

    // Default completions
    completions.extend(global_symbol::completions());
    completions.extend(elementary_type::completions());
    completions.extend(keyword::completions());

    // Add AST-based scoped completions
    completions.extend(get_scoped_completions(ast_data, text, position));

    (completions, query)
}

pub fn get_scoped_completions(
    ast_data: &Value,
    _text: &str,
    _position: Position,
) -> Vec<CompletionItem> {
    // Extract symbols from AST and provide completions for in-scope items
    // This is a simplified version - a full implementation would need proper scope analysis
    let symbols = crate::symbols::extract_symbols(ast_data);

    // Group functions by name to handle overloads
    let mut function_signatures: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    // First, collect all function signatures
    if let Some(sources) = ast_data.get("sources") {
        if let Some(sources_obj) = sources.as_object() {
            for (_path, contents) in sources_obj {
                if let Some(contents_array) = contents.as_array() {
                    if let Some(first_content) = contents_array.first() {
                        if let Some(ast) = first_content.get("source_file").and_then(|sf| sf.get("ast")) {
                            collect_all_function_signatures_from_ast(ast, &mut function_signatures);
                        }
                    }
                }
            }
        }
    }

    let mut completions = Vec::new();

    for symbol in symbols {
        if symbol.name.is_empty() {
            continue; // Filter out symbols with empty names
        }

        let kind = match symbol.kind {
            tower_lsp::lsp_types::SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
            tower_lsp::lsp_types::SymbolKind::VARIABLE => CompletionItemKind::VARIABLE,
            tower_lsp::lsp_types::SymbolKind::CLASS => CompletionItemKind::CLASS,
            tower_lsp::lsp_types::SymbolKind::INTERFACE => CompletionItemKind::INTERFACE,
            tower_lsp::lsp_types::SymbolKind::STRUCT => CompletionItemKind::STRUCT,
            tower_lsp::lsp_types::SymbolKind::ENUM => CompletionItemKind::ENUM,
            tower_lsp::lsp_types::SymbolKind::EVENT => CompletionItemKind::EVENT,
            _ => CompletionItemKind::VARIABLE,
        };

        if symbol.kind == tower_lsp::lsp_types::SymbolKind::FUNCTION {
            // For functions, create a completion item for each signature
            if let Some(signatures) = function_signatures.get(&symbol.name) {
                for signature in signatures {
                    let snippet = create_function_snippet(&symbol.name, signature);
                    completions.push(CompletionItem {
                        label: symbol.name.clone(),
                        kind: Some(kind),
                        detail: Some(signature.clone()),
                        insert_text: Some(snippet),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    });
                }
            } else {
                // Fallback if no signatures found
                completions.push(CompletionItem {
                    label: symbol.name.clone(),
                    kind: Some(kind),
                    detail: Some(symbol.name.clone()),
                    ..Default::default()
                });
            }
        } else {
            completions.push(CompletionItem {
                label: symbol.name.clone(),
                kind: Some(kind),
                detail: Some(symbol.name.clone()),
                ..Default::default()
            });
        }
    }

    // Remove duplicates based on label and detail
    let mut seen = std::collections::HashSet::new();
    completions.retain(|completion| {
        let key = format!("{}:{:?}", completion.label, completion.detail);
        seen.insert(key)
    });

    completions
}

fn split_params_by_comma(params_str: &str) -> Vec<String> {
    let mut params = Vec::new();
    let mut current = String::new();
    let mut paren_depth = 0;

    for c in params_str.chars() {
        match c {
            '(' | '[' => {
                paren_depth += 1;
                current.push(c);
            }
            ')' | ']' => {
                paren_depth -= 1;
                current.push(c);
            }
            ',' if paren_depth == 0 => {
                params.push(current.trim().to_string());
                current = String::new();
            }
            _ => current.push(c),
        }
    }

    if !current.trim().is_empty() {
        params.push(current.trim().to_string());
    }

    params
}

fn collect_all_function_signatures_from_ast(node: &Value, all_signatures: &mut std::collections::HashMap<String, Vec<String>>) {
    if let Some(node_type) = node.get("nodeType").and_then(|v| v.as_str()) {
        if node_type == "FunctionDefinition" {
            if let Some(name) = node.get("name").and_then(|v| v.as_str()) {
                if !name.is_empty() {
                    // Build function signature
                    let mut signature_parts = Vec::new();

                    // Add parameters - handle both nested ParameterList and direct array structures
                    let param_array = node.get("parameters").and_then(|p| p.get("parameters")).and_then(|p| p.as_array())
                        .or_else(|| node.get("parameters").and_then(|p| p.as_array()));

                    let mut param_strings = Vec::new();
                    if let Some(parameters) = param_array {
                        for param in parameters {
                            let param_name = param.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let type_name = if let Some(type_node) = param.get("typeName") {
                                crate::symbols::extract_type_name(type_node).unwrap_or_else(|| {
                                    // Try to get a fallback type representation
                                    if let Some(type_str) = type_node.get("name").and_then(|v| v.as_str()) {
                                        type_str.to_string()
                                    } else if let Some(node_type) = type_node.get("nodeType").and_then(|v| v.as_str()) {
                                        node_type.to_string()
                                    } else {
                                        "unknown".to_string()
                                    }
                                })
                            } else {
                                "unknown".to_string()
                            };

                            if param_name.is_empty() {
                                param_strings.push(type_name);
                            } else {
                                param_strings.push(format!("{} {}", type_name, param_name));
                            }
                        }
                    }
                    signature_parts.push(format!("({})", param_strings.join(", ")));

                    // Add return types
                    let return_array = node.get("returnParameters").and_then(|p| p.get("parameters")).and_then(|p| p.as_array())
                        .or_else(|| node.get("returnParameters").and_then(|p| p.as_array()));

                    let mut return_strings = Vec::new();
                    if let Some(returns) = return_array {
                        for ret in returns {
                            let param_name = ret.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let type_name = if let Some(type_node) = ret.get("typeName") {
                                crate::symbols::extract_type_name(type_node).unwrap_or_else(|| "unknown".to_string())
                            } else {
                                "unknown".to_string()
                            };

                            if param_name.is_empty() {
                                return_strings.push(type_name);
                            } else {
                                return_strings.push(format!("{} {}", type_name, param_name));
                            }
                        }
                    }
                    if !return_strings.is_empty() {
                        signature_parts.push(format!(" returns ({})", return_strings.join(", ")));
                    }

                    let result = format!("{}{}", name, signature_parts.join(""));
                    all_signatures.entry(name.to_string()).or_insert_with(Vec::new).push(result);
                }
            }
        }

        // Recursively search child nodes
        if let Some(children) = node.as_object() {
            for value in children.values() {
                match value {
                    Value::Array(arr) => {
                        for item in arr {
                            collect_all_function_signatures_from_ast(item, all_signatures);
                        }
                    }
                    Value::Object(_) => {
                        collect_all_function_signatures_from_ast(value, all_signatures);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn create_function_snippet(function_name: &str, signature: &str) -> String {
    // Parse the signature to extract parameters
    // Signature format: "(type1 param1, type2 param2)" or "functionName(type1 param1, type2 param2)"
    if let Some(start) = signature.find('(') {
        // Find the matching closing parenthesis
        let mut paren_count = 0;
        let mut end = None;
        for (i, c) in signature[start..].chars().enumerate() {
            match c {
                '(' => paren_count += 1,
                ')' => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        end = Some(start + i);
                        break;
                    }
                }
                _ => {}
            }
        }

        if let Some(end_pos) = end {
            let params_str = &signature[start + 1..end_pos];
            if !params_str.trim().is_empty() {
                // Split parameters by comma, but be careful with commas inside types
                let params = split_params_by_comma(params_str);
                println!("DEBUG: params_str='{}', params={:?}", params_str, params);
                println!("DEBUG: params_str='{}', params={:?}", params_str, params);
                println!("DEBUG: params_str='{}', params={:?}", params_str, params);
                let mut snippet_parts = Vec::new();
                for (i, param) in params.iter().enumerate() {
                    // Extract parameter name - since signatures are built as "type name" or just "type"
                    let trimmed = param.trim();
                    if trimmed.is_empty() {
                        snippet_parts.push(format!("${{{}:param{}}}", i + 1, i + 1));
                        continue;
                    }

                    // Find the last space - everything after it is the parameter name (if present)
                    if let Some(last_space_idx) = trimmed.rfind(' ') {
                        let after_space = &trimmed[last_space_idx + 1..];
                        if !after_space.is_empty() {
                            // Has a parameter name
                            snippet_parts.push(format!("${{{}:{}}}", i + 1, after_space));
                        } else {
                            // Space but nothing after - treat as unnamed
                            snippet_parts.push(format!("${{{}:param{}}}", i + 1, i + 1));
                        }
                    } else {
                        // No space - just a type, unnamed parameter
                        snippet_parts.push(format!("${{{}:param{}}}", i + 1, i + 1));
                    }
                }
                return format!("{}({})", function_name, snippet_parts.join(", "));
            }
        }
    }
    // No parameters or couldn't parse
    format!("{}()", function_name)
}



fn get_dot_completions(text: &str, ast_data: &Value, position: Position) -> (Vec<CompletionItem>, Option<String>) {
    // Use the member_access module for proper type detection
    member_access::get_dot_completions(text, ast_data, position).map(|(comps, query)| (comps, Some(query))).unwrap_or_default()
}
