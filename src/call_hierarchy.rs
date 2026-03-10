//! Call hierarchy support for the Solidity language server.
//!
//! This module provides the data structures and index-building logic for
//! the LSP Call Hierarchy feature (`textDocument/prepareCallHierarchy`,
//! `callHierarchy/incomingCalls`, `callHierarchy/outgoingCalls`).
//!
//! # Architecture
//!
//! The index is built at cache time (in `CachedBuild::new()`) by walking
//! the raw solc AST JSON. It records **call-site edges** from
//! `FunctionCall` nodes (with `kind == "functionCall"`) and
//! `ModifierInvocation` nodes. Edges from `structConstructorCall`,
//! `typeConversion`, event emits, and negative `referencedDeclaration`
//! (builtins) are skipped.
//!
//! Call edges are stored at expression granularity but exposed at
//! function/modifier/contract level for the LSP protocol.
//!
//! # Canonicalization
//!
//! The `call_src` strings extracted from the raw AST use solc's original
//! file IDs, which differ between compilations. After building the index,
//! `CachedBuild::new()` applies `remap_src_canonical()` to all `call_src`
//! strings so they match the canonical `id_to_path_map`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_lsp::lsp_types::{CallHierarchyItem, Range, SymbolKind, Url};

use crate::goto::{CachedBuild, NodeInfo, bytes_to_pos};
use crate::references::byte_to_id;
use crate::solc_ast::DeclNode;
use crate::types::{AbsPath, NodeId, SolcFileId, SourceLoc};

// ── Data structures ────────────────────────────────────────────────────────

/// The kind of call site (function call or modifier invocation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallSiteKind {
    /// A `FunctionCall` with `kind == "functionCall"`.
    FunctionCall,
    /// A `ModifierInvocation` (or base constructor specifier).
    ModifierInvocation,
}

/// A single call-site edge: "caller X calls callee Y at source location Z".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSite {
    /// Node ID of the **caller** (the function/modifier/constructor containing this call).
    pub caller_id: NodeId,
    /// Node ID of the **callee** (the target function/modifier being called).
    pub callee_id: NodeId,
    /// The narrow source location of the call expression itself.
    /// For direct identifier calls: the `expression.src`.
    /// For member access calls: the `expression.memberLocation`.
    pub call_src: String,
    /// The kind of call site.
    pub kind: CallSiteKind,
}

/// Per-file call-site index: maps `AbsPath` → list of `CallSite` records.
pub type CallSiteIndex = HashMap<AbsPath, Vec<CallSite>>;

/// Per-file container-to-callables map: maps `AbsPath` → (container_id → [callable_ids]).
/// A "container" is a contract/interface/library. A "callable" is a function/modifier/constructor.
pub type ContainerCallables = HashMap<AbsPath, HashMap<NodeId, Vec<NodeId>>>;

// ── Index building ─────────────────────────────────────────────────────────

/// AST child keys to traverse when looking for call sites.
/// Same as `goto::CHILD_KEYS` but we import the constant directly.
use crate::goto::CHILD_KEYS;

/// Build the call hierarchy index from the raw `sources` section of solc output.
///
/// Walks the entire AST to find:
/// 1. `FunctionCall` nodes with `kind == "functionCall"` — records a call edge
///    from the enclosing callable to the referenced declaration.
/// 2. `ModifierInvocation` nodes — records a call edge from the enclosing
///    function to the modifier being invoked.
/// 3. Container-callable relationships — which callables belong to which
///    contract/interface/library.
///
/// Returns `(call_sites, container_callables)`.
pub fn build_call_hierarchy_index(
    sources: &serde_json::Value,
) -> (CallSiteIndex, ContainerCallables) {
    let sources_obj = match sources.as_object() {
        Some(obj) => obj,
        None => return (HashMap::new(), HashMap::new()),
    };

    let source_count = sources_obj.len();
    let mut call_sites: CallSiteIndex = HashMap::with_capacity(source_count);
    let mut container_callables: ContainerCallables = HashMap::with_capacity(source_count);

    for (path, source_data) in sources_obj {
        let ast = match source_data.get("ast") {
            Some(a) => a,
            None => continue,
        };

        let abs_path = AbsPath::new(
            ast.get("absolutePath")
                .and_then(|v| v.as_str())
                .unwrap_or(path)
                .to_string(),
        );

        let file_sites = call_sites.entry(abs_path.clone()).or_default();
        let file_containers = container_callables.entry(abs_path.clone()).or_default();

        // Walk the top-level `nodes` array of the SourceUnit.
        if let Some(nodes) = ast.get("nodes").and_then(|v| v.as_array()) {
            for node in nodes {
                walk_for_calls(node, None, None, file_sites, file_containers);
            }
        }
    }

    (call_sites, container_callables)
}

