use tower_lsp::lsp_types::{Position, Range, SelectionRange};
use tree_sitter::{Node, Parser, Point};

/// Compute selection ranges for each requested position.
///
/// For every position, walks up the tree-sitter node ancestry from the
/// deepest (leaf) node to the root, producing a linked list of
/// `SelectionRange` values that editors use for expand/shrink selection.
pub fn selection_ranges(source: &str, positions: &[Position]) -> Vec<SelectionRange> {
    let tree = match parse(source) {
        Some(t) => t,
        None => return positions.iter().map(|_| empty_selection_range()).collect(),
    };

    let root = tree.root_node();

    positions
        .iter()
        .map(|pos| build_selection_range(root, source, *pos))
        .collect()
}

/// Build the nested `SelectionRange` chain for a single cursor position.
fn build_selection_range(root: Node, source: &str, position: Position) -> SelectionRange {
    let point = Point {
        row: position.line as usize,
        column: position.character as usize,
    };

    // Find the deepest node at the cursor position.
    let leaf = match root.descendant_for_point_range(point, point) {
        Some(n) => n,
        None => return empty_selection_range(),
    };

    // Walk up the ancestry, collecting each node with a distinct range.
    let mut ancestors = Vec::new();
    let mut current = leaf;
    let mut last_range: Option<Range> = None;

    loop {
        let range = node_range(current, source);
        // Only push if the range differs from the previous one — avoids
        // redundant wrapper nodes that span the exact same region.
        if last_range != Some(range) {
            ancestors.push(range);
            last_range = Some(range);
        }
        match current.parent() {
            Some(p) => current = p,
            None => break,
        }
    }

    // Build the linked list from outermost (last) to innermost (first).
    let mut result: Option<SelectionRange> = None;
    for range in ancestors.into_iter().rev() {
        result = Some(SelectionRange {
            range,
            parent: result.map(Box::new),
        });
    }

    result.unwrap_or_else(empty_selection_range)
}

/// Convert a tree-sitter node to an LSP `Range`, accounting for UTF-16
/// column offsets.
fn node_range(node: Node, source: &str) -> Range {
    let start = node.start_position();
    let end = node.end_position();
    Range {
        start: Position {
            line: start.row as u32,
            character: utf16_col(source, start.row, start.column),
        },
        end: Position {
            line: end.row as u32,
            character: utf16_col(source, end.row, end.column),
        },
    }
}

/// Convert a byte-column offset to a UTF-16 code-unit offset for the given
/// line.  Falls back to the byte column if the line is not found (ASCII).
fn utf16_col(source: &str, row: usize, byte_col: usize) -> u32 {
    let line_start = source
        .split('\n')
        .take(row)
        .map(|l| l.len() + 1)
        .sum::<usize>();
    let slice = &source[line_start..line_start + byte_col.min(source.len() - line_start)];
    slice.encode_utf16().count() as u32
}

/// A zero-width selection range at origin — used as fallback.
fn empty_selection_range() -> SelectionRange {
    SelectionRange {
        range: Range::default(),
        parent: None,
    }
}

