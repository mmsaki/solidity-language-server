//! Yul (inline assembly) AST node types.
//!
//! These represent the Yul sub-AST embedded inside `InlineAssembly` nodes.
//! Field names use `nativeSrc` (not `src`) in the JSON output.

use serde::{Deserialize, Serialize};

use super::YulLiteralKind;

// ── Yul expressions ────────────────────────────────────────────────────────

/// A Yul expression — either an identifier, literal, or function call.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum YulExpression {
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
    YulFunctionCall(YulFunctionCall),
}

/// A Yul identifier reference.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub name: String,
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
}

/// A Yul literal value.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteral {
    pub kind: YulLiteralKind,
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    /// The Yul type (e.g. `""` or `"bool"`).
    #[serde(rename = "type", default)]
    pub yul_type: Option<String>,
}

/// A Yul function call.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub function_name: YulIdentifier,
    #[serde(default)]
    pub arguments: Vec<YulExpression>,
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
}

// ── Yul statements ─────────────────────────────────────────────────────────

/// A Yul statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulVariableDeclaration(YulVariableDeclaration),
    YulAssignment(YulAssignment),
    YulExpressionStatement(YulExpressionStatement),
    YulIf(YulIf),
    YulForLoop(YulForLoop),
    YulBreak(YulBreak),
    YulContinue(YulContinue),
    YulLeave(YulLeave),
    YulSwitch(YulSwitch),
    YulFunctionDefinition(YulFunctionDefinition),
}

/// A Yul block — a sequence of statements.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    #[serde(default)]
    pub statements: Vec<YulStatement>,
}

/// A typed name in Yul (used in variable declarations, function params).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub name: String,
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    #[serde(rename = "type", default)]
    pub yul_type: Option<String>,
}

/// A Yul variable declaration (`let x := expr`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    #[serde(default)]
    pub variables: Vec<YulTypedName>,
    pub value: Option<YulExpression>,
}

/// A Yul assignment (`x := expr`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    #[serde(default)]
    pub variable_names: Vec<YulIdentifier>,
    pub value: Option<YulExpression>,
}

/// A Yul expression statement (a bare function call).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    pub expression: YulExpression,
}

/// A Yul `if` statement (no `else` in Yul).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulIf {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    pub condition: YulExpression,
    pub body: YulBlock,
}

/// A Yul `for` loop.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulForLoop {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    pub pre: YulBlock,
    pub condition: YulExpression,
    pub post: YulBlock,
    pub body: YulBlock,
}

/// A Yul `break` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulBreak {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
}

/// A Yul `continue` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulContinue {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
}

/// A Yul `leave` statement (return from Yul function).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulLeave {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
}

/// A Yul `switch` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    pub expression: YulExpression,
    #[serde(default)]
    pub cases: Vec<YulCase>,
}

/// A single `case` in a Yul `switch`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    /// `None` for the `default` case.
    /// Solc encodes the default case's value as the string `"default"` rather
    /// than `null`, so we need a custom deserializer.
    #[serde(default, deserialize_with = "deserialize_yul_case_value")]
    pub value: Option<YulLiteral>,
    pub body: YulBlock,
}

/// Deserialize a YulCase value field that can be:
/// - a YulLiteral object (normal case)
/// - the string `"default"` (default case — treated as None)
/// - null (also default case)
fn deserialize_yul_case_value<'de, D>(deserializer: D) -> Result<Option<YulLiteral>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;

    struct YulCaseValueVisitor;

    impl<'de> de::Visitor<'de> for YulCaseValueVisitor {
        type Value = Option<YulLiteral>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a YulLiteral object, the string \"default\", or null")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            if v == "default" {
                Ok(None)
            } else {
                Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
            }
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
            let literal = YulLiteral::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(literal))
        }
    }

    deserializer.deserialize_any(YulCaseValueVisitor)
}

/// A Yul function definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionDefinition {
    pub src: String,
    #[serde(default)]
    pub native_src: Option<String>,
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<YulTypedName>,
    #[serde(default)]
    pub return_variables: Vec<YulTypedName>,
    pub body: YulBlock,
}
