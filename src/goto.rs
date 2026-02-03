use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Location,Position,Range, Url};


#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub src: String,
    pub name_location: Option<String>,
    pub name_locations: Vec<String>,
    pub referenced_declaration: Option<u64>,
    pub node_type: Option<String>,
    pub member_location: Option<String>,
}

fn push_if_node_or_array<'a>(tree: &'a Value, key: &str, stack: &mut Vec<&'a Value>) {
    if let Some(value) = tree.get(key) {
        match value {
            Value::Array(arr) => {
                stack.extend(arr);
            }
            Value::Object(_) => {
                stack.push(value);
            }
            _ => {}
        }
    }
}

pub fn cache_ids(
    sources: &Value,
) -> (
    HashMap<String, HashMap<u64, NodeInfo>>,
    HashMap<String, String>,
) {
    let mut nodes: HashMap<String, HashMap<u64, NodeInfo>> = HashMap::new();
    let mut path_to_abs: HashMap<String, String> = HashMap::new();

    if let Some(sources_obj) = sources.as_object() {
        for (path, contents) in sources_obj {
            if let Some(contents_array) = contents.as_array()
                && let Some(first_content) = contents_array.first()
                && let Some(source_file) = first_content.get("source_file")
                && let Some(ast) = source_file.get("ast")
            {
                // Get the absolute path for this file
                let abs_path = ast
                    .get("absolutePath")
                    .and_then(|v| v.as_str())
                    .unwrap_or(path)
                    .to_string();

                path_to_abs.insert(path.clone(), abs_path.clone());

                // Initialize the nodes map for this file
                if !nodes.contains_key(&abs_path) {
                    nodes.insert(abs_path.clone(), HashMap::new());
                }

                if let Some(id) = ast.get("id").and_then(|v| v.as_u64())
                    && let Some(src) = ast.get("src").and_then(|v| v.as_str())
                {
                     nodes.get_mut(&abs_path).unwrap().insert(
                         id,
                         NodeInfo {
                             src: src.to_string(),
                             name_location: None,
                             name_locations: vec![],
                             referenced_declaration: None,
                             node_type: ast
                                 .get("nodeType")
                                 .and_then(|v| v.as_str())
                                 .map(|s| s.to_string()),
                             member_location: None,
                         },
                     );
                }

                let mut stack = vec![ast];

                while let Some(tree) = stack.pop() {
                    if let Some(id) = tree.get("id").and_then(|v| v.as_u64())
                        && let Some(src) = tree.get("src").and_then(|v| v.as_str())
                    {
                        // Check for nameLocation first
                        let mut name_location = tree
                            .get("nameLocation")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());

                        // Check for nameLocations array and use appropriate element
                        // For IdentifierPath (qualified names like D.State), use the last element (the actual identifier)
                        // For other nodes, use the first element
                        if name_location.is_none()
                            && let Some(name_locations) = tree.get("nameLocations")
                            && let Some(locations_array) = name_locations.as_array()
                            && !locations_array.is_empty()
                        {
                            let node_type = tree.get("nodeType").and_then(|v| v.as_str());
                            if node_type == Some("IdentifierPath") {
                                name_location = locations_array
                                    .last()
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                            } else {
                                name_location = locations_array[0].as_str().map(|s| s.to_string());
                            }
                        }

                        let name_locations = if let Some(name_locations) = tree.get("nameLocations")
                            && let Some(locations_array) = name_locations.as_array()
                        {
                            locations_array
                                .iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        } else {
                            vec![]
                        };

                        let mut final_name_location = name_location;
                        if final_name_location.is_none()
                            && let Some(member_loc) = tree.get("memberLocation").and_then(|v| v.as_str()) {
                                final_name_location = Some(member_loc.to_string());
                            }

                        let node_info = NodeInfo {
                            src: src.to_string(),
                            name_location: final_name_location,
                            name_locations,
                            referenced_declaration: tree
                                .get("referencedDeclaration")
                                .and_then(|v| v.as_u64()),
                            node_type: tree
                                .get("nodeType")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            member_location: tree
                                .get("memberLocation")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                        };

                        nodes.get_mut(&abs_path).unwrap().insert(id, node_info);
                    }

                    push_if_node_or_array(tree, "arguments", &mut stack);
                    push_if_node_or_array(tree, "arguments", &mut stack);
                    push_if_node_or_array(tree, "baseContracts", &mut stack);
                    push_if_node_or_array(tree, "baseContracts", &mut stack);
                    push_if_node_or_array(tree, "baseExpression", &mut stack);
                    push_if_node_or_array(tree, "baseName", &mut stack);
                    push_if_node_or_array(tree, "baseType", &mut stack);
                    push_if_node_or_array(tree, "block", &mut stack);
                    push_if_node_or_array(tree, "body", &mut stack);
                    push_if_node_or_array(tree, "components", &mut stack);
                    push_if_node_or_array(tree, "components", &mut stack);
                    push_if_node_or_array(tree, "condition", &mut stack);
                    push_if_node_or_array(tree, "declarations", &mut stack);
                    push_if_node_or_array(tree, "endExpression", &mut stack);
                    push_if_node_or_array(tree, "errorCall", &mut stack);
                    push_if_node_or_array(tree, "eventCall", &mut stack);
                    push_if_node_or_array(tree, "expression", &mut stack);
                    push_if_node_or_array(tree, "externalCall", &mut stack);
                    push_if_node_or_array(tree, "falseBody", &mut stack);
                    push_if_node_or_array(tree, "falseExpression", &mut stack);
                    push_if_node_or_array(tree, "file", &mut stack);
                    push_if_node_or_array(tree, "foreign", &mut stack);
                    push_if_node_or_array(tree, "indexExpression", &mut stack);
                    push_if_node_or_array(tree, "initialValue", &mut stack);
                    push_if_node_or_array(tree, "initialValue", &mut stack);
                    push_if_node_or_array(tree, "initializationExpression", &mut stack);
                    push_if_node_or_array(tree, "keyType", &mut stack);
                    push_if_node_or_array(tree, "leftExpression", &mut stack);
                    push_if_node_or_array(tree, "leftHandSide", &mut stack);
                    push_if_node_or_array(tree, "libraryName", &mut stack);
                    push_if_node_or_array(tree, "literals", &mut stack);
                    push_if_node_or_array(tree, "loopExpression", &mut stack);
                    push_if_node_or_array(tree, "members", &mut stack);
                    push_if_node_or_array(tree, "modifierName", &mut stack);
                    push_if_node_or_array(tree, "modifiers", &mut stack);
                    push_if_node_or_array(tree, "name", &mut stack);
                    push_if_node_or_array(tree, "names", &mut stack);
                    push_if_node_or_array(tree, "nodes", &mut stack);
                    push_if_node_or_array(tree, "options", &mut stack);
                    push_if_node_or_array(tree, "options", &mut stack);
                    push_if_node_or_array(tree, "options", &mut stack);
                    push_if_node_or_array(tree, "overrides", &mut stack);
                    push_if_node_or_array(tree, "overrides", &mut stack);
                    push_if_node_or_array(tree, "parameters", &mut stack);
                    push_if_node_or_array(tree, "parameters", &mut stack);
                    push_if_node_or_array(tree, "pathNode", &mut stack);
                    push_if_node_or_array(tree, "returnParameters", &mut stack);
                    push_if_node_or_array(tree, "returnParameters", &mut stack);
                    push_if_node_or_array(tree, "rightExpression", &mut stack);
                    push_if_node_or_array(tree, "rightHandSide", &mut stack);
                    push_if_node_or_array(tree, "startExpression", &mut stack);
                    push_if_node_or_array(tree, "statements", &mut stack);
                    push_if_node_or_array(tree, "statements", &mut stack);
                    push_if_node_or_array(tree, "storageLayout", &mut stack);
                    push_if_node_or_array(tree, "subExpression", &mut stack);
                    push_if_node_or_array(tree, "subdenomination", &mut stack);
                    push_if_node_or_array(tree, "symbolAliases", &mut stack);
                    push_if_node_or_array(tree, "trueBody", &mut stack);
                    push_if_node_or_array(tree, "trueExpression", &mut stack);
                    push_if_node_or_array(tree, "typeName", &mut stack);
                    push_if_node_or_array(tree, "unitAlias", &mut stack);
                    push_if_node_or_array(tree, "value", &mut stack);
                    push_if_node_or_array(tree, "valueType", &mut stack);
                }
            }
        }
    }

    (nodes, path_to_abs)
}

