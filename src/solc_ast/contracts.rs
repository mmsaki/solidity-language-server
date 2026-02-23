//! Contract definition and related AST node types.

use serde::{Deserialize, Serialize};

use super::{
    ContractKind, Documentation, ErrorDefinition, EventDefinition, Expression, FunctionDefinition,
    IdentifierPath, ModifierDefinition, NodeID, TypeName, UsingForFunction, VariableDeclaration,
};

/// A node that can appear inside a contract definition's `nodes` array.
///
/// Discriminated by `nodeType` in the JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum ContractDefinitionNode {
    UsingForDirective(UsingForDirective),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
}

impl ContractDefinitionNode {
    /// Extract the function/error/event selector from this contract child node.
    ///
    /// Returns the hex selector string (no `0x` prefix):
    /// - 4-byte for functions, public variables, and errors
    /// - 32-byte for events
    /// - `None` for other node types
    pub fn selector(&self) -> Option<&str> {
        match self {
            Self::FunctionDefinition(n) => n.function_selector.as_deref(),
            Self::VariableDeclaration(n) => n.function_selector.as_deref(),
            Self::EventDefinition(n) => n.event_selector.as_deref(),
            Self::ErrorDefinition(n) => n.error_selector.as_deref(),
            _ => None,
        }
    }

    /// Extract the documentation text from this contract child node.
    pub fn documentation_text(&self) -> Option<String> {
        let doc = match self {
            Self::FunctionDefinition(n) => n.documentation.as_ref()?,
            Self::VariableDeclaration(n) => n.documentation.as_ref()?,
            Self::EventDefinition(n) => n.documentation.as_ref()?,
            Self::ErrorDefinition(n) => n.documentation.as_ref()?,
            Self::StructDefinition(n) => n.documentation.as_ref()?,
            Self::EnumDefinition(n) => n.documentation.as_ref()?,
            Self::ModifierDefinition(n) => n.documentation.as_ref()?,
            _ => return None,
        };
        match doc {
            super::Documentation::String(s) => Some(s.clone()),
            super::Documentation::Structured(s) => Some(s.text.clone()),
        }
    }
}

/// An inheritance specifier (`is Base(arg1, arg2)`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub id: NodeID,
    pub src: String,
    pub base_name: IdentifierPath,
    #[serde(default)]
    pub arguments: Option<Vec<Expression>>,
}

/// A `using ... for ...` directive.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub id: NodeID,
    pub src: String,
    /// The type this directive applies to. `None` means `*` (all types).
    #[serde(default)]
    pub type_name: Option<TypeName>,
    /// Library name (when `using Lib for Type`).
    #[serde(default)]
    pub library_name: Option<IdentifierPath>,
    /// Function list (when `using { f, g } for Type`).
    #[serde(default)]
    pub function_list: Option<Vec<UsingForFunction>>,
    /// Whether this is a file-level `using ... for ... global` directive.
    #[serde(default)]
    pub global: Option<bool>,
}

/// A struct definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub documentation: Option<Documentation>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub members: Vec<VariableDeclaration>,
    #[serde(default)]
    pub scope: Option<NodeID>,
    #[serde(default)]
    pub canonical_name: Option<String>,
}

/// An enum definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub documentation: Option<Documentation>,
    #[serde(default)]
    pub members: Vec<EnumValue>,
    #[serde(default)]
    pub canonical_name: Option<String>,
}

/// A single enum member value.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
}

/// A user-defined value type (`type MyUint is uint256`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    pub underlying_type: TypeName,
    #[serde(default)]
    pub canonical_name: Option<String>,
}

/// A contract definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub documentation: Option<Documentation>,
    pub contract_kind: ContractKind,
    #[serde(rename = "abstract", default)]
    pub is_abstract: Option<bool>,
    #[serde(default)]
    pub base_contracts: Vec<InheritanceSpecifier>,
    #[serde(default)]
    pub contract_dependencies: Vec<NodeID>,
    #[serde(default)]
    pub used_errors: Option<Vec<NodeID>>,
    #[serde(default)]
    pub used_events: Option<Vec<NodeID>>,
    #[serde(default)]
    pub nodes: Vec<ContractDefinitionNode>,
    #[serde(default)]
    pub scope: Option<NodeID>,
    #[serde(default)]
    pub fully_implemented: Option<bool>,
    #[serde(default)]
    pub linearized_base_contracts: Option<Vec<NodeID>>,
    #[serde(default)]
    pub canonical_name: Option<String>,
}
