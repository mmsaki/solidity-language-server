use crate::gas;
use crate::goto::CachedBuild;
use crate::types::SourceLoc;
use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tree_sitter::{Node, Parser};

/// Where to place gas inlay hints on function definitions.
/// Contracts always use the opening brace.
#[allow(dead_code)]
enum FnGasHintPosition {
    /// Show after the opening `{` brace.
    Opening,
    /// Show after the closing `}` brace.
    Closing,
}

/// Change this to switch function gas hint placement.
const FN_GAS_HINT_POSITION: FnGasHintPosition = FnGasHintPosition::Closing;

/// Parameter info resolved from the AST for a callable.
#[derive(Debug, Clone)]
struct ParamInfo {
    /// Parameter names from the declaration.
    names: Vec<String>,
    /// Number of leading params to skip (1 for using-for library calls).
    skip: usize,
}

/// Call-site info extracted from the AST, keyed by source byte offset.
#[derive(Debug, Clone)]
struct CallSite {
    /// The resolved parameter info for this specific call.
    info: ParamInfo,
    /// Function/event name (for matching with tree-sitter).
    name: String,
}

/// Both lookup strategies: exact byte-offset match and (name, arg_count) fallback.
/// Built once per file when the AST is cached, reused on every inlay hint request.
#[derive(Debug, Clone)]
pub struct HintLookup {
    /// Primary: byte_offset → CallSite (exact match when AST offsets are fresh).
    by_offset: HashMap<usize, CallSite>,
    /// Fallback: (name, arg_count) → ParamInfo (works even with stale offsets).
    by_name: HashMap<(String, usize), ParamInfo>,
}

/// Pre-computed hint lookups for all files, keyed by absolutePath.
/// Built once in `CachedBuild::new()`, reused on every inlay hint request.
pub type HintIndex = HashMap<String, HintLookup>;

/// Build the hint index for all files from the AST sources.
/// Called once in `CachedBuild::new()`.
pub fn build_hint_index(sources: &Value) -> HintIndex {
    let id_index = build_id_index(sources);
    let mut hint_index = HashMap::new();

    if let Some(obj) = sources.as_object() {
        for (_, source_data) in obj {
            if let Some(ast) = source_data.get("ast")
                && let Some(abs_path) = ast.get("absolutePath").and_then(|v| v.as_str())
            {
                let lookup = build_hint_lookup(ast, &id_index);
                hint_index.insert(abs_path.to_string(), lookup);
            }
        }
    }

    hint_index
}

/// Generate inlay hints for a given range of source.
///
/// Uses tree-sitter on the **live buffer** for argument positions (so hints
/// follow edits in real time) and the pre-cached hint index for semantic
/// info (parameter names via `referencedDeclaration`).
pub fn inlay_hints(
    build: &CachedBuild,
    uri: &Url,
    range: Range,
    live_source: &[u8],
) -> Vec<InlayHint> {
    let path_str = match uri.to_file_path() {
        Ok(p) => p.to_str().unwrap_or("").to_string(),
        Err(_) => return vec![],
    };

    let abs = match build
        .path_to_abs
        .iter()
        .find(|(k, _)| path_str.ends_with(k.as_str()))
    {
        Some((_, v)) => v.clone(),
        None => return vec![],
    };

    // Use the pre-cached hint lookup for this file
    let lookup = match build.hint_index.get(&abs) {
        Some(l) => l,
        None => return vec![],
    };

    // Walk tree-sitter on the live buffer for real-time argument positions
    let source_str = String::from_utf8_lossy(live_source);
    let tree = match ts_parse(&source_str) {
        Some(t) => t,
        None => return vec![],
    };

    let mut hints = Vec::new();
    collect_ts_hints(tree.root_node(), &source_str, &range, lookup, &mut hints);

    // Gas inlay hints: use tree-sitter positions (tracks live buffer)
    if !build.gas_index.is_empty() {
        collect_ts_gas_hints(
            tree.root_node(),
            &source_str,
            &range,
            &build.gas_index,
            &abs,
            &mut hints,
        );
    }

    hints
}

