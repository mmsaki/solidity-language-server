//! Function, modifier, and parameter list AST node types.

use serde::{Deserialize, Serialize};

use super::{
    Documentation, Expression, FunctionKind, IdentifierPath, ModifierInvocationKind, NodeID,
    StateMutability, Statement, TypeName, VariableDeclaration, Visibility,
};

/// A function definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub documentation: Option<Documentation>,
    pub kind: FunctionKind,
    pub state_mutability: StateMutability,
    #[serde(default)]
    pub visibility: Option<Visibility>,
    #[serde(rename = "virtual", default)]
    pub is_virtual: Option<bool>,
    #[serde(default)]
    pub overrides: Option<OverrideSpecifier>,
    pub parameters: ParameterList,
    pub return_parameters: ParameterList,
    #[serde(default)]
    pub modifiers: Vec<ModifierInvocation>,
    /// The function body. `None` for interface/abstract functions.
    #[serde(default)]
    pub body: Option<Statement>,
    #[serde(default)]
    pub implemented: Option<bool>,
    #[serde(default)]
    pub scope: Option<NodeID>,
    /// 4-byte function selector (hex string, no `0x` prefix).
    #[serde(default)]
    pub function_selector: Option<String>,
    /// Base functions this function overrides.
    #[serde(default)]
    pub base_functions: Option<Vec<NodeID>>,
}

/// A parameter list (used for function params and return params).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterList {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub parameters: Vec<VariableDeclaration>,
}

/// A modifier definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub documentation: Option<Documentation>,
    #[serde(default)]
    pub visibility: Option<Visibility>,
    pub parameters: ParameterList,
    #[serde(rename = "virtual", default)]
    pub is_virtual: Option<bool>,
    #[serde(default)]
    pub overrides: Option<OverrideSpecifier>,
    /// The modifier body. `None` for abstract modifiers.
    #[serde(default)]
    pub body: Option<Statement>,
    /// Base modifiers this modifier overrides.
    #[serde(default)]
    pub base_modifiers: Option<Vec<NodeID>>,
}

/// A modifier invocation (or base constructor specifier) on a function.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub id: NodeID,
    pub src: String,
    pub modifier_name: IdentifierPath,
    #[serde(default)]
    pub arguments: Option<Vec<Expression>>,
    #[serde(default)]
    pub kind: Option<ModifierInvocationKind>,
}

/// An override specifier (`override` or `override(A, B)`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OverrideSpecifier {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub overrides: Vec<TypeName>,
}
