use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Location, Position, Range, Url};
use tree_sitter::{Node, Parser};

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

type Type = (
    HashMap<String, HashMap<u64, NodeInfo>>,
    HashMap<String, String>,
    ExternalRefs,
);

pub fn cache_ids(sources: &Value) -> Type {
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
                        if tree.get("nodeType").and_then(|v| v.as_str()) == Some("InlineAssembly")
                            && let Some(ext_refs) =
                                tree.get("externalReferences").and_then(|v| v.as_array())
                        {
                            for ext_ref in ext_refs {
                                if let Some(src_str) = ext_ref.get("src").and_then(|v| v.as_str())
                                    && let Some(decl_id) =
                                        ext_ref.get("declaration").and_then(|v| v.as_u64())
                                {
                                    external_refs.insert(src_str.to_string(), decl_id);
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
    crate::utils::position_to_byte_offset(&text, position)
}

pub fn bytes_to_pos(source_bytes: &[u8], byte_offset: usize) -> Option<Position> {
    let text = String::from_utf8_lossy(source_bytes);
    let pos = crate::utils::byte_offset_to_position(&text, byte_offset);
    Some(pos)
}

/// Convert a `"offset:length:fileId"` src string to an LSP Location.
pub fn src_to_location(src: &str, id_to_path: &HashMap<String, String>) -> Option<Location> {
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
) -> Option<(String, usize, usize)> {
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
            let (location_str, length_str, file_id) =
                if let Some(name_location) = &node.name_location {
                    let parts: Vec<&str> = name_location.split(':').collect();
                    if parts.len() == 3 {
                        (parts[0], parts[1], parts[2])
                    } else {
                        return None;
                    }
                } else {
                    let parts: Vec<&str> = node.src.split(':').collect();
                    if parts.len() == 3 {
                        (parts[0], parts[1], parts[2])
                    } else {
                        return None;
                    }
                };
            let location: usize = location_str.parse().ok()?;
            let len: usize = length_str.parse().ok()?;
            let file_path = id_to_path.get(file_id)?.clone();
            return Some((file_path, location, len));
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
                    return Some((import_path.clone(), 0, 0));
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
    let (location_str, length_str, file_id) = if let Some(name_location) = &node.name_location {
        let parts: Vec<&str> = name_location.split(':').collect();
        if parts.len() == 3 {
            (parts[0], parts[1], parts[2])
        } else {
            return None;
        }
    } else {
        let parts: Vec<&str> = node.src.split(':').collect();
        if parts.len() == 3 {
            (parts[0], parts[1], parts[2])
        } else {
            return None;
        }
    };

    let location: usize = location_str.parse().ok()?;
    let len: usize = length_str.parse().ok()?;
    let file_path = id_to_path.get(file_id)?.clone();

    Some((file_path, location, len))
}

pub fn goto_declaration(
    ast_data: &Value,
    file_uri: &Url,
    position: Position,
    source_bytes: &[u8],
) -> Option<Location> {
    let sources = ast_data.get("sources")?;
    let build_infos = ast_data.get("build_infos")?.as_array()?;
    let first_build_info = build_infos.first()?;
    let id_to_path = first_build_info.get("source_id_to_path")?.as_object()?;

    let id_to_path_map: HashMap<String, String> = id_to_path
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();

    let (nodes, path_to_abs, external_refs) = cache_ids(sources);
    let byte_position = pos_to_bytes(source_bytes, position);

    if let Some((file_path, location_bytes, length)) = goto_bytes(
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
            && let Some(start_pos) = bytes_to_pos(&target_source_bytes, location_bytes)
            && let Some(end_pos) = bytes_to_pos(&target_source_bytes, location_bytes + length)
            && let Ok(target_uri) = Url::from_file_path(&absolute_path)
        {
            return Some(Location {
                uri: target_uri,
                range: Range {
                    start: start_pos,
                    end: end_pos,
                },
            });
        }
    };

    None
}

// ── Tree-sitter enhanced goto ──────────────────────────────────────────────

/// Context extracted from the cursor position via tree-sitter.
#[derive(Debug, Clone)]
pub struct CursorContext {
    /// The identifier text under the cursor.
    pub name: String,
    /// Enclosing function name (if any).
    pub function: Option<String>,
    /// Enclosing contract/interface/library name (if any).
    pub contract: Option<String>,
}

/// Parse Solidity source with tree-sitter.
fn ts_parse(source: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_solidity::LANGUAGE.into())
        .expect("failed to load Solidity grammar");
    parser.parse(source, None)
}

/// Find the deepest named node at the given byte offset.
fn ts_node_at_byte(node: Node, byte: usize) -> Option<Node> {
    if byte < node.start_byte() || byte >= node.end_byte() {
        return None;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.start_byte() <= byte
            && byte < child.end_byte()
            && let Some(deeper) = ts_node_at_byte(child, byte)
        {
            return Some(deeper);
        }
    }
    Some(node)
}

/// Get the identifier name from a node (first `identifier` child or the node itself).
fn ts_child_id_text<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|c| c.kind() == "identifier" && c.is_named())
        .map(|c| &source[c.byte_range()])
}

