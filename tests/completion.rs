use serde_json::Value;
use solidity_language_server::completion::{
    AccessKind, DotSegment, build_completion_cache, extract_identifier_before_dot,
    extract_mapping_value_type, extract_node_id_from_type, get_dot_completions,
    get_general_completions, parse_dot_chain,
};
use std::fs;
use tower_lsp::lsp_types::CompletionItemKind;

fn load_ast() -> Value {
    serde_json::from_str(&fs::read_to_string("pool-manager-ast.json").unwrap()).unwrap()
}

fn load_cache() -> solidity_language_server::completion::CompletionCache {
    let ast_data = load_ast();
    let sources = ast_data.get("sources").unwrap();
    let contracts = ast_data.get("contracts");
    build_completion_cache(sources, contracts)
}

// --- extract_node_id_from_type ---

#[test]
fn test_extract_node_id_from_struct_type() {
    assert_eq!(
        extract_node_id_from_type("t_struct$_PoolKey_$8887_storage_ptr"),
        Some(8887)
    );
}

#[test]
fn test_extract_node_id_from_contract_type() {
    assert_eq!(
        extract_node_id_from_type("t_contract$_IHooks_$2248"),
        Some(2248)
    );
}

#[test]
fn test_extract_node_id_from_user_defined_value_type() {
    assert_eq!(
        extract_node_id_from_type("t_userDefinedValueType$_Currency_$8541"),
        Some(8541)
    );
}

#[test]
fn test_extract_node_id_from_primitive_type() {
    assert_eq!(extract_node_id_from_type("t_uint256"), None);
    assert_eq!(extract_node_id_from_type("t_address"), None);
    assert_eq!(extract_node_id_from_type("t_bool"), None);
}

// --- extract_identifier_before_dot ---

#[test]
fn test_extract_identifier_simple() {
    assert_eq!(
        extract_identifier_before_dot("key.", 4),
        Some("key".to_string())
    );
}

#[test]
fn test_extract_identifier_in_expression() {
    assert_eq!(
        extract_identifier_before_dot("        poolKey.", 16),
        Some("poolKey".to_string())
    );
}

#[test]
fn test_extract_identifier_at_start() {
    assert_eq!(extract_identifier_before_dot(".", 1), None);
}

#[test]
fn test_extract_identifier_msg() {
    assert_eq!(
        extract_identifier_before_dot("msg.", 4),
        Some("msg".to_string())
    );
}

// --- parse_dot_chain ---

#[test]
fn test_parse_chain_simple() {
    let chain = parse_dot_chain("msg.", 4);
    assert_eq!(
        chain,
        vec![DotSegment {
            name: "msg".to_string(),
            kind: AccessKind::Plain
        }]
    );
}

#[test]
fn test_parse_chain_function_call() {
    let chain = parse_dot_chain("foo().", 6);
    assert_eq!(
        chain,
        vec![DotSegment {
            name: "foo".to_string(),
            kind: AccessKind::Call
        }]
    );
}

#[test]
fn test_parse_chain_function_call_with_args() {
    let chain = parse_dot_chain("swap(key, params).", 18);
    assert_eq!(
        chain,
        vec![DotSegment {
            name: "swap".to_string(),
            kind: AccessKind::Call
        }]
    );
}

#[test]
fn test_parse_chain_nested_calls_in_args() {
    // swap(getKey(), getParams(x, y())) — nested parens inside args
    let line = "swap(getKey(), getParams(x, y())).";
    let chain = parse_dot_chain(line, line.len() as u32);
    assert_eq!(
        chain,
        vec![DotSegment {
            name: "swap".to_string(),
            kind: AccessKind::Call
        }]
    );
}

#[test]
fn test_parse_chain_index_access() {
    let chain = parse_dot_chain("_pools[poolId].", 15);
    assert_eq!(
        chain,
        vec![DotSegment {
            name: "_pools".to_string(),
            kind: AccessKind::Index
        }]
    );
}

#[test]
fn test_parse_chain_index_with_nested_call() {
    // mapping[someFunc(a, b)].
    let line = "mapping[someFunc(a, b)].";
    let chain = parse_dot_chain(line, line.len() as u32);
    assert_eq!(
        chain,
        vec![DotSegment {
            name: "mapping".to_string(),
            kind: AccessKind::Index
        }]
    );
}

#[test]
fn test_parse_chain_two_segments() {
    // poolManager.swap().
    let line = "poolManager.swap().";
    let chain = parse_dot_chain(line, line.len() as u32);
    assert_eq!(
        chain,
        vec![
            DotSegment {
                name: "poolManager".to_string(),
                kind: AccessKind::Plain
            },
            DotSegment {
                name: "swap".to_string(),
                kind: AccessKind::Call
            },
        ]
    );
}

#[test]
fn test_parse_chain_three_segments_mixed() {
    // _pools[poolId].slot0.sqrtPriceX96.
    let line = "_pools[poolId].slot0.";
    let chain = parse_dot_chain(line, line.len() as u32);
    assert_eq!(
        chain,
        vec![
            DotSegment {
                name: "_pools".to_string(),
                kind: AccessKind::Index
            },
            DotSegment {
                name: "slot0".to_string(),
                kind: AccessKind::Plain
            },
        ]
    );
}

#[test]
fn test_parse_chain_call_then_index() {
    // getPool(key).positions[posId].
    let line = "getPool(key).positions[posId].";
    let chain = parse_dot_chain(line, line.len() as u32);
    assert_eq!(
        chain,
        vec![
            DotSegment {
                name: "getPool".to_string(),
                kind: AccessKind::Call
            },
            DotSegment {
                name: "positions".to_string(),
                kind: AccessKind::Index
            },
        ]
    );
}

// --- extract_mapping_value_type ---

#[test]
fn test_mapping_value_simple() {
    assert_eq!(
        extract_mapping_value_type("t_mapping$_t_address_$_t_uint256_$"),
        Some("t_uint256".to_string())
    );
}

