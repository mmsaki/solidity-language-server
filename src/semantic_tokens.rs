use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens, SemanticTokensLegend,
};
use tree_sitter::{Node, Parser};

// ── Token type indices ─────────────────────────────────────────────────────

/// Index into [`LEGEND`]`.token_types`.
mod ty {
    pub const NAMESPACE: u32 = 0;
    pub const TYPE: u32 = 1;
    pub const CLASS: u32 = 2;
    pub const ENUM: u32 = 3;
    pub const INTERFACE: u32 = 4;
    pub const STRUCT: u32 = 5;
    pub const PARAMETER: u32 = 6;
    pub const VARIABLE: u32 = 7;
    pub const PROPERTY: u32 = 8;
    pub const ENUM_MEMBER: u32 = 9;
    pub const EVENT: u32 = 10;
    pub const FUNCTION: u32 = 11;
    pub const METHOD: u32 = 12;
    pub const KEYWORD: u32 = 13;
    pub const MODIFIER: u32 = 14;
    pub const COMMENT: u32 = 15;
    pub const STRING: u32 = 16;
    pub const NUMBER: u32 = 17;
    pub const OPERATOR: u32 = 18;
    pub const DECORATOR: u32 = 19;
    pub const TYPE_PARAMETER: u32 = 20;
}

/// Index into [`LEGEND`]`.token_modifiers` (bit flags).
mod modif {
    pub const DECLARATION: u32 = 1 << 0;
    pub const DEFINITION: u32 = 1 << 1;
    pub const READONLY: u32 = 1 << 2;
    // pub const STATIC: u32 = 1 << 3;
    // pub const DEPRECATED: u32 = 1 << 4;
    // pub const ABSTRACT: u32 = 1 << 5;
    pub const DOCUMENTATION: u32 = 1 << 6;
    pub const DEFAULT_LIBRARY: u32 = 1 << 7;
}

/// Build the legend announced in `ServerCapabilities`.
pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::NAMESPACE,      // 0
            SemanticTokenType::TYPE,           // 1
            SemanticTokenType::CLASS,          // 2
            SemanticTokenType::ENUM,           // 3
            SemanticTokenType::INTERFACE,      // 4
            SemanticTokenType::STRUCT,         // 5
            SemanticTokenType::PARAMETER,      // 6
            SemanticTokenType::VARIABLE,       // 7
            SemanticTokenType::PROPERTY,       // 8
            SemanticTokenType::ENUM_MEMBER,    // 9
            SemanticTokenType::EVENT,          // 10
            SemanticTokenType::FUNCTION,       // 11
            SemanticTokenType::METHOD,         // 12
            SemanticTokenType::KEYWORD,        // 13
            SemanticTokenType::MODIFIER,       // 14
            SemanticTokenType::COMMENT,        // 15
            SemanticTokenType::STRING,         // 16
            SemanticTokenType::NUMBER,         // 17
            SemanticTokenType::OPERATOR,       // 18
            SemanticTokenType::DECORATOR,      // 19
            SemanticTokenType::TYPE_PARAMETER, // 20
        ],
        token_modifiers: vec![
            SemanticTokenModifier::DECLARATION,     // bit 0
            SemanticTokenModifier::DEFINITION,      // bit 1
            SemanticTokenModifier::READONLY,        // bit 2
            SemanticTokenModifier::STATIC,          // bit 3
            SemanticTokenModifier::DEPRECATED,      // bit 4
            SemanticTokenModifier::ABSTRACT,        // bit 5
            SemanticTokenModifier::DOCUMENTATION,   // bit 6
            SemanticTokenModifier::DEFAULT_LIBRARY, // bit 7
        ],
    }
}

// ── Raw token before delta encoding ────────────────────────────────────────

#[derive(Debug)]
struct RawToken {
    line: u32,
    col: u32,
    length: u32,
    token_type: u32,
    modifiers: u32,
}

// ── Public entry point ─────────────────────────────────────────────────────

/// Parse `source` with tree-sitter and return semantic tokens.
pub fn semantic_tokens_full(source: &str) -> SemanticTokens {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_solidity::LANGUAGE.into())
        .expect("failed to load Solidity grammar");

    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => {
            return SemanticTokens {
                result_id: None,
                data: vec![],
            };
        }
    };

    let mut tokens: Vec<RawToken> = Vec::new();
    collect_tokens(tree.root_node(), source, &mut tokens);

    // Sort by position (tree-sitter walks depth-first so this is mostly sorted,
    // but we sort to be safe for the delta encoding).
    tokens.sort_by(|a, b| a.line.cmp(&b.line).then(a.col.cmp(&b.col)));

    // Delta-encode
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;
    let data: Vec<SemanticToken> = tokens
        .iter()
        .map(|t| {
            let delta_line = t.line - prev_line;
            let delta_start = if delta_line == 0 {
                t.col - prev_start
            } else {
                t.col
            };
            prev_line = t.line;
            prev_start = t.col;
            SemanticToken {
                delta_line,
                delta_start,
                length: t.length,
                token_type: t.token_type,
                token_modifiers_bitset: t.modifiers,
            }
        })
        .collect();

    SemanticTokens {
        result_id: None,
        data,
    }
}