/// Build a flat node-id → AST-node index from all sources.
/// This is O(total_nodes) and replaces the O(calls × total_nodes)
/// `find_declaration` that walked the entire AST per lookup.
fn build_id_index(sources: &Value) -> HashMap<u64, &Value> {
    let mut index = HashMap::new();
    if let Some(obj) = sources.as_object() {
        for (_, source_data) in obj {
            if let Some(ast) = source_data.get("ast") {
                index_node_ids(ast, &mut index);
            }
        }
    }
    index
}

/// Recursively index all nodes that have an `id` field.
fn index_node_ids<'a>(node: &'a Value, index: &mut HashMap<u64, &'a Value>) {
    if let Some(id) = node.get("id").and_then(|v| v.as_u64()) {
        index.insert(id, node);
    }
    for key in crate::goto::CHILD_KEYS {
        if let Some(child) = node.get(*key) {
            if child.is_array() {
                if let Some(arr) = child.as_array() {
                    for item in arr {
                        index_node_ids(item, index);
                    }
                }
            } else if child.is_object() {
                index_node_ids(child, index);
            }
        }
    }
    if let Some(nodes) = node.get("nodes").and_then(|v| v.as_array()) {
        for child in nodes {
            index_node_ids(child, index);
        }
    }
}

/// Parse Solidity source with tree-sitter.
fn ts_parse(source: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_solidity::LANGUAGE.into())
        .expect("failed to load Solidity grammar");
    parser.parse(source, None)
}

/// Build both lookup strategies from the AST.
fn build_hint_lookup(file_ast: &Value, id_index: &HashMap<u64, &Value>) -> HintLookup {
    let mut lookup = HintLookup {
        by_offset: HashMap::new(),
        by_name: HashMap::new(),
    };
    collect_ast_calls(file_ast, id_index, &mut lookup);
    lookup
}

/// Parse the `src` field ("offset:length:fileId") and return the byte offset.
fn parse_src_offset(node: &Value) -> Option<usize> {
    let src = node.get("src").and_then(|v| v.as_str())?;
    SourceLoc::parse(src).map(|loc| loc.offset)
}

/// Recursively walk AST nodes collecting call site info.
fn collect_ast_calls(node: &Value, id_index: &HashMap<u64, &Value>, lookup: &mut HintLookup) {
    let node_type = node.get("nodeType").and_then(|v| v.as_str()).unwrap_or("");

    match node_type {
        "FunctionCall" => {
            if let Some((name, info)) = extract_call_info(node, id_index) {
                let arg_count = node
                    .get("arguments")
                    .and_then(|v| v.as_array())
                    .map(|a| a.len())
                    .unwrap_or(0);
                if let Some(offset) = parse_src_offset(node) {
                    lookup.by_offset.insert(
                        offset,
                        CallSite {
                            info: ParamInfo {
                                names: info.names.clone(),
                                skip: info.skip,
                            },
                            name: name.clone(),
                        },
                    );
                }
                lookup.by_name.entry((name, arg_count)).or_insert(info);
            }
        }
        "EmitStatement" => {
            if let Some(event_call) = node.get("eventCall")
                && let Some((name, info)) = extract_call_info(event_call, id_index)
            {
                let arg_count = event_call
                    .get("arguments")
                    .and_then(|v| v.as_array())
                    .map(|a| a.len())
                    .unwrap_or(0);
                if let Some(offset) = parse_src_offset(node) {
                    lookup.by_offset.insert(
                        offset,
                        CallSite {
                            info: ParamInfo {
                                names: info.names.clone(),
                                skip: info.skip,
                            },
                            name: name.clone(),
                        },
                    );
                }
                lookup.by_name.entry((name, arg_count)).or_insert(info);
            }
        }
        _ => {}
    }

    // Recurse into children
    for key in crate::goto::CHILD_KEYS {
        if let Some(child) = node.get(*key) {
            if child.is_array() {
                if let Some(arr) = child.as_array() {
                    for item in arr {
                        collect_ast_calls(item, id_index, lookup);
                    }
                }
            } else if child.is_object() {
                collect_ast_calls(child, id_index, lookup);
            }
        }
    }
}

