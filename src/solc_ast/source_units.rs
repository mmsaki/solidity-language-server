//! Source unit (file-level) AST node types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    ContractDefinition, EnumDefinition, ErrorDefinition, FunctionDefinition, NodeID,
    StructDefinition, UserDefinedValueTypeDefinition, UsingForDirective, VariableDeclaration,
};

/// A node that can appear at the top level of a source file.
///
/// Discriminated by `nodeType` in the JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum SourceUnitNode {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractDefinition(ContractDefinition),
    /// Free (file-level) function.
    FunctionDefinition(FunctionDefinition),
    /// File-level struct.
    StructDefinition(StructDefinition),
    /// File-level enum.
    EnumDefinition(EnumDefinition),
    /// File-level error.
    ErrorDefinition(ErrorDefinition),
    /// File-level `using ... for ...`.
    UsingForDirective(UsingForDirective),
    /// File-level constant.
    VariableDeclaration(VariableDeclaration),
    /// File-level `type ... is ...`.
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
}

/// A pragma directive (`pragma solidity ^0.8.0;`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PragmaDirective {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub literals: Vec<String>,
}

/// An import directive.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportDirective {
    pub id: NodeID,
    pub src: String,
    /// The import path string.
    #[serde(default)]
    pub file: Option<String>,
    /// Resolved absolute path.
    #[serde(default)]
    pub absolute_path: Option<String>,
    /// Unit alias (`import "..." as Alias`).
    #[serde(default)]
    pub unit_alias: Option<String>,
    #[serde(default)]
    pub name_location: Option<String>,
    /// The ID of the imported source unit.
    #[serde(default)]
    pub source_unit: Option<NodeID>,
    /// The scope (containing source unit ID).
    #[serde(default)]
    pub scope: Option<NodeID>,
    /// Symbol aliases (`import { A as B } from "..."`).
    #[serde(default)]
    pub symbol_aliases: Vec<serde_json::Value>,
}

/// A source unit — the root AST node for a single `.sol` file.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub id: NodeID,
    pub src: String,
    #[serde(default)]
    pub absolute_path: Option<String>,
    /// SPDX license identifier.
    #[serde(default)]
    pub license: Option<String>,
    /// Exported symbol name to node ID mapping.
    #[serde(default)]
    pub exported_symbols: HashMap<String, Vec<NodeID>>,
    #[serde(default)]
    pub nodes: Vec<SourceUnitNode>,
}