/// Recursively walk an AST node, collecting call-site edges and
/// container-callable relationships.
///
/// `enclosing_callable`: the nearest enclosing function/modifier/constructor ID.
/// `enclosing_container`: the nearest enclosing contract/interface/library ID.
fn walk_for_calls(
    node: &serde_json::Value,
    enclosing_callable: Option<NodeId>,
    enclosing_container: Option<NodeId>,
    call_sites: &mut Vec<CallSite>,
    container_callables: &mut HashMap<NodeId, Vec<NodeId>>,
) {
    let obj = match node.as_object() {
        Some(o) => o,
        None => return,
    };

    let node_type = obj.get("nodeType").and_then(|v| v.as_str()).unwrap_or("");
    let node_id = obj.get("id").and_then(|v| v.as_i64()).map(NodeId);

    // Track the current enclosing callable and container for child traversal.
    let mut current_callable = enclosing_callable;
    let mut current_container = enclosing_container;

    match node_type {
        // ── Container nodes ────────────────────────────────────────────
        "ContractDefinition" => {
            if let Some(id) = node_id {
                current_container = Some(id);
                // Don't reset callable — contracts can't be callables themselves.
            }
        }

        // ── Callable nodes ─────────────────────────────────────────────
        "FunctionDefinition" | "ModifierDefinition" => {
            if let Some(id) = node_id {
                current_callable = Some(id);
                // Register this callable in its container.
                if let Some(container_id) = current_container {
                    container_callables
                        .entry(container_id)
                        .or_default()
                        .push(id);
                }
            }
        }

        // ── FunctionCall nodes ─────────────────────────────────────────
        "FunctionCall" => {
            if let Some(caller_id) = current_callable {
                // Only record `functionCall` kind (skip typeConversion,
                // structConstructorCall).
                let kind_str = obj.get("kind").and_then(|v| v.as_str()).unwrap_or("");
                if kind_str == "functionCall" {
                    // The `expression` child is the callee reference.
                    if let Some(expr) = obj.get("expression") {
                        let expr_type = expr.get("nodeType").and_then(|v| v.as_str()).unwrap_or("");
                        let ref_decl = expr.get("referencedDeclaration").and_then(|v| v.as_i64());

                        if let Some(callee_raw) = ref_decl {
                            // Skip builtins (negative IDs) and event emits.
                            if callee_raw >= 0 {
                                let callee_id = NodeId(callee_raw);
                                // Determine the narrow call-site src.
                                let call_src = if expr_type == "MemberAccess" {
                                    // Use memberLocation for member-access calls
                                    // (e.g., `pool.swap(...)` → location of `.swap`).
                                    expr.get("memberLocation")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_else(|| {
                                            expr.get("src").and_then(|v| v.as_str()).unwrap_or("")
                                        })
                                } else {
                                    // Use expression.src for direct identifier calls
                                    // (e.g., `foo(...)` → location of `foo`).
                                    expr.get("src").and_then(|v| v.as_str()).unwrap_or("")
                                };

                                if !call_src.is_empty() {
                                    call_sites.push(CallSite {
                                        caller_id,
                                        callee_id,
                                        call_src: call_src.to_string(),
                                        kind: CallSiteKind::FunctionCall,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // ── ModifierInvocation nodes ───────────────────────────────────
        "ModifierInvocation" => {
            if let Some(caller_id) = current_callable {
                // The `modifierName` child has the referencedDeclaration.
                if let Some(modifier_name) = obj.get("modifierName") {
                    let ref_decl = modifier_name
                        .get("referencedDeclaration")
                        .and_then(|v| v.as_i64());

                    if let Some(callee_raw) = ref_decl {
                        if callee_raw >= 0 {
                            let callee_id = NodeId(callee_raw);
                            // Use the ModifierInvocation's own src for the call range.
                            let call_src = obj.get("src").and_then(|v| v.as_str()).unwrap_or("");

                            if !call_src.is_empty() {
                                call_sites.push(CallSite {
                                    caller_id,
                                    callee_id,
                                    call_src: call_src.to_string(),
                                    kind: CallSiteKind::ModifierInvocation,
                                });
                            }
                        }
                    }
                }
            }
        }

        _ => {}
    }

    // Recurse into children.
    for key in CHILD_KEYS {
        if let Some(child) = obj.get(*key) {
            if let Some(arr) = child.as_array() {
                for item in arr {
                    walk_for_calls(
                        item,
                        current_callable,
                        current_container,
                        call_sites,
                        container_callables,
                    );
                }
            } else if child.is_object() {
                walk_for_calls(
                    child,
                    current_callable,
                    current_container,
                    call_sites,
                    container_callables,
                );
            }
        }
    }
}

// ── Canonicalization ───────────────────────────────────────────────────────

/// Rewrite file IDs in all `call_src` strings using the canonical remap table.
///
/// Must be called after `build_call_hierarchy_index()` when a `PathInterner`
/// is present, so that `call_src` file IDs match the canonical `id_to_path_map`.
pub fn canonicalize_call_sites(
    call_sites: &mut CallSiteIndex,
    remap: &HashMap<u64, crate::types::FileId>,
) {
    for file_sites in call_sites.values_mut() {
        for site in file_sites.iter_mut() {
            site.call_src = crate::goto::remap_src_canonical(&site.call_src, remap);
        }
    }
}

// ── Query functions ────────────────────────────────────────────────────────

/// Find all incoming calls to a given callee node ID.
///
/// Returns `(caller_id, call_src)` pairs from all files in the index.
pub fn incoming_calls_for(call_sites: &CallSiteIndex, callee_id: NodeId) -> Vec<(NodeId, String)> {
    let mut results = Vec::new();
    for file_sites in call_sites.values() {
        for site in file_sites {
            if site.callee_id == callee_id {
                results.push((site.caller_id, site.call_src.clone()));
            }
        }
    }
    results
}

/// Find all outgoing calls from a given caller node ID.
///
/// Returns `(callee_id, call_src)` pairs from all files in the index.
pub fn outgoing_calls_for(call_sites: &CallSiteIndex, caller_id: NodeId) -> Vec<(NodeId, String)> {
    let mut results = Vec::new();
    for file_sites in call_sites.values() {
        for site in file_sites {
            if site.caller_id == caller_id {
                results.push((site.callee_id, site.call_src.clone()));
            }
        }
    }
    results
}

/// Find all incoming calls to a container (contract/interface/library).
///
/// This is the union of incoming calls to all callables within the container.
pub fn incoming_calls_for_container(
    call_sites: &CallSiteIndex,
    container_callables: &ContainerCallables,
    container_id: NodeId,
) -> Vec<(NodeId, String)> {
    let callable_ids: Vec<NodeId> = container_callables
        .values()
        .flat_map(|file_map| {
            file_map
                .get(&container_id)
                .map(|ids| ids.as_slice())
                .unwrap_or(&[])
        })
        .copied()
        .collect();

    let mut results = Vec::new();
    for file_sites in call_sites.values() {
        for site in file_sites {
            if callable_ids.contains(&site.callee_id) {
                results.push((site.caller_id, site.call_src.clone()));
            }
        }
    }
    results
}

/// Find all outgoing calls from a container (contract/interface/library).
///
/// This is the union of outgoing calls from all callables within the container.
pub fn outgoing_calls_for_container(
    call_sites: &CallSiteIndex,
    container_callables: &ContainerCallables,
    container_id: NodeId,
) -> Vec<(NodeId, String)> {
    let callable_ids: Vec<NodeId> = container_callables
        .values()
        .flat_map(|file_map| {
            file_map
                .get(&container_id)
                .map(|ids| ids.as_slice())
                .unwrap_or(&[])
        })
        .copied()
        .collect();

    let mut results = Vec::new();
    for file_sites in call_sites.values() {
        for site in file_sites {
            if callable_ids.contains(&site.caller_id) {
                results.push((site.callee_id, site.call_src.clone()));
            }
        }
    }
    results
}

// ── LSP conversion helpers ─────────────────────────────────────────────────

/// Convert a `DeclNode` into a `CallHierarchyItem`.
///
/// Uses `node_id_to_source_path` (not `decl.src()` file IDs) to find the
/// file path — this is critical because `decl.src()` uses raw
/// (pre-canonicalization) file IDs that don't match `id_to_path_map`.
///
/// `nodes` is the full node index, used to look up `nameLocation` from
/// the `NodeInfo` (since `extract_decl_nodes` strips `nameLocation` from
/// declaration nodes for memory efficiency).
pub fn decl_to_hierarchy_item(
    decl: &DeclNode,
    node_id: NodeId,
    node_id_to_source_path: &HashMap<NodeId, AbsPath>,
    id_to_path_map: &HashMap<SolcFileId, String>,
    nodes: &HashMap<AbsPath, HashMap<NodeId, NodeInfo>>,
) -> Option<CallHierarchyItem> {
    // Get the file path from node_id_to_source_path (canonical).
    let abs_path = node_id_to_source_path.get(&node_id)?;

    // Find the file on disk and read source bytes to convert byte offsets to positions.
    let file_path = find_file_path(abs_path.as_str(), id_to_path_map)?;
    let source_bytes = std::fs::read(&file_path).ok()?;
    let uri = Url::from_file_path(&file_path).ok()?;

    // Parse the declaration's full src for the range.
    let src_loc = SourceLoc::parse(decl.src())?;
    let start = bytes_to_pos(&source_bytes, src_loc.offset)?;
    let end = bytes_to_pos(&source_bytes, src_loc.end())?;
    let range = Range { start, end };

    // For selection range, use nameLocation from the NodeInfo in the nodes index.
    // (DeclNode's nameLocation is stripped by extract_decl_nodes for memory efficiency.)
    let selection_range = find_node_info(nodes, node_id)
        .and_then(|info| {
            name_loc_range(
                &info.name_location.as_ref().map(|s| s.to_string()),
                &source_bytes,
            )
        })
        .unwrap_or(range);

    let symbol_kind = match decl {
        DeclNode::FunctionDefinition(_) => SymbolKind::FUNCTION,
        DeclNode::ModifierDefinition(_) => SymbolKind::FUNCTION,
        DeclNode::ContractDefinition(c) => match c.contract_kind {
            crate::solc_ast::ContractKind::Interface => SymbolKind::INTERFACE,
            _ => SymbolKind::CLASS,
        },
        _ => SymbolKind::FUNCTION,
    };

    // Store the node ID in `data` so handlers can extract it later.
    let data = Some(serde_json::json!({ "nodeId": node_id.0 }));

    Some(CallHierarchyItem {
        name: decl.name().to_string(),
        kind: symbol_kind,
        tags: None,
        detail: decl.build_signature(),
        uri,
        range,
        selection_range,
        data,
    })
}

/// Convert a `NodeInfo` (from the nodes index) into a `CallHierarchyItem`.
///
/// This is the fallback used when `decl_index` doesn't contain the node
/// (e.g., warm-loaded project builds where `decl_index` is empty).
pub fn node_info_to_hierarchy_item(
    node_id: NodeId,
    info: &NodeInfo,
    id_to_path_map: &HashMap<SolcFileId, String>,
) -> Option<CallHierarchyItem> {
    let src_loc = SourceLoc::parse(info.src.as_str())?;
    let file_path_str = id_to_path_map.get(&src_loc.file_id_str())?;

    let file_path = if std::path::Path::new(file_path_str).is_absolute() {
        std::path::PathBuf::from(file_path_str)
    } else {
        std::env::current_dir().ok()?.join(file_path_str)
    };

    let source_bytes = std::fs::read(&file_path).ok()?;
    let uri = Url::from_file_path(&file_path).ok()?;

    let start = bytes_to_pos(&source_bytes, src_loc.offset)?;
    let end = bytes_to_pos(&source_bytes, src_loc.end())?;
    let range = Range { start, end };

    // Try nameLocation for selection range.
    let selection_range = info
        .name_location
        .as_deref()
        .and_then(|nl| {
            let nl_loc = SourceLoc::parse(nl)?;
            let ns = bytes_to_pos(&source_bytes, nl_loc.offset)?;
            let ne = bytes_to_pos(&source_bytes, nl_loc.end())?;
            Some(Range { start: ns, end: ne })
        })
        .unwrap_or(range);

    let node_type = info.node_type.as_deref().unwrap_or("");
    let kind = match node_type {
        "FunctionDefinition" | "ModifierDefinition" => SymbolKind::FUNCTION,
        "ContractDefinition" => SymbolKind::CLASS,
        _ => SymbolKind::FUNCTION,
    };

    // Extract a name from the nameLocation or use the node type as fallback.
    let name = extract_name_from_source(&source_bytes, &selection_range)
        .unwrap_or_else(|| node_type.to_string());

    let data = Some(serde_json::json!({ "nodeId": node_id.0 }));

    Some(CallHierarchyItem {
        name,
        kind,
        tags: None,
        detail: None,
        uri,
        range,
        selection_range,
        data,
    })
}

/// Convert a `call_src` string to an LSP `Range`.
pub fn call_src_to_range(
    call_src: &str,
    id_to_path_map: &HashMap<SolcFileId, String>,
) -> Option<Range> {
    let loc = SourceLoc::parse(call_src)?;
    let file_path_str = id_to_path_map.get(&loc.file_id_str())?;

    let file_path = if std::path::Path::new(file_path_str).is_absolute() {
        std::path::PathBuf::from(file_path_str)
    } else {
        std::env::current_dir().ok()?.join(file_path_str)
    };

    let source_bytes = std::fs::read(&file_path).ok()?;
    let start = bytes_to_pos(&source_bytes, loc.offset)?;
    let end = bytes_to_pos(&source_bytes, loc.end())?;
    Some(Range { start, end })
}

// ── Internal helpers ───────────────────────────────────────────────────────

/// Find a node in the `nodes` index by ID, searching all files.
pub fn find_node_info<'a>(
    nodes: &'a HashMap<AbsPath, HashMap<NodeId, NodeInfo>>,
    node_id: NodeId,
) -> Option<&'a NodeInfo> {
    for file_nodes in nodes.values() {
        if let Some(info) = file_nodes.get(&node_id) {
            return Some(info);
        }
    }
    None
}

/// Resolve an absolute path to a filesystem path by searching `id_to_path_map`.
fn find_file_path(
    abs_path: &str,
    id_to_path_map: &HashMap<SolcFileId, String>,
) -> Option<std::path::PathBuf> {
    // First, check if abs_path is already an absolute filesystem path.
    let as_path = std::path::Path::new(abs_path);
    if as_path.is_absolute() && as_path.exists() {
        return Some(as_path.to_path_buf());
    }

    // Otherwise, search id_to_path_map for a matching path.
    for file_path in id_to_path_map.values() {
        if file_path == abs_path || file_path.ends_with(abs_path) {
            let fp = std::path::Path::new(file_path);
            if fp.is_absolute() {
                return Some(fp.to_path_buf());
            } else {
                return std::env::current_dir().ok().map(|cwd| cwd.join(fp));
            }
        }
    }

    // Try resolving relative to cwd.
    std::env::current_dir()
        .ok()
        .map(|cwd| cwd.join(abs_path))
        .filter(|p| p.exists())
}

/// Parse a `nameLocation` string into a `Range`.
fn name_loc_range(name_location: &Option<String>, source_bytes: &[u8]) -> Option<Range> {
    let loc_str = name_location.as_deref()?;
    let loc = SourceLoc::parse(loc_str)?;
    let start = bytes_to_pos(source_bytes, loc.offset)?;
    let end = bytes_to_pos(source_bytes, loc.end())?;
    Some(Range { start, end })
}

/// Extract the text at a given range from source bytes.
fn extract_name_from_source(source_bytes: &[u8], range: &Range) -> Option<String> {
    let text = String::from_utf8_lossy(source_bytes);
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(range.start.line as usize)?;
    let start = range.start.character as usize;
    let end = range.end.character as usize;
    if range.start.line == range.end.line && start < line.len() && end <= line.len() {
        Some(line[start..end].to_string())
    } else {
        None
    }
}

/// Resolve a callable at a cursor position.
///
/// Uses `byte_to_id()` to find the innermost node at the cursor, then:
/// 1. If the node itself is a callable declaration (FunctionDefinition,
///    ModifierDefinition, ContractDefinition), return its ID.
/// 2. If the node has `referencedDeclaration` pointing to a callable, return that.
/// 3. Walk up via scope chain to find the enclosing callable.
pub fn resolve_callable_at_position(
    build: &CachedBuild,
    abs_path: &str,
    byte_position: usize,
) -> Option<NodeId> {
    let node_id = byte_to_id(&build.nodes, abs_path, byte_position)?;
    let file_nodes = build.nodes.get(abs_path)?;
    let info = file_nodes.get(&node_id)?;
    let node_type = info.node_type.as_deref().unwrap_or("");

    // Case 1: cursor is directly on a callable declaration.
    if matches!(
        node_type,
        "FunctionDefinition" | "ModifierDefinition" | "ContractDefinition"
    ) {
        return Some(node_id);
    }

    // Case 2: the node references a callable declaration.
    if let Some(ref_id) = info.referenced_declaration {
        // Check if the referenced node is a callable in decl_index.
        if let Some(decl) = build.decl_index.get(&ref_id) {
            if matches!(
                decl,
                DeclNode::FunctionDefinition(_)
                    | DeclNode::ModifierDefinition(_)
                    | DeclNode::ContractDefinition(_)
            ) {
                return Some(ref_id);
            }
        }
        // Also check nodes index for node_type.
        if let Some(ref_info) = find_node_info(&build.nodes, ref_id) {
            let ref_type = ref_info.node_type.as_deref().unwrap_or("");
            if matches!(
                ref_type,
                "FunctionDefinition" | "ModifierDefinition" | "ContractDefinition"
            ) {
                return Some(ref_id);
            }
        }
    }

    // Case 3: walk up via scope to find enclosing callable.
    // Find the narrowest enclosing callable by checking all nodes in this file.
    let mut best_callable: Option<(NodeId, usize)> = None; // (id, span_length)
    for (id, ni) in file_nodes {
        let nt = ni.node_type.as_deref().unwrap_or("");
        if !matches!(
            nt,
            "FunctionDefinition" | "ModifierDefinition" | "ContractDefinition"
        ) {
            continue;
        }
        if let Some(src_loc) = SourceLoc::parse(ni.src.as_str()) {
            if src_loc.offset <= byte_position && byte_position < src_loc.end() {
                // Prefer the narrowest enclosing span (smallest length).
                match best_callable {
                    None => best_callable = Some((*id, src_loc.length)),
                    Some((_, best_len)) if src_loc.length < best_len => {
                        best_callable = Some((*id, src_loc.length));
                    }
                    _ => {}
                }
            }
        }
    }

    best_callable.map(|(id, _)| id)
}
