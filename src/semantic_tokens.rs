use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens, SemanticTokensLegend,
};
use tree_sitter::{Node, Parser};

// ── Token type indices ─────────────────────────────────────────────────────

#[allow(dead_code)]
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
    pub const COMMENT: u32 = 14;
    pub const STRING: u32 = 15;
    pub const NUMBER: u32 = 16;
    pub const OPERATOR: u32 = 17;
}

/// Build the legend announced in `ServerCapabilities`.
pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::NAMESPACE,   // 0
            SemanticTokenType::TYPE,        // 1
            SemanticTokenType::CLASS,       // 2
            SemanticTokenType::ENUM,        // 3
            SemanticTokenType::INTERFACE,   // 4
            SemanticTokenType::STRUCT,      // 5
            SemanticTokenType::PARAMETER,   // 6
            SemanticTokenType::VARIABLE,    // 7
            SemanticTokenType::PROPERTY,    // 8
            SemanticTokenType::ENUM_MEMBER, // 9
            SemanticTokenType::EVENT,       // 10
            SemanticTokenType::FUNCTION,    // 11
            SemanticTokenType::METHOD,      // 12
            SemanticTokenType::KEYWORD,     // 13
            SemanticTokenType::COMMENT,     // 14
            SemanticTokenType::STRING,      // 15
            SemanticTokenType::NUMBER,      // 16
            SemanticTokenType::OPERATOR,    // 17
        ],
        token_modifiers: vec![
            SemanticTokenModifier::DECLARATION,
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::DEPRECATED,
            SemanticTokenModifier::ABSTRACT,
            SemanticTokenModifier::DOCUMENTATION,
            SemanticTokenModifier::DEFAULT_LIBRARY,
        ],
    }
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
    collect(tree.root_node(), source, &mut tokens);
    tokens.sort_by(|a, b| a.line.cmp(&b.line).then(a.col.cmp(&b.col)));

    // Delta-encode
    let mut prev_line = 0u32;
    let mut prev_col = 0u32;
    let data = tokens
        .iter()
        .map(|t| {
            let dl = t.line - prev_line;
            let dc = if dl == 0 { t.col - prev_col } else { t.col };
            prev_line = t.line;
            prev_col = t.col;
            SemanticToken {
                delta_line: dl,
                delta_start: dc,
                length: t.length,
                token_type: t.token_type,
                token_modifiers_bitset: 0,
            }
        })
        .collect();

    SemanticTokens {
        result_id: None,
        data,
    }
}

#[derive(Debug)]
struct RawToken {
    line: u32,
    col: u32,
    length: u32,
    token_type: u32,
}

// ── Tree walker ────────────────────────────────────────────────────────────

