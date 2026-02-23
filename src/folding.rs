use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};
use tree_sitter::{Node, Parser};

/// Extract folding ranges from Solidity source using tree-sitter.
///
/// Returns ranges for contracts, functions, structs, enums, block statements,
/// multi-line comments, consecutive single-line comments, and import groups.
pub fn folding_ranges(source: &str) -> Vec<FoldingRange> {
    let tree = match parse(source) {
        Some(t) => t,
        None => return vec![],
    };
    let mut ranges = Vec::new();
    collect_folding_ranges(tree.root_node(), source, &mut ranges);
    collect_comment_folds(tree.root_node(), source, &mut ranges);
    collect_import_folds(tree.root_node(), &mut ranges);
    ranges
}

/// Recursively walk the tree and emit folding ranges for multi-line nodes.
fn collect_folding_ranges(node: Node, source: &str, out: &mut Vec<FoldingRange>) {
    match node.kind() {
        // Top-level declarations with bodies — emit a fold for the body then
        // recurse into the body's children (functions, state vars, etc.).
        "contract_declaration" | "interface_declaration" | "library_declaration" => {
            if let Some(body) = find_child(node, "contract_body") {
                push_brace_fold(body, None, out);
                walk_children(body, source, out);
            }
            return;
        }
        "struct_declaration" => {
            if let Some(body) = find_child(node, "struct_body") {
                push_brace_fold(body, None, out);
            }
            return;
        }
        "enum_declaration" => {
            if let Some(body) = find_child(node, "enum_body") {
                push_brace_fold(body, None, out);
            }
            return;
        }

        // Functions, constructors, modifiers, fallback/receive — emit a fold
        // for the function body then recurse into it for nested blocks.
        "function_definition"
        | "constructor_definition"
        | "modifier_definition"
        | "fallback_receive_definition" => {
            if let Some(body) = find_child(node, "function_body") {
                push_brace_fold(body, None, out);
                walk_children(body, source, out);
            }
            return;
        }

        // Block statements inside function bodies
        "block_statement" | "unchecked_block" => {
            push_brace_fold(node, None, out);
        }

        // Control-flow with braces — recurse into children which will emit
        // folds for their block_statement bodies.
        "if_statement" | "for_statement" | "while_statement" | "do_while_statement"
        | "try_statement" => {}

        // Assembly blocks
        "assembly_statement" => {
            if let Some(body) = find_child(node, "yul_block") {
                push_brace_fold(body, None, out);
            }
        }

        // Event/error with multi-line parameter lists
        "event_definition" | "error_declaration" => {
            push_multiline_fold(node, None, out);
        }

        _ => {}
    }

    walk_children(node, source, out);
}

fn walk_children(node: Node, source: &str, out: &mut Vec<FoldingRange>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.is_named() {
            collect_folding_ranges(child, source, out);
        }
    }
}

