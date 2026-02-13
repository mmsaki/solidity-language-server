use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Location, Position, Range, Url};

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub src: String,
    pub name_location: Option<String>,
    pub name_locations: Vec<String>,
    pub referenced_declaration: Option<u64>,
    pub node_type: Option<String>,
    pub member_location: Option<String>,
    pub absolute_path: Option<String>,
}

/// All AST child keys to traverse (Solidity + Yul).
pub const CHILD_KEYS: &[&str] = &[
    "AST",
    "arguments",
    "baseContracts",
    "baseExpression",
    "baseName",
    "baseType",
    "block",
    "body",
    "components",
    "condition",
    "declarations",
    "endExpression",
    "errorCall",
    "eventCall",
    "expression",
    "externalCall",
    "falseBody",
    "falseExpression",
    "file",
    "foreign",
    "functionName",
    "indexExpression",
    "initialValue",
    "initializationExpression",
    "keyType",
    "leftExpression",
    "leftHandSide",
    "libraryName",
    "literals",
    "loopExpression",
    "members",
    "modifierName",
    "modifiers",
    "name",
    "names",
    "nodes",
    "options",
    "overrides",
    "parameters",
    "pathNode",
    "post",
    "pre",
    "returnParameters",
    "rightExpression",
    "rightHandSide",
    "startExpression",
    "statements",
    "storageLayout",
    "subExpression",
    "subdenomination",
    "symbolAliases",
    "trueBody",
    "trueExpression",
    "typeName",
    "unitAlias",
    "value",
    "valueType",
    "variableNames",
    "variables",
];

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

/// Maps `"offset:length:fileId"` src strings from Yul externalReferences
/// to the Solidity declaration node id they refer to.
pub type ExternalRefs = HashMap<String, u64>;

/// Pre-computed AST index. Built once when an AST enters the cache,
/// then reused on every goto/references/rename/hover request.
#[derive(Debug, Clone)]
pub struct CachedBuild {
    pub ast: Value,
    pub nodes: HashMap<String, HashMap<u64, NodeInfo>>,
    pub path_to_abs: HashMap<String, String>,
    pub external_refs: ExternalRefs,
    pub id_to_path_map: HashMap<String, String>,
}

impl CachedBuild {
    /// Build the index from raw `forge build --ast` output.
    pub fn new(ast: Value) -> Self {
        let (nodes, path_to_abs, external_refs) = if let Some(sources) = ast.get("sources") {
            cache_ids(sources)
        } else {
            (HashMap::new(), HashMap::new(), HashMap::new())
        };

        let id_to_path_map = ast
            .get("build_infos")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|info| info.get("source_id_to_path"))
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Self {
            ast,
            nodes,
            path_to_abs,
            external_refs,
            id_to_path_map,
        }
    }
}