fn collect(node: Node, source: &str, tokens: &mut Vec<RawToken>) {
    if !node.is_named() {
        if let Some(tt) = classify_anon(node, source) {
            push(node, tt, tokens);
        }
        return;
    }

    match node.kind() {
        // Literals
        "comment" => push_multiline(node, source, ty::COMMENT, tokens),
        "number_literal" => push(node, ty::NUMBER, tokens),
        "string" | "string_literal" | "hex_string_literal" | "unicode_string_literal" => {
            push_multiline(node, source, ty::STRING, tokens)
        }
        "boolean_literal" => push(node, ty::KEYWORD, tokens),

        // Declarations — tag the identifier, walk body
        "contract_declaration" => {
            push_child_id(node, ty::CLASS, tokens);
            walk(node, source, tokens);
        }
        "interface_declaration" => {
            push_child_id(node, ty::INTERFACE, tokens);
            walk(node, source, tokens);
        }
        "library_declaration" => {
            push_child_id(node, ty::NAMESPACE, tokens);
            walk(node, source, tokens);
        }
        "struct_definition" | "struct_declaration" => {
            push_child_id(node, ty::STRUCT, tokens);
            walk(node, source, tokens);
        }
        "enum_definition" | "enum_declaration" => {
            push_child_id(node, ty::ENUM, tokens);
            walk(node, source, tokens);
        }
        "enum_value" => push(node, ty::ENUM_MEMBER, tokens),
        "type_alias" => {
            push_child_id(node, ty::TYPE, tokens);
            walk(node, source, tokens);
        }
        "event_definition" => {
            push_child_id(node, ty::EVENT, tokens);
            walk(node, source, tokens);
        }
        "error_declaration" => {
            push_child_id(node, ty::TYPE, tokens);
            walk(node, source, tokens);
        }
        "function_definition" => {
            push_child_id(node, ty::FUNCTION, tokens);
            walk(node, source, tokens);
        }
        "state_variable_declaration" => {
            push_child_id(node, ty::PROPERTY, tokens);
            walk(node, source, tokens);
        }

        // Parameters
        "parameter" | "event_parameter" | "error_parameter" => {
            push_child_id(node, ty::PARAMETER, tokens);
            walk(node, source, tokens);
        }

        // Inheritance
        "inheritance_specifier" => {
            push_child_id(node, ty::INTERFACE, tokens);
            walk(node, source, tokens);
        }

        // Type references
        "user_defined_type" => push(node, ty::TYPE, tokens),

        // Identifiers — only emit where LSP adds value over tree-sitter
        "identifier" => {
            if let Some(tt) = classify_id(node) {
                push(node, tt, tokens);
            }
        }

        // Visibility / state mutability
        "visibility" | "state_mutability" => push(node, ty::KEYWORD, tokens),

        // Let tree-sitter handle: pragmas, builtins, modifiers, members, variables
        "pragma_directive" | "solidity_pragma_token" | "solidity_version" => {}
        "type_name" | "primitive_type" if node.child_count() == 0 => {}
        "type_name" | "primitive_type" => walk(node, source, tokens),

        // Everything else
        _ => walk(node, source, tokens),
    }
}

// ── Identifier classification ──────────────────────────────────────────────

/// Only returns a token type for identifiers where LSP improves on tree-sitter.
fn classify_id(node: Node) -> Option<u32> {
    let parent = node.parent()?;
    match parent.kind() {
        "call_expression" => Some(ty::FUNCTION),
        "inheritance_specifier" => Some(ty::INTERFACE),
        "using_directive" => Some(ty::NAMESPACE),
        "user_defined_type" => Some(ty::TYPE),
        "import_directive" | "import_clause" | "source_import" => Some(ty::NAMESPACE),
        "enum_value" => Some(ty::ENUM_MEMBER),
        "expression" => match parent.parent().map(|gp| gp.kind()) {
            Some("revert_statement" | "emit_statement") => Some(ty::TYPE),
            _ => None,
        },
        _ => None,
    }
}

/// Classify anonymous nodes (keywords, operators, punctuation).
fn classify_anon(node: Node, source: &str) -> Option<u32> {
    let text = &source[node.byte_range()];
    match text {
        // Keywords
        "import" | "from" | "as" | "contract" | "interface" | "library" | "is" | "using"
        | "for" | "struct" | "enum" | "event" | "error" | "function" | "modifier"
        | "constructor" | "fallback" | "receive" | "mapping" | "if" | "else" | "while" | "do"
        | "break" | "continue" | "return" | "returns" | "emit" | "revert" | "try" | "catch"
        | "throw" | "new" | "delete" | "assembly" | "type" | "let" | "switch" | "case"
        | "default" | "leave" | "public" | "private" | "internal" | "external" | "pure"
        | "view" | "payable" | "constant" | "immutable" | "override" | "virtual" | "abstract"
        | "memory" | "storage" | "calldata" | "indexed" | "anonymous" | "unchecked" => {
            Some(ty::KEYWORD)
        }

        // Builtin types — let tree-sitter handle (@type.builtin)
        "address" | "bool" | "string" | "bytes" | "int" | "uint" | "fixed" | "ufixed" => None,
        _ if is_sized_type(text) => None,

        // Operators
        "+" | "-" | "*" | "/" | "%" | "**" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&"
        | "||" | "!" | "&" | "|" | "^" | "~" | "<<" | ">>" | "=" | "+=" | "-=" | "*=" | "/="
        | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | "=>" | "?" | ":" => Some(ty::OPERATOR),

        _ => None,
    }
}