/// Collect folding ranges for comments.
///
/// - Multi-line block comments (`/* ... */`) get a Comment fold.
/// - Consecutive single-line comments (`// ...`) on adjacent lines are grouped
///   into a single Comment fold.
fn collect_comment_folds(root: Node, source: &str, out: &mut Vec<FoldingRange>) {
    let mut cursor = root.walk();
    let children: Vec<Node> = root
        .children(&mut cursor)
        .filter(|c| c.kind() == "comment")
        .collect();

    let mut i = 0;
    while i < children.len() {
        let node = children[i];
        let text = &source[node.byte_range()];
        let start_line = node.start_position().row as u32;
        let end_line = node.end_position().row as u32;

        if text.starts_with("/*") {
            // Multi-line block comment
            if end_line > start_line {
                out.push(FoldingRange {
                    start_line,
                    start_character: Some(node.start_position().column as u32),
                    end_line,
                    end_character: Some(node.end_position().column as u32),
                    kind: Some(FoldingRangeKind::Comment),
                    collapsed_text: None,
                });
            }
            i += 1;
        } else if text.starts_with("//") {
            // Group consecutive single-line comments
            let group_start = start_line;
            let mut group_end = end_line;
            let mut j = i + 1;
            while j < children.len() {
                let next = children[j];
                let next_text = &source[next.byte_range()];
                let next_start = next.start_position().row as u32;
                if next_text.starts_with("//") && next_start == group_end + 1 {
                    group_end = next.end_position().row as u32;
                    j += 1;
                } else {
                    break;
                }
            }
            if group_end > group_start {
                out.push(FoldingRange {
                    start_line: group_start,
                    start_character: Some(node.start_position().column as u32),
                    end_line: group_end,
                    end_character: None,
                    kind: Some(FoldingRangeKind::Comment),
                    collapsed_text: None,
                });
            }
            i = j;
        } else {
            i += 1;
        }
    }

    // Also recurse into contract/struct/enum bodies for inner comments
    let mut cursor2 = root.walk();
    for child in root.children(&mut cursor2) {
        if child.is_named()
            && has_body(child)
            && let Some(body) = find_body(child)
        {
            collect_comment_folds(body, source, out);
        }
    }
}

/// Group consecutive `import_directive` nodes into a single Imports fold.
fn collect_import_folds(root: Node, out: &mut Vec<FoldingRange>) {
    let mut cursor = root.walk();
    let children: Vec<Node> = root
        .children(&mut cursor)
        .filter(|c| c.is_named())
        .collect();

    let mut i = 0;
    while i < children.len() {
        if children[i].kind() == "import_directive" {
            let start_line = children[i].start_position().row as u32;
            let start_char = children[i].start_position().column as u32;
            let mut end_line = children[i].end_position().row as u32;

            // Also fold individual multi-line imports (e.g. `import { A, B, C } from "...";`)
            if end_line > start_line {
                out.push(FoldingRange {
                    start_line,
                    start_character: Some(start_char),
                    end_line,
                    end_character: Some(children[i].end_position().column as u32),
                    kind: Some(FoldingRangeKind::Imports),
                    collapsed_text: None,
                });
            }

            // Group consecutive imports
            let mut j = i + 1;
            while j < children.len() && children[j].kind() == "import_directive" {
                end_line = children[j].end_position().row as u32;
                j += 1;
            }
            if j > i + 1 {
                // Multiple consecutive imports — create a group fold
                out.push(FoldingRange {
                    start_line,
                    start_character: Some(start_char),
                    end_line,
                    end_character: None,
                    kind: Some(FoldingRangeKind::Imports),
                    collapsed_text: None,
                });
            }
            i = j;
        } else {
            i += 1;
        }
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn parse(source: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_solidity::LANGUAGE.into())
        .expect("failed to load Solidity grammar");
    parser.parse(source, None)
}

/// Push a fold for a brace-delimited node (e.g. `{ ... }`).
/// Only emits a fold when the node spans multiple lines.
fn push_brace_fold(node: Node, kind: Option<FoldingRangeKind>, out: &mut Vec<FoldingRange>) {
    let start_line = node.start_position().row as u32;
    let end_line = node.end_position().row as u32;
    if end_line > start_line {
        out.push(FoldingRange {
            start_line,
            start_character: Some(node.start_position().column as u32),
            end_line,
            end_character: Some(node.end_position().column as u32),
            kind,
            collapsed_text: None,
        });
    }
}

/// Push a fold for any multi-line node (events, errors with long param lists).
fn push_multiline_fold(node: Node, kind: Option<FoldingRangeKind>, out: &mut Vec<FoldingRange>) {
    let start_line = node.start_position().row as u32;
    let end_line = node.end_position().row as u32;
    if end_line > start_line {
        out.push(FoldingRange {
            start_line,
            start_character: Some(node.start_position().column as u32),
            end_line,
            end_character: Some(node.end_position().column as u32),
            kind,
            collapsed_text: None,
        });
    }
}

fn find_child<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor).find(|c| c.kind() == kind)
}