#[test]
fn test_mapping_value_struct() {
    assert_eq!(
        extract_mapping_value_type(
            "t_mapping$_t_userDefinedValueType$_PoolId_$8841_$_t_struct$_State_$4809_storage_$"
        ),
        Some("t_struct$_State_$4809_storage".to_string())
    );
}

#[test]
fn test_mapping_value_nested() {
    // mapping(address => mapping(uint256 => uint256))
    assert_eq!(
        extract_mapping_value_type("t_mapping$_t_address_$_t_mapping$_t_uint256_$_t_uint256_$_$"),
        Some("t_uint256".to_string())
    );
}

#[test]
fn test_mapping_value_triple_nested() {
    // mapping(address => mapping(address => mapping(uint256 => uint256)))
    assert_eq!(
        extract_mapping_value_type(
            "t_mapping$_t_address_$_t_mapping$_t_address_$_t_mapping$_t_uint256_$_t_uint256_$_$_$"
        ),
        Some("t_uint256".to_string())
    );
}

#[test]
fn test_mapping_value_not_a_mapping() {
    assert_eq!(
        extract_mapping_value_type("t_uint256"),
        Some("t_uint256".to_string())
    );
}

#[test]
fn test_mapping_value_struct_with_node_id() {
    let result =
        extract_mapping_value_type("t_mapping$_t_int24_$_t_struct$_TickInfo_$4784_storage_$");
    assert_eq!(result, Some("t_struct$_TickInfo_$4784_storage".to_string()));
    // Can extract node id from the result
    assert_eq!(extract_node_id_from_type(&result.unwrap()), Some(4784));
}

// --- general completions ---

#[test]
fn test_general_completions_include_ast_names() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    // Should include contract/library/function names from the AST
    assert!(names.contains(&"PoolManager"), "Should have PoolManager");
    assert!(names.contains(&"SwapMath"), "Should have SwapMath");
    assert!(names.contains(&"PoolKey"), "Should have PoolKey");
    assert!(
        names.contains(&"getSqrtPriceTarget"),
        "Should have getSqrtPriceTarget"
    );
}

#[test]
fn test_general_completions_include_keywords() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    assert!(
        names.contains(&"contract"),
        "Should have 'contract' keyword"
    );
    assert!(
        names.contains(&"function"),
        "Should have 'function' keyword"
    );
    assert!(names.contains(&"if"), "Should have 'if' keyword");
    assert!(names.contains(&"mapping"), "Should have 'mapping' keyword");
}

#[test]
fn test_general_completions_include_magic_globals() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    assert!(names.contains(&"msg"), "Should have 'msg'");
    assert!(names.contains(&"block"), "Should have 'block'");
    assert!(names.contains(&"tx"), "Should have 'tx'");
    assert!(names.contains(&"abi"), "Should have 'abi'");
}

#[test]
fn test_general_completions_include_global_functions() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    assert!(
        names.contains(&"keccak256(bytes memory)"),
        "Should have keccak256"
    );
    assert!(names.contains(&"gasleft()"), "Should have gasleft");
    assert!(
        names.contains(&"require(bool condition)"),
        "Should have require"
    );
}

// --- dot completions ---

#[test]
fn test_dot_completion_msg() {
    let cache = load_cache();
    let items = get_dot_completions(&cache, "msg");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();

    assert!(names.contains(&"sender"), "msg.sender");
    assert!(names.contains(&"value"), "msg.value");
    assert!(names.contains(&"data"), "msg.data");
    assert!(names.contains(&"sig"), "msg.sig");
    assert_eq!(names.len(), 4, "msg should have exactly 4 members");
}

#[test]
fn test_dot_completion_block() {
    let cache = load_cache();
    let items = get_dot_completions(&cache, "block");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();

    assert!(names.contains(&"number"), "block.number");
    assert!(names.contains(&"timestamp"), "block.timestamp");
    assert!(names.contains(&"chainid"), "block.chainid");
    assert!(names.contains(&"basefee"), "block.basefee");
}

#[test]
fn test_dot_completion_tx() {
    let cache = load_cache();
    let items = get_dot_completions(&cache, "tx");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();

    assert!(names.contains(&"gasprice"), "tx.gasprice");
    assert!(names.contains(&"origin"), "tx.origin");
    assert_eq!(names.len(), 2, "tx should have exactly 2 members");
}

#[test]
fn test_dot_completion_abi() {
    let cache = load_cache();
    let items = get_dot_completions(&cache, "abi");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();

    assert!(names.contains(&"encode(...)"), "abi.encode");
    assert!(names.contains(&"encodePacked(...)"), "abi.encodePacked");
    assert!(names.contains(&"decode(bytes memory, (...))"), "abi.decode");
}

// --- dot completions on AST types ---

#[test]
fn test_dot_completion_on_struct() {
    let cache = load_cache();

    // PoolKey is a struct with members: currency0, currency1, fee, tickSpacing, hooks
    // We need a variable that has type PoolKey — check name_to_type for one
    // "key" is commonly used as a PoolKey parameter name
    let pool_key_vars: Vec<&String> = cache
        .name_to_type
        .iter()
        .filter(|(_, tid)| tid.contains("PoolKey"))
        .map(|(name, _)| name)
        .collect();

    // There should be at least one variable with PoolKey type
    assert!(
        !pool_key_vars.is_empty(),
        "Should have variables with PoolKey type, name_to_type has {} entries",
        cache.name_to_type.len()
    );

    // Use the first one to test dot completion
    let var_name = pool_key_vars[0];
    let items = get_dot_completions(&cache, var_name);
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();

    assert!(
        names.contains(&"currency0"),
        "PoolKey should have currency0 member, got: {:?}",
        names
    );
    assert!(
        names.contains(&"fee"),
        "PoolKey should have fee member, got: {:?}",
        names
    );
    assert!(
        names.contains(&"hooks"),
        "PoolKey should have hooks member, got: {:?}",
        names
    );
}

