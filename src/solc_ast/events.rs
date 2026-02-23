//! Event and error definition AST node types.

use serde::{Deserialize, Serialize};

use super::{Documentation, NodeID, ParameterList};

/// An event definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub documentation: Option<Documentation>,
    pub parameters: ParameterList,
    #[serde(default)]
    pub anonymous: Option<bool>,
    /// 32-byte event selector (keccak256 of signature, hex string, no `0x`).
    #[serde(default)]
    pub event_selector: Option<String>,
}

impl EventDefinition {
    /// Clone only the fields used by `DeclNode` consumers.
    pub fn decl_clone(&self) -> Self {
        Self {
            id: self.id,
            src: self.src.clone(),
            name: self.name.clone(),
            name_location: None,
            documentation: self.documentation.clone(),
            parameters: self.parameters.clone(),
            anonymous: None,
            event_selector: self.event_selector.clone(),
        }
    }
}

/// An error definition (`error InsufficientBalance(uint256, uint256)`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub id: NodeID,
    pub src: String,
    pub name: String,
    #[serde(default)]
    pub name_location: Option<String>,
    #[serde(default)]
    pub documentation: Option<Documentation>,
    pub parameters: ParameterList,
    /// 4-byte error selector (hex string, no `0x`).
    #[serde(default)]
    pub error_selector: Option<String>,
}

impl ErrorDefinition {
    /// Clone only the fields used by `DeclNode` consumers.
    pub fn decl_clone(&self) -> Self {
        Self {
            id: self.id,
            src: self.src.clone(),
            name: self.name.clone(),
            name_location: None,
            documentation: self.documentation.clone(),
            parameters: self.parameters.clone(),
            error_selector: self.error_selector.clone(),
        }
    }
}