fn parse(source: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_solidity::LANGUAGE.into())
        .ok()?;
    parser.parse(source, None)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: return the chain of ranges as a Vec, from innermost to outermost.
    fn chain(sr: &SelectionRange) -> Vec<Range> {
        let mut out = vec![sr.range];
        let mut cur = &sr.parent;
        while let Some(p) = cur {
            out.push(p.range);
            cur = &p.parent;
        }
        out
    }

    /// Helper: number of ranges in the chain.
    fn depth(sr: &SelectionRange) -> usize {
        chain(sr).len()
    }

    // -----------------------------------------------------------------------
    // Basic behaviour
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_source() {
        let results = selection_ranges("", &[Position::new(0, 0)]);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_single_identifier() {
        let src =
            "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\nuint256 constant X = 1;\n";
        // cursor on 'X' at line 3, col 18
        let results = selection_ranges(src, &[Position::new(3, 18)]);
        assert_eq!(results.len(), 1);
        let ranges = chain(&results[0]);
        // Innermost should be the identifier itself
        assert_eq!(ranges[0].start.line, ranges[0].end.line);
        // Outermost should be the root (line 0..last line)
        let last = ranges.last().unwrap();
        assert_eq!(last.start.line, 0);
    }

    #[test]
    fn test_multiple_positions() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    uint256 x;\n    uint256 y;\n}\n";
        let results = selection_ranges(
            src,
            &[
                Position::new(4, 12), // on 'x'
                Position::new(5, 12), // on 'y'
            ],
        );
        assert_eq!(results.len(), 2);
        // Both should have non-trivial depth
        assert!(depth(&results[0]) >= 3);
        assert!(depth(&results[1]) >= 3);
    }

    #[test]
    fn test_ranges_are_nested() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    function bar() public pure returns (uint256) {\n        return 42;\n    }\n}\n";
        // cursor on '42' inside `return 42;`
        let results = selection_ranges(src, &[Position::new(5, 15)]);
        let ranges = chain(&results[0]);
        // Each range must contain the previous one
        for i in 1..ranges.len() {
            let inner = &ranges[i - 1];
            let outer = &ranges[i];
            assert!(
                (outer.start.line < inner.start.line
                    || (outer.start.line == inner.start.line
                        && outer.start.character <= inner.start.character))
                    && (outer.end.line > inner.end.line
                        || (outer.end.line == inner.end.line
                            && outer.end.character >= inner.end.character)),
                "range[{}] {:?} must contain range[{}] {:?}",
                i,
                outer,
                i - 1,
                inner
            );
        }
    }

    #[test]
    fn test_no_duplicate_ranges() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    uint256 x;\n}\n";
        let results = selection_ranges(src, &[Position::new(4, 12)]);
        let ranges = chain(&results[0]);
        // No two consecutive ranges should be identical
        for i in 1..ranges.len() {
            assert_ne!(
                ranges[i - 1],
                ranges[i],
                "consecutive ranges should not be identical"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Specific Solidity constructs
    // -----------------------------------------------------------------------

    #[test]
    fn test_function_parameter() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    function add(uint256 a, uint256 b) public pure returns (uint256) {\n        return a + b;\n    }\n}\n";
        // cursor on 'a' parameter at line 4
        let results = selection_ranges(src, &[Position::new(4, 25)]);
        // Should walk from identifier → parameter → parameter list → function → contract body → contract → source_file
        assert!(depth(&results[0]) >= 5, "depth = {}", depth(&results[0]));
    }

    #[test]
    fn test_struct_field() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\nstruct Point {\n    uint256 x;\n    uint256 y;\n}\n";
        // cursor on 'x' at line 4
        let results = selection_ranges(src, &[Position::new(4, 12)]);
        let ranges = chain(&results[0]);
        assert!(depth(&results[0]) >= 3);
        // Outermost is source_file
        let last = ranges.last().unwrap();
        assert_eq!(last.start.line, 0);
    }

    #[test]
    fn test_nested_expression() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    function calc(uint256 a, uint16 b, uint16 c) internal pure returns (uint256) {\n        return a + (a * b / c);\n    }\n}\n";
        // cursor on 'b' inside the parenthesized expression at line 5
        let results = selection_ranges(src, &[Position::new(5, 24)]);
        // Should walk through: b → a * b → a * b / c → (a * b / c) → a + (...) → return stmt → function body → function → contract body → contract → source
        assert!(
            depth(&results[0]) >= 8,
            "nested expression depth = {}",
            depth(&results[0])
        );
    }

    #[test]
    fn test_event_parameter() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    event Transfer(address indexed from, address indexed to, uint256 value);\n}\n";
        // cursor on 'from' at line 4
        let results = selection_ranges(src, &[Position::new(4, 35)]);
        assert!(depth(&results[0]) >= 3);
    }

    #[test]
    fn test_mapping_type() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    mapping(address => uint256) public balances;\n}\n";
        // cursor on 'address' at line 4
        let results = selection_ranges(src, &[Position::new(4, 12)]);
        assert!(depth(&results[0]) >= 3);
    }

    #[test]
    fn test_if_condition() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    function check(uint256 x) public pure returns (bool) {\n        if (x > 0) {\n            return true;\n        }\n        return false;\n    }\n}\n";
        // cursor on 'x' in the condition at line 5
        let results = selection_ranges(src, &[Position::new(5, 12)]);
        // Should include: x → x > 0 → if statement → function body → function → contract body → contract → source
        assert!(depth(&results[0]) >= 6);
    }

    #[test]
    fn test_comment_position() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\n// This is a comment\ncontract Foo {}\n";
        // cursor inside the comment at line 3
        let results = selection_ranges(src, &[Position::new(3, 5)]);
        assert!(depth(&results[0]) >= 1);
    }

    #[test]
    fn test_pragma_position() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {}\n";
        // cursor on 'solidity' in pragma at line 1
        let results = selection_ranges(src, &[Position::new(1, 10)]);
        assert!(depth(&results[0]) >= 2);
    }

    #[test]
    fn test_innermost_is_leaf() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    uint256 public value;\n}\n";
        // cursor on 'value' at line 4
        let results = selection_ranges(src, &[Position::new(4, 19)]);
        let ranges = chain(&results[0]);
        // The innermost range should be tight around 'value' (5 chars)
        let inner = &ranges[0];
        assert_eq!(inner.start.line, inner.end.line);
        assert_eq!(inner.end.character - inner.start.character, 5); // "value"
    }

    #[test]
    fn test_outermost_is_source_file() {
        let src = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Foo {\n    uint256 x;\n}\n";
        let results = selection_ranges(src, &[Position::new(4, 12)]);
        let ranges = chain(&results[0]);
        let last = ranges.last().unwrap();
        // Source file starts at 0,0
        assert_eq!(last.start.line, 0);
        assert_eq!(last.start.character, 0);
    }

    // -----------------------------------------------------------------------
    // Integration: Shop.sol
    // -----------------------------------------------------------------------

    #[test]
    fn test_shop_sol() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("example/Shop.sol");
        let source = std::fs::read_to_string(&path).expect("read Shop.sol");

        // Test multiple positions across the file
        let positions = vec![
            Position::new(42, 15), // inside addTax function body
            Position::new(29, 8),  // struct field 'buyer'
            Position::new(68, 22), // PRICE constant
            Position::new(0, 5),   // comment at top
        ];
        let results = selection_ranges(&source, &positions);
        assert_eq!(results.len(), 4);

        // Every result should have a non-trivial chain
        for (i, sr) in results.iter().enumerate() {
            assert!(
                depth(sr) >= 2,
                "position {} should have depth >= 2, got {}",
                i,
                depth(sr)
            );
        }

        // All chains should have nested (containing) ranges
        for (i, sr) in results.iter().enumerate() {
            let ranges = chain(sr);
            for j in 1..ranges.len() {
                let inner = &ranges[j - 1];
                let outer = &ranges[j];
                assert!(
                    (outer.start.line < inner.start.line
                        || (outer.start.line == inner.start.line
                            && outer.start.character <= inner.start.character))
                        && (outer.end.line > inner.end.line
                            || (outer.end.line == inner.end.line
                                && outer.end.character >= inner.end.character)),
                    "position {}: range[{}] {:?} must contain range[{}] {:?}",
                    i,
                    j,
                    outer,
                    j - 1,
                    inner
                );
            }
        }
    }
}