/// Extract cursor context: the identifier under the cursor and its ancestor names.
///
/// Walks up the tree-sitter parse tree to find the enclosing function and contract.
pub fn cursor_context(source: &str, position: Position) -> Option<CursorContext> {
    let tree = ts_parse(source)?;
    let byte = pos_to_bytes(source.as_bytes(), position);
    let leaf = ts_node_at_byte(tree.root_node(), byte)?;

    // The leaf should be an identifier (or we find the nearest identifier)
    let id_node = if leaf.kind() == "identifier" {
        leaf
    } else {
        // Check parent — cursor might be just inside a node that contains an identifier
        let parent = leaf.parent()?;
        if parent.kind() == "identifier" {
            parent
        } else {
            return None;
        }
    };

    let name = source[id_node.byte_range()].to_string();
    let mut function = None;
    let mut contract = None;

    // Walk ancestors
    let mut current = id_node.parent();
    while let Some(node) = current {
        match node.kind() {
            "function_definition" | "modifier_definition" if function.is_none() => {
                function = ts_child_id_text(node, source).map(String::from);
            }
            "constructor_definition" if function.is_none() => {
                function = Some("constructor".into());
            }
            "contract_declaration" | "interface_declaration" | "library_declaration"
                if contract.is_none() =>
            {
                contract = ts_child_id_text(node, source).map(String::from);
            }
            _ => {}
        }
        current = node.parent();
    }

    Some(CursorContext {
        name,
        function,
        contract,
    })
}

/// Information about a declaration found by tree-sitter.
#[derive(Debug, Clone)]
pub struct TsDeclaration {
    /// Position range of the declaration identifier.
    pub range: Range,
    /// What kind of declaration (contract, function, state_variable, etc.).
    pub kind: &'static str,
    /// Container name (contract/struct that owns this declaration).
    pub container: Option<String>,
}

/// Find all declarations of a name in a source file using tree-sitter.
///
/// Scans the parse tree for declaration nodes (state variables, functions, events,
/// errors, structs, enums, contracts, etc.) whose identifier matches `name`.
pub fn find_declarations_by_name(source: &str, name: &str) -> Vec<TsDeclaration> {
    let tree = match ts_parse(source) {
        Some(t) => t,
        None => return vec![],
    };
    let mut results = Vec::new();
    collect_declarations(tree.root_node(), source, name, None, &mut results);
    results
}

