//! Variable declaration AST node types.

use serde::{Deserialize, Serialize};

use super::{
    Documentation, Expression, Mutability, NodeID, OverrideSpecifier, StorageLocation,
    TypeDescriptions, TypeName, Visibility,
};

/// A variable declaration.
///
/// Used for state variables, local variables, function parameters, and
/// return parameters.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclaration {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub type_name: Option<TypeName>,
    #[serde(default)]
    pub constant: Option<bool>,
    #[serde(default)]
    pub mutability: Option<Mutability>,
    #[serde(default)]
    pub state_variable: Option<bool>,
    #[serde(default)]
    pub storage_location: Option<StorageLocation>,
    #[serde(default)]
    pub visibility: Option<Visibility>,
    /// Initial value expression (for state variables with initializers).
    #[serde(default)]
    pub value: Option<Expression>,
    /// Whether this parameter is `indexed` (only relevant in events).
    #[serde(default)]
    pub indexed: Option<bool>,
    #[serde(default)]
    pub scope: Option<NodeID>,
    #[serde(default)]
    pub type_descriptions: TypeDescriptions,
    /// Override specifier, if present.
    #[serde(default)]
    pub overrides: Option<OverrideSpecifier>,
    /// Function selector for public state variables.
    #[serde(default)]
    pub function_selector: Option<String>,
    /// Base functions this variable overrides.
    #[serde(default)]
    pub base_functions: Option<Vec<NodeID>>,
    /// Documentation (NatSpec) for state variables.
    #[serde(default)]
    pub documentation: Option<Documentation>,
}