/// Extract function/event name and parameter info from an AST FunctionCall node.
fn extract_call_info(node: &Value, id_index: &HashMap<u64, &Value>) -> Option<(String, ParamInfo)> {
    let args = node.get("arguments")?.as_array()?;
    if args.is_empty() {
        return None;
    }

    // Skip struct constructors with named args
    let kind = node.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    if kind == "structConstructorCall"
        && node
            .get("names")
            .and_then(|v| v.as_array())
            .is_some_and(|n| !n.is_empty())
    {
        return None;
    }

    let expr = node.get("expression")?;
    let decl_id = expr.get("referencedDeclaration").and_then(|v| v.as_u64())?;

    let decl_node = id_index.get(&decl_id)?;
    let names = get_parameter_names(decl_node)?;

    // Extract the function name from the expression
    let func_name = extract_function_name(expr)?;

    // Using-for library calls pass the receiver as the implicit first param,
    // so the AST has one fewer arg than the declaration has params.
    // Direct library calls (Transaction.addTax) and struct constructors
    // pass all params explicitly — arg count matches param count.
    let arg_count = node
        .get("arguments")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let skip = if is_member_access(expr) && arg_count < names.len() {
        1
    } else {
        0
    };

    Some((func_name, ParamInfo { names, skip }))
}

/// Extract the function/event name from an AST expression node.
fn extract_function_name(expr: &Value) -> Option<String> {
    let node_type = expr.get("nodeType").and_then(|v| v.as_str())?;
    match node_type {
        "Identifier" => expr.get("name").and_then(|v| v.as_str()).map(String::from),
        "MemberAccess" => expr
            .get("memberName")
            .and_then(|v| v.as_str())
            .map(String::from),
        _ => None,
    }
}

/// Check if expression is a MemberAccess (potential using-for call).
fn is_member_access(expr: &Value) -> bool {
    expr.get("nodeType")
        .and_then(|v| v.as_str())
        .is_some_and(|t| t == "MemberAccess")
}

// ── Tree-sitter walk ──────────────────────────────────────────────────────

/// Look up param info: try exact byte-offset match first, fall back to (name, arg_count).
fn lookup_info<'a>(
    lookup: &'a HintLookup,
    offset: usize,
    name: &str,
    arg_count: usize,
) -> Option<&'a ParamInfo> {
    // Exact match by byte offset (works when AST is fresh)
    if let Some(site) = lookup.by_offset.get(&offset)
        && site.name == name
    {
        return Some(&site.info);
    }
    // Fallback by (name, arg_count) (works with stale offsets after edits)
    lookup.by_name.get(&(name.to_string(), arg_count))
}

/// Recursively walk tree-sitter nodes, emitting hints for calls in the visible range.
fn collect_ts_hints(
    node: Node,
    source: &str,
    range: &Range,
    lookup: &HintLookup,
    hints: &mut Vec<InlayHint>,
) {
    // Quick range check — skip nodes entirely outside the visible range
    let node_start = node.start_position();
    let node_end = node.end_position();
    if (node_end.row as u32) < range.start.line || (node_start.row as u32) > range.end.line {
        return;
    }

    match node.kind() {
        "call_expression" => {
            emit_call_hints(node, source, lookup, hints);
        }
        "emit_statement" => {
            emit_emit_hints(node, source, lookup, hints);
        }
        _ => {}
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_ts_hints(child, source, range, lookup, hints);
    }
}

/// Emit parameter hints for a `call_expression` node.
fn emit_call_hints(node: Node, source: &str, lookup: &HintLookup, hints: &mut Vec<InlayHint>) {
    let func_name = match ts_call_function_name(node, source) {
        Some(n) => n,
        None => return,
    };

    let args = ts_call_arguments(node);
    if args.is_empty() {
        return;
    }

    let info = match lookup_info(lookup, node.start_byte(), func_name, args.len()) {
        Some(i) => i,
        None => return,
    };

    emit_param_hints(&args, info, hints);
}

/// Emit parameter hints for an `emit_statement` node.
fn emit_emit_hints(node: Node, source: &str, lookup: &HintLookup, hints: &mut Vec<InlayHint>) {
    let event_name = match ts_emit_event_name(node, source) {
        Some(n) => n,
        None => return,
    };

    let args = ts_call_arguments(node);
    if args.is_empty() {
        return;
    }

    let info = match lookup_info(lookup, node.start_byte(), event_name, args.len()) {
        Some(i) => i,
        None => return,
    };

    emit_param_hints(&args, info, hints);
}