// ── Tree walker ────────────────────────────────────────────────────────────

fn collect_tokens(node: Node, source: &str, tokens: &mut Vec<RawToken>) {
    // Anonymous nodes are keywords, operators, punctuation
    if !node.is_named() {
        if let Some(tok) = classify_anonymous(node, source) {
            tokens.push(tok);
        }
        return; // anonymous nodes have no named children
    }

    match node.kind() {
        // ── Comments ───────────────────────────────────────────────────
        "comment" => {
            push_multiline(node, source, ty::COMMENT, modif::DOCUMENTATION, tokens);
        }

        // ── Literals ───────────────────────────────────────────────────
        "number_literal" => {
            push_node(node, ty::NUMBER, 0, tokens);
        }
        "string" | "string_literal" | "hex_string_literal" | "unicode_string_literal" => {
            push_multiline(node, source, ty::STRING, 0, tokens);
        }
        "boolean_literal" => {
            push_node(node, ty::KEYWORD, 0, tokens);
        }

        // ── Contract-level declarations ────────────────────────────────
        "contract_declaration" => {
            push_child_identifier(
                node,
                ty::CLASS,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }
        "interface_declaration" => {
            push_child_identifier(
                node,
                ty::INTERFACE,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }
        "library_declaration" => {
            push_child_identifier(
                node,
                ty::NAMESPACE,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }

        // ── Type definitions ───────────────────────────────────────────
        "struct_definition" | "struct_declaration" => {
            push_child_identifier(
                node,
                ty::STRUCT,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }
        "enum_definition" | "enum_declaration" => {
            push_child_identifier(
                node,
                ty::ENUM,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }
        "enum_value" => {
            push_node(node, ty::ENUM_MEMBER, modif::READONLY, tokens);
        }
        "type_alias" => {
            push_child_identifier(
                node,
                ty::TYPE,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }

        // ── Events & Errors ────────────────────────────────────────────
        "event_definition" => {
            push_child_identifier(
                node,
                ty::EVENT,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }
        "error_declaration" => {
            push_child_identifier(
                node,
                ty::TYPE,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }

        // ── Functions ──────────────────────────────────────────────────
        "function_definition" => {
            push_child_identifier(
                node,
                ty::FUNCTION,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }
        "constructor_definition" => {
            walk_children(node, source, tokens);
        }
        "fallback_receive_definition" => {
            walk_children(node, source, tokens);
        }

        // ── Modifiers ──────────────────────────────────────────────────
        "modifier_definition" => {
            push_child_identifier(
                node,
                ty::DECORATOR,
                modif::DECLARATION | modif::DEFINITION,
                tokens,
            );
            walk_children(node, source, tokens);
        }
        "modifier_invocation" => {
            push_child_identifier(node, ty::DECORATOR, 0, tokens);
            walk_children(node, source, tokens);
        }

        // ── State variables ────────────────────────────────────────────
        "state_variable_declaration" => {
            let mods = state_var_modifiers(node, source);
            push_child_identifier(node, ty::PROPERTY, modif::DECLARATION | mods, tokens);
            walk_children(node, source, tokens);
        }

        // ── Parameters ──────────────────────────────────────────────────
        "parameter" | "event_parameter" | "error_parameter" => {
            push_child_identifier(node, ty::PARAMETER, modif::DECLARATION, tokens);
            walk_children(node, source, tokens);
        }

        // ── Local variables ────────────────────────────────────────────
        "variable_declaration" => {
            push_child_identifier(node, ty::VARIABLE, modif::DECLARATION, tokens);
            walk_children(node, source, tokens);
        }
        "variable_declaration_statement" => {
            walk_children(node, source, tokens);
        }
        "variable_declaration_tuple" => {
            walk_children(node, source, tokens);
        }

        // ── Type names ─────────────────────────────────────────────────
        "type_name" | "primitive_type" => {
            // Primitive types like uint256, address, bool, bytes
            if node.child_count() == 0 {
                push_node(node, ty::TYPE, modif::DEFAULT_LIBRARY, tokens);
            } else {
                walk_children(node, source, tokens);
            }
        }
        "user_defined_type" => {
            // Contract/struct/enum/error type references
            push_node(node, ty::TYPE, 0, tokens);
        }
        "mapping" => {
            walk_children(node, source, tokens);
        }

        // ── Expressions ────────────────────────────────────────────────
        "identifier" => {
            // Bare identifier — classify by parent context
            let tt = classify_identifier(node);
            push_node(node, tt, 0, tokens);
        }
        "member_expression" => {
            // object.member — walk object, then classify member
            let mut cursor = node.walk();
            let children: Vec<Node> = node.children(&mut cursor).collect();
            // Walk all children except the last identifier (the member name)
            for (i, child) in children.iter().enumerate() {
                if i == children.len() - 1 && child.kind() == "identifier" {
                    // The member name — could be property or method
                    let tt = if is_call_target(node) {
                        ty::METHOD
                    } else {
                        ty::PROPERTY
                    };
                    push_node(*child, tt, 0, tokens);
                } else {
                    collect_tokens(*child, source, tokens);
                }
            }
        }
        "call_expression" => {
            walk_children(node, source, tokens);
        }

        // ── Imports ────────────────────────────────────────────────────
        "import_directive" => {
            walk_children(node, source, tokens);
        }
        "source_import" | "import_clause" => {
            walk_children(node, source, tokens);
        }

        // ── Pragmas ────────────────────────────────────────────────────
        "pragma_directive" | "solidity_pragma_token" => {
            walk_children(node, source, tokens);
        }
        "solidity_version" => {
            push_node(node, ty::NUMBER, 0, tokens);
        }

        // ── Inheritance ────────────────────────────────────────────────
        "inheritance_specifier" => {
            push_child_identifier(node, ty::INTERFACE, 0, tokens);
            walk_children(node, source, tokens);
        }

        // ── Using directive ────────────────────────────────────────────
        "using_directive" => {
            walk_children(node, source, tokens);
        }

        // ── Visibility / state mutability (named leaf nodes) ───────────
        "visibility" | "state_mutability" => {
            push_node(node, ty::KEYWORD, 0, tokens);
        }

        // ── Everything else: just walk children ────────────────────────
        _ => {
            walk_children(node, source, tokens);
        }
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Walk all children of `node`.
fn walk_children(node: Node, source: &str, tokens: &mut Vec<RawToken>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_tokens(child, source, tokens);
    }
}

/// Push a single-line token for an entire node.
fn push_node(node: Node, token_type: u32, modifiers: u32, tokens: &mut Vec<RawToken>) {
    let start = node.start_position();
    let len = node.end_byte() - node.start_byte();
    if len == 0 {
        return;
    }
    // For multi-line nodes (like comments), we need push_multiline instead.
    // This function is for single-line tokens only.
    tokens.push(RawToken {
        line: start.row as u32,
        col: start.column as u32,
        length: len as u32,
        token_type,
        modifiers,
    });
}

/// Push tokens for a node that may span multiple lines (e.g. block comments).
fn push_multiline(
    node: Node,
    source: &str,
    token_type: u32,
    modifiers: u32,
    tokens: &mut Vec<RawToken>,
) {
    let text = &source[node.byte_range()];
    let start_line = node.start_position().row as u32;
    let start_col = node.start_position().column as u32;

    for (i, line) in text.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let col = if i == 0 { start_col } else { 0 };
        // Find the actual start of content (skip leading whitespace for continuation lines)
        let trimmed_start = if i > 0 {
            line.len() - line.trim_start().len()
        } else {
            0
        };
        let trimmed_len = line.trim_end().len() - trimmed_start;
        if trimmed_len == 0 {
            continue;
        }
        tokens.push(RawToken {
            line: start_line + i as u32,
            col: col + trimmed_start as u32,
            length: trimmed_len as u32,
            token_type,
            modifiers,
        });
    }
}

/// Find the first named `identifier` child and push a token for it.
fn push_child_identifier(node: Node, token_type: u32, modifiers: u32, tokens: &mut Vec<RawToken>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" && child.is_named() {
            push_node(child, token_type, modifiers, tokens);
            return;
        }
    }
}

/// Classify an anonymous (keyword/operator/punctuation) node.
fn classify_anonymous(node: Node, source: &str) -> Option<RawToken> {
    let text = &source[node.byte_range()];
    let start = node.start_position();

    let token_type = match text {
        // Keywords
        "pragma" | "solidity" | "import" | "from" | "as" | "contract" | "interface" | "library"
        | "is" | "using" | "for" | "struct" | "enum" | "event" | "error" | "function"
        | "modifier" | "constructor" | "fallback" | "receive" | "mapping" | "if" | "else"
        | "while" | "do" | "break" | "continue" | "return" | "returns" | "emit" | "revert"
        | "try" | "catch" | "throw" | "new" | "delete" | "assembly" | "type" | "let" | "switch"
        | "case" | "default" | "leave" | "public" | "private" | "internal" | "external"
        | "pure" | "view" | "payable" | "constant" | "immutable" | "override" | "virtual"
        | "abstract" | "memory" | "storage" | "calldata" | "indexed" | "anonymous"
        | "unchecked" => ty::KEYWORD,

        // Primitive type keywords
        "address" | "bool" | "string" | "bytes" | "int" | "uint" | "fixed" | "ufixed" => ty::TYPE,

        // Sized integer / bytes types: uint256, int128, bytes32, etc.
        _ if (text.starts_with("uint") || text.starts_with("int"))
            && text[3..]
                .chars()
                .skip(if text.starts_with("uint") { 1 } else { 0 })
                .all(|c| c.is_ascii_digit())
            && text.len() > 3 =>
        {
            ty::TYPE
        }
        _ if text.starts_with("bytes")
            && text.len() > 5
            && text[5..].chars().all(|c| c.is_ascii_digit()) =>
        {
            ty::TYPE
        }

        // Operators
        "+" | "-" | "*" | "/" | "%" | "**" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&"
        | "||" | "!" | "&" | "|" | "^" | "~" | "<<" | ">>" | "=" | "+=" | "-=" | "*=" | "/="
        | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | "=>" | "?" | ":" => ty::OPERATOR,

        // Everything else (punctuation like {, }, (, ), [, ], ;, ., ,) — skip
        _ => return None,
    };

    Some(RawToken {
        line: start.row as u32,
        col: start.column as u32,
        length: (node.end_byte() - node.start_byte()) as u32,
        token_type,
        modifiers: 0,
    })
}

/// Classify an `identifier` node based on its parent context.
fn classify_identifier(node: Node) -> u32 {
    match node.parent().map(|p| p.kind()) {
        Some("call_expression") => ty::FUNCTION,
        Some("emit_statement") => ty::EVENT,
        Some("inheritance_specifier") => ty::INTERFACE,
        Some("using_directive") => ty::NAMESPACE,
        Some("user_defined_type") => ty::TYPE,
        Some("import_directive") | Some("import_clause") | Some("source_import") => ty::NAMESPACE,
        Some("modifier_invocation") => ty::DECORATOR,
        Some("enum_value") => ty::ENUM_MEMBER,
        _ => ty::VARIABLE,
    }
}

/// Check whether a `member_expression` node is the callee of a `call_expression`.
fn is_call_target(node: Node) -> bool {
    node.parent()
        .map(|p| p.kind() == "call_expression")
        .unwrap_or(false)
}

/// Determine modifiers for a state variable declaration.
fn state_var_modifiers(node: Node, source: &str) -> u32 {
    let mut mods = 0u32;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "constant" | "immutable" if !child.is_named() => {
                mods |= modif::READONLY;
            }
            _ => {
                let text = &source[child.byte_range()];
                if text == "constant" || text == "immutable" {
                    mods |= modif::READONLY;
                }
            }
        }
    }
    mods
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let result = semantic_tokens_full("");
        assert!(result.data.is_empty());
    }

    #[test]
    fn test_simple_contract() {
        let source = "contract Foo {}";
        let result = semantic_tokens_full(source);
        assert!(!result.data.is_empty());

        // Should have at least: "contract" keyword, "Foo" class identifier
        let types: Vec<u32> = result.data.iter().map(|t| t.token_type).collect();
        assert!(types.contains(&ty::KEYWORD), "missing keyword token");
        assert!(types.contains(&ty::CLASS), "missing class token");
    }

    #[test]
    fn test_function_definition() {
        let source = r#"
contract Foo {
    function bar(uint256 x) public pure returns (uint256) {
        return x + 1;
    }
}
"#;
        let result = semantic_tokens_full(source);
        let types: Vec<u32> = result.data.iter().map(|t| t.token_type).collect();
        assert!(types.contains(&ty::FUNCTION), "missing function token");
        assert!(types.contains(&ty::PARAMETER), "missing parameter token");
        assert!(types.contains(&ty::TYPE), "missing type token");
    }

    #[test]
    fn test_delta_encoding() {
        let source = "contract A {}\ncontract B {}";
        let result = semantic_tokens_full(source);
        // Verify delta encoding: tokens on different lines should have delta_line > 0
        let mut found_new_line = false;
        for tok in &result.data {
            if tok.delta_line > 0 {
                found_new_line = true;
            }
        }
        assert!(found_new_line, "expected tokens on multiple lines");
    }

    #[test]
    fn test_counter_sol() {
        let source = std::fs::read_to_string("example/Counter.sol").unwrap();
        let result = semantic_tokens_full(&source);
        assert!(
            result.data.len() > 20,
            "Counter.sol should produce many tokens, got {}",
            result.data.len()
        );
    }
}
