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

impl VariableDeclaration {
    /// Clone only the fields used by `DeclNode` consumers.
    ///
    /// Skips `value` (initializer expression — can be a large AST subtree),
    /// `name_location`, `constant`, `indexed`, `overrides`, and `base_functions`.
    pub fn decl_clone(&self) -> Self {
        Self {
            id: self.id,
            src: self.src.clone(),
            name: self.name.clone(),
            name_location: None,
            type_name: self.type_name.clone(),
            constant: None,
            mutability: self.mutability.clone(),
            state_variable: self.state_variable,
            storage_location: self.storage_location.clone(),
            visibility: self.visibility.clone(),
            value: None,
            indexed: None,
            scope: self.scope,
            type_descriptions: self.type_descriptions.clone(),
            overrides: None,
            function_selector: self.function_selector.clone(),
            base_functions: None,
            documentation: self.documentation.clone(),
        }
    }
}