/// Emit InlayHint items for each argument, using tree-sitter positions.
fn emit_param_hints(args: &[Node], info: &ParamInfo, hints: &mut Vec<InlayHint>) {
    for (i, arg) in args.iter().enumerate() {
        let pi = i + info.skip;
        if pi >= info.names.len() || info.names[pi].is_empty() {
            continue;
        }

        let start = arg.start_position();
        let position = Position::new(start.row as u32, start.column as u32);

        hints.push(InlayHint {
            position,
            kind: Some(InlayHintKind::PARAMETER),
            label: InlayHintLabel::String(format!("{}:", info.names[pi])),
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: Some(true),
            data: None,
        });
    }
}

// ── Tree-sitter helpers ───────────────────────────────────────────────────

/// Get the function name from a `call_expression` node.
///
/// For `transfer(...)` → "transfer"
/// For `PRICE.addTax(...)` → "addTax"
fn ts_call_function_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let func_expr = node.child_by_field_name("function")?;
    // The expression wrapper has one named child
    let inner = first_named_child(func_expr)?;
    match inner.kind() {
        "identifier" => Some(&source[inner.byte_range()]),
        "member_expression" => {
            let prop = inner.child_by_field_name("property")?;
            Some(&source[prop.byte_range()])
        }
        _ => None,
    }
}

/// Get the event name from an `emit_statement` node.
fn ts_emit_event_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let name_expr = node.child_by_field_name("name")?;
    let inner = first_named_child(name_expr)?;
    match inner.kind() {
        "identifier" => Some(&source[inner.byte_range()]),
        "member_expression" => {
            let prop = inner.child_by_field_name("property")?;
            Some(&source[prop.byte_range()])
        }
        _ => None,
    }
}

/// Collect `call_argument` children from a node (works for both
/// `call_expression` and `emit_statement` since `_call_arguments` is hidden).
fn ts_call_arguments(node: Node) -> Vec<Node> {
    let mut args = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "call_argument" {
            args.push(child);
        }
    }
    args
}

/// Get the first named child of a node.
fn first_named_child(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    node.children(&mut cursor).find(|c| c.is_named())
}

// ── Gas inlay hints (tree-sitter based) ──────────────────────────────────

/// Walk tree-sitter nodes for function/contract definitions, emitting gas
/// cost hints using **live buffer positions** so they track edits in real time.
fn collect_ts_gas_hints(
    node: Node,
    source: &str,
    range: &Range,
    gas_index: &gas::GasIndex,
    abs_path: &str,
    hints: &mut Vec<InlayHint>,
) {
    let node_start = node.start_position();
    let node_end = node.end_position();
    if (node_end.row as u32) < range.start.line || (node_start.row as u32) > range.end.line {
        return;
    }

    match node.kind() {
        "function_definition" => {
            if let Some(hint) = ts_gas_hint_for_function(node, source, range, gas_index, abs_path) {
                hints.push(hint);
            }
        }
        "contract_declaration" | "library_declaration" | "interface_declaration" => {
            if let Some(hint) = ts_gas_hint_for_contract(node, source, range, gas_index, abs_path) {
                hints.push(hint);
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_ts_gas_hints(child, source, range, gas_index, abs_path, hints);
    }
}

/// Extract the identifier (name) child from a tree-sitter node.
fn ts_node_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|c| c.kind() == "identifier" && c.is_named())
        .map(|c| &source[c.byte_range()])
}

/// Find the opening `{` position of a body node.
fn ts_body_open_brace(node: Node, body_kind: &str) -> Option<Position> {
    let mut cursor = node.walk();
    let body = node.children(&mut cursor).find(|c| c.kind() == body_kind)?;
    let start = body.start_position();
    Some(Position::new(start.row as u32, start.column as u32))
}

/// Find the closing `}` position of a body node.
fn ts_body_close_brace(node: Node, body_kind: &str) -> Option<Position> {
    let mut cursor = node.walk();
    let body = node.children(&mut cursor).find(|c| c.kind() == body_kind)?;
    let end = body.end_position();
    // end_position points one past the `}`, so column - 1
    Some(Position::new(
        end.row as u32,
        end.column.saturating_sub(1) as u32,
    ))
}

/// Find the enclosing contract name for a function_definition node.
fn ts_enclosing_contract_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "contract_declaration"
            || p.kind() == "library_declaration"
            || p.kind() == "interface_declaration"
        {
            return ts_node_name(p, source);
        }
        parent = p.parent();
    }
    None
}

