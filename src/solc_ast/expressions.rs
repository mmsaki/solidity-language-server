//! Expression AST node types.
//!
//! Corresponds to the expression hierarchy rooted at `Expression` in the
//! official solc AST (`AST.h`).

use serde::{Deserialize, Serialize};

use super::{FunctionCallKind, LiteralKind, NodeID, TypeDescriptions, TypeName};

/// The union of all expression node types.
///
/// Discriminated by `nodeType` in the JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum Expression {
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    Identifier(Identifier),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
}

// ── Individual expression types ────────────────────────────────────────────

/// An assignment expression (`a = b`, `a += b`, etc.).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub id: NodeID,
    pub src: String,
    pub operator: String,
    pub left_hand_side: Box<Expression>,
    pub right_hand_side: Box<Expression>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A binary operation (`a + b`, `a == b`, etc.).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub id: NodeID,
    pub src: String,
    pub operator: String,
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    #[serde(default)]
    pub common_type: Option<TypeDescriptions>,
    /// Reference to user-defined operator function, if any.
    #[serde(default)]
    pub function: Option<NodeID>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A ternary conditional (`cond ? a : b`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub id: NodeID,
    pub src: String,
    pub condition: Box<Expression>,
    pub true_expression: Box<Expression>,
    pub false_expression: Box<Expression>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// An `ElementaryTypeNameExpression` — using a type as a value (e.g. `uint256` in a cast).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub id: NodeID,
    pub src: String,
    /// The type name node. In the JSON this can be either a string or an
    /// `ElementaryTypeName` object depending on solc version.
    pub type_name: serde_json::Value,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub argument_types: Option<serde_json::Value>,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A function call expression.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub id: NodeID,
    pub src: String,
    pub expression: Box<Expression>,
    #[serde(default)]
    pub arguments: Vec<Expression>,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub name_locations: Vec<String>,
    #[serde(default)]
    pub kind: Option<FunctionCallKind>,
    #[serde(default)]
    pub try_call: Option<bool>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// Function call options (`addr.call{value: 1}(data)`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {
    pub id: NodeID,
    pub src: String,
    pub expression: Box<Expression>,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub options: Vec<Expression>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// An identifier reference.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    pub referenced_declaration: Option<NodeID>,
    #[serde(default)]
    pub overloaded_declarations: Vec<NodeID>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub argument_types: Option<serde_json::Value>,
}

/// An index access (`arr[i]`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub id: NodeID,
    pub src: String,
    pub base_expression: Box<Expression>,
    pub index_expression: Option<Box<Expression>>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// An index range access (`arr[start:end]`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub id: NodeID,
    pub src: String,
    pub base_expression: Box<Expression>,
    pub start_expression: Option<Box<Expression>>,
    pub end_expression: Option<Box<Expression>>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A literal value (`42`, `"hello"`, `true`, `hex"dead"`, etc.).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub id: NodeID,
    pub src: String,
    pub kind: LiteralKind,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub hex_value: Option<String>,
    #[serde(default)]
    pub subdenomination: Option<String>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A member access (`expr.member`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {
    pub id: NodeID,
    pub src: String,
    pub member_name: String,
    #[serde(default)]
    pub member_location: Option<String>,
    pub expression: Box<Expression>,
    pub referenced_declaration: Option<NodeID>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub argument_types: Option<serde_json::Value>,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A `new` expression (`new MyContract()`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub id: NodeID,
    pub src: String,
    pub type_name: Box<TypeName>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A tuple expression or inline array (`(a, b)` or `[a, b]`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpression {
    pub id: NodeID,
    pub src: String,
    /// Components may be `null` for omitted elements in destructuring.
    #[serde(default)]
    pub components: Vec<Option<Expression>>,
    #[serde(default)]
    pub is_inline_array: Option<bool>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}

/// A unary operation (`!a`, `++a`, `delete x`, etc.).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub id: NodeID,
    pub src: String,
    pub operator: String,
    pub prefix: bool,
    pub sub_expression: Box<Expression>,
    /// Reference to user-defined operator function, if any.
    #[serde(default)]
    pub function: Option<NodeID>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    #[serde(default)]
    pub is_constant: Option<bool>,
    #[serde(default)]
    pub is_l_value: Option<bool>,
    #[serde(default)]
    pub is_pure: Option<bool>,
    #[serde(default)]
    pub l_value_requested: Option<bool>,
}
