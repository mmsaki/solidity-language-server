//! Type name AST nodes.
//!
//! These represent type annotations in Solidity source code, e.g. `uint256`,
//! `address payable`, `mapping(address => uint256)`, `uint256[]`.

use serde::{Deserialize, Serialize};

use super::{
    Expression, IdentifierPath, NodeID, ParameterList, StateMutability, TypeDescriptions,
    Visibility,
};

/// A type name — the union of all possible type annotations.
///
/// Discriminated by `nodeType` in the JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(UserDefinedTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    ArrayTypeName(ArrayTypeName),
}

/// An elementary (built-in) type name like `uint256`, `address`, `bool`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub state_mutability: Option<StateMutability>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
}

/// A user-defined type name (contract, struct, enum, UDVT).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedTypeName {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub path_node: Option<IdentifierPath>,
    pub referenced_declaration: Option<NodeID>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
}

/// A function type name (e.g. `function(uint256) external returns (bool)`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTypeName {
    pub id: NodeID,
    pub src: String,
    pub visibility: Visibility,
    pub state_mutability: StateMutability,
    pub parameter_types: ParameterList,
    pub return_parameter_types: ParameterList,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
}

/// A `mapping(K => V)` type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub id: NodeID,
    pub src: String,
    pub key_type: Box<TypeName>,
    #[serde(default)]
    pub key_name: Option<String>,
    #[serde(default)]
    pub key_name_location: Option<String>,
    pub value_type: Box<TypeName>,
    #[serde(default)]
    pub value_name: Option<String>,
    #[serde(default)]
    pub value_name_location: Option<String>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
}

/// An array type like `uint256[]` or `bytes32[10]`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeName {
    pub id: NodeID,
    pub src: String,
    pub base_type: Box<TypeName>,
    /// The length expression, if fixed-size (e.g. `[10]`).
    #[serde(default)]
    pub length: Option<Expression>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
}