fn collect_declarations(
    node: Node,
    source: &str,
    name: &str,
    container: Option<&str>,
    out: &mut Vec<TsDeclaration>,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            continue;
        }
        match child.kind() {
            "contract_declaration" | "interface_declaration" | "library_declaration" => {
                if let Some(id_name) = ts_child_id_text(child, source) {
                    if id_name == name {
                        out.push(TsDeclaration {
                            range: id_range(child),
                            kind: child.kind(),
                            container: container.map(String::from),
                        });
                    }
                    // Recurse into contract body
                    if let Some(body) = ts_find_child(child, "contract_body") {
                        collect_declarations(body, source, name, Some(id_name), out);
                    }
                }
            }
            "function_definition" | "modifier_definition" => {
                if let Some(id_name) = ts_child_id_text(child, source) {
                    if id_name == name {
                        out.push(TsDeclaration {
                            range: id_range(child),
                            kind: child.kind(),
                            container: container.map(String::from),
                        });
                    }
                    // Check function parameters
                    collect_parameters(child, source, name, container, out);
                    // Recurse into function body for local variables
                    if let Some(body) = ts_find_child(child, "function_body") {
                        collect_declarations(body, source, name, container, out);
                    }
                }
            }
            "constructor_definition" => {
                if name == "constructor" {
                    out.push(TsDeclaration {
                        range: ts_range(child),
                        kind: "constructor_definition",
                        container: container.map(String::from),
                    });
                }
                // Check constructor parameters
                collect_parameters(child, source, name, container, out);
                if let Some(body) = ts_find_child(child, "function_body") {
                    collect_declarations(body, source, name, container, out);
                }
            }
            "state_variable_declaration" | "variable_declaration" => {
                if let Some(id_name) = ts_child_id_text(child, source)
                    && id_name == name
                {
                    out.push(TsDeclaration {
                        range: id_range(child),
                        kind: child.kind(),
                        container: container.map(String::from),
                    });
                }
            }
            "struct_declaration" => {
                if let Some(id_name) = ts_child_id_text(child, source) {
                    if id_name == name {
                        out.push(TsDeclaration {
                            range: id_range(child),
                            kind: "struct_declaration",
                            container: container.map(String::from),
                        });
                    }
                    if let Some(body) = ts_find_child(child, "struct_body") {
                        collect_declarations(body, source, name, Some(id_name), out);
                    }
                }
            }
            "enum_declaration" => {
                if let Some(id_name) = ts_child_id_text(child, source) {
                    if id_name == name {
                        out.push(TsDeclaration {
                            range: id_range(child),
                            kind: "enum_declaration",
                            container: container.map(String::from),
                        });
                    }
                    // Check enum values
                    if let Some(body) = ts_find_child(child, "enum_body") {
                        let mut ecur = body.walk();
                        for val in body.children(&mut ecur) {
                            if val.kind() == "enum_value" && &source[val.byte_range()] == name {
                                out.push(TsDeclaration {
                                    range: ts_range(val),
                                    kind: "enum_value",
                                    container: Some(id_name.to_string()),
                                });
                            }
                        }
                    }
                }
            }
            "event_definition" | "error_declaration" => {
                if let Some(id_name) = ts_child_id_text(child, source)
                    && id_name == name
                {
                    out.push(TsDeclaration {
                        range: id_range(child),
                        kind: child.kind(),
                        container: container.map(String::from),
                    });
                }
            }
            "user_defined_type_definition" => {
                if let Some(id_name) = ts_child_id_text(child, source)
                    && id_name == name
                {
                    out.push(TsDeclaration {
                        range: id_range(child),
                        kind: "user_defined_type_definition",
                        container: container.map(String::from),
                    });
                }
            }
            // Recurse into blocks, if-else, loops, etc.
            _ => {
                collect_declarations(child, source, name, container, out);
            }
        }
    }
}

/// Collect parameter declarations from a function/constructor node.
fn collect_parameters(
    node: Node,
    source: &str,
    name: &str,
    container: Option<&str>,
    out: &mut Vec<TsDeclaration>,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "parameter"
            && let Some(id_name) = ts_child_id_text(child, source)
            && id_name == name
        {
            out.push(TsDeclaration {
                range: id_range(child),
                kind: "parameter",
                container: container.map(String::from),
            });
        }
    }
}

/// Tree-sitter range helper.
fn ts_range(node: Node) -> Range {
    let s = node.start_position();
    let e = node.end_position();
    Range {
        start: Position::new(s.row as u32, s.column as u32),
        end: Position::new(e.row as u32, e.column as u32),
    }
}

/// Get the range of the identifier child within a declaration node.
fn id_range(node: Node) -> Range {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|c| c.kind() == "identifier" && c.is_named())
        .map(|c| ts_range(c))
        .unwrap_or_else(|| ts_range(node))
}

fn ts_find_child<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor).find(|c| c.kind() == kind)
}