/// Find the gas index key matching a file path and contract name.
fn find_gas_key<'a>(
    gas_index: &'a gas::GasIndex,
    abs_path: &str,
    contract_name: &str,
) -> Option<&'a str> {
    let exact = format!("{abs_path}:{contract_name}");
    if gas_index.contains_key(&exact) {
        return Some(gas_index.get_key_value(&exact)?.0.as_str());
    }
    let file_name = std::path::Path::new(abs_path).file_name()?.to_str()?;
    let suffix = format!("{file_name}:{contract_name}");
    gas_index
        .keys()
        .find(|k| k.ends_with(&suffix))
        .map(|k| k.as_str())
}

/// Create a gas inlay hint for a function definition using tree-sitter positions.
fn ts_gas_hint_for_function(
    node: Node,
    source: &str,
    range: &Range,
    gas_index: &gas::GasIndex,
    abs_path: &str,
) -> Option<InlayHint> {
    let fn_name = ts_node_name(node, source)?;
    let contract_name = ts_enclosing_contract_name(node, source)?;
    let gas_key = find_gas_key(gas_index, abs_path, contract_name)?;
    let contract_gas = gas_index.get(gas_key)?;

    let prefix = format!("{fn_name}(");
    let cost = contract_gas
        .external
        .iter()
        .find(|(sig, _)| sig.starts_with(&prefix))
        .map(|(_, c)| c.as_str())
        .or_else(|| {
            contract_gas
                .internal
                .iter()
                .find(|(sig, _)| sig.starts_with(&prefix))
                .map(|(_, c)| c.as_str())
        })?;

    // Position: opening or closing brace based on FN_GAS_HINT_POSITION
    let (brace_pos, offset) = match FN_GAS_HINT_POSITION {
        FnGasHintPosition::Opening => (ts_body_open_brace(node, "function_body")?, 1),
        FnGasHintPosition::Closing => (ts_body_close_brace(node, "function_body")?, 1),
    };
    if brace_pos.line < range.start.line || brace_pos.line > range.end.line {
        return None;
    }

    Some(InlayHint {
        position: Position::new(brace_pos.line, brace_pos.character + offset),
        kind: Some(InlayHintKind::TYPE),
        label: InlayHintLabel::String(format!("{} {}", gas::GAS_ICON, gas::format_gas(cost))),
        text_edits: None,
        tooltip: Some(InlayHintTooltip::String("Estimated gas cost".to_string())),
        padding_left: Some(true),
        padding_right: None,
        data: None,
    })
}

/// Create a gas inlay hint for a contract/library/interface definition.
/// Always uses the opening brace.
fn ts_gas_hint_for_contract(
    node: Node,
    source: &str,
    range: &Range,
    gas_index: &gas::GasIndex,
    abs_path: &str,
) -> Option<InlayHint> {
    let contract_name = ts_node_name(node, source)?;
    let gas_key = find_gas_key(gas_index, abs_path, contract_name)?;
    let contract_gas = gas_index.get(gas_key)?;

    // Prefer totalCost, but when it's "infinite" show codeDepositCost instead
    let display_cost = match contract_gas.creation.get("totalCost").map(|s| s.as_str()) {
        Some("infinite") | None => contract_gas
            .creation
            .get("codeDepositCost")
            .map(|s| s.as_str())?,
        Some(total) => total,
    };

    let brace_pos = ts_body_open_brace(node, "contract_body")?;
    if brace_pos.line < range.start.line || brace_pos.line > range.end.line {
        return None;
    }

    Some(InlayHint {
        position: Position::new(brace_pos.line, brace_pos.character + 1),
        kind: Some(InlayHintKind::TYPE),
        label: InlayHintLabel::String(format!(
            "{} Deploy: {}",
            gas::GAS_ICON,
            gas::format_gas(display_cost)
        )),
        text_edits: None,
        tooltip: Some(InlayHintTooltip::String(format!(
            "Deploy cost — code deposit: {}, execution: {}",
            gas::format_gas(
                contract_gas
                    .creation
                    .get("codeDepositCost")
                    .map(|s| s.as_str())
                    .unwrap_or("?")
            ),
            gas::format_gas(
                contract_gas
                    .creation
                    .get("executionCost")
                    .map(|s| s.as_str())
                    .unwrap_or("?")
            )
        ))),
        padding_left: Some(true),
        padding_right: None,
        data: None,
    })
}

