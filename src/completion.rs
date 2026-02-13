use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionList, CompletionResponse, Position,
};

use crate::goto::CHILD_KEYS;

/// Completion cache built from the AST.
pub struct CompletionCache {
    /// All named identifiers as completion items (flat, unscoped).
    pub names: Vec<CompletionItem>,

    /// name → typeIdentifier (for dot-completion: look up what type a variable is).
    pub name_to_type: HashMap<String, String>,

    /// node id → Vec<CompletionItem> (members of structs, contracts, enums, libraries).
    pub node_members: HashMap<u64, Vec<CompletionItem>>,

    /// typeIdentifier → node id (resolve a type string to its definition).
    pub type_to_node: HashMap<String, u64>,

    /// contract/library/interface name → node id (for direct name dot-completion like `FullMath.`).
    pub name_to_node_id: HashMap<String, u64>,

    /// node id → Vec<CompletionItem> from methodIdentifiers in .contracts section.
    /// Full function signatures with 4-byte selectors for contracts/interfaces.
    pub method_identifiers: HashMap<u64, Vec<CompletionItem>>,

    /// (contract_node_id, fn_name) → return typeIdentifier.
    /// For resolving `foo().` — look up what `foo` returns.
    pub function_return_types: HashMap<(u64, String), String>,

    /// typeIdentifier → Vec<CompletionItem> from UsingForDirective.
    /// Library functions available on a type via `using X for Y`.
    pub using_for: HashMap<String, Vec<CompletionItem>>,

    /// Wildcard using-for: `using X for *` — available on all types.
    pub using_for_wildcard: Vec<CompletionItem>,

    /// Pre-built general completions (AST names + keywords + globals + units).
    /// Built once, returned by reference on every non-dot completion request.
    pub general_completions: Vec<CompletionItem>,
}

fn push_if_node_or_array<'a>(tree: &'a Value, key: &str, stack: &mut Vec<&'a Value>) {
    if let Some(value) = tree.get(key) {
        match value {
            Value::Array(arr) => stack.extend(arr),
            Value::Object(_) => stack.push(value),
            _ => {}
        }
    }
}

/// Map AST nodeType to LSP CompletionItemKind.
fn node_type_to_completion_kind(node_type: &str) -> CompletionItemKind {
    match node_type {
        "FunctionDefinition" => CompletionItemKind::FUNCTION,
        "VariableDeclaration" => CompletionItemKind::VARIABLE,
        "ContractDefinition" => CompletionItemKind::CLASS,
        "StructDefinition" => CompletionItemKind::STRUCT,
        "EnumDefinition" => CompletionItemKind::ENUM,
        "EnumValue" => CompletionItemKind::ENUM_MEMBER,
        "EventDefinition" => CompletionItemKind::EVENT,
        "ErrorDefinition" => CompletionItemKind::EVENT,
        "ModifierDefinition" => CompletionItemKind::METHOD,
        "ImportDirective" => CompletionItemKind::MODULE,
        _ => CompletionItemKind::TEXT,
    }
}

/// Extract the trailing node id from a typeIdentifier string.
/// e.g. `t_struct$_PoolKey_$8887_storage_ptr` → Some(8887)
///      `t_contract$_IHooks_$2248` → Some(2248)
///      `t_uint256` → None
pub fn extract_node_id_from_type(type_id: &str) -> Option<u64> {
    // Pattern: ..._$<digits>... where digits follow the last _$
    // We find all _$<digits> groups and take the last one that's part of the type name
    let mut last_id = None;
    let mut i = 0;
    let bytes = type_id.as_bytes();
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'_' && bytes[i + 1] == b'$' {
            i += 2;
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i > start
                && let Ok(id) = type_id[start..i].parse::<u64>()
            {
                last_id = Some(id);
            }
        } else {
            i += 1;
        }
    }
    last_id
}

/// Build a human-readable function signature from a FunctionDefinition AST node.
/// e.g. `swap(PoolKey key, SwapParams params, bytes hookData) returns (BalanceDelta swapDelta)`
fn build_function_signature(node: &Value) -> Option<String> {
    let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");
    if name.is_empty() {
        return None;
    }

    let params = node
        .get("parameters")
        .and_then(|p| p.get("parameters"))
        .and_then(|v| v.as_array());

    let mut sig = String::new();
    sig.push_str(name);
    sig.push('(');

    if let Some(params) = params {
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                sig.push_str(", ");
            }
            let type_str = param
                .get("typeDescriptions")
                .and_then(|td| td.get("typeString"))
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            // Clean up the type string: "struct PoolKey" → "PoolKey", "contract IHooks" → "IHooks"
            let clean_type = type_str
                .strip_prefix("struct ")
                .or_else(|| type_str.strip_prefix("contract "))
                .or_else(|| type_str.strip_prefix("enum "))
                .unwrap_or(type_str);
            let param_name = param.get("name").and_then(|v| v.as_str()).unwrap_or("");
            sig.push_str(clean_type);
            if !param_name.is_empty() {
                sig.push(' ');
                sig.push_str(param_name);
            }
        }
    }
    sig.push(')');

    // Add return parameters if present
    let returns = node
        .get("returnParameters")
        .and_then(|p| p.get("parameters"))
        .and_then(|v| v.as_array());

    if let Some(returns) = returns
        && !returns.is_empty()
    {
        sig.push_str(" returns (");
        for (i, ret) in returns.iter().enumerate() {
            if i > 0 {
                sig.push_str(", ");
            }
            let type_str = ret
                .get("typeDescriptions")
                .and_then(|td| td.get("typeString"))
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let clean_type = type_str
                .strip_prefix("struct ")
                .or_else(|| type_str.strip_prefix("contract "))
                .or_else(|| type_str.strip_prefix("enum "))
                .unwrap_or(type_str);
            let ret_name = ret.get("name").and_then(|v| v.as_str()).unwrap_or("");
            sig.push_str(clean_type);
            if !ret_name.is_empty() {
                sig.push(' ');
                sig.push_str(ret_name);
            }
        }
        sig.push(')');
    }

    Some(sig)
}