/// Tree-sitter enhanced goto definition.
///
/// Uses tree-sitter to find the identifier under the cursor and its scope,
/// then resolves via the CompletionCache (for cross-file/semantic resolution),
/// and finally uses tree-sitter to find the declaration position in the target file.
///
/// Falls back to None if resolution fails — caller should try the existing AST-based path.
pub fn goto_definition_ts(
    source: &str,
    position: Position,
    file_uri: &Url,
    completion_cache: &crate::completion::CompletionCache,
    text_cache: &HashMap<String, (i32, String)>,
) -> Option<Location> {
    let ctx = cursor_context(source, position)?;

    // Step 1: Try to resolve via CompletionCache to find which file + name the declaration is in.
    // Use the scope chain by names: find the contract scope, then resolve the name.
    let resolved = resolve_via_cache(&ctx, file_uri, completion_cache);

    match resolved {
        Some(ResolvedTarget::SameFile) => {
            // Declaration is in the same file — find it with tree-sitter
            find_best_declaration(source, &ctx, file_uri)
        }
        Some(ResolvedTarget::OtherFile { path, name }) => {
            // Declaration is in another file — read target source and find by name
            let target_source = read_target_source(&path, text_cache);
            let target_source = target_source?;
            let target_uri = Url::from_file_path(&path).ok()?;
            let decls = find_declarations_by_name(&target_source, &name);
            decls.first().map(|d| Location {
                uri: target_uri,
                range: d.range,
            })
        }
        None => {
            // CompletionCache couldn't resolve — try same-file tree-sitter lookup as fallback
            find_best_declaration(source, &ctx, file_uri)
        }
    }
}

#[derive(Debug)]
enum ResolvedTarget {
    /// Declaration is in the same file as the usage.
    SameFile,
    /// Declaration is in a different file.
    OtherFile { path: String, name: String },
}

/// Try to resolve an identifier using the CompletionCache.
///
/// Finds the scope by matching ancestor names (contract, function) against
/// the cache's scope data, then resolves the name to a type and traces
/// back to the declaring file.
fn resolve_via_cache(
    ctx: &CursorContext,
    file_uri: &Url,
    cache: &crate::completion::CompletionCache,
) -> Option<ResolvedTarget> {
    // Find the contract scope node_id by name
    let contract_scope = ctx
        .contract
        .as_ref()
        .and_then(|name| cache.name_to_node_id.get(name.as_str()))
        .copied();

    // Try scope-based resolution: look in the contract's scope_declarations
    if let Some(contract_id) = contract_scope {
        // Check function scope if we're inside one
        if let Some(func_name) = &ctx.function {
            // Find the function scope: look for a scope whose parent is this contract
            // and which has a declaration for this function name
            if let Some(func_scope_id) = find_function_scope(cache, contract_id, func_name) {
                // Check declarations in this function scope first
                if let Some(decls) = cache.scope_declarations.get(&func_scope_id)
                    && decls.iter().any(|d| d.name == ctx.name)
                {
                    return Some(ResolvedTarget::SameFile);
                }
            }
        }

        // Check contract scope declarations (state variables, functions)
        if let Some(decls) = cache.scope_declarations.get(&contract_id)
            && decls.iter().any(|d| d.name == ctx.name)
        {
            return Some(ResolvedTarget::SameFile);
        }

        // Check inherited contracts (C3 linearization)
        if let Some(bases) = cache.linearized_base_contracts.get(&contract_id) {
            for &base_id in bases.iter().skip(1) {
                if let Some(decls) = cache.scope_declarations.get(&base_id)
                    && decls.iter().any(|d| d.name == ctx.name)
                {
                    // Found in a base contract — find which file it's in
                    // Reverse lookup: base_id → contract name → file
                    let base_name = cache
                        .name_to_node_id
                        .iter()
                        .find(|&(_, &id)| id == base_id)
                        .map(|(name, _)| name.clone());

                    if let Some(base_name) = base_name
                        && let Some(path) = find_file_for_contract(cache, &base_name, file_uri)
                    {
                        return Some(ResolvedTarget::OtherFile {
                            path,
                            name: ctx.name.clone(),
                        });
                    }
                    // Base contract might be in the same file
                    return Some(ResolvedTarget::SameFile);
                }
            }
        }
    }

    // Check if the name is a contract/library/interface name
    if cache.name_to_node_id.contains_key(&ctx.name) {
        // Could be same file or different file — check if it's in the current file
        if let Some(path) = find_file_for_contract(cache, &ctx.name, file_uri) {
            let current_path = file_uri.to_file_path().ok()?;
            let current_str = current_path.to_str()?;
            if path == current_str || path.ends_with(current_str) || current_str.ends_with(&path) {
                return Some(ResolvedTarget::SameFile);
            }
            return Some(ResolvedTarget::OtherFile {
                path,
                name: ctx.name.clone(),
            });
        }
        return Some(ResolvedTarget::SameFile);
    }

    // Flat fallback — name_to_type knows about it but we can't determine the file
    if cache.name_to_type.contains_key(&ctx.name) {
        return Some(ResolvedTarget::SameFile);
    }

    None
}

