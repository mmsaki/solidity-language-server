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