/// Check for sized types like uint256, int128, bytes32.
fn is_sized_type(text: &str) -> bool {
    if let Some(rest) = text
        .strip_prefix("uint")
        .or_else(|| text.strip_prefix("int"))
    {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
    } else if let Some(rest) = text.strip_prefix("bytes") {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}

// ── Token emitters ─────────────────────────────────────────────────────────

fn walk(node: Node, source: &str, tokens: &mut Vec<RawToken>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect(child, source, tokens);
    }
}

fn push(node: Node, token_type: u32, tokens: &mut Vec<RawToken>) {
    let start = node.start_position();
    let len = node.end_byte() - node.start_byte();
    if len > 0 {
        tokens.push(RawToken {
            line: start.row as u32,
            col: start.column as u32,
            length: len as u32,
            token_type,
        });
    }
}

fn push_multiline(node: Node, source: &str, token_type: u32, tokens: &mut Vec<RawToken>) {
    let text = &source[node.byte_range()];
    let start_line = node.start_position().row as u32;
    let start_col = node.start_position().column as u32;

    for (i, line) in text.split('\n').enumerate() {
        let col = if i == 0 { start_col } else { 0 };
        let ws = if i > 0 {
            line.len() - line.trim_start().len()
        } else {
            0
        };
        let len = line.trim_end().len().saturating_sub(ws);
        if len > 0 {
            tokens.push(RawToken {
                line: start_line + i as u32,
                col: col + ws as u32,
                length: len as u32,
                token_type,
            });
        }
    }
}

fn push_child_id(node: Node, token_type: u32, tokens: &mut Vec<RawToken>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" && child.is_named() {
            push(child, token_type, tokens);
            return;
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        assert!(semantic_tokens_full("").data.is_empty());
    }

    #[test]
    fn test_simple_contract() {
        let source = "contract Foo {}";
        let types: Vec<u32> = semantic_tokens_full(source)
            .data
            .iter()
            .map(|t| t.token_type)
            .collect();
        assert!(types.contains(&ty::KEYWORD));
        assert!(types.contains(&ty::CLASS));
    }

    #[test]
    fn test_function_definition() {
        let source = "contract Foo { function bar(uint256 x) public pure returns (uint256) { return x + 1; } }";
        let types: Vec<u32> = semantic_tokens_full(source)
            .data
            .iter()
            .map(|t| t.token_type)
            .collect();
        assert!(types.contains(&ty::FUNCTION));
        assert!(types.contains(&ty::PARAMETER));
    }

    #[test]
    fn test_delta_encoding() {
        let result = semantic_tokens_full("contract A {}\ncontract B {}");
        assert!(result.data.iter().any(|t| t.delta_line > 0));
    }

    #[test]
    fn test_revert_emits_type() {
        let source = "contract Foo { error Bad(); function f() internal { revert Bad(); } }";
        let result = semantic_tokens_full(source);
        // "Bad" in error declaration and "Bad" in revert should both be TYPE
        let type_tokens: Vec<_> = result
            .data
            .iter()
            .filter(|t| t.token_type == ty::TYPE)
            .collect();
        assert!(
            type_tokens.len() >= 2,
            "expected error name as TYPE in both declaration and revert"
        );
    }

    #[test]
    fn test_emit_emits_type() {
        let source = "contract Foo { event Ping(); function f() internal { emit Ping(); } }";
        let result = semantic_tokens_full(source);
        let type_count = result
            .data
            .iter()
            .filter(|t| t.token_type == ty::TYPE)
            .count();
        assert!(type_count >= 1, "expected event name as TYPE in emit");
    }

    #[test]
    fn test_counter_sol() {
        let source = std::fs::read_to_string("example/Counter.sol").unwrap();
        let result = semantic_tokens_full(&source);
        assert!(
            result.data.len() > 20,
            "Counter.sol should produce many tokens"
        );
    }
}