pub fn cache_ids(
    sources: &Value,
) -> (
    HashMap<String, HashMap<u64, NodeInfo>>,
    HashMap<String, String>,
    ExternalRefs,
) {
    let mut nodes: HashMap<String, HashMap<u64, NodeInfo>> = HashMap::new();
    let mut path_to_abs: HashMap<String, String> = HashMap::new();
    let mut external_refs: ExternalRefs = HashMap::new();

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
                            absolute_path: ast
                                .get("absolutePath")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
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
                            && let Some(member_loc) =
                                tree.get("memberLocation").and_then(|v| v.as_str())
                        {
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
                            absolute_path: tree
                                .get("absolutePath")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                        };

                        nodes.get_mut(&abs_path).unwrap().insert(id, node_info);

                        // Collect externalReferences from InlineAssembly nodes
                        if tree.get("nodeType").and_then(|v| v.as_str())
                            == Some("InlineAssembly")
                        {
                            if let Some(ext_refs) =
                                tree.get("externalReferences").and_then(|v| v.as_array())
                            {
                                for ext_ref in ext_refs {
                                    if let Some(src_str) =
                                        ext_ref.get("src").and_then(|v| v.as_str())
                                    {
                                        if let Some(decl_id) =
                                            ext_ref.get("declaration").and_then(|v| v.as_u64())
                                        {
                                            external_refs
                                                .insert(src_str.to_string(), decl_id);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    for key in CHILD_KEYS {
                        push_if_node_or_array(tree, key, &mut stack);
                    }
                }
            }
        }
    }

    (nodes, path_to_abs, external_refs)
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

/// Convert a `"offset:length:fileId"` src string to an LSP Location.
pub fn src_to_location(
    src: &str,
    id_to_path: &HashMap<String, String>,
) -> Option<Location> {
    let parts: Vec<&str> = src.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let byte_offset: usize = parts[0].parse().ok()?;
    let length: usize = parts[1].parse().ok()?;
    let file_id = parts[2];
    let file_path = id_to_path.get(file_id)?;

    let absolute_path = if std::path::Path::new(file_path).is_absolute() {
        std::path::PathBuf::from(file_path)
    } else {
        std::env::current_dir().ok()?.join(file_path)
    };

    let source_bytes = std::fs::read(&absolute_path).ok()?;
    let start_pos = bytes_to_pos(&source_bytes, byte_offset)?;
    let end_pos = bytes_to_pos(&source_bytes, byte_offset + length)?;
    let uri = Url::from_file_path(&absolute_path).ok()?;

    Some(Location {
        uri,
        range: Range {
            start: start_pos,
            end: end_pos,
        },
    })
}

pub fn goto_bytes(
    nodes: &HashMap<String, HashMap<u64, NodeInfo>>,
    path_to_abs: &HashMap<String, String>,
    id_to_path: &HashMap<String, String>,
    external_refs: &ExternalRefs,
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

    // Build reverse map: file_path -> file_id for filtering external refs by current file
    let path_to_file_id: HashMap<&str, &str> = id_to_path
        .iter()
        .map(|(id, p)| (p.as_str(), id.as_str()))
        .collect();

    // Determine the file id for the current file
    // path_to_abs maps filesystem path -> absolutePath (e.g. "src/libraries/SwapMath.sol")
    // id_to_path maps file_id -> relative path (e.g. "34" -> "src/libraries/SwapMath.sol")
    let current_file_id = path_to_file_id.get(abs_path.as_str());

    // Check if cursor is on a Yul external reference first
    for (src_str, decl_id) in external_refs {
        let src_parts: Vec<&str> = src_str.split(':').collect();
        if src_parts.len() != 3 {
            continue;
        }

        // Only consider external refs in the current file
        if let Some(file_id) = current_file_id {
            if src_parts[2] != *file_id {
                continue;
            }
        } else {
            continue;
        }

        let start_b: usize = src_parts[0].parse().ok()?;
        let length: usize = src_parts[1].parse().ok()?;
        let end_b = start_b + length;

        if start_b <= position && position < end_b {
            // Found a Yul external reference — resolve to the declaration target
            let mut target_node: Option<&NodeInfo> = None;
            for file_nodes in nodes.values() {
                if let Some(node) = file_nodes.get(decl_id) {
                    target_node = Some(node);
                    break;
                }
            }
            let node = target_node?;
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
            return Some((file_path, location));
        }
    }

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
        // Check if we're on the string part of an import statement
        // ImportDirective nodes have absolutePath pointing to the imported file
        let tmp = current_file_nodes.iter();
        for (_id, content) in tmp {
            if content.node_type == Some("ImportDirective".to_string()) {
                let src_parts: Vec<&str> = content.src.split(':').collect();
                if src_parts.len() != 3 {
                    continue;
                }

                let start_b: usize = src_parts[0].parse().ok()?;
                let length: usize = src_parts[1].parse().ok()?;
                let end_b = start_b + length;

                if start_b <= position
                    && position < end_b
                    && let Some(import_path) = &content.absolute_path
                {
                    return Some((import_path.clone(), 0));
                }
            }
        }
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

    let (nodes, path_to_abs, external_refs) = cache_ids(sources);
    let byte_position = pos_to_bytes(source_bytes, position);

    if let Some((file_path, location_bytes)) = goto_bytes(
        &nodes,
        &path_to_abs,
        &id_to_path_map,
        &external_refs,
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