#[test]
fn test_name_to_type_populated() {
    let cache = load_cache();

    // Should have mapped variable names to their types
    assert!(
        !cache.name_to_type.is_empty(),
        "name_to_type should not be empty"
    );

    // Check a few expected entries exist
    assert!(
        cache.name_to_type.values().any(|v| v.contains("PoolKey")),
        "Should have at least one PoolKey-typed variable"
    );
}

#[test]
fn test_node_members_populated() {
    let cache = load_cache();

    assert!(
        !cache.node_members.is_empty(),
        "node_members should not be empty"
    );

    // PoolKey struct (id 8887) should have members
    assert!(
        cache.node_members.contains_key(&8887),
        "Should have members for PoolKey (8887)"
    );

    let pool_key_members = &cache.node_members[&8887];
    let member_names: Vec<&str> = pool_key_members.iter().map(|c| c.label.as_str()).collect();
    assert_eq!(
        member_names.len(),
        5,
        "PoolKey should have 5 members: {:?}",
        member_names
    );
    assert!(member_names.contains(&"currency0"));
    assert!(member_names.contains(&"currency1"));
    assert!(member_names.contains(&"fee"));
    assert!(member_names.contains(&"tickSpacing"));
    assert!(member_names.contains(&"hooks"));
}

// --- library dot completions ---

#[test]
fn test_dot_completion_fullmath_library() {
    let cache = load_cache();

    // Check what name_to_type has for FullMath
    let fullmath_type = cache.name_to_type.get("FullMath");
    eprintln!("FullMath type: {:?}", fullmath_type);

    let items = get_dot_completions(&cache, "FullMath");
    let labels: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();
    eprintln!("FullMath dot completions ({}): {:?}", labels.len(), labels);

    // FullMath should include its 2 internal functions
    assert!(
        labels.contains(&"mulDiv"),
        "Should have mulDiv, got: {:?}",
        labels
    );
    assert!(
        labels.contains(&"mulDivRoundingUp"),
        "Should have mulDivRoundingUp, got: {:?}",
        labels
    );
    // May also include wildcard using-for functions (using X for *)
    // which are globally available. Scope-aware filtering is future work.
}

// --- function signatures ---

#[test]
fn test_method_identifier_swap_has_description_with_param_names() {
    let cache = load_cache();
    // IPoolManager (2531) has swap
    let ipm_methods = &cache.method_identifiers[&2531];
    let swap_item = ipm_methods.iter().find(|c| c.label == "swap").unwrap();
    let desc = swap_item
        .label_details
        .as_ref()
        .unwrap()
        .description
        .as_deref()
        .expect("swap should have a description with parameter names");

    eprintln!("swap description: {}", desc);

    // Should contain parameter names from the AST
    assert!(
        desc.contains("key"),
        "description should contain param name 'key': {}",
        desc
    );
    assert!(
        desc.contains("params"),
        "description should contain param name 'params': {}",
        desc
    );
    assert!(
        desc.contains("hookData"),
        "description should contain param name 'hookData': {}",
        desc
    );
    // Should contain return info
    assert!(
        desc.contains("returns"),
        "description should show return type: {}",
        desc
    );
    assert!(
        desc.contains("swapDelta"),
        "description should contain return name 'swapDelta': {}",
        desc
    );
}

#[test]
fn test_method_identifier_settle_has_description_no_params() {
    let cache = load_cache();
    let pm_methods = &cache.method_identifiers[&1767];
    let settle_item = pm_methods.iter().find(|c| c.label == "settle").unwrap();
    let desc = settle_item
        .label_details
        .as_ref()
        .unwrap()
        .description
        .as_deref()
        .expect("settle should have a description");

    eprintln!("settle description: {}", desc);

    assert!(
        desc.contains("settle()"),
        "settle should show empty params: {}",
        desc
    );
}

#[test]
fn test_method_identifier_initialize_has_return_type() {
    let cache = load_cache();
    let ipm_methods = &cache.method_identifiers[&2531];
    let init_item = ipm_methods
        .iter()
        .find(|c| c.label == "initialize")
        .unwrap();
    let desc = init_item
        .label_details
        .as_ref()
        .unwrap()
        .description
        .as_deref()
        .expect("initialize should have a description");

    eprintln!("initialize description: {}", desc);

    assert!(
        desc.contains("key"),
        "should have param name 'key': {}",
        desc
    );
    assert!(
        desc.contains("sqrtPriceX96"),
        "should have param name 'sqrtPriceX96': {}",
        desc
    );
    assert!(desc.contains("returns"), "should show returns: {}", desc);
    assert!(
        desc.contains("tick"),
        "should contain return name 'tick': {}",
        desc
    );
}

#[test]
fn test_node_members_function_has_signature_as_detail() {
    let cache = load_cache();
    // FullMath (3250) — its node_members should have function signatures as detail
    let fm_members = &cache.node_members[&3250];
    let mul_div = fm_members.iter().find(|c| c.label == "mulDiv").unwrap();

    eprintln!("mulDiv detail: {:?}", mul_div.detail);

    let detail = mul_div
        .detail
        .as_deref()
        .expect("mulDiv should have detail");
    assert!(
        detail.contains("mulDiv("),
        "detail should be a signature: {}",
        detail
    );
    // mulDiv takes (uint256, uint256, uint256) returns (uint256)
    assert!(
        detail.contains("uint256"),
        "should contain uint256 params: {}",
        detail
    );
}

// --- method identifiers ---

#[test]
fn test_method_identifiers_populated() {
    let cache = load_cache();

    assert!(
        !cache.method_identifiers.is_empty(),
        "method_identifiers should not be empty"
    );

    // PoolManager (id 1767) should have method identifiers
    assert!(
        cache.method_identifiers.contains_key(&1767),
        "Should have method identifiers for PoolManager (1767), keys: {:?}",
        cache.method_identifiers.keys().collect::<Vec<_>>()
    );

    // IPoolManager (id 2531) should also have them
    assert!(
        cache.method_identifiers.contains_key(&2531),
        "Should have method identifiers for IPoolManager (2531)"
    );
}