/// Find the scope node_id for a function within a contract.
fn find_function_scope(
    cache: &crate::completion::CompletionCache,
    contract_id: u64,
    func_name: &str,
) -> Option<u64> {
    // Look for a scope whose parent is the contract and which is a function scope.
    // The function name should appear as a declaration in the contract scope,
    // and the function's own scope is the one whose parent is the contract.
    for (&scope_id, &parent_id) in &cache.scope_parent {
        if parent_id == contract_id {
            // This scope's parent is our contract — it might be a function scope.
            // Check if this scope has declarations (functions/blocks do).
            // We also check if the contract declares a function with this name.
            if let Some(contract_decls) = cache.scope_declarations.get(&contract_id)
                && contract_decls.iter().any(|d| d.name == func_name)
            {
                // Found a child scope of the contract — could be the function.
                // Check if this scope_id has child scopes or declarations
                // that match what we'd expect for a function body.
                if cache.scope_declarations.contains_key(&scope_id)
                    || cache.scope_parent.values().any(|&p| p == scope_id)
                {
                    return Some(scope_id);
                }
            }
        }
    }
    None
}

/// Find the file path for a contract by searching the CompletionCache's path_to_file_id.
fn find_file_for_contract(
    cache: &crate::completion::CompletionCache,
    contract_name: &str,
    _file_uri: &Url,
) -> Option<String> {
    // The completion cache doesn't directly map contract → file.
    // But scope_ranges + path_to_file_id can help.
    // For now, check if the contract's node_id appears in any scope_range,
    // then map file_id back to path.
    let node_id = cache.name_to_node_id.get(contract_name)?;
    let scope_range = cache.scope_ranges.iter().find(|r| r.node_id == *node_id)?;
    let file_id = scope_range.file_id;

    // Reverse lookup: file_id → path
    cache
        .path_to_file_id
        .iter()
        .find(|&(_, &fid)| fid == file_id)
        .map(|(path, _)| path.clone())
}

/// Read source for a target file — prefer text_cache (open buffers), fallback to disk.
fn read_target_source(path: &str, text_cache: &HashMap<String, (i32, String)>) -> Option<String> {
    // Try text_cache by URI
    let uri = Url::from_file_path(path).ok()?;
    if let Some((_, content)) = text_cache.get(&uri.to_string()) {
        return Some(content.clone());
    }
    // Fallback to disk
    std::fs::read_to_string(path).ok()
}

/// Find the best matching declaration in the same file.
fn find_best_declaration(source: &str, ctx: &CursorContext, file_uri: &Url) -> Option<Location> {
    let decls = find_declarations_by_name(source, &ctx.name);
    if decls.is_empty() {
        return None;
    }

    // If there's only one declaration, use it
    if decls.len() == 1 {
        return Some(Location {
            uri: file_uri.clone(),
            range: decls[0].range,
        });
    }

    // Multiple declarations — prefer the one in the same contract
    if let Some(contract_name) = &ctx.contract
        && let Some(d) = decls
            .iter()
            .find(|d| d.container.as_deref() == Some(contract_name))
    {
        return Some(Location {
            uri: file_uri.clone(),
            range: d.range,
        });
    }

    // Fallback: return first declaration
    Some(Location {
        uri: file_uri.clone(),
        range: decls[0].range,
    })
}

#[cfg(test)]
mod ts_tests {
    use super::*;

