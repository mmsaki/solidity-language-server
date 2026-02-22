//! Statement AST node types.
//!
//! Corresponds to the statement hierarchy in the official solc AST.

use serde::{Deserialize, Serialize};

use super::{Expression, NodeID, VariableDeclaration};

/// The union of all statement node types.
///
/// Discriminated by `nodeType` in the JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    PlaceholderStatement(PlaceholderStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ForStatement(ForStatement),
    Continue(Continue),
    Break(Break),
    Return(Return),
    Throw(Throw),
    EmitStatement(EmitStatement),
    RevertStatement(RevertStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    TryStatement(TryStatement),
    InlineAssembly(InlineAssembly),
}

/// A block of statements (`{ ... }`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub statements: Vec<Statement>,
}

/// An `unchecked { ... }` block.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UncheckedBlock {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub statements: Vec<Statement>,
}

/// A placeholder statement (`_`) inside a modifier body.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderStatement {
    pub id: NodeID,
    pub src: String,
}

/// An `if` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub id: NodeID,
    pub src: String,
    pub condition: Expression,
    pub true_body: Box<Statement>,
    pub false_body: Option<Box<Statement>>,
}

/// A `while` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub id: NodeID,
    pub src: String,
    pub condition: Expression,
    pub body: Box<Statement>,
}

/// A `do { ... } while (...)` statement.
///
/// The JSON exporter emits `"nodeType": "DoWhileStatement"` for do-while loops.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: NodeID,
    pub src: String,
    pub condition: Expression,
    pub body: Box<Statement>,
}

/// A `for` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub id: NodeID,
    pub src: String,
    pub initialization_expression: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub loop_expression: Option<Box<Statement>>,
    pub body: Box<Statement>,
    #[serde(default)]
    pub is_simple_counter_loop: Option<bool>,
}

/// A `continue` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Continue {
    pub id: NodeID,
    pub src: String,
}

/// A `break` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Break {
    pub id: NodeID,
    pub src: String,
}

/// A `return` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub expression: Option<Expression>,
    pub function_return_parameters: Option<NodeID>,
}

/// A `throw` statement (deprecated, pre-0.5).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Throw {
    pub id: NodeID,
    pub src: String,
}

/// An `emit` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub id: NodeID,
    pub src: String,
    pub event_call: Expression,
}

/// A `revert` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub id: NodeID,
    pub src: String,
    pub error_call: Expression,
}

/// A variable declaration statement (`uint256 x = 42;`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub id: NodeID,
    pub src: String,
    /// Node IDs of the declared variables (may contain `null` for tuple destructuring).
    #[serde(default)]
    pub assignments: Vec<Option<NodeID>>,
    /// The variable declaration nodes.
    #[serde(default)]
    pub declarations: Vec<Option<VariableDeclaration>>,
    /// Initial value expression.
    pub initial_value: Option<Expression>,
}

/// An expression statement (a bare expression used as a statement).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: NodeID,
    pub src: String,
    pub expression: Expression,
    /// Rarely present; only when the expression statement has documentation.
    #[serde(default)]
    pub documentation: Option<String>,
}

/// A `try` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub id: NodeID,
    pub src: String,
    pub external_call: Expression,
    #[serde(default)]
    pub clauses: Vec<TryCatchClause>,
}

/// A single `catch` clause in a `try` statement.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub error_name: Option<String>,
    #[serde(default)]
    pub parameters: Option<super::ParameterList>,
    pub block: Block,
}

/// An inline assembly block.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    pub id: NodeID,
    pub src: String,
    /// The Yul AST root.
    #[serde(rename = "AST", default)]
    pub ast: Option<super::YulBlock>,
    #[serde(default)]
    pub external_references: Vec<super::ExternalReference>,
    #[serde(default)]
    pub evm_version: Option<String>,
    #[serde(default)]
    pub flags: Option<Vec<String>>,
}