fn has_body(node: Node) -> bool {
    matches!(
        node.kind(),
        "contract_declaration"
            | "interface_declaration"
            | "library_declaration"
            | "struct_declaration"
            | "enum_declaration"
    )
}

fn find_body(node: Node) -> Option<Node> {
    match node.kind() {
        "contract_declaration" | "interface_declaration" | "library_declaration" => {
            find_child(node, "contract_body")
        }
        "struct_declaration" => find_child(node, "struct_body"),
        "enum_declaration" => find_child(node, "enum_body"),
        _ => None,
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        assert!(folding_ranges("").is_empty());
    }

    #[test]
    fn test_single_line_contract() {
        // No folds for single-line constructs
        let source = "contract Foo {}";
        let ranges = folding_ranges(source);
        assert!(ranges.is_empty(), "single-line contract should not fold");
    }

    #[test]
    fn test_contract_body_fold() {
        let source = r#"
contract Counter {
    uint256 public count;
    function increment() public {
        count += 1;
    }
}
"#;
        let ranges = folding_ranges(source);
        // Should have folds for: contract body, function body
        let contract_folds: Vec<_> = ranges.iter().filter(|r| r.kind.is_none()).collect();
        assert!(
            contract_folds.len() >= 2,
            "expected at least 2 region folds (contract body + function body), got {}",
            contract_folds.len()
        );
    }

    #[test]
    fn test_function_body_fold() {
        let source = r#"
contract Foo {
    function bar() public {
        uint256 x = 1;
        uint256 y = 2;
    }
}
"#;
        let ranges = folding_ranges(source);
        // The function body `{ ... }` starts on line 2 (same line as the
        // function signature) and ends on line 5 (`}`).
        let func_fold = ranges
            .iter()
            .find(|r| r.start_line == 2 && r.end_line == 5 && r.kind.is_none());
        assert!(
            func_fold.is_some(),
            "expected fold for function body, got ranges: {:?}",
            ranges
                .iter()
                .map(|r| (r.start_line, r.end_line, &r.kind))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_struct_fold() {
        let source = r#"
struct Info {
    string name;
    uint256 value;
    address owner;
}
"#;
        let ranges = folding_ranges(source);
        let struct_fold = ranges.iter().find(|r| r.start_line == 1);
        assert!(struct_fold.is_some(), "expected fold for struct body");
    }

    #[test]
    fn test_enum_fold() {
        let source = r#"
enum Status {
    Active,
    Paused,
    Stopped
}
"#;
        let ranges = folding_ranges(source);
        let enum_fold = ranges.iter().find(|r| r.start_line == 1);
        assert!(enum_fold.is_some(), "expected fold for enum body");
    }

    #[test]
    fn test_block_comment_fold() {
        let source = r#"
/*
 * This is a multi-line
 * block comment
 */
contract Foo {}
"#;
        let ranges = folding_ranges(source);
        let comment_folds: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Comment))
            .collect();
        assert!(
            !comment_folds.is_empty(),
            "expected a comment fold for block comment"
        );
        assert_eq!(comment_folds[0].start_line, 1);
        assert_eq!(comment_folds[0].end_line, 4);
    }

    #[test]
    fn test_consecutive_line_comments_fold() {
        let source = r#"// line 1
// line 2
// line 3
contract Foo {}
"#;
        let ranges = folding_ranges(source);
        let comment_folds: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Comment))
            .collect();
        assert!(
            !comment_folds.is_empty(),
            "expected a fold for consecutive line comments"
        );
        assert_eq!(comment_folds[0].start_line, 0);
        assert_eq!(comment_folds[0].end_line, 2);
    }

    #[test]
    fn test_single_line_comment_no_fold() {
        let source = r#"
// just one line
contract Foo {}
"#;
        let ranges = folding_ranges(source);
        let comment_folds: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Comment))
            .collect();
        assert!(
            comment_folds.is_empty(),
            "single line comment should not produce a fold"
        );
    }

    #[test]
    fn test_import_group_fold() {
        let source = r#"
import "./A.sol";
import "./B.sol";
import "./C.sol";

contract Foo {}
"#;
        let ranges = folding_ranges(source);
        let import_folds: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Imports))
            .collect();
        assert!(
            !import_folds.is_empty(),
            "expected an import group fold for consecutive imports"
        );
        // The group fold should span from first to last import
        let group = import_folds
            .iter()
            .find(|r| r.start_line == 1 && r.end_line == 3);
        assert!(group.is_some(), "expected group fold spanning lines 1-3");
    }

    #[test]
    fn test_multiline_import_fold() {
        let source = r#"
import {
    Foo,
    Bar,
    Baz
} from "./Lib.sol";
"#;
        let ranges = folding_ranges(source);
        let import_folds: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Imports))
            .collect();
        assert!(
            !import_folds.is_empty(),
            "expected fold for multi-line import"
        );
    }

    #[test]
    fn test_counter_sol() {
        let source = std::fs::read_to_string("example/Counter.sol").unwrap();
        let ranges = folding_ranges(&source);

        // Should have several folds: contract body, constructor, functions, etc.
        assert!(
            ranges.len() >= 5,
            "Counter.sol should have at least 5 folding ranges, got {}",
            ranges.len()
        );

        // Contract body fold
        let contract_fold = ranges.iter().find(|r| r.start_line == 3);
        assert!(
            contract_fold.is_some(),
            "expected fold starting at contract body (line 3)"
        );
    }

    #[test]
    fn test_interface_fold() {
        let source = r#"
interface IToken {
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}
"#;
        let ranges = folding_ranges(source);
        let interface_fold = ranges.iter().find(|r| r.start_line == 1);
        assert!(interface_fold.is_some(), "expected fold for interface body");
    }

    #[test]
    fn test_library_fold() {
        let source = r#"
library SafeMath {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }
}
"#;
        let ranges = folding_ranges(source);
        assert!(
            ranges.len() >= 2,
            "library should produce at least 2 folds (body + function)"
        );
    }

    #[test]
    fn test_nested_blocks_fold() {
        let source = r#"
contract Foo {
    function bar() public {
        if (true) {
            uint256 x = 1;
        }
        for (uint256 i = 0; i < 10; i++) {
            uint256 y = i;
        }
    }
}
"#;
        let ranges = folding_ranges(source);
        // Should have folds for: contract body, function body, if block, for block
        let region_folds: Vec<_> = ranges.iter().filter(|r| r.kind.is_none()).collect();
        assert!(
            region_folds.len() >= 4,
            "expected at least 4 folds for nested blocks, got {}",
            region_folds.len()
        );
    }

    #[test]
    fn test_modifier_fold() {
        let source = r#"
contract Foo {
    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }
}
"#;
        let ranges = folding_ranges(source);
        // Should fold the modifier body
        let modifier_fold = ranges.iter().find(|r| r.start_line == 2);
        assert!(modifier_fold.is_some(), "expected fold for modifier body");
    }

    #[test]
    fn test_constructor_fold() {
        let source = r#"
contract Foo {
    constructor() {
        owner = msg.sender;
    }
}
"#;
        let ranges = folding_ranges(source);
        let ctor_fold = ranges.iter().find(|r| r.start_line == 2);
        assert!(ctor_fold.is_some(), "expected fold for constructor body");
    }

    #[test]
    fn test_inner_block_comment_fold() {
        let source = r#"
contract Foo {
    /*
     * This is a comment
     * inside a contract
     */
    function bar() public {}
}
"#;
        let ranges = folding_ranges(source);
        let comment_folds: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Comment))
            .collect();
        assert!(
            !comment_folds.is_empty(),
            "expected comment fold inside contract body"
        );
    }
}