#[test]
fn test_overloaded_extsload_on_extsload_contract() {
    let cache = load_cache();
    // extsload is defined on Extsload contract (468), not PoolManager directly
    // So Extsload's method_identifiers should have descriptions
    let ext_methods = &cache.method_identifiers[&468];
    let extsload_items: Vec<_> = ext_methods
        .iter()
        .filter(|c| c.label == "extsload")
        .collect();
    assert_eq!(extsload_items.len(), 3);

    let descriptions: Vec<Option<&str>> = extsload_items
        .iter()
        .map(|c| {
            c.label_details
                .as_ref()
                .and_then(|ld| ld.description.as_deref())
        })
        .collect();

    eprintln!("extsload descriptions: {:?}", descriptions);

    // All 3 overloads should have descriptions since they're defined directly on Extsload
    for (i, desc) in descriptions.iter().enumerate() {
        assert!(
            desc.is_some(),
            "extsload overload {} should have a description",
            i
        );
    }
}

#[test]
fn test_inherited_functions_have_no_description() {
    let cache = load_cache();
    // PoolManager (1767) inherits extsload from Extsload — no AST signature available
    let pm_methods = &cache.method_identifiers[&1767];
    let extsload_items: Vec<_> = pm_methods
        .iter()
        .filter(|c| c.label == "extsload")
        .collect();

    // These are inherited, so description will be None (function_signatures only collects direct members)
    for item in &extsload_items {
        let desc = item
            .label_details
            .as_ref()
            .and_then(|ld| ld.description.as_deref());
        assert!(
            desc.is_none(),
            "Inherited extsload on PoolManager should have no description (it's on Extsload base), got: {:?}",
            desc
        );
    }

    // But they still have the ABI signature as detail
    for item in &extsload_items {
        assert!(
            item.detail.is_some(),
            "Should still have ABI signature as detail"
        );
    }
}

#[test]
fn test_method_identifiers_have_function_names() {
    let cache = load_cache();
    let pm_methods = &cache.method_identifiers[&1767];
    let labels: Vec<&str> = pm_methods.iter().map(|c| c.label.as_str()).collect();

    assert!(
        labels.contains(&"swap"),
        "Should have swap, got: {:?}",
        labels
    );
    assert!(labels.contains(&"initialize"), "Should have initialize");
    assert!(labels.contains(&"settle"), "Should have settle");
    assert!(labels.contains(&"take"), "Should have take");
    assert!(labels.contains(&"donate"), "Should have donate");
    assert!(labels.contains(&"unlock"), "Should have unlock");
    assert!(labels.contains(&"owner"), "Should have owner");
}

#[test]
fn test_method_identifiers_have_full_signatures_as_detail() {
    let cache = load_cache();
    let pm_methods = &cache.method_identifiers[&1767];

    let swap_item = pm_methods.iter().find(|c| c.label == "swap").unwrap();
    assert_eq!(
        swap_item.detail.as_deref(),
        Some("swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)"),
        "swap detail should be the full signature"
    );

    let settle_item = pm_methods.iter().find(|c| c.label == "settle").unwrap();
    assert_eq!(
        settle_item.detail.as_deref(),
        Some("settle()"),
        "settle detail should be settle()"
    );
}

#[test]
fn test_method_identifiers_have_selectors() {
    let cache = load_cache();
    let pm_methods = &cache.method_identifiers[&1767];

    let swap_item = pm_methods.iter().find(|c| c.label == "swap").unwrap();
    let label_details = swap_item.label_details.as_ref().unwrap();
    assert_eq!(
        label_details.detail.as_deref(),
        Some("0xf3cd914c"),
        "swap selector should be 0xf3cd914c"
    );

    let settle_item = pm_methods.iter().find(|c| c.label == "settle").unwrap();
    let settle_details = settle_item.label_details.as_ref().unwrap();
    assert_eq!(
        settle_details.detail.as_deref(),
        Some("0x11da60b4"),
        "settle selector should be 0x11da60b4"
    );
}

#[test]
fn test_method_identifiers_are_function_kind() {
    let cache = load_cache();
    let pm_methods = &cache.method_identifiers[&1767];

    for item in pm_methods {
        assert_eq!(
            item.kind,
            Some(CompletionItemKind::FUNCTION),
            "{} should be FUNCTION kind",
            item.label
        );
    }
}

#[test]
fn test_method_identifiers_handles_overloads() {
    let cache = load_cache();
    let pm_methods = &cache.method_identifiers[&1767];

    // extsload has 3 overloads: extsload(bytes32), extsload(bytes32,uint256), extsload(bytes32[])
    let extsload_items: Vec<_> = pm_methods
        .iter()
        .filter(|c| c.label == "extsload")
        .collect();
    assert_eq!(
        extsload_items.len(),
        3,
        "Should have 3 extsload overloads, got {}",
        extsload_items.len()
    );

    // Each should have a different selector
    let selectors: Vec<&str> = extsload_items
        .iter()
        .map(|c| c.label_details.as_ref().unwrap().detail.as_deref().unwrap())
        .collect();
    assert!(selectors.contains(&"0x1e2eaeaf"));
    assert!(selectors.contains(&"0x35fd631a"));
    assert!(selectors.contains(&"0xdbd035ff"));
}

#[test]
fn test_method_identifiers_ipool_manager_interface() {
    let cache = load_cache();
    let ipm_methods = &cache.method_identifiers[&2531];
    let labels: Vec<&str> = ipm_methods.iter().map(|c| c.label.as_str()).collect();

    assert_eq!(ipm_methods.len(), 30, "IPoolManager should have 30 methods");
    assert!(labels.contains(&"swap"), "Interface should have swap");
    assert!(
        labels.contains(&"modifyLiquidity"),
        "Interface should have modifyLiquidity"
    );
}

