use solidity_language_server::completion::member_access;
use solidity_language_server::symbols::extract_symbols;
use solidity_language_server::completion::handler::get_scoped_completions;
use std::process::Command;
use tower_lsp::lsp_types::{Position, CompletionItemKind, InsertTextFormat};

fn get_test_ast_data() -> Option<serde_json::Value> {
    println!("Running forge build in example directory");
    let output = Command::new("forge")
        .args(["build", "--json", "--no-cache", "--ast"])
        .current_dir("example")
        .output();

    match output {
        Ok(out) => {
            println!("Forge command succeeded, stdout len: {}", out.stdout.len());
            if !out.status.success() {
                println!("Forge command failed with status: {:?}", out.status);
                println!("Stderr: {}", String::from_utf8_lossy(&out.stderr));
                return None;
            }
            let stdout_str = String::from_utf8(out.stdout).ok()?;
            println!("Parsing JSON...");
            let json = serde_json::from_str(&stdout_str);
            match json {
                Ok(j) => Some(j),
                Err(e) => {
                    println!("JSON parse error: {:?}", e);
                    return None;
                }
            }
        }
        Err(e) => {
            println!("Failed to run forge command: {:?}", e);
            return None;
        }
    }
}

#[test]
fn test_function_completion_with_snippets() {
    let ast_data = get_test_ast_data().unwrap();
    let completions = get_scoped_completions(&ast_data, "", Position::new(0, 0));

    // Find function completions
    let function_completions: Vec<_> = completions.iter()
        .filter(|c| c.kind == Some(CompletionItemKind::FUNCTION))
        .collect();

    assert!(!function_completions.is_empty(), "Should have function completions");

    // Check that function completions have snippets
    let functions_with_snippets: Vec<_> = function_completions.iter()
        .filter(|c| c.insert_text.is_some() && c.insert_text_format == Some(InsertTextFormat::SNIPPET))
        .collect();

    assert!(!functions_with_snippets.is_empty(), "Should have function completions with snippets");

    // Verify snippet format for a function with parameters
    let function_with_params = functions_with_snippets.iter()
        .find(|c| {
            if let Some(insert_text) = &c.insert_text {
                insert_text.contains("${1:")
            } else {
                false
            }
        });

    if let Some(completion) = function_with_params {
        if let Some(insert_text) = &completion.insert_text {
            // Should be in format: functionName(${1:param1}, ${2:param2})
            assert!(insert_text.starts_with(&completion.label), "Snippet should start with function name");
            assert!(insert_text.contains("(") && insert_text.contains(")"), "Snippet should have parentheses");
            assert!(insert_text.contains("${1:"), "Snippet should have first parameter placeholder");
        }
    }

    // Check for functions with no parameters (should have empty parentheses)
    let function_no_params = functions_with_snippets.iter()
        .find(|c| {
            if let Some(insert_text) = &c.insert_text {
                insert_text.ends_with("()")
            } else {
                false
            }
        });

    assert!(function_no_params.is_some(), "Should have at least one function with no parameters");
}

#[test]
fn test_function_snippet_edge_cases() {
    // Test the create_function_snippet function directly with various edge cases
    use solidity_language_server::completion::handler::create_function_snippet;

    // Function with no parameters
    let snippet = create_function_snippet("myFunction", "myFunction()");
    assert_eq!(snippet, "myFunction()");

    // Function with simple parameters
    let snippet = create_function_snippet("add", "add(uint256 a, uint256 b)");
    assert_eq!(snippet, "add(${1:a}, ${2:b})");

    // Function with unnamed parameters (should use generic names)
    let snippet = create_function_snippet("func", "func(uint256, address)");
    assert_eq!(snippet, "func(${1:param1}, ${2:param2})");

    // Function with complex types
    let snippet = create_function_snippet("complex", "(mapping(address => uint256) storage orders, uint256[] memory amounts)");
    assert_eq!(snippet, "complex(${1:orders}, ${2:amounts})");

    // Function with return types (should ignore returns)
    let snippet = create_function_snippet("withReturns", "withReturns(uint256 x) returns (uint256)");
    assert_eq!(snippet, "withReturns(${1:x})");

    // Malformed signature (should default to no parameters)
    let snippet = create_function_snippet("malformed", "malformed");
    assert_eq!(snippet, "malformed()");
}

#[test]
fn test_struct_member_completion_for_order() {
    let ast_data = match get_test_ast_data() {
        Some(data) => {
            println!("AST data obtained successfully");
            data
        }
        None => {
            println!("Failed to get AST data");
            return;
        }
    };

    let _symbols = extract_symbols(&ast_data);
    println!("Extracted {} symbols", _symbols.len());

    // Test dot completion for "order."
    let text = "order.";
    let position = Position {
        line: 67,
        character: 10,
    }; // After "order."

    let result = member_access::get_dot_completions(text, &ast_data, position);

    if let Some((ref comps, ref query)) = result {
        println!(
            "Got {} completions for query '{}': {:?}",
            comps.len(),
            query,
            comps.iter().map(|c| &c.label).collect::<Vec<_>>()
        );
    } else {
        println!("No completions returned");
    }

    assert!(
        result.is_some(),
        "Should return completions for 'order.'"
    );

    let (completions, _query) = result.unwrap();
    let completion_labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    assert!(
        completion_labels.contains(&"buyer"),
        "Completions should include buyer"
    );
    assert!(
        completion_labels.contains(&"nonce"),
        "Completions should include nonce"
    );
    assert!(
        completion_labels.contains(&"amount"),
        "Completions should include amount"
    );
    assert!(
        completion_labels.contains(&"date"),
        "Completions should include date"
    );
}

