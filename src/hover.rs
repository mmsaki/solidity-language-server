use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Url};

use crate::gas::{self, GasIndex};
use crate::goto::{CHILD_KEYS, cache_ids, pos_to_bytes};
use crate::references::{byte_to_decl_via_external_refs, byte_to_id};
use crate::types::NodeId;

/// Find the raw AST node with the given id by walking all sources.
pub fn find_node_by_id(sources: &Value, target_id: NodeId) -> Option<&Value> {
    let sources_obj = sources.as_object()?;
    for (_path, source_data) in sources_obj {
        let ast = source_data.get("ast")?;

        // Check root
        if ast.get("id").and_then(|v| v.as_u64()) == Some(target_id.0) {
            return Some(ast);
        }

        let mut stack = vec![ast];
        while let Some(node) = stack.pop() {
            if node.get("id").and_then(|v| v.as_u64()) == Some(target_id.0) {
                return Some(node);
            }
            for key in CHILD_KEYS {
                if let Some(value) = node.get(key) {
                    match value {
                        Value::Array(arr) => stack.extend(arr.iter()),
                        Value::Object(_) => stack.push(value),
                        _ => {}
                    }
                }
            }
        }
    }
    None
}

/// Extract documentation text from a node.
/// Handles both object form `{text: "..."}` and plain string form.
pub fn extract_documentation(node: &Value) -> Option<String> {
    let doc = node.get("documentation")?;
    match doc {
        Value::Object(_) => doc
            .get("text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

/// Extract the selector from a declaration node.
/// Returns (selector_hex, selector_kind) where kind is "function", "error", or "event".
pub fn extract_selector(node: &Value) -> Option<(String, &'static str)> {
    let node_type = node.get("nodeType").and_then(|v| v.as_str())?;
    match node_type {
        "FunctionDefinition" => node
            .get("functionSelector")
            .and_then(|v| v.as_str())
            .map(|s| (s.to_string(), "function")),
        "VariableDeclaration" => node
            .get("functionSelector")
            .and_then(|v| v.as_str())
            .map(|s| (s.to_string(), "function")),
        "ErrorDefinition" => node
            .get("errorSelector")
            .and_then(|v| v.as_str())
            .map(|s| (s.to_string(), "error")),
        "EventDefinition" => node
            .get("eventSelector")
            .and_then(|v| v.as_str())
            .map(|s| (s.to_string(), "event")),
        _ => None,
    }
}

/// Resolve `@inheritdoc ParentName` by matching function selectors.
///
/// 1. Parse the parent contract name from `@inheritdoc ParentName`
/// 2. Get the declaration's `functionSelector`
/// 3. Find the parent contract in `baseContracts` of the scope contract
/// 4. Match by selector in the parent's child nodes
/// 5. Return the matched parent node's documentation
pub fn resolve_inheritdoc<'a>(
    sources: &'a Value,
    decl_node: &'a Value,
    doc_text: &str,
) -> Option<String> {
    // Parse "@inheritdoc ParentName"
    let parent_name = doc_text
        .lines()
        .find_map(|line| {
            let trimmed = line.trim().trim_start_matches('*').trim();
            trimmed.strip_prefix("@inheritdoc ")
        })?
        .trim();

    // Get the selector from the implementation function
    let (impl_selector, _) = extract_selector(decl_node)?;

    // Get the scope (containing contract id)
    let scope_id = decl_node.get("scope").and_then(|v| v.as_u64())?;

    // Find the scope contract
    let scope_contract = find_node_by_id(sources, NodeId(scope_id))?;

    // Find the parent contract in baseContracts by name
    let base_contracts = scope_contract
        .get("baseContracts")
        .and_then(|v| v.as_array())?;
    let parent_id = base_contracts.iter().find_map(|base| {
        let name = base
            .get("baseName")
            .and_then(|bn| bn.get("name"))
            .and_then(|n| n.as_str())?;
        if name == parent_name {
            base.get("baseName")
                .and_then(|bn| bn.get("referencedDeclaration"))
                .and_then(|v| v.as_u64())
        } else {
            None
        }
    })?;

    // Find the parent contract node
    let parent_contract = find_node_by_id(sources, NodeId(parent_id))?;

    // Search parent's children for matching selector
    let parent_nodes = parent_contract.get("nodes").and_then(|v| v.as_array())?;
    for child in parent_nodes {
        if let Some((child_selector, _)) = extract_selector(child)
            && child_selector == impl_selector
        {
            return extract_documentation(child);
        }
    }

    None
}

/// Format NatSpec documentation as markdown.
/// Strips leading `@` tags and formats them nicely.
/// When `inherited_doc` is provided, it replaces `@inheritdoc` lines with the resolved content.
pub fn format_natspec(text: &str, inherited_doc: Option<&str>) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut in_params = false;
    let mut in_returns = false;

    for raw_line in text.lines() {
        let line = raw_line.trim().trim_start_matches('*').trim();
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("@title ") {
            in_params = false;
            in_returns = false;
            lines.push(format!("**{rest}**"));
            lines.push(String::new());
        } else if let Some(rest) = line.strip_prefix("@notice ") {
            in_params = false;
            in_returns = false;
            lines.push(rest.to_string());
        } else if let Some(rest) = line.strip_prefix("@dev ") {
            in_params = false;
            in_returns = false;
            lines.push(String::new());
            lines.push("**@dev**".to_string());
            lines.push(format!("*{rest}*"));
        } else if let Some(rest) = line.strip_prefix("@param ") {
            if !in_params {
                in_params = true;
                in_returns = false;
                lines.push(String::new());
                lines.push("**Parameters:**".to_string());
            }
            if let Some((name, desc)) = rest.split_once(' ') {
                lines.push(format!("- `{name}` — {desc}"));
            } else {
                lines.push(format!("- `{rest}`"));
            }
        } else if let Some(rest) = line.strip_prefix("@return ") {
            if !in_returns {
                in_returns = true;
                in_params = false;
                lines.push(String::new());
                lines.push("**Returns:**".to_string());
            }
            if let Some((name, desc)) = rest.split_once(' ') {
                lines.push(format!("- `{name}` — {desc}"));
            } else {
                lines.push(format!("- `{rest}`"));
            }
        } else if let Some(rest) = line.strip_prefix("@author ") {
            in_params = false;
            in_returns = false;
            lines.push(format!("*@author {rest}*"));
        } else if line.starts_with("@inheritdoc ") {
            // Resolve inherited docs if available
            if let Some(inherited) = inherited_doc {
                // Recursively format the inherited doc (it won't have another @inheritdoc)
                let formatted = format_natspec(inherited, None);
                if !formatted.is_empty() {
                    lines.push(formatted);
                }
            } else {
                let parent = line.strip_prefix("@inheritdoc ").unwrap_or("");
                lines.push(format!("*Inherits documentation from `{parent}`*"));
            }
        } else if line.starts_with('@') {
            // Any other tag (@custom:xyz, @dev, etc.)
            in_params = false;
            in_returns = false;
            if let Some((tag, rest)) = line.split_once(' ') {
                lines.push(String::new());
                lines.push(format!("**{tag}**"));
                lines.push(format!("*{rest}*"));
            } else {
                lines.push(String::new());
                lines.push(format!("**{line}**"));
            }
        } else {
            // Continuation line
            lines.push(line.to_string());
        }
    }

    lines.join("\n")
}

/// Build a function/modifier signature string from a raw AST node.
fn build_function_signature(node: &Value) -> Option<String> {
    let node_type = node.get("nodeType").and_then(|v| v.as_str())?;
    let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");

    match node_type {
        "FunctionDefinition" => {
            let kind = node
                .get("kind")
                .and_then(|v| v.as_str())
                .unwrap_or("function");
            let visibility = node
                .get("visibility")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let state_mutability = node
                .get("stateMutability")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let params = format_parameters(node.get("parameters"));
            let returns = format_parameters(node.get("returnParameters"));

            let mut sig = match kind {
                "constructor" => format!("constructor({params})"),
                "receive" => "receive() external payable".to_string(),
                "fallback" => format!("fallback({params})"),
                _ => format!("function {name}({params})"),
            };

            if !visibility.is_empty() && kind != "constructor" && kind != "receive" {
                sig.push_str(&format!(" {visibility}"));
            }
            if !state_mutability.is_empty() && state_mutability != "nonpayable" {
                sig.push_str(&format!(" {state_mutability}"));
            }
            if !returns.is_empty() {
                sig.push_str(&format!(" returns ({returns})"));
            }
            Some(sig)
        }
        "ModifierDefinition" => {
            let params = format_parameters(node.get("parameters"));
            Some(format!("modifier {name}({params})"))
        }
        "EventDefinition" => {
            let params = format_parameters(node.get("parameters"));
            Some(format!("event {name}({params})"))
        }
        "ErrorDefinition" => {
            let params = format_parameters(node.get("parameters"));
            Some(format!("error {name}({params})"))
        }
        "VariableDeclaration" => {
            let type_str = node
                .get("typeDescriptions")
                .and_then(|v| v.get("typeString"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let visibility = node
                .get("visibility")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let mutability = node
                .get("mutability")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut sig = type_str.to_string();
            if !visibility.is_empty() {
                sig.push_str(&format!(" {visibility}"));
            }
            if mutability == "constant" || mutability == "immutable" {
                sig.push_str(&format!(" {mutability}"));
            }
            sig.push_str(&format!(" {name}"));
            Some(sig)
        }
        "ContractDefinition" => {
            let contract_kind = node
                .get("contractKind")
                .and_then(|v| v.as_str())
                .unwrap_or("contract");

            let mut sig = format!("{contract_kind} {name}");

            // Add base contracts
            if let Some(bases) = node.get("baseContracts").and_then(|v| v.as_array())
                && !bases.is_empty()
            {
                let base_names: Vec<&str> = bases
                    .iter()
                    .filter_map(|b| {
                        b.get("baseName")
                            .and_then(|bn| bn.get("name"))
                            .and_then(|n| n.as_str())
                    })
                    .collect();
                if !base_names.is_empty() {
                    sig.push_str(&format!(" is {}", base_names.join(", ")));
                }
            }
            Some(sig)
        }
        "StructDefinition" => {
            let mut sig = format!("struct {name} {{\n");
            if let Some(members) = node.get("members").and_then(|v| v.as_array()) {
                for member in members {
                    let mname = member.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                    let mtype = member
                        .get("typeDescriptions")
                        .and_then(|v| v.get("typeString"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("?");
                    sig.push_str(&format!("    {mtype} {mname};\n"));
                }
            }
            sig.push('}');
            Some(sig)
        }
        "EnumDefinition" => {
            let mut sig = format!("enum {name} {{\n");
            if let Some(members) = node.get("members").and_then(|v| v.as_array()) {
                let names: Vec<&str> = members
                    .iter()
                    .filter_map(|m| m.get("name").and_then(|v| v.as_str()))
                    .collect();
                for n in &names {
                    sig.push_str(&format!("    {n},\n"));
                }
            }
            sig.push('}');
            Some(sig)
        }
        "UserDefinedValueTypeDefinition" => {
            let underlying = node
                .get("underlyingType")
                .and_then(|v| v.get("typeDescriptions"))
                .and_then(|v| v.get("typeString"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            Some(format!("type {name} is {underlying}"))
        }
        _ => None,
    }
}

/// Format parameter list from a parameters node.
fn format_parameters(params_node: Option<&Value>) -> String {
    let params_node = match params_node {
        Some(v) => v,
        None => return String::new(),
    };
    let params = match params_node.get("parameters").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return String::new(),
    };

    let parts: Vec<String> = params
        .iter()
        .map(|p| {
            let type_str = p
                .get("typeDescriptions")
                .and_then(|v| v.get("typeString"))
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let storage = p
                .get("storageLocation")
                .and_then(|v| v.as_str())
                .unwrap_or("default");

            if name.is_empty() {
                type_str.to_string()
            } else if storage != "default" {
                format!("{type_str} {storage} {name}")
            } else {
                format!("{type_str} {name}")
            }
        })
        .collect();

    parts.join(", ")
}

/// Build gas hover text for a function declaration.
fn gas_hover_for_function(
    decl_node: &Value,
    sources: &Value,
    gas_index: &GasIndex,
) -> Option<String> {
    let node_type = decl_node.get("nodeType").and_then(|v| v.as_str())?;
    if node_type != "FunctionDefinition" {
        return None;
    }

    // Try by selector first (external/public functions)
    if let Some(selector) = decl_node.get("functionSelector").and_then(|v| v.as_str())
        && let Some((_contract, cost)) = gas::gas_by_selector(gas_index, selector)
    {
        return Some(format!("Gas: `{}`", gas::format_gas(cost)));
    }

    // Try by name (internal functions)
    let fn_name = decl_node.get("name").and_then(|v| v.as_str())?;
    let contract_key = gas::resolve_contract_key(sources, decl_node, gas_index)?;
    let contract_gas = gas_index.get(&contract_key)?;

    // Match by name prefix in internal gas estimates
    let prefix = format!("{fn_name}(");
    for (sig, cost) in &contract_gas.internal {
        if sig.starts_with(&prefix) {
            return Some(format!("Gas: `{}`", gas::format_gas(cost)));
        }
    }

    None
}

/// Build gas hover text for a contract declaration.
fn gas_hover_for_contract(
    decl_node: &Value,
    sources: &Value,
    gas_index: &GasIndex,
) -> Option<String> {
    let node_type = decl_node.get("nodeType").and_then(|v| v.as_str())?;
    if node_type != "ContractDefinition" {
        return None;
    }

    let contract_key = gas::resolve_contract_key(sources, decl_node, gas_index)?;
    let contract_gas = gas_index.get(&contract_key)?;

    let mut lines = Vec::new();

    // Creation/deploy costs
    if !contract_gas.creation.is_empty() {
        lines.push("**Deploy Cost**".to_string());
        if let Some(cost) = contract_gas.creation.get("totalCost") {
            lines.push(format!("- Total: `{}`", gas::format_gas(cost)));
        }
        if let Some(cost) = contract_gas.creation.get("codeDepositCost") {
            lines.push(format!("- Code deposit: `{}`", gas::format_gas(cost)));
        }
        if let Some(cost) = contract_gas.creation.get("executionCost") {
            lines.push(format!("- Execution: `{}`", gas::format_gas(cost)));
        }
    }

    // External function gas
    if !contract_gas.external.is_empty() {
        lines.push(String::new());
        lines.push("**Function Gas**".to_string());

        // Collect only signature entries (skip raw selector entries)
        let mut fns: Vec<(&String, &String)> = contract_gas
            .external
            .iter()
            .filter(|(k, _)| k.contains('('))
            .collect();
        fns.sort_by_key(|(k, _)| (*k).clone());

        for (sig, cost) in fns {
            let fn_name = sig.split('(').next().unwrap_or(sig);
            lines.push(format!("- `{fn_name}`: `{}`", gas::format_gas(cost)));
        }
    }

    if lines.is_empty() {
        return None;
    }

    Some(lines.join("\n"))
}

/// Produce hover information for the symbol at the given position.
pub fn hover_info(
    ast_data: &Value,
    file_uri: &Url,
    position: Position,
    source_bytes: &[u8],
    gas_index: &GasIndex,
) -> Option<Hover> {
    let sources = ast_data.get("sources")?;
    let source_id_to_path = ast_data
        .get("source_id_to_path")
        .and_then(|v| v.as_object())?;

    let id_to_path: HashMap<String, String> = source_id_to_path
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();

    let (nodes, path_to_abs, external_refs) = cache_ids(sources);

    // Resolve the file path
    let file_path = file_uri.to_file_path().ok()?;
    let file_path_str = file_path.to_str()?;

    // Find the absolute path for this file
    let abs_path = path_to_abs
        .iter()
        .find(|(k, _)| file_path_str.ends_with(k.as_str()))
        .map(|(_, v)| v.clone())?;

    let byte_pos = pos_to_bytes(source_bytes, position);

    // Resolve: first try Yul external refs, then normal node lookup
    let node_id = byte_to_decl_via_external_refs(&external_refs, &id_to_path, &abs_path, byte_pos)
        .or_else(|| byte_to_id(&nodes, &abs_path, byte_pos))?;

    // Get the NodeInfo for this node
    let node_info = nodes
        .values()
        .find_map(|file_nodes| file_nodes.get(&node_id))?;

    // Follow referenced_declaration to the declaration node
    let decl_id = node_info.referenced_declaration.unwrap_or(node_id);

    // Find the raw AST node for the declaration
    let decl_node = find_node_by_id(sources, decl_id)?;

    // Build hover content
    let mut parts: Vec<String> = Vec::new();

    // Signature in a code block
    if let Some(sig) = build_function_signature(decl_node) {
        parts.push(format!("```solidity\n{sig}\n```"));
    } else {
        // Fallback: show type description for any node
        if let Some(type_str) = decl_node
            .get("typeDescriptions")
            .and_then(|v| v.get("typeString"))
            .and_then(|v| v.as_str())
        {
            let name = decl_node.get("name").and_then(|v| v.as_str()).unwrap_or("");
            parts.push(format!("```solidity\n{type_str} {name}\n```"));
        }
    }

    // Selector (function, error, or event)
    if let Some((selector, kind)) = extract_selector(decl_node) {
        match kind {
            "event" => parts.push(format!("Selector: `0x{selector}`")),
            _ => parts.push(format!("Selector: `0x{selector}`")),
        }
    }

    // Gas estimates
    if !gas_index.is_empty() {
        if let Some(gas_text) = gas_hover_for_function(decl_node, sources, gas_index) {
            parts.push(gas_text);
        } else if let Some(gas_text) = gas_hover_for_contract(decl_node, sources, gas_index) {
            parts.push(gas_text);
        }
    }

    // Documentation — resolve @inheritdoc via selector matching
    if let Some(doc_text) = extract_documentation(decl_node) {
        let inherited_doc = resolve_inheritdoc(sources, decl_node, &doc_text);
        let formatted = format_natspec(&doc_text, inherited_doc.as_deref());
        if !formatted.is_empty() {
            parts.push(format!("---\n{formatted}"));
        }
    }

    if parts.is_empty() {
        return None;
    }

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: parts.join("\n\n"),
        }),
        range: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_ast() -> Value {
        let data = std::fs::read_to_string("pool-manager-ast.json").expect("test fixture");
        let raw: Value = serde_json::from_str(&data).expect("valid json");
        crate::solc::normalize_forge_output(raw)
    }

    #[test]
    fn test_find_node_by_id_pool_manager() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        let node = find_node_by_id(sources, NodeId(1767)).unwrap();
        assert_eq!(
            node.get("name").and_then(|v| v.as_str()),
            Some("PoolManager")
        );
        assert_eq!(
            node.get("nodeType").and_then(|v| v.as_str()),
            Some("ContractDefinition")
        );
    }

    #[test]
    fn test_find_node_by_id_initialize() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // IPoolManager.initialize has the full docs
        let node = find_node_by_id(sources, NodeId(2411)).unwrap();
        assert_eq!(
            node.get("name").and_then(|v| v.as_str()),
            Some("initialize")
        );
    }

    #[test]
    fn test_extract_documentation_object() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // IPoolManager.initialize (id=2411) has full NatSpec
        let node = find_node_by_id(sources, NodeId(2411)).unwrap();
        let doc = extract_documentation(node).unwrap();
        assert!(doc.contains("@notice"));
        assert!(doc.contains("@param key"));
    }

    #[test]
    fn test_extract_documentation_none() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // PoolKey struct (id=8887) — check if it has docs
        let node = find_node_by_id(sources, NodeId(8887)).unwrap();
        // PoolKey may or may not have docs, just verify no crash
        let _ = extract_documentation(node);
    }

    #[test]
    fn test_format_natspec_notice_and_params() {
        let text = "@notice Initialize the state for a given pool ID\n @param key The pool key\n @param sqrtPriceX96 The initial square root price\n @return tick The initial tick";
        let formatted = format_natspec(text, None);
        assert!(formatted.contains("Initialize the state"));
        assert!(formatted.contains("**Parameters:**"));
        assert!(formatted.contains("`key`"));
        assert!(formatted.contains("**Returns:**"));
        assert!(formatted.contains("`tick`"));
    }

    #[test]
    fn test_format_natspec_inheritdoc() {
        let text = "@inheritdoc IPoolManager";
        let formatted = format_natspec(text, None);
        assert!(formatted.contains("Inherits documentation from `IPoolManager`"));
    }

    #[test]
    fn test_format_natspec_dev() {
        let text = "@notice Do something\n @dev This is an implementation detail";
        let formatted = format_natspec(text, None);
        assert!(formatted.contains("Do something"));
        assert!(formatted.contains("**@dev**"));
        assert!(formatted.contains("*This is an implementation detail*"));
    }

    #[test]
    fn test_format_natspec_custom_tag() {
        let text = "@notice Do something\n @custom:security Non-reentrant";
        let formatted = format_natspec(text, None);
        assert!(formatted.contains("Do something"));
        assert!(formatted.contains("**@custom:security**"));
        assert!(formatted.contains("*Non-reentrant*"));
    }

    #[test]
    fn test_build_function_signature_initialize() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        let node = find_node_by_id(sources, NodeId(2411)).unwrap();
        let sig = build_function_signature(node).unwrap();
        assert!(sig.starts_with("function initialize("));
        assert!(sig.contains("returns"));
    }

    #[test]
    fn test_build_signature_contract() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        let node = find_node_by_id(sources, NodeId(1767)).unwrap();
        let sig = build_function_signature(node).unwrap();
        assert!(sig.contains("contract PoolManager"));
        assert!(sig.contains(" is "));
    }

    #[test]
    fn test_build_signature_struct() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        let node = find_node_by_id(sources, NodeId(8887)).unwrap();
        let sig = build_function_signature(node).unwrap();
        assert!(sig.starts_with("struct PoolKey"));
        assert!(sig.contains('{'));
    }

    #[test]
    fn test_build_signature_error() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // Find an ErrorDefinition
        let node = find_node_by_id(sources, NodeId(508)).unwrap();
        assert_eq!(
            node.get("nodeType").and_then(|v| v.as_str()),
            Some("ErrorDefinition")
        );
        let sig = build_function_signature(node).unwrap();
        assert!(sig.starts_with("error "));
    }

    #[test]
    fn test_build_signature_event() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // Find an EventDefinition
        let node = find_node_by_id(sources, NodeId(8)).unwrap();
        assert_eq!(
            node.get("nodeType").and_then(|v| v.as_str()),
            Some("EventDefinition")
        );
        let sig = build_function_signature(node).unwrap();
        assert!(sig.starts_with("event "));
    }

    #[test]
    fn test_build_signature_variable() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // Find a VariableDeclaration with documentation — check a state var
        // PoolManager has state variables, find one
        let pm = find_node_by_id(sources, NodeId(1767)).unwrap();
        if let Some(nodes) = pm.get("nodes").and_then(|v| v.as_array()) {
            for node in nodes {
                if node.get("nodeType").and_then(|v| v.as_str()) == Some("VariableDeclaration") {
                    let sig = build_function_signature(node);
                    assert!(sig.is_some());
                    break;
                }
            }
        }
    }

    #[test]
    fn test_pool_manager_has_documentation() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // Owned contract (id=59) has NatSpec
        let node = find_node_by_id(sources, NodeId(59)).unwrap();
        let doc = extract_documentation(node).unwrap();
        assert!(doc.contains("@notice"));
    }

    #[test]
    fn test_format_parameters_empty() {
        let result = format_parameters(None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_parameters_with_data() {
        let params: Value = serde_json::json!({
            "parameters": [
                {
                    "name": "key",
                    "typeDescriptions": { "typeString": "struct PoolKey" },
                    "storageLocation": "memory"
                },
                {
                    "name": "sqrtPriceX96",
                    "typeDescriptions": { "typeString": "uint160" },
                    "storageLocation": "default"
                }
            ]
        });
        let result = format_parameters(Some(&params));
        assert!(result.contains("struct PoolKey memory key"));
        assert!(result.contains("uint160 sqrtPriceX96"));
    }

    // --- Selector tests ---

    #[test]
    fn test_extract_selector_function() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // PoolManager.swap (id=1167) has functionSelector "f3cd914c"
        let node = find_node_by_id(sources, NodeId(1167)).unwrap();
        let (selector, kind) = extract_selector(node).unwrap();
        assert_eq!(selector, "f3cd914c");
        assert_eq!(kind, "function");
    }

    #[test]
    fn test_extract_selector_error() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // DelegateCallNotAllowed (id=508) has errorSelector
        let node = find_node_by_id(sources, NodeId(508)).unwrap();
        let (selector, kind) = extract_selector(node).unwrap();
        assert_eq!(selector, "0d89438e");
        assert_eq!(kind, "error");
    }

    #[test]
    fn test_extract_selector_event() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // OwnershipTransferred (id=8) has eventSelector
        let node = find_node_by_id(sources, NodeId(8)).unwrap();
        let (selector, kind) = extract_selector(node).unwrap();
        assert!(selector.len() == 64); // 32-byte keccak hash
        assert_eq!(kind, "event");
    }

    #[test]
    fn test_extract_selector_public_variable() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // owner (id=10) is public, has functionSelector
        let node = find_node_by_id(sources, NodeId(10)).unwrap();
        let (selector, kind) = extract_selector(node).unwrap();
        assert_eq!(selector, "8da5cb5b");
        assert_eq!(kind, "function");
    }

    #[test]
    fn test_extract_selector_internal_function_none() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // Pool.swap (id=5960) is internal, no selector
        let node = find_node_by_id(sources, NodeId(5960)).unwrap();
        assert!(extract_selector(node).is_none());
    }

    // --- @inheritdoc resolution tests ---

    #[test]
    fn test_resolve_inheritdoc_swap() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // PoolManager.swap (id=1167) has "@inheritdoc IPoolManager"
        let decl = find_node_by_id(sources, NodeId(1167)).unwrap();
        let doc_text = extract_documentation(decl).unwrap();
        assert!(doc_text.contains("@inheritdoc"));

        let resolved = resolve_inheritdoc(sources, decl, &doc_text).unwrap();
        assert!(resolved.contains("@notice"));
        assert!(resolved.contains("Swap against the given pool"));
    }

    #[test]
    fn test_resolve_inheritdoc_initialize() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // PoolManager.initialize (id=881) has "@inheritdoc IPoolManager"
        let decl = find_node_by_id(sources, NodeId(881)).unwrap();
        let doc_text = extract_documentation(decl).unwrap();

        let resolved = resolve_inheritdoc(sources, decl, &doc_text).unwrap();
        assert!(resolved.contains("Initialize the state"));
        assert!(resolved.contains("@param key"));
    }

    #[test]
    fn test_resolve_inheritdoc_extsload_overload() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();

        // extsload(bytes32) — id=442, selector "1e2eaeaf"
        let decl = find_node_by_id(sources, NodeId(442)).unwrap();
        let doc_text = extract_documentation(decl).unwrap();
        let resolved = resolve_inheritdoc(sources, decl, &doc_text).unwrap();
        assert!(resolved.contains("granular pool state"));
        // Should match the single-slot overload doc
        assert!(resolved.contains("@param slot"));

        // extsload(bytes32, uint256) — id=455, selector "35fd631a"
        let decl2 = find_node_by_id(sources, NodeId(455)).unwrap();
        let doc_text2 = extract_documentation(decl2).unwrap();
        let resolved2 = resolve_inheritdoc(sources, decl2, &doc_text2).unwrap();
        assert!(resolved2.contains("@param startSlot"));

        // extsload(bytes32[]) — id=467, selector "dbd035ff"
        let decl3 = find_node_by_id(sources, NodeId(467)).unwrap();
        let doc_text3 = extract_documentation(decl3).unwrap();
        let resolved3 = resolve_inheritdoc(sources, decl3, &doc_text3).unwrap();
        assert!(resolved3.contains("sparse pool state"));
    }

    #[test]
    fn test_resolve_inheritdoc_formats_in_hover() {
        let ast = load_test_ast();
        let sources = ast.get("sources").unwrap();
        // PoolManager.swap with @inheritdoc — verify format_natspec resolves it
        let decl = find_node_by_id(sources, NodeId(1167)).unwrap();
        let doc_text = extract_documentation(decl).unwrap();
        let inherited = resolve_inheritdoc(sources, decl, &doc_text);
        let formatted = format_natspec(&doc_text, inherited.as_deref());
        // Should have the resolved content, not "@inheritdoc"
        assert!(!formatted.contains("@inheritdoc"));
        assert!(formatted.contains("Swap against the given pool"));
        assert!(formatted.contains("**Parameters:**"));
    }
}