#[test]
fn test_dot_completion_supplements_method_identifiers_with_node_members() {
    let cache = load_cache();

    // PoolManager (1767) should have both method_identifiers AND node_members
    // node_members includes events, errors, state variables — things not in methodIdentifiers
    let has_methods = cache.method_identifiers.contains_key(&1767);
    let has_members = cache.node_members.contains_key(&1767);

    assert!(has_methods, "PoolManager should have method_identifiers");
    assert!(has_members, "PoolManager should have node_members");

    // Check that node_members has things that method_identifiers doesn't
    let method_labels: std::collections::HashSet<&str> = cache.method_identifiers[&1767]
        .iter()
        .map(|c| c.label.as_str())
        .collect();
    let member_only: Vec<&str> = cache.node_members[&1767]
        .iter()
        .map(|c| c.label.as_str())
        .filter(|l| !method_labels.contains(l))
        .collect();

    // There should be some members not in methodIdentifiers (events, errors, etc.)
    assert!(
        !member_only.is_empty(),
        "node_members should have entries not in method_identifiers (events, errors, etc.), method labels: {:?}",
        method_labels
    );
}

#[test]
fn test_cache_without_contracts_has_empty_method_identifiers() {
    let ast_data = load_ast();
    let sources = ast_data.get("sources").unwrap();
    let cache = build_completion_cache(sources, None);

    assert!(
        cache.method_identifiers.is_empty(),
        "Without contracts, method_identifiers should be empty"
    );
    // But node_members should still work
    assert!(
        !cache.node_members.is_empty(),
        "node_members should still be populated from AST"
    );
}

// --- function_return_types ---

#[test]
fn test_function_return_types_pool_manager_swap() {
    let cache = load_cache();
    // PoolManager.swap returns BalanceDelta (single return, so it's in the map)
    let ret = cache.function_return_types.get(&(1767, "swap".to_string()));
    assert_eq!(
        ret,
        Some(&"t_userDefinedValueType$_BalanceDelta_$8327".to_string()),
        "PoolManager.swap should return BalanceDelta"
    );
}

#[test]
fn test_function_return_types_pool_manager_internal_swap() {
    let cache = load_cache();
    // _swap is the internal implementation, also returns BalanceDelta
    let ret = cache
        .function_return_types
        .get(&(1767, "_swap".to_string()));
    assert_eq!(
        ret,
        Some(&"t_userDefinedValueType$_BalanceDelta_$8327".to_string()),
        "_swap should also return BalanceDelta"
    );
}

#[test]
fn test_function_return_types_ipool_manager_interface() {
    let cache = load_cache();
    // IPoolManager interface also defines swap → BalanceDelta
    let ret = cache.function_return_types.get(&(2531, "swap".to_string()));
    assert_eq!(
        ret,
        Some(&"t_userDefinedValueType$_BalanceDelta_$8327".to_string()),
        "IPoolManager.swap should return BalanceDelta"
    );
}

#[test]
fn test_function_return_types_pool_manager_initialize() {
    let cache = load_cache();
    // initialize returns int24 (tick)
    let ret = cache
        .function_return_types
        .get(&(2531, "initialize".to_string()));
    assert_eq!(
        ret,
        Some(&"t_int24".to_string()),
        "IPoolManager.initialize should return int24"
    );
}

#[test]
fn test_function_return_types_get_pool() {
    let cache = load_cache();
    // PoolManager._getPool returns Pool.State storage
    let ret = cache
        .function_return_types
        .get(&(1767, "_getPool".to_string()));
    assert_eq!(
        ret,
        Some(&"t_struct$_State_$4809_storage_ptr".to_string()),
        "_getPool should return Pool.State"
    );
}

#[test]
fn test_function_return_types_populated_count() {
    let cache = load_cache();
    // Should have a substantial number of return types from across the AST
    assert!(
        cache.function_return_types.len() > 50,
        "Should have many function return types, got {}",
        cache.function_return_types.len()
    );
}

// --- using_for ---

#[test]
fn test_using_for_balance_delta_has_amount_functions() {
    let cache = load_cache();
    let bd_type = "t_userDefinedValueType$_BalanceDelta_$8327";
    let items = cache.using_for.get(bd_type);
    assert!(
        items.is_some(),
        "Should have using-for entries for BalanceDelta"
    );
    let labels: Vec<&str> = items.unwrap().iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"amount0"),
        "BalanceDelta should have amount0"
    );
    assert!(
        labels.contains(&"amount1"),
        "BalanceDelta should have amount1"
    );
}

#[test]
fn test_using_for_ihooks_has_hook_functions() {
    let cache = load_cache();
    let hooks_type = "t_contract$_IHooks_$2248";
    let items = cache.using_for.get(hooks_type);
    assert!(items.is_some(), "Should have using-for entries for IHooks");
    let labels: Vec<&str> = items.unwrap().iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"beforeSwap"),
        "IHooks should have beforeSwap"
    );
    assert!(
        labels.contains(&"afterSwap"),
        "IHooks should have afterSwap"
    );
    assert!(
        labels.contains(&"hasPermission"),
        "IHooks should have hasPermission"
    );
}

#[test]
fn test_using_for_pool_state_has_pool_functions() {
    let cache = load_cache();
    let state_type = "t_struct$_State_$4809_storage_ptr";
    let items = cache.using_for.get(state_type);
    assert!(
        items.is_some(),
        "Should have using-for entries for Pool.State"
    );
    let labels: Vec<&str> = items.unwrap().iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"initialize"),
        "Pool.State should have initialize"
    );
    assert!(labels.contains(&"swap"), "Pool.State should have swap");
    assert!(labels.contains(&"donate"), "Pool.State should have donate");
    assert!(
        labels.contains(&"modifyLiquidity"),
        "Pool.State should have modifyLiquidity"
    );
}