#[test]
fn test_mapping_access_struct_member_completion() {
    let ast_data = match get_test_ast_data() {
        Some(data) => {
            println!("AST data obtained successfully");
            data
        }
        None => {
            println!("Failed to get AST data");
            return;
        }
    };

    let _symbols = extract_symbols(&ast_data);
    println!("Extracted {} symbols", _symbols.len());

    // Test dot completion for "orders[orderId]."
    let text = "orders[orderId].";
    let position = Position {
        line: 59,
        character: 16,
    }; // After "orders[orderId]."

    let result = member_access::get_dot_completions(text, &ast_data, position);

    if let Some((ref comps, ref query)) = result {
        println!(
            "Got {} completions for query '{}': {:?}",
            comps.len(),
            query,
            comps.iter().map(|c| &c.label).collect::<Vec<_>>()
        );
    } else {
        println!("No completions returned");
    }

    assert!(
        result.is_some(),
        "Should return completions for 'orders[orderId].'"
    );

    let (completions, _query) = result.unwrap();
    let completion_labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    assert!(
        completion_labels.contains(&"buyer"),
        "Completions should include buyer"
    );
    assert!(
        completion_labels.contains(&"nonce"),
        "Completions should include nonce"
    );
    assert!(
        completion_labels.contains(&"amount"),
        "Completions should include amount"
    );
    assert!(
        completion_labels.contains(&"date"),
        "Completions should include date"
    );
}

#[test]
fn test_library_function_completion_on_uint256() {
    let ast_data = match get_test_ast_data() {
        Some(data) => {
            println!("AST data obtained successfully");
            println!("AST keys: {:?}", data.as_object().map(|o| o.keys().collect::<Vec<_>>()));
            data
        }
        None => {
            println!("Failed to get AST data");
            return;
        }
    };

    let _symbols = extract_symbols(&ast_data);
    println!("Extracted {} symbols", _symbols.len());

    // Test dot completion for "orders[orderId].amount."
    let text = "orders[orderId].amount.";
    let position = Position {
        line: 60,
        character: 32,
    }; // After "orders[orderId].amount."

    let result = member_access::get_dot_completions(text, &ast_data, position);

    if let Some((ref comps, ref query)) = result {
        println!(
            "Got {} completions for query '{}': {:?}",
            comps.len(),
            query,
            comps.iter().map(|c| &c.label).collect::<Vec<_>>()
        );
    } else {
        println!("No completions returned");
    }

    assert!(
        result.is_some(),
        "Should return completions for 'orders[orderId].amount.'"
    );

    let (completions, _query) = result.unwrap();
    let completion_labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    assert!(
        completion_labels.contains(&"addTax"),
        "Completions should include addTax"
    );
    assert!(
        completion_labels.contains(&"getRefund"),
        "Completions should include getRefund"
    );
}

#[test]
fn test_function_completion_with_signature() {
    let ast_data = get_test_ast_data().unwrap();
    let completions = get_scoped_completions(&ast_data, "", Position::new(0, 0));

    // Find function completions
    let function_completions: Vec<_> = completions.iter()
        .filter(|c| c.kind == Some(CompletionItemKind::FUNCTION))
        .collect();

    assert!(!function_completions.is_empty(), "Should have function completions");

    // Check that at least one function has detailed signature information
    let functions_with_signatures: Vec<_> = function_completions.iter()
        .filter(|c| {
            if let Some(detail) = &c.detail {
                detail.contains("(") && detail.contains(")")
            } else {
                false
            }
        })
        .collect();

    // We should have at least some functions with signatures (like addTax, getRefund, etc.)
    assert!(!functions_with_signatures.is_empty(), "Should have function completions with signature details");

    // Verify that at least one function has detailed signature information
    let detailed_function = function_completions.iter()
        .find(|c| {
            if let Some(detail) = &c.detail {
                detail.contains("(") && detail.contains(")") && detail.len() > c.label.len() + 2
            } else {
                false
            }
        });

    assert!(detailed_function.is_some(), "Should have at least one function with detailed signature");

    if let Some(completion) = detailed_function {
        if let Some(detail) = &completion.detail {
            assert!(detail.contains(&completion.label), "Detail should contain function name");
            assert!(detail.contains("("), "Detail should contain opening parenthesis");
            assert!(detail.contains(")"), "Detail should contain closing parenthesis");
            // Should show parameter types (at least one type should be present)
            assert!(detail.contains("uint256") || detail.contains("address") || detail.contains("bytes32") ||
                   detail.contains("bool") || detail.contains("string"),
                   "Detail should show parameter types");
        }
    }
}