pub fn pos_to_bytes(source_bytes: &[u8], position: Position) -> usize {
    let text = String::from_utf8_lossy(source_bytes);
    let lines: Vec<&str> = text.lines().collect();

    let mut byte_offset = 0;

    for (line_num, line_text) in lines.iter().enumerate() {
        if line_num < position.line as usize {
            byte_offset += line_text.len() + 1; // +1 for newline
        } else if line_num == position.line as usize {
            let char_offset = std::cmp::min(position.character as usize, line_text.len());
            byte_offset += char_offset;
            break;
        }
    }

    byte_offset
}

pub fn bytes_to_pos(source_bytes: &[u8], byte_offset: usize) -> Option<Position> {
    let text = String::from_utf8_lossy(source_bytes);
    let mut curr_offset = 0;

    for (line_num, line_text) in text.lines().enumerate() {
        let line_bytes = line_text.len() + 1; // +1 for newline
        if curr_offset + line_bytes > byte_offset {
            let col = byte_offset - curr_offset;
            return Some(Position::new(line_num as u32, col as u32));
        }
        curr_offset += line_bytes;
    }

    None
}

pub fn goto_bytes(
    nodes: &HashMap<String, HashMap<u64, NodeInfo>>,
    path_to_abs: &HashMap<String, String>,
    id_to_path: &HashMap<String, String>,
    uri: &str,
    position: usize,
) -> Option<(String, usize)> {
    let path = match uri.starts_with("file://") {
        true => &uri[7..],
        false => uri,
    };

    // Get absolute path for this file
    let abs_path = path_to_abs.get(path)?;

    // Get nodes for the current file only
    let current_file_nodes = nodes.get(abs_path)?;

    let mut refs = HashMap::new();

    // Only consider nodes from the current file that have references
    for (id, content) in current_file_nodes {
        if content.referenced_declaration.is_none() {
            continue;
        }

        let src_parts: Vec<&str> = content.src.split(':').collect();
        if src_parts.len() != 3 {
            continue;
        }

        let start_b: usize = src_parts[0].parse().ok()?;
        let length: usize = src_parts[1].parse().ok()?;
        let end_b = start_b + length;

        if start_b <= position && position < end_b {
            let diff = end_b - start_b;
            if !refs.contains_key(&diff) || refs[&diff] <= *id {
                refs.insert(diff, *id);
            }
        }
    }

    if refs.is_empty() {
        return None;
    }

    // Find the reference with minimum diff (most specific)
    let min_diff = *refs.keys().min()?;
    let chosen_id = refs[&min_diff];
    let ref_id = current_file_nodes[&chosen_id].referenced_declaration?;

    // Search for the referenced declaration across all files
    let mut target_node: Option<&NodeInfo> = None;
    for file_nodes in nodes.values() {
        if let Some(node) = file_nodes.get(&ref_id) {
            target_node = Some(node);
            break;
        }
    }

    let node = target_node?;

    // Get location from nameLocation or src
    let (location_str, file_id) = if let Some(name_location) = &node.name_location {
        let parts: Vec<&str> = name_location.split(':').collect();
        if parts.len() == 3 {
            (parts[0], parts[2])
        } else {
            return None;
        }
    } else {
        let parts: Vec<&str> = node.src.split(':').collect();
        if parts.len() == 3 {
            (parts[0], parts[2])
        } else {
            return None;
        }
    };

    let location: usize = location_str.parse().ok()?;
    let file_path = id_to_path.get(file_id)?.clone();

    Some((file_path, location))
}