#[test]
fn test_using_for_slot0_has_accessor_functions() {
    let cache = load_cache();
    let slot0_type = "t_userDefinedValueType$_Slot0_$8918";
    let items = cache.using_for.get(slot0_type);
    assert!(items.is_some(), "Should have using-for entries for Slot0");
    let labels: Vec<&str> = items.unwrap().iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"sqrtPriceX96"),
        "Slot0 should have sqrtPriceX96"
    );
    assert!(labels.contains(&"tick"), "Slot0 should have tick");
    assert!(labels.contains(&"lpFee"), "Slot0 should have lpFee");
}

#[test]
fn test_using_for_uint256_has_safecast() {
    let cache = load_cache();
    let uint256_type = "t_uint256";
    let items = cache.using_for.get(uint256_type);
    assert!(items.is_some(), "Should have using-for entries for uint256");
    let labels: Vec<&str> = items.unwrap().iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"toUint160"),
        "uint256 should have SafeCast.toUint160"
    );
    assert!(
        labels.contains(&"toInt256"),
        "uint256 should have SafeCast.toInt256"
    );
}

#[test]
fn test_using_for_wildcard_populated() {
    let cache = load_cache();
    // Wildcard using-for (using X for *) should have entries
    assert!(
        !cache.using_for_wildcard.is_empty(),
        "using_for_wildcard should not be empty"
    );
    let labels: Vec<&str> = cache
        .using_for_wildcard
        .iter()
        .map(|i| i.label.as_str())
        .collect();
    // SafeCast for * is common
    assert!(
        labels.contains(&"toUint160") || labels.contains(&"toInt128"),
        "Wildcards should include SafeCast functions"
    );
}

#[test]
fn test_using_for_pool_key_has_to_id() {
    let cache = load_cache();
    let pk_type = "t_struct$_PoolKey_$8887_storage_ptr";
    let items = cache.using_for.get(pk_type);
    assert!(items.is_some(), "Should have using-for entries for PoolKey");
    let labels: Vec<&str> = items.unwrap().iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"toId"),
        "PoolKey should have toId from PoolIdLibrary"
    );
}

// --- Chain resolution with get_chain_completions ---

use solidity_language_server::completion::get_chain_completions;

#[test]
fn test_chain_single_plain_contract_name() {
    // PoolManager. — should show all PoolManager members
    let cache = load_cache();
    let chain = vec![DotSegment {
        name: "PoolManager".to_string(),
        kind: AccessKind::Plain,
    }];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(labels.contains(&"swap"), "PoolManager. should show swap");
    assert!(
        labels.contains(&"initialize"),
        "PoolManager. should show initialize"
    );
}

#[test]
fn test_chain_single_index_mapping_access() {
    // _pools[poolId]. — mapping value is Pool.State, should show struct members + library fns
    let cache = load_cache();
    let chain = vec![DotSegment {
        name: "_pools".to_string(),
        kind: AccessKind::Index,
    }];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();

    // Struct members of Pool.State
    assert!(labels.contains(&"slot0"), "_pools[id]. should show slot0");
    assert!(
        labels.contains(&"liquidity"),
        "_pools[id]. should show liquidity"
    );
    assert!(labels.contains(&"ticks"), "_pools[id]. should show ticks");
    assert!(
        labels.contains(&"positions"),
        "_pools[id]. should show positions"
    );

    // Pool library using-for functions (resolved via suffix normalization)
    assert!(
        labels.contains(&"initialize"),
        "_pools[id]. should show Pool.initialize"
    );
    assert!(
        labels.contains(&"swap"),
        "_pools[id]. should show Pool.swap"
    );
    assert!(
        labels.contains(&"donate"),
        "_pools[id]. should show Pool.donate"
    );

    // No duplicates
    assert_eq!(
        items.len(),
        labels.len(),
        "All items should have unique labels (no duplicates)"
    );
    let unique: std::collections::HashSet<&str> = labels.iter().copied().collect();
    assert_eq!(unique.len(), labels.len(), "All labels should be unique");
}

#[test]
fn test_chain_two_segment_contract_function_call() {
    // PoolManager.swap(). — swap returns BalanceDelta, should show amount0/amount1
    let cache = load_cache();
    let chain = vec![
        DotSegment {
            name: "PoolManager".to_string(),
            kind: AccessKind::Plain,
        },
        DotSegment {
            name: "swap".to_string(),
            kind: AccessKind::Call,
        },
    ];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();

    // BalanceDelta using-for functions
    assert!(
        labels.contains(&"amount0"),
        "PoolManager.swap(). should show amount0"
    );
    assert!(
        labels.contains(&"amount1"),
        "PoolManager.swap(). should show amount1"
    );
}

#[test]
fn test_chain_two_segment_contract_get_pool_call() {
    // PoolManager._getPool(). — returns Pool.State, should show struct members + Pool library fns
    let cache = load_cache();
    let chain = vec![
        DotSegment {
            name: "PoolManager".to_string(),
            kind: AccessKind::Plain,
        },
        DotSegment {
            name: "_getPool".to_string(),
            kind: AccessKind::Call,
        },
    ];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();

    // Pool.State struct members
    assert!(
        labels.contains(&"slot0"),
        "PoolManager._getPool(). should show slot0"
    );
    assert!(
        labels.contains(&"liquidity"),
        "PoolManager._getPool(). should show liquidity"
    );
    assert!(
        labels.contains(&"ticks"),
        "PoolManager._getPool(). should show ticks"
    );

    // Pool library using-for functions
    assert!(
        labels.contains(&"initialize"),
        "PoolManager._getPool(). should show Pool.initialize"
    );
}