// ── AST helpers ──────────────────────────────────────────────────────────

/// Extract parameter names from a function/event/error/struct declaration.
fn get_parameter_names(decl: &Value) -> Option<Vec<String>> {
    // Functions, events, errors: parameters.parameters[]
    // Structs: members[]
    let items = decl
        .get("parameters")
        .and_then(|p| p.get("parameters"))
        .and_then(|v| v.as_array())
        .or_else(|| decl.get("members").and_then(|v| v.as_array()))?;
    Some(
        items
            .iter()
            .map(|p| {
                p.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parameter_names() {
        let decl: Value = serde_json::json!({
            "parameters": {
                "parameters": [
                    {"name": "to", "nodeType": "VariableDeclaration"},
                    {"name": "amount", "nodeType": "VariableDeclaration"},
                ]
            }
        });
        let names = get_parameter_names(&decl).unwrap();
        assert_eq!(names, vec!["to", "amount"]);
    }

    #[test]
    fn test_ts_call_function_name() {
        let source = r#"
contract Foo {
    function bar(uint x) public {}
    function test() public {
        bar(42);
    }
}
"#;
        let tree = ts_parse(source).unwrap();
        let mut found = Vec::new();
        find_calls(tree.root_node(), source, &mut found);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], "bar");
    }

    #[test]
    fn test_ts_member_call_name() {
        let source = r#"
contract Foo {
    function test() public {
        PRICE.addTax(TAX, TAX_BASE);
    }
}
"#;
        let tree = ts_parse(source).unwrap();
        let mut found = Vec::new();
        find_calls(tree.root_node(), source, &mut found);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], "addTax");
    }

    #[test]
    fn test_ts_emit_event_name() {
        let source = r#"
contract Foo {
    event Purchase(address buyer, uint256 price);
    function test() public {
        emit Purchase(msg.sender, 100);
    }
}
"#;
        let tree = ts_parse(source).unwrap();
        let mut found = Vec::new();
        find_emits(tree.root_node(), source, &mut found);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], "Purchase");
    }

    #[test]
    fn test_ts_call_arguments_count() {
        let source = r#"
contract Foo {
    function bar(uint x, uint y) public {}
    function test() public {
        bar(1, 2);
    }
}
"#;
        let tree = ts_parse(source).unwrap();
        let mut arg_counts = Vec::new();
        find_call_arg_counts(tree.root_node(), &mut arg_counts);
        assert_eq!(arg_counts, vec![2]);
    }

    #[test]
    fn test_ts_argument_positions_follow_live_buffer() {
        // Simulate an edited buffer with extra whitespace
        let source = r#"
contract Foo {
    function bar(uint x, uint y) public {}
    function test() public {
        bar(
            1,
            2
        );
    }
}
"#;
        let tree = ts_parse(source).unwrap();
        let mut positions = Vec::new();
        find_arg_positions(tree.root_node(), &mut positions);
        // First arg "1" is on line 5 (0-indexed), second "2" on line 6
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0].0, 5); // row of "1"
        assert_eq!(positions[1].0, 6); // row of "2"
    }

    // Test helpers

    fn find_calls<'a>(node: Node<'a>, source: &'a str, out: &mut Vec<&'a str>) {
        if node.kind() == "call_expression"
            && let Some(name) = ts_call_function_name(node, source)
        {
            out.push(name);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            find_calls(child, source, out);
        }
    }

    fn find_emits<'a>(node: Node<'a>, source: &'a str, out: &mut Vec<&'a str>) {
        if node.kind() == "emit_statement"
            && let Some(name) = ts_emit_event_name(node, source)
        {
            out.push(name);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            find_emits(child, source, out);
        }
    }

    fn find_call_arg_counts(node: Node, out: &mut Vec<usize>) {
        if node.kind() == "call_expression" {
            out.push(ts_call_arguments(node).len());
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            find_call_arg_counts(child, out);
        }
    }

    fn find_arg_positions(node: Node, out: &mut Vec<(usize, usize)>) {
        if node.kind() == "call_expression" {
            for arg in ts_call_arguments(node) {
                let p = arg.start_position();
                out.push((p.row, p.column));
            }
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            find_arg_positions(child, out);
        }
    }
}