pub fn goto_declaration(
    ast_data: &Value,
    file_uri: &Url,
    position: Position,
    source_bytes: &[u8]
) -> Option<Location> {
    let sources = ast_data.get("sources")?;
    let build_infos = ast_data.get("build_infos")?.as_array()?;
    let first_build_info = build_infos.first()?;
    let id_to_path = first_build_info.get("source_id_to_path")?.as_object()?;

    let id_to_path_map: HashMap<String, String> = id_to_path
        .iter()
        .map(|(k,v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();

    let (nodes, path_to_abs) = cache_ids(sources);
    let byte_position = pos_to_bytes(source_bytes, position);

    if let Some((file_path, location_bytes)) = goto_bytes(
        &nodes,
        &path_to_abs,
        &id_to_path_map,
        file_uri.as_ref(),
        byte_position,
    ) {
        let target_file_path = std::path::Path::new(&file_path);
        let absolute_path = if target_file_path.is_absolute() {
            target_file_path.to_path_buf()
        } else {
            std::env::current_dir().ok()?.join(target_file_path)
        };

        if let Ok(target_source_bytes) = std::fs::read(&absolute_path)
            && let Some(target_position) = bytes_to_pos(&target_source_bytes, location_bytes)
            && let Ok(target_uri) = Url::from_file_path(&absolute_path)
        {
            return Some(Location {
                uri: target_uri,
                range: Range {
                    start: target_position,
                    end: target_position,
                }
            });
        }

    };

    None


}