/// Extract the deepest value type from a mapping typeIdentifier.
/// Peels off all `t_mapping$_<key>_$_<value>` layers and returns the innermost value type.
///
/// e.g. `t_mapping$_t_address_$_t_uint256_$` → `t_uint256`
///      `t_mapping$_t_address_$_t_mapping$_t_uint256_$_t_uint256_$_$` → `t_uint256`
///      `t_mapping$_t_userDefinedValueType$_PoolId_$8841_$_t_struct$_State_$4809_storage_$` → `t_struct$_State_$4809_storage`
pub fn extract_mapping_value_type(type_id: &str) -> Option<String> {
    let mut current = type_id;

    loop {
        if !current.starts_with("t_mapping$_") {
            // Not a mapping — this is the value type
            // Strip trailing _$ suffixes (mapping closers)
            let result = current.trim_end_matches("_$");
            return if result.is_empty() {
                None
            } else {
                Some(result.to_string())
            };
        }

        // Strip "t_mapping$_" prefix to get "<key>_$_<value>_$"
        let inner = &current["t_mapping$_".len()..];

        // Find the boundary between key and value.
        // We need to find the _$_ that separates key from value at depth 0.
        // Each $_ opens a nesting level, each _$ closes one.
        let mut depth = 0i32;
        let bytes = inner.as_bytes();
        let mut split_pos = None;

        let mut i = 0;
        while i < bytes.len() {
            if i + 1 < bytes.len() && bytes[i] == b'$' && bytes[i + 1] == b'_' {
                depth += 1;
                i += 2;
            } else if i + 2 < bytes.len()
                && bytes[i] == b'_'
                && bytes[i + 1] == b'$'
                && bytes[i + 2] == b'_'
                && depth == 0
            {
                // This is the _$_ separator at depth 0
                split_pos = Some(i);
                break;
            } else if i + 1 < bytes.len() && bytes[i] == b'_' && bytes[i + 1] == b'$' {
                depth -= 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        if let Some(pos) = split_pos {
            // Value type starts after "_$_"
            current = &inner[pos + 3..];
        } else {
            return None;
        }
    }
}

/// Count parameters in an ABI method signature like "swap((address,address),uint256,bytes)".
/// Counts commas at depth 0 (inside the outer parens), handling nested tuples.
fn count_abi_params(signature: &str) -> usize {
    // Find the first '(' and work from there
    let start = match signature.find('(') {
        Some(i) => i + 1,
        None => return 0,
    };
    let bytes = signature.as_bytes();
    if start >= bytes.len() {
        return 0;
    }
    // Check for empty params "()"
    if bytes[start] == b')' {
        return 0;
    }
    let mut count = 1; // at least one param if not empty
    let mut depth = 0;
    for &b in &bytes[start..] {
        match b {
            b'(' => depth += 1,
            b')' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            b',' if depth == 0 => count += 1,
            _ => {}
        }
    }
    count
}

/// Count parameters in an AST-derived signature like "swap(PoolKey key, SwapParams params, bytes hookData)".
fn count_signature_params(sig: &str) -> usize {
    count_abi_params(sig)
}

/// Build a CompletionCache from AST sources and contracts.
/// `contracts` is the `.contracts` section of the compiler output (optional).
pub fn build_completion_cache(sources: &Value, contracts: Option<&Value>) -> CompletionCache {
    let mut names: Vec<CompletionItem> = Vec::new();
    let mut seen_names: HashMap<String, usize> = HashMap::new(); // name → index in names vec
    let mut name_to_type: HashMap<String, String> = HashMap::new();
    let mut node_members: HashMap<u64, Vec<CompletionItem>> = HashMap::new();
    let mut type_to_node: HashMap<String, u64> = HashMap::new();
    let mut method_identifiers: HashMap<u64, Vec<CompletionItem>> = HashMap::new();
    let mut name_to_node_id: HashMap<String, u64> = HashMap::new();

    // Collect (path, contract_name, node_id) during AST walk for methodIdentifiers lookup after.
    let mut contract_locations: Vec<(String, String, u64)> = Vec::new();

    // contract_node_id → fn_name → Vec<signature> (for matching method_identifiers to AST signatures)
    let mut function_signatures: HashMap<u64, HashMap<String, Vec<String>>> = HashMap::new();

    // (contract_node_id, fn_name) → return typeIdentifier
    let mut function_return_types: HashMap<(u64, String), String> = HashMap::new();

    // typeIdentifier → Vec<CompletionItem> from UsingForDirective
    let mut using_for: HashMap<String, Vec<CompletionItem>> = HashMap::new();
    let mut using_for_wildcard: Vec<CompletionItem> = Vec::new();

    // Temp: (library_node_id, target_type_id_or_none) for resolving after walk
    let mut using_for_directives: Vec<(u64, Option<String>)> = Vec::new();

    if let Some(sources_obj) = sources.as_object() {
        for (path, contents) in sources_obj {
            if let Some(contents_array) = contents.as_array()
                && let Some(first_content) = contents_array.first()
                && let Some(source_file) = first_content.get("source_file")
                && let Some(ast) = source_file.get("ast")
            {
                let mut stack: Vec<&Value> = vec![ast];

                while let Some(tree) = stack.pop() {
                    let node_type = tree.get("nodeType").and_then(|v| v.as_str()).unwrap_or("");
                    let name = tree.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let node_id = tree.get("id").and_then(|v| v.as_u64());

                    // Collect named nodes as completion items
                    if !name.is_empty() && !seen_names.contains_key(name) {
                        let type_string = tree
                            .get("typeDescriptions")
                            .and_then(|td| td.get("typeString"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());

                        let type_id = tree
                            .get("typeDescriptions")
                            .and_then(|td| td.get("typeIdentifier"))
                            .and_then(|v| v.as_str());

                        let kind = node_type_to_completion_kind(node_type);

                        let item = CompletionItem {
                            label: name.to_string(),
                            kind: Some(kind),
                            detail: type_string,
                            ..Default::default()
                        };

                        let idx = names.len();
                        names.push(item);
                        seen_names.insert(name.to_string(), idx);

                        // Store name → typeIdentifier mapping
                        if let Some(tid) = type_id {
                            name_to_type.insert(name.to_string(), tid.to_string());
                        }
                    }

                    // Collect struct members
                    if node_type == "StructDefinition"
                        && let Some(id) = node_id
                    {
                        let mut members = Vec::new();
                        if let Some(member_array) = tree.get("members").and_then(|v| v.as_array()) {
                            for member in member_array {
                                let member_name =
                                    member.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                if member_name.is_empty() {
                                    continue;
                                }
                                let member_type = member
                                    .get("typeDescriptions")
                                    .and_then(|td| td.get("typeString"))
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());

                                members.push(CompletionItem {
                                    label: member_name.to_string(),
                                    kind: Some(CompletionItemKind::FIELD),
                                    detail: member_type,
                                    ..Default::default()
                                });
                            }
                        }
                        if !members.is_empty() {
                            node_members.insert(id, members);
                        }

                        // Map typeIdentifier → node id
                        if let Some(tid) = tree
                            .get("typeDescriptions")
                            .and_then(|td| td.get("typeIdentifier"))
                            .and_then(|v| v.as_str())
                        {
                            type_to_node.insert(tid.to_string(), id);
                        }
                    }

                    // Collect contract/library members (functions, state variables, events, etc.)
                    if node_type == "ContractDefinition"
                        && let Some(id) = node_id
                    {
                        let mut members = Vec::new();
                        let mut fn_sigs: HashMap<String, Vec<String>> = HashMap::new();
                        if let Some(nodes_array) = tree.get("nodes").and_then(|v| v.as_array()) {
                            for member in nodes_array {
                                let member_type = member
                                    .get("nodeType")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                let member_name =
                                    member.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                if member_name.is_empty() {
                                    continue;
                                }

                                // Build function signature and collect return types for FunctionDefinitions
                                let (member_detail, label_details) =
                                    if member_type == "FunctionDefinition" {
                                        // Collect return type for chain resolution.
                                        // Only single-return functions can be dot-chained
                                        // (tuples require destructuring).
                                        if let Some(ret_params) = member
                                            .get("returnParameters")
                                            .and_then(|rp| rp.get("parameters"))
                                            .and_then(|v| v.as_array())
                                            && ret_params.len() == 1
                                            && let Some(ret_tid) = ret_params[0]
                                                .get("typeDescriptions")
                                                .and_then(|td| td.get("typeIdentifier"))
                                                .and_then(|v| v.as_str())
                                        {
                                            function_return_types.insert(
                                                (id, member_name.to_string()),
                                                ret_tid.to_string(),
                                            );
                                        }

                                        if let Some(sig) = build_function_signature(member) {
                                            fn_sigs
                                                .entry(member_name.to_string())
                                                .or_default()
                                                .push(sig.clone());
                                            (Some(sig), None)
                                        } else {
                                            (
                                                member
                                                    .get("typeDescriptions")
                                                    .and_then(|td| td.get("typeString"))
                                                    .and_then(|v| v.as_str())
                                                    .map(|s| s.to_string()),
                                                None,
                                            )
                                        }
                                    } else {
                                        (
                                            member
                                                .get("typeDescriptions")
                                                .and_then(|td| td.get("typeString"))
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            None,
                                        )
                                    };

                                let kind = node_type_to_completion_kind(member_type);
                                members.push(CompletionItem {
                                    label: member_name.to_string(),
                                    kind: Some(kind),
                                    detail: member_detail,
                                    label_details,
                                    ..Default::default()
                                });
                            }
                        }
                        if !members.is_empty() {
                            node_members.insert(id, members);
                        }
                        if !fn_sigs.is_empty() {
                            function_signatures.insert(id, fn_sigs);
                        }

                        if let Some(tid) = tree
                            .get("typeDescriptions")
                            .and_then(|td| td.get("typeIdentifier"))
                            .and_then(|v| v.as_str())
                        {
                            type_to_node.insert(tid.to_string(), id);
                        }

                        // Record for methodIdentifiers lookup after traversal
                        if !name.is_empty() {
                            contract_locations.push((path.clone(), name.to_string(), id));
                            name_to_node_id.insert(name.to_string(), id);
                        }
                    }

                    // Collect enum members
                    if node_type == "EnumDefinition"
                        && let Some(id) = node_id
                    {
                        let mut members = Vec::new();
                        if let Some(member_array) = tree.get("members").and_then(|v| v.as_array()) {
                            for member in member_array {
                                let member_name =
                                    member.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                if member_name.is_empty() {
                                    continue;
                                }
                                members.push(CompletionItem {
                                    label: member_name.to_string(),
                                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                                    detail: None,
                                    ..Default::default()
                                });
                            }
                        }
                        if !members.is_empty() {
                            node_members.insert(id, members);
                        }

                        if let Some(tid) = tree
                            .get("typeDescriptions")
                            .and_then(|td| td.get("typeIdentifier"))
                            .and_then(|v| v.as_str())
                        {
                            type_to_node.insert(tid.to_string(), id);
                        }
                    }

                    // Collect UsingForDirective: using Library for Type
                    if node_type == "UsingForDirective" {
                        // Get target type (None = wildcard `for *`)
                        let target_type = tree.get("typeName").and_then(|tn| {
                            tn.get("typeDescriptions")
                                .and_then(|td| td.get("typeIdentifier"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                        });

                        // Form 1: library name object with referencedDeclaration
                        if let Some(lib) = tree.get("libraryName") {
                            if let Some(lib_id) =
                                lib.get("referencedDeclaration").and_then(|v| v.as_u64())
                            {
                                using_for_directives.push((lib_id, target_type));
                            }
                        }
                        // Form 2: functionList array — individual function references
                        // These are typically operator overloads (not dot-callable),
                        // but collect non-operator ones just in case
                        else if let Some(func_list) =
                            tree.get("functionList").and_then(|v| v.as_array())
                        {
                            for entry in func_list {
                                // Skip operator overloads
                                if entry.get("operator").is_some() {
                                    continue;
                                }
                                if let Some(def) = entry.get("definition") {
                                    let fn_name =
                                        def.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                    if !fn_name.is_empty() {
                                        let items = if let Some(ref tid) = target_type {
                                            using_for.entry(tid.clone()).or_default()
                                        } else {
                                            &mut using_for_wildcard
                                        };
                                        items.push(CompletionItem {
                                            label: fn_name.to_string(),
                                            kind: Some(CompletionItemKind::FUNCTION),
                                            detail: None,
                                            ..Default::default()
                                        });
                                    }
                                }
                            }
                        }
                    }

                    // Traverse children
                    for key in CHILD_KEYS {
                        push_if_node_or_array(tree, key, &mut stack);
                    }
                }
            }
        }
    }

    // Resolve UsingForDirective library references (Form 1)
    // Now that node_members is populated, look up each library's functions
    for (lib_id, target_type) in &using_for_directives {
        if let Some(lib_members) = node_members.get(lib_id) {
            let items: Vec<CompletionItem> = lib_members
                .iter()
                .filter(|item| item.kind == Some(CompletionItemKind::FUNCTION))
                .cloned()
                .collect();
            if !items.is_empty() {
                if let Some(tid) = target_type {
                    using_for.entry(tid.clone()).or_default().extend(items);
                } else {
                    using_for_wildcard.extend(items);
                }
            }
        }
    }

    // Build method_identifiers from .contracts section
    if let Some(contracts_val) = contracts
        && let Some(contracts_obj) = contracts_val.as_object()
    {
        for (path, contract_name, node_id) in &contract_locations {
            // Get AST function signatures for this contract (if available)
            let fn_sigs = function_signatures.get(node_id);

            if let Some(path_entry) = contracts_obj.get(path)
                && let Some(contract_entry) = path_entry.get(contract_name)
                && let Some(first) = contract_entry.get(0)
                && let Some(evm) = first.get("contract").and_then(|c| c.get("evm"))
                && let Some(methods) = evm.get("methodIdentifiers")
                && let Some(methods_obj) = methods.as_object()
            {
                let mut items: Vec<CompletionItem> = Vec::new();
                for (signature, selector) in methods_obj {
                    // signature is e.g. "swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)"
                    // selector is e.g. "f3cd914c"
                    let fn_name = signature.split('(').next().unwrap_or(signature).to_string();
                    let selector_str = selector
                        .as_str()
                        .map(|s| format!("0x{}", s))
                        .unwrap_or_default();

                    // Look up the AST signature with parameter names
                    let description =
                        fn_sigs
                            .and_then(|sigs| sigs.get(&fn_name))
                            .and_then(|sig_list| {
                                if sig_list.len() == 1 {
                                    // Only one overload — use it directly
                                    Some(sig_list[0].clone())
                                } else {
                                    // Multiple overloads — match by parameter count
                                    let abi_param_count = count_abi_params(signature);
                                    sig_list
                                        .iter()
                                        .find(|s| count_signature_params(s) == abi_param_count)
                                        .cloned()
                                }
                            });

                    items.push(CompletionItem {
                        label: fn_name,
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some(signature.clone()),
                        label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                            detail: Some(selector_str),
                            description,
                        }),
                        ..Default::default()
                    });
                }
                if !items.is_empty() {
                    method_identifiers.insert(*node_id, items);
                }
            }
        }
    }

    // Pre-build the general completions list (names + statics) once
    let mut general_completions = names.clone();
    general_completions.extend(get_static_completions());

    CompletionCache {
        names,
        name_to_type,
        node_members,
        type_to_node,
        name_to_node_id,
        method_identifiers,
        function_return_types,
        using_for,
        using_for_wildcard,
        general_completions,
    }
}

/// Magic type member definitions (msg, block, tx, abi, address).
fn magic_members(name: &str) -> Option<Vec<CompletionItem>> {
    let items = match name {
        "msg" => vec![
            ("data", "bytes calldata"),
            ("sender", "address"),
            ("sig", "bytes4"),
            ("value", "uint256"),
        ],
        "block" => vec![
            ("basefee", "uint256"),
            ("blobbasefee", "uint256"),
            ("chainid", "uint256"),
            ("coinbase", "address payable"),
            ("difficulty", "uint256"),
            ("gaslimit", "uint256"),
            ("number", "uint256"),
            ("prevrandao", "uint256"),
            ("timestamp", "uint256"),
        ],
        "tx" => vec![("gasprice", "uint256"), ("origin", "address")],
        "abi" => vec![
            ("decode(bytes memory, (...))", "..."),
            ("encode(...)", "bytes memory"),
            ("encodePacked(...)", "bytes memory"),
            ("encodeWithSelector(bytes4, ...)", "bytes memory"),
            ("encodeWithSignature(string memory, ...)", "bytes memory"),
            ("encodeCall(function, (...))", "bytes memory"),
        ],
        // type(X) — contract type properties
        // Also includes interface (interfaceId) and integer (min, max) properties
        "type" => vec![
            ("name", "string"),
            ("creationCode", "bytes memory"),
            ("runtimeCode", "bytes memory"),
            ("interfaceId", "bytes4"),
            ("min", "T"),
            ("max", "T"),
        ],
        // bytes and string type-level members
        "bytes" => vec![("concat(...)", "bytes memory")],
        "string" => vec![("concat(...)", "string memory")],
        _ => return None,
    };

    Some(
        items
            .into_iter()
            .map(|(label, detail)| CompletionItem {
                label: label.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(detail.to_string()),
                ..Default::default()
            })
            .collect(),
    )
}

/// Address type members (available on any address value).
fn address_members() -> Vec<CompletionItem> {
    [
        ("balance", "uint256", CompletionItemKind::PROPERTY),
        ("code", "bytes memory", CompletionItemKind::PROPERTY),
        ("codehash", "bytes32", CompletionItemKind::PROPERTY),
        ("transfer(uint256)", "", CompletionItemKind::FUNCTION),
        ("send(uint256)", "bool", CompletionItemKind::FUNCTION),
        (
            "call(bytes memory)",
            "(bool, bytes memory)",
            CompletionItemKind::FUNCTION,
        ),
        (
            "delegatecall(bytes memory)",
            "(bool, bytes memory)",
            CompletionItemKind::FUNCTION,
        ),
        (
            "staticcall(bytes memory)",
            "(bool, bytes memory)",
            CompletionItemKind::FUNCTION,
        ),
    ]
    .iter()
    .map(|(label, detail, kind)| CompletionItem {
        label: label.to_string(),
        kind: Some(*kind),
        detail: if detail.is_empty() {
            None
        } else {
            Some(detail.to_string())
        },
        ..Default::default()
    })
    .collect()
}

/// What kind of access precedes the dot.
#[derive(Debug, Clone, PartialEq)]
pub enum AccessKind {
    /// Plain identifier: `foo.`
    Plain,
    /// Function call: `foo().` or `foo(x, bar()).`
    Call,
    /// Index/storage access: `foo[key].` or `foo[func()].`
    Index,
}

/// A segment of a dot-expression chain.
#[derive(Debug, Clone, PartialEq)]
pub struct DotSegment {
    pub name: String,
    pub kind: AccessKind,
}

/// Skip backwards over a matched bracket pair (parens or square brackets).
/// `pos` should point to the closing bracket. Returns the position of the matching
/// opening bracket, or 0 if not found.
fn skip_brackets_backwards(bytes: &[u8], pos: usize) -> usize {
    let close = bytes[pos];
    let open = match close {
        b')' => b'(',
        b']' => b'[',
        _ => return pos,
    };
    let mut depth = 1u32;
    let mut i = pos;
    while i > 0 && depth > 0 {
        i -= 1;
        if bytes[i] == close {
            depth += 1;
        } else if bytes[i] == open {
            depth -= 1;
        }
    }
    i
}

/// Parse the expression chain before the dot into segments.
/// e.g. `poolManager.swap(key, params).` → [("poolManager", Plain), ("swap", Call)]
///      `_pools[poolId].fee.` → [("_pools", Index), ("fee", Plain)]
///      `msg.` → [("msg", Plain)]
pub fn parse_dot_chain(line: &str, character: u32) -> Vec<DotSegment> {
    let col = character as usize;
    if col == 0 {
        return vec![];
    }

    let bytes = line.as_bytes();
    let mut segments: Vec<DotSegment> = Vec::new();

    // Start from the cursor position, skip trailing dot
    let mut pos = col;
    if pos > 0 && pos <= bytes.len() && bytes[pos - 1] == b'.' {
        pos -= 1;
    }

    loop {
        if pos == 0 {
            break;
        }

        // Determine access kind by what's immediately before: ')' = Call, ']' = Index, else Plain
        let kind = if bytes[pos - 1] == b')' {
            pos -= 1; // point to ')'
            pos = skip_brackets_backwards(bytes, pos);
            AccessKind::Call
        } else if bytes[pos - 1] == b']' {
            pos -= 1; // point to ']'
            pos = skip_brackets_backwards(bytes, pos);
            AccessKind::Index
        } else {
            AccessKind::Plain
        };

        // Now extract the identifier name (walk backwards over alphanumeric + underscore)
        let end = pos;
        while pos > 0 && (bytes[pos - 1].is_ascii_alphanumeric() || bytes[pos - 1] == b'_') {
            pos -= 1;
        }

        if pos == end {
            // No identifier found (could be something like `().`)
            break;
        }

        let name = String::from_utf8_lossy(&bytes[pos..end]).to_string();
        segments.push(DotSegment { name, kind });

        // Check if there's a dot before this segment (meaning more chain)
        if pos > 0 && bytes[pos - 1] == b'.' {
            pos -= 1; // skip the dot, continue parsing next segment
        } else {
            break;
        }
    }

    segments.reverse(); // We parsed right-to-left, flip to left-to-right
    segments
}

/// Extract the identifier before the cursor (the word before the dot).
/// Returns just the last identifier name for backward compatibility.
pub fn extract_identifier_before_dot(line: &str, character: u32) -> Option<String> {
    let segments = parse_dot_chain(line, character);
    segments.last().map(|s| s.name.clone())
}

#[doc = r"Strip all storage/memory location suffixes from a typeIdentifier to get the base type.
Solidity AST uses different suffixes in different contexts:
  - `t_struct$_State_$4809_storage_ptr` (UsingForDirective typeName)
  - `t_struct$_State_$4809_storage` (mapping value type after extraction)
  - `t_struct$_PoolKey_$8887_memory_ptr` (function parameter)
All refer to the same logical type. This strips `_ptr` and `_storage`/`_memory`/`_calldata`."]
fn strip_type_suffix(type_id: &str) -> &str {
    let s = type_id.strip_suffix("_ptr").unwrap_or(type_id);
    s.strip_suffix("_storage")
        .or_else(|| s.strip_suffix("_memory"))
        .or_else(|| s.strip_suffix("_calldata"))
        .unwrap_or(s)
}

/// Look up using-for completions for a type, trying suffix variants.
/// The AST stores types with different suffixes (_storage_ptr, _storage, _memory_ptr, etc.)
/// across different contexts, so we try multiple forms.
fn lookup_using_for(cache: &CompletionCache, type_id: &str) -> Vec<CompletionItem> {
    // Exact match first
    if let Some(items) = cache.using_for.get(type_id) {
        return items.clone();
    }

    // Strip to base form, then try all common suffix variants
    let base = strip_type_suffix(type_id);
    let variants = [
        base.to_string(),
        format!("{}_storage", base),
        format!("{}_storage_ptr", base),
        format!("{}_memory", base),
        format!("{}_memory_ptr", base),
        format!("{}_calldata", base),
    ];
    for variant in &variants {
        if variant.as_str() != type_id
            && let Some(items) = cache.using_for.get(variant.as_str())
        {
            return items.clone();
        }
    }

    vec![]
}

/// Collect completions available for a given typeIdentifier.
/// Includes node_members, method_identifiers, using_for, and using_for_wildcard.
fn completions_for_type(cache: &CompletionCache, type_id: &str) -> Vec<CompletionItem> {
    // Address type
    if type_id == "t_address" || type_id == "t_address_payable" {
        let mut items = address_members();
        // Also add using-for on address
        if let Some(uf) = cache.using_for.get(type_id) {
            items.extend(uf.iter().cloned());
        }
        items.extend(cache.using_for_wildcard.iter().cloned());
        return items;
    }

    let resolved_node_id = extract_node_id_from_type(type_id)
        .or_else(|| cache.type_to_node.get(type_id).copied())
        .or_else(|| {
            // Handle synthetic __node_id_ markers from name_to_node_id fallback
            type_id
                .strip_prefix("__node_id_")
                .and_then(|s| s.parse::<u64>().ok())
        });

    let mut items = Vec::new();
    let mut seen_labels: std::collections::HashSet<String> = std::collections::HashSet::new();

    if let Some(node_id) = resolved_node_id {
        // Method identifiers first — they have full signatures with selectors
        if let Some(method_items) = cache.method_identifiers.get(&node_id) {
            for item in method_items {
                seen_labels.insert(item.label.clone());
                items.push(item.clone());
            }
        }

        // Supplement with node_members (state variables, events, errors, modifiers, etc.)
        if let Some(members) = cache.node_members.get(&node_id) {
            for item in members {
                if !seen_labels.contains(&item.label) {
                    seen_labels.insert(item.label.clone());
                    items.push(item.clone());
                }
            }
        }
    }

    // Add using-for library functions for this type
    // Try exact match first, then try normalized variants (storage_ptr vs storage vs memory_ptr etc.)
    let uf_items = lookup_using_for(cache, type_id);
    for item in &uf_items {
        if !seen_labels.contains(&item.label) {
            seen_labels.insert(item.label.clone());
            items.push(item.clone());
        }
    }

    // Add wildcard using-for (using X for *)
    for item in &cache.using_for_wildcard {
        if !seen_labels.contains(&item.label) {
            seen_labels.insert(item.label.clone());
            items.push(item.clone());
        }
    }

    items
}

/// Resolve a type identifier for a name, considering name_to_type and name_to_node_id.
fn resolve_name_to_type_id(cache: &CompletionCache, name: &str) -> Option<String> {
    // Direct type lookup
    if let Some(tid) = cache.name_to_type.get(name) {
        return Some(tid.clone());
    }
    // Contract/library/interface name → synthesize a type id from node id
    if let Some(node_id) = cache.name_to_node_id.get(name) {
        // Find a matching typeIdentifier in type_to_node (reverse lookup)
        for (tid, nid) in &cache.type_to_node {
            if nid == node_id {
                return Some(tid.clone());
            }
        }
        // Fallback: use a synthetic marker so completions_for_type can resolve via node_id
        return Some(format!("__node_id_{}", node_id));
    }
    None
}

/// Resolve a name within a type context to get the member's type.
/// `context_type_id` is the type of the object before the dot.
/// `member_name` is the name after the dot.
/// `kind` determines how to interpret the result (Call = return type, Index = mapping value, Plain = member type).
fn resolve_member_type(
    cache: &CompletionCache,
    context_type_id: &str,
    member_name: &str,
    kind: &AccessKind,
) -> Option<String> {
    let resolved_node_id = extract_node_id_from_type(context_type_id)
        .or_else(|| cache.type_to_node.get(context_type_id).copied())
        .or_else(|| {
            // Handle synthetic __node_id_ markers
            context_type_id
                .strip_prefix("__node_id_")
                .and_then(|s| s.parse::<u64>().ok())
        });

    let node_id = resolved_node_id?;

    match kind {
        AccessKind::Call => {
            // Look up the function's return type
            cache
                .function_return_types
                .get(&(node_id, member_name.to_string()))
                .cloned()
        }
        AccessKind::Index => {
            // Look up the member's type, then extract mapping value type
            if let Some(members) = cache.node_members.get(&node_id) {
                for member in members {
                    if member.label == member_name {
                        // Get the typeIdentifier from name_to_type
                        if let Some(tid) = cache.name_to_type.get(member_name) {
                            if tid.starts_with("t_mapping") {
                                return extract_mapping_value_type(tid);
                            }
                            return Some(tid.clone());
                        }
                    }
                }
            }
            // Also check: the identifier itself might be a mapping variable
            if let Some(tid) = cache.name_to_type.get(member_name)
                && tid.starts_with("t_mapping")
            {
                return extract_mapping_value_type(tid);
            }
            None
        }
        AccessKind::Plain => {
            // Look up member's own type from name_to_type
            cache.name_to_type.get(member_name).cloned()
        }
    }
}

/// Get completions for a dot-completion request by resolving the full expression chain.
pub fn get_dot_completions(cache: &CompletionCache, identifier: &str) -> Vec<CompletionItem> {
    // Simple single-segment case (backward compat) — just use the identifier directly
    if let Some(items) = magic_members(identifier) {
        return items;
    }

    // Try to resolve the identifier's type
    let type_id = resolve_name_to_type_id(cache, identifier);

    if let Some(tid) = type_id {
        return completions_for_type(cache, &tid);
    }

    vec![]
}

/// Get completions by resolving a full dot-expression chain.
/// This is the main entry point for dot-completions with chaining support.
pub fn get_chain_completions(cache: &CompletionCache, chain: &[DotSegment]) -> Vec<CompletionItem> {
    if chain.is_empty() {
        return vec![];
    }

    // Single segment: simple dot-completion
    if chain.len() == 1 {
        let seg = &chain[0];

        // For Call/Index on the single segment, we need to resolve the return/value type
        match seg.kind {
            AccessKind::Plain => {
                return get_dot_completions(cache, &seg.name);
            }
            AccessKind::Call => {
                // foo(). — could be a function call or a type cast like IFoo(addr).
                // First check if it's a type cast: name matches a contract/interface/library
                if let Some(type_id) = resolve_name_to_type_id(cache, &seg.name) {
                    return completions_for_type(cache, &type_id);
                }
                // Otherwise look up as a function call — check all function_return_types
                for ((_, fn_name), ret_type) in &cache.function_return_types {
                    if fn_name == &seg.name {
                        return completions_for_type(cache, ret_type);
                    }
                }
                return vec![];
            }
            AccessKind::Index => {
                // foo[key]. — look up foo's type and extract mapping value type
                if let Some(tid) = cache.name_to_type.get(&seg.name)
                    && tid.starts_with("t_mapping")
                    && let Some(val_type) = extract_mapping_value_type(tid)
                {
                    return completions_for_type(cache, &val_type);
                }
                return vec![];
            }
        }
    }

    // Multi-segment chain: resolve step by step
    // First segment: resolve to a type
    let first = &chain[0];
    let mut current_type = match first.kind {
        AccessKind::Plain => resolve_name_to_type_id(cache, &first.name),
        AccessKind::Call => {
            // Type cast (e.g. IFoo(addr).) or free function call at the start
            resolve_name_to_type_id(cache, &first.name).or_else(|| {
                cache
                    .function_return_types
                    .iter()
                    .find(|((_, fn_name), _)| fn_name == &first.name)
                    .map(|(_, ret_type)| ret_type.clone())
            })
        }
        AccessKind::Index => {
            // Mapping access at the start
            cache.name_to_type.get(&first.name).and_then(|tid| {
                if tid.starts_with("t_mapping") {
                    extract_mapping_value_type(tid)
                } else {
                    Some(tid.clone())
                }
            })
        }
    };

    // Middle segments: resolve each to advance the type
    for seg in &chain[1..] {
        let ctx_type = match &current_type {
            Some(t) => t.clone(),
            None => return vec![],
        };

        current_type = resolve_member_type(cache, &ctx_type, &seg.name, &seg.kind);
    }

    // Return completions for the final resolved type
    match current_type {
        Some(tid) => completions_for_type(cache, &tid),
        None => vec![],
    }
}

/// Get static completions that never change (keywords, magic globals, global functions, units).
/// These are available immediately without an AST cache.
pub fn get_static_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Add Solidity keywords
    for kw in SOLIDITY_KEYWORDS {
        items.push(CompletionItem {
            label: kw.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        });
    }

    // Add magic globals
    for (name, detail) in MAGIC_GLOBALS {
        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    // Add global functions
    for (name, detail) in GLOBAL_FUNCTIONS {
        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    // Add ether denomination units
    for (name, detail) in ETHER_UNITS {
        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::UNIT),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    // Add time units
    for (name, detail) in TIME_UNITS {
        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::UNIT),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    items
}

/// Get general completions (all known names + static completions).
pub fn get_general_completions(cache: &CompletionCache) -> Vec<CompletionItem> {
    let mut items = cache.names.clone();
    items.extend(get_static_completions());
    items
}

/// Handle a completion request.
///
/// When `cache` is `Some`, full AST-aware completions are returned.
/// When `cache` is `None`, only static completions (keywords, globals, units)
/// and magic dot completions (msg., block., tx., abi., type().) are returned
/// immediately — no blocking.
///
/// When `fast` is true, general completions use the pre-built list from the
/// cache (zero per-request allocation). When false, `get_general_completions`
/// is called which allows per-request filtering (e.g. scope-aware completions).
pub fn handle_completion(
    cache: Option<&CompletionCache>,
    source_text: &str,
    position: Position,
    trigger_char: Option<&str>,
    fast: bool,
) -> Option<CompletionResponse> {
    let lines: Vec<&str> = source_text.lines().collect();
    let line = lines.get(position.line as usize)?;

    // Convert encoding-aware column to a byte offset within this line.
    let abs_byte =
        crate::utils::position_to_byte_offset(source_text, position.line, position.character);
    let line_start_byte: usize = source_text[..abs_byte]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let col_byte = (abs_byte - line_start_byte) as u32;

    let items = if trigger_char == Some(".") {
        let chain = parse_dot_chain(line, col_byte);
        if chain.is_empty() {
            return None;
        }
        match cache {
            Some(c) => get_chain_completions(c, &chain),
            None => {
                // No cache yet — serve magic dot completions (msg., block., etc.)
                if chain.len() == 1 && chain[0].kind == AccessKind::Plain {
                    magic_members(&chain[0].name).unwrap_or_default()
                } else {
                    vec![]
                }
            }
        }
    } else {
        match cache {
            Some(c) if fast => c.general_completions.clone(),
            Some(c) => get_general_completions(c),
            None => get_static_completions(),
        }
    };

    Some(CompletionResponse::List(CompletionList {
        is_incomplete: cache.is_none(),
        items,
    }))
}

const SOLIDITY_KEYWORDS: &[&str] = &[
    "abstract",
    "address",
    "assembly",
    "bool",
    "break",
    "bytes",
    "bytes1",
    "bytes4",
    "bytes32",
    "calldata",
    "constant",
    "constructor",
    "continue",
    "contract",
    "delete",
    "do",
    "else",
    "emit",
    "enum",
    "error",
    "event",
    "external",
    "fallback",
    "false",
    "for",
    "function",
    "if",
    "immutable",
    "import",
    "indexed",
    "int8",
    "int24",
    "int128",
    "int256",
    "interface",
    "internal",
    "library",
    "mapping",
    "memory",
    "modifier",
    "new",
    "override",
    "payable",
    "pragma",
    "private",
    "public",
    "pure",
    "receive",
    "return",
    "returns",
    "revert",
    "storage",
    "string",
    "struct",
    "true",
    "type",
    "uint8",
    "uint24",
    "uint128",
    "uint160",
    "uint256",
    "unchecked",
    "using",
    "view",
    "virtual",
    "while",
];

/// Ether denomination units — suffixes for literal numbers.
const ETHER_UNITS: &[(&str, &str)] = &[("wei", "1"), ("gwei", "1e9"), ("ether", "1e18")];

/// Time units — suffixes for literal numbers.
const TIME_UNITS: &[(&str, &str)] = &[
    ("seconds", "1"),
    ("minutes", "60 seconds"),
    ("hours", "3600 seconds"),
    ("days", "86400 seconds"),
    ("weeks", "604800 seconds"),
];

const MAGIC_GLOBALS: &[(&str, &str)] = &[
    ("msg", "msg"),
    ("block", "block"),
    ("tx", "tx"),
    ("abi", "abi"),
    ("this", "address"),
    ("super", "contract"),
    ("type", "type information"),
];

const GLOBAL_FUNCTIONS: &[(&str, &str)] = &[
    // Mathematical and Cryptographic Functions
    ("addmod(uint256, uint256, uint256)", "uint256"),
    ("mulmod(uint256, uint256, uint256)", "uint256"),
    ("keccak256(bytes memory)", "bytes32"),
    ("sha256(bytes memory)", "bytes32"),
    ("ripemd160(bytes memory)", "bytes20"),
    (
        "ecrecover(bytes32 hash, uint8 v, bytes32 r, bytes32 s)",
        "address",
    ),
    // Block and Transaction Properties (functions)
    ("blockhash(uint256 blockNumber)", "bytes32"),
    ("blobhash(uint256 index)", "bytes32"),
    ("gasleft()", "uint256"),
    // Error Handling
    ("assert(bool condition)", ""),
    ("require(bool condition)", ""),
    ("require(bool condition, string memory message)", ""),
    ("revert()", ""),
    ("revert(string memory reason)", ""),
    // Contract-related
    ("selfdestruct(address payable recipient)", ""),
];
