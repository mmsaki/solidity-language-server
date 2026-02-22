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
    pub value: Option<YulLiteral>,
    pub body: YulBlock,
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