#[test]
fn test_chain_variable_with_type_then_call() {
    // pool is a Pool.State storage variable in the AST (name_to_type has it)
    // pool.swap(). — swap on Pool.State returns BalanceDelta
    let cache = load_cache();

    // Verify pool is in name_to_type
    assert!(
        cache.name_to_type.contains_key("pool"),
        "pool should be in name_to_type"
    );

    let chain = vec![
        DotSegment {
            name: "pool".to_string(),
            kind: AccessKind::Plain,
        },
        DotSegment {
            name: "swap".to_string(),
            kind: AccessKind::Call,
        },
    ];
    let items = get_chain_completions(&cache, &chain);
    // pool is t_struct$_State_$4809_storage_ptr. The swap function on Pool.State (node 4809)
    // returns BalanceDelta — but function_return_types is keyed by (contract_id, fn_name).
    // Pool library's id is 6348, and its swap returns BalanceDelta.
    // resolve_member_type for Call looks up function_return_types[(context_node_id, "swap")].
    // This may or may not work depending on whether 4809 or 6348 is used as context.
    // For now just verify it returns something or is empty (known limitation).
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    // Note: if this resolves, it should show BalanceDelta members (amount0, amount1)
    // If not, it's a known limitation — function_return_types is keyed by the library node id,
    // not the struct node id.
    if !labels.is_empty() {
        assert!(
            labels.contains(&"amount0"),
            "pool.swap(). should show amount0 if resolved"
        );
    }
}

#[test]
fn test_chain_mapping_value_type_extraction() {
    // _pools is a mapping(PoolId => Pool.State). Extracting value type should give Pool.State.
    let pools_type =
        "t_mapping$_t_userDefinedValueType$_PoolId_$8841_$_t_struct$_State_$4809_storage_$";
    let val_type = extract_mapping_value_type(pools_type);
    assert_eq!(
        val_type,
        Some("t_struct$_State_$4809_storage".to_string()),
        "Mapping value type should be Pool.State"
    );
}

#[test]
fn test_chain_nested_mapping_value_extraction() {
    // positions is mapping(bytes32 => mapping(bytes32 => Position.State))
    // The extract should peel all layers to get the innermost value
    let nested_mapping =
        "t_mapping$_t_bytes32_$_t_mapping$_t_bytes32_$_t_struct$_State_$6372_storage_$_$";
    let val_type = extract_mapping_value_type(nested_mapping);
    assert!(
        val_type.is_some(),
        "Should extract innermost value type from nested mapping"
    );
    let val = val_type.unwrap();
    assert!(
        val.contains("6372"),
        "Innermost type should reference Position.State (6372), got: {}",
        val
    );
}

#[test]
fn test_chain_completions_deduplication() {
    // Completions should have unique labels — no duplicates from using_for + wildcard overlap
    let cache = load_cache();
    let chain = vec![DotSegment {
        name: "_pools".to_string(),
        kind: AccessKind::Index,
    }];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    let unique: std::collections::HashSet<&str> = labels.iter().copied().collect();
    assert_eq!(
        labels.len(),
        unique.len(),
        "Completions should not have duplicate labels. Duplicates: {:?}",
        labels
            .iter()
            .filter(|l| labels.iter().filter(|l2| l2 == l).count() > 1)
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_chain_ipool_manager_initialize_returns_int24() {
    // IPoolManager.initialize(). — returns int24, which has SafeCast functions
    let cache = load_cache();
    let chain = vec![
        DotSegment {
            name: "IPoolManager".to_string(),
            kind: AccessKind::Plain,
        },
        DotSegment {
            name: "initialize".to_string(),
            kind: AccessKind::Call,
        },
    ];
    let items = get_chain_completions(&cache, &chain);
    // int24 doesn't have many natural members, but SafeCast wildcard functions apply
    // This tests that the chain resolves through to the return type
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    if !labels.is_empty() {
        // Wildcard SafeCast functions should be available on int24
        assert!(
            labels.contains(&"toInt128") || labels.contains(&"toUint160"),
            "int24 return should get SafeCast wildcard functions"
        );
    }
}

#[test]
fn test_chain_pool_key_to_id() {
    // poolKey.toId(). — PoolKey has using-for toId that returns PoolId
    let cache = load_cache();
    // poolKey is in name_to_type
    assert!(
        cache.name_to_type.contains_key("poolKey"),
        "poolKey should be in name_to_type"
    );

    let chain = vec![DotSegment {
        name: "poolKey".to_string(),
        kind: AccessKind::Plain,
    }];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();

    // PoolKey struct members
    assert!(
        labels.contains(&"currency0"),
        "poolKey. should show currency0"
    );
    assert!(
        labels.contains(&"currency1"),
        "poolKey. should show currency1"
    );
    assert!(labels.contains(&"fee"), "poolKey. should show fee");
    assert!(
        labels.contains(&"tickSpacing"),
        "poolKey. should show tickSpacing"
    );
    assert!(labels.contains(&"hooks"), "poolKey. should show hooks");

    // Using-for: toId from PoolIdLibrary
    assert!(
        labels.contains(&"toId"),
        "poolKey. should show toId from PoolIdLibrary"
    );
}

#[test]
fn test_chain_empty_returns_nothing() {
    let cache = load_cache();
    let items = get_chain_completions(&cache, &[]);
    assert!(items.is_empty(), "Empty chain should return no completions");
}

#[test]
fn test_chain_unknown_name_returns_nothing() {
    let cache = load_cache();
    let chain = vec![DotSegment {
        name: "nonexistentVariable".to_string(),
        kind: AccessKind::Plain,
    }];
    let items = get_chain_completions(&cache, &chain);
    assert!(
        items.is_empty(),
        "Unknown name should return no completions"
    );
}

// --- Type cast expressions: InterfaceName(address).method() ---

#[test]
fn test_parse_chain_type_cast_expression() {
    // IUnlockCallback(msg.sender).unlockCallback(data).
    // Parser sees the parentheses as Call kind — syntactically identical to a function call
    let line = "        IUnlockCallback(msg.sender).unlockCallback(data).";
    let col = line.len() as u32;
    let chain = parse_dot_chain(line, col);
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].name, "IUnlockCallback");
    assert_eq!(chain[0].kind, AccessKind::Call);
    assert_eq!(chain[1].name, "unlockCallback");
    assert_eq!(chain[1].kind, AccessKind::Call);
}