    #[test]
    fn test_cursor_context_state_var() {
        let source = r#"
contract Token {
    uint256 public totalSupply;
    function mint(uint256 amount) public {
        totalSupply += amount;
    }
}
"#;
        // Cursor on `totalSupply` inside mint (line 4, col 8)
        let ctx = cursor_context(source, Position::new(4, 8)).unwrap();
        assert_eq!(ctx.name, "totalSupply");
        assert_eq!(ctx.function.as_deref(), Some("mint"));
        assert_eq!(ctx.contract.as_deref(), Some("Token"));
    }

    #[test]
    fn test_cursor_context_top_level() {
        let source = r#"
contract Foo {}
contract Bar {}
"#;
        // Cursor on `Foo` (line 1, col 9) — the identifier of the contract declaration
        let ctx = cursor_context(source, Position::new(1, 9)).unwrap();
        assert_eq!(ctx.name, "Foo");
        assert!(ctx.function.is_none());
        // The identifier `Foo` is a child of contract_declaration, so contract is set
        assert_eq!(ctx.contract.as_deref(), Some("Foo"));
    }

    #[test]
    fn test_find_declarations() {
        let source = r#"
contract Token {
    uint256 public totalSupply;
    function mint(uint256 amount) public {
        totalSupply += amount;
    }
}
"#;
        let decls = find_declarations_by_name(source, "totalSupply");
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].kind, "state_variable_declaration");
        assert_eq!(decls[0].container.as_deref(), Some("Token"));
    }

    #[test]
    fn test_find_declarations_multiple_contracts() {
        let source = r#"
contract A {
    uint256 public value;
}
contract B {
    uint256 public value;
}
"#;
        let decls = find_declarations_by_name(source, "value");
        assert_eq!(decls.len(), 2);
        assert_eq!(decls[0].container.as_deref(), Some("A"));
        assert_eq!(decls[1].container.as_deref(), Some("B"));
    }

    #[test]
    fn test_find_declarations_enum_value() {
        let source = "contract Foo { enum Status { Active, Paused } }";
        let decls = find_declarations_by_name(source, "Active");
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].kind, "enum_value");
        assert_eq!(decls[0].container.as_deref(), Some("Status"));
    }

    #[test]
    fn test_cursor_context_short_param() {
        let source = r#"
contract Shop {
    uint256 public TAX;
    constructor(uint256 price, uint16 tax, uint16 taxBase) {
        TAX = tax;
    }
}
"#;
        // Cursor on `tax` usage at line 4, col 14 (TAX = tax;)
        let ctx = cursor_context(source, Position::new(4, 14)).unwrap();
        assert_eq!(ctx.name, "tax");
        assert_eq!(ctx.contract.as_deref(), Some("Shop"));

        // Cursor on `TAX` at line 4, col 8
        let ctx2 = cursor_context(source, Position::new(4, 8)).unwrap();
        assert_eq!(ctx2.name, "TAX");

        // Parameters are found as declarations
        let decls = find_declarations_by_name(source, "tax");
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].kind, "parameter");

        let decls_tax_base = find_declarations_by_name(source, "taxBase");
        assert_eq!(decls_tax_base.len(), 1);
        assert_eq!(decls_tax_base[0].kind, "parameter");

        let decls_price = find_declarations_by_name(source, "price");
        assert_eq!(decls_price.len(), 1);
        assert_eq!(decls_price[0].kind, "parameter");

        // State variable is also found
        let decls_tax_upper = find_declarations_by_name(source, "TAX");
        assert_eq!(decls_tax_upper.len(), 1);
        assert_eq!(decls_tax_upper[0].kind, "state_variable_declaration");
    }

    #[test]
    fn test_find_best_declaration_same_contract() {
        let source = r#"
contract A { uint256 public x; }
contract B { uint256 public x; }
"#;
        let ctx = CursorContext {
            name: "x".into(),
            function: None,
            contract: Some("B".into()),
        };
        let uri = Url::parse("file:///test.sol").unwrap();
        let loc = find_best_declaration(source, &ctx, &uri).unwrap();
        // Should pick B's x (line 2), not A's x (line 1)
        assert_eq!(loc.range.start.line, 2);
    }
}