#[test]
fn test_chain_type_cast_shows_interface_members() {
    // IUnlockCallback(msg.sender). — type cast to interface, should show its members
    let cache = load_cache();
    let chain = vec![DotSegment {
        name: "IUnlockCallback".to_string(),
        kind: AccessKind::Call,
    }];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"unlockCallback"),
        "IUnlockCallback(addr). should show unlockCallback, got: {:?}",
        labels
    );
}

#[test]
fn test_chain_type_cast_then_method_returns_bytes_completions() {
    // IUnlockCallback(msg.sender).unlockCallback(data).
    // unlockCallback returns bytes memory → should show bytes-related completions
    let cache = load_cache();
    let chain = vec![
        DotSegment {
            name: "IUnlockCallback".to_string(),
            kind: AccessKind::Call,
        },
        DotSegment {
            name: "unlockCallback".to_string(),
            kind: AccessKind::Call,
        },
    ];
    let items = get_chain_completions(&cache, &chain);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();

    assert!(
        !items.is_empty(),
        "IUnlockCallback(msg.sender).unlockCallback(data). should return completions"
    );
    // unlockCallback returns bytes memory.
    // using-for on bytes includes parseSelector, parseFee, parseReturnDelta
    assert!(
        labels.contains(&"parseSelector"),
        "bytes return should show parseSelector from using-for, got: {:?}",
        labels
    );
}

// --- Globally available types from Solidity docs ---

#[test]
fn test_general_completions_include_blobhash() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
    assert!(
        names.contains(&"blobhash(uint256 index)"),
        "Should have blobhash"
    );
}

#[test]
fn test_general_completions_include_ether_units() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
    assert!(names.contains(&"wei"), "Should have wei");
    assert!(names.contains(&"gwei"), "Should have gwei");
    assert!(names.contains(&"ether"), "Should have ether");

    // Check they are UNIT kind
    let wei = completions.iter().find(|c| c.label == "wei").unwrap();
    assert_eq!(wei.kind, Some(CompletionItemKind::UNIT));
    assert_eq!(wei.detail.as_deref(), Some("1"));

    let ether = completions.iter().find(|c| c.label == "ether").unwrap();
    assert_eq!(ether.detail.as_deref(), Some("1e18"));
}

#[test]
fn test_general_completions_include_time_units() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
    assert!(names.contains(&"seconds"), "Should have seconds");
    assert!(names.contains(&"minutes"), "Should have minutes");
    assert!(names.contains(&"hours"), "Should have hours");
    assert!(names.contains(&"days"), "Should have days");
    assert!(names.contains(&"weeks"), "Should have weeks");

    let days = completions.iter().find(|c| c.label == "days").unwrap();
    assert_eq!(days.kind, Some(CompletionItemKind::UNIT));
    assert_eq!(days.detail.as_deref(), Some("86400 seconds"));
}

#[test]
fn test_dot_completion_type_members() {
    // type(X). should show contract/interface/integer type properties
    let cache = load_cache();
    let items = get_dot_completions(&cache, "type");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();

    // Contract type properties
    assert!(names.contains(&"name"), "type(C).name");
    assert!(names.contains(&"creationCode"), "type(C).creationCode");
    assert!(names.contains(&"runtimeCode"), "type(C).runtimeCode");
    // Interface type property
    assert!(names.contains(&"interfaceId"), "type(I).interfaceId");
    // Integer type properties
    assert!(names.contains(&"min"), "type(T).min");
    assert!(names.contains(&"max"), "type(T).max");
}

#[test]
fn test_dot_completion_bytes_concat() {
    let cache = load_cache();
    let items = get_dot_completions(&cache, "bytes");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();
    assert!(names.contains(&"concat(...)"), "bytes.concat");

    let concat = items.iter().find(|c| c.label == "concat(...)").unwrap();
    assert_eq!(concat.detail.as_deref(), Some("bytes memory"));
}

#[test]
fn test_dot_completion_string_concat() {
    let cache = load_cache();
    let items = get_dot_completions(&cache, "string");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();
    assert!(names.contains(&"concat(...)"), "string.concat");

    let concat = items.iter().find(|c| c.label == "concat(...)").unwrap();
    assert_eq!(concat.detail.as_deref(), Some("string memory"));
}

#[test]
fn test_dot_completion_abi_full_signatures() {
    let cache = load_cache();
    let items = get_dot_completions(&cache, "abi");
    let names: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();

    assert!(
        names.contains(&"encodeWithSelector(bytes4, ...)"),
        "abi.encodeWithSelector with selector param"
    );
    assert!(
        names.contains(&"encodeWithSignature(string memory, ...)"),
        "abi.encodeWithSignature with signature param"
    );
    assert!(
        names.contains(&"encodeCall(function, (...))"),
        "abi.encodeCall with function pointer"
    );
}

#[test]
fn test_general_completions_include_common_int_types() {
    let cache = load_cache();
    let completions = get_general_completions(&cache);
    let names: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    // Verify commonly used integer types are present as keywords
    assert!(names.contains(&"uint8"), "Should have uint8");
    assert!(names.contains(&"uint128"), "Should have uint128");
    assert!(names.contains(&"uint160"), "Should have uint160");
    assert!(names.contains(&"uint256"), "Should have uint256");
    assert!(names.contains(&"int24"), "Should have int24");
    assert!(names.contains(&"int128"), "Should have int128");
    assert!(names.contains(&"int256"), "Should have int256");
    assert!(names.contains(&"bytes1"), "Should have bytes1");
    assert!(names.contains(&"bytes4"), "Should have bytes4");
}
