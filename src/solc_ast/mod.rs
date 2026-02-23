//! Typed Solidity AST deserialized from solc's JSON output.
//!
//! This module provides Rust structs that mirror the AST node types emitted
//! by the Solidity compiler (`solc --standard-json`), along with a visitor
//! trait (`AstVisitor`) for traversal.
//!
//! # Design
//!
//! - All structs derive `serde::Deserialize` to parse directly from solc JSON.
//! - Fields use `Option<T>` liberally for cross-version compatibility.
//! - The `AstVisitor` trait follows the official C++ `ASTConstVisitor` pattern
//!   from [`libsolidity/ast/ASTVisitor.h`](https://github.com/argotorg/solidity/blob/main/libsolidity/ast/ASTVisitor.h).
//! - Each struct implements `Node::accept()` following the patterns in
//!   [`AST_accept.h`](https://github.com/argotorg/solidity/blob/main/libsolidity/ast/AST_accept.h).

pub mod contracts;
pub mod enums;
pub mod events;
pub mod expressions;
pub mod functions;
pub mod source_units;
pub mod statements;
pub mod types;
pub mod variables;
pub mod visitor;
pub mod yul;

// Re-export everything for convenient access via `use crate::solc_ast::*`.
pub use contracts::*;
pub use enums::*;
pub use events::*;
pub use expressions::*;
pub use functions::*;
pub use source_units::*;
pub use statements::*;
pub use types::*;
pub use variables::*;
pub use visitor::*;
pub use yul::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Core types ─────────────────────────────────────────────────────────────

/// AST node ID, matching solc's signed 64-bit integer.
pub type NodeID = i64;

/// Type description attached to expressions and some declarations.
///
/// Contains the compiler's resolved type information as strings.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    /// Human-readable type string, e.g. `"uint256"`, `"mapping(address => uint256)"`.
    pub type_string: Option<String>,
    /// Machine-readable type identifier, e.g. `"t_uint256"`, `"t_mapping$_t_address_$_t_uint256_$"`.
    pub type_identifier: Option<String>,
}

/// Structured documentation (NatSpec) attached to declarations.
///
/// In the AST JSON this appears either as a string or as a node with `id`, `src`, `text`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Documentation {
    /// Simple string (older solc versions or some contexts).
    String(String),
    /// Structured documentation node.
    Structured(StructuredDocumentation),
}

/// A `StructuredDocumentation` AST node.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub id: NodeID,
    pub src: String,
    pub text: String,
}

/// External reference inside an `InlineAssembly` node.
///
/// Maps a Yul identifier to the Solidity declaration it refers to.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalReference {
    pub declaration: NodeID,
    #[serde(default)]
    pub is_offset: bool,
    #[serde(default)]
    pub is_slot: bool,
    pub src: String,
    #[serde(default)]
    pub suffix: Option<String>,
    pub value_size: Option<i64>,
}

/// An entry in a `UsingForDirective`'s `functionList`.
///
/// Either a plain `function` reference or a `definition` + `operator` pair
/// for user-defined operators.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UsingForFunction {
    /// Plain library function reference (when no operator).
    pub function: Option<IdentifierPath>,
    /// Function definition for a user-defined operator.
    pub definition: Option<IdentifierPath>,
    /// The operator being overloaded (e.g. `"+"`).
    pub operator: Option<String>,
}

/// An `IdentifierPath` AST node — a dotted name like `IERC20` or `MyLib.add`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub id: NodeID,
    pub name: String,
    #[serde(default)]
    pub name_locations: Vec<String>,
    pub referenced_declaration: Option<NodeID>,
    pub src: String,
}

/// Top-level output from `solc --standard-json` after normalization.
///
/// Contains the per-file ASTs and compiled contract artifacts.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SolcOutput {
    /// Map of source file path to its source entry.
    #[serde(default)]
    pub sources: HashMap<String, SourceEntry>,

    /// Reverse map from source ID to file path.
    #[serde(default)]
    pub source_id_to_path: HashMap<String, String>,
}

/// A single source file entry in the solc output.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceEntry {
    /// Source file ID assigned by the compiler.
    pub id: i64,
    /// The AST root for this file.
    pub ast: SourceUnit,
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Load and deserialize the entire poolmanager.json fixture.
    fn load_fixture() -> SolcOutput {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("poolmanager.json");
        let json = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
        serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("failed to deserialize poolmanager.json: {e}"))
    }

    #[test]
    fn deserialize_poolmanager() {
        let output = load_fixture();

        // The fixture has 45 source files.
        assert_eq!(
            output.sources.len(),
            45,
            "expected 45 source files in poolmanager.json"
        );

        // Every source entry should have a valid AST with a non-empty src.
        for (path, entry) in &output.sources {
            assert!(entry.ast.id > 0, "bad AST id for {path}");
            assert!(!entry.ast.src.is_empty(), "empty src for {path}");
        }
    }

    #[test]
    fn deserialize_all_source_units() {
        let output = load_fixture();

        // Verify that we can access nodes in every source unit
        // and that the top-level contract count matches expectations.
        let mut contract_count = 0;
        for (_path, entry) in output.sources {
            for node in &entry.ast.nodes {
                if matches!(node, SourceUnitNode::ContractDefinition(_)) {
                    contract_count += 1;
                }
            }
        }
        assert_eq!(
            contract_count, 43,
            "expected 43 top-level ContractDefinitions"
        );
    }

    // ── Visitor tests ──────────────────────────────────────────────────

    /// A visitor that counts specific node types.
    struct NodeCounter {
        functions: usize,
        contracts: usize,
        events: usize,
        errors: usize,
        variables: usize,
        total_nodes: usize,
    }

    impl NodeCounter {
        fn new() -> Self {
            Self {
                functions: 0,
                contracts: 0,
                events: 0,
                errors: 0,
                variables: 0,
                total_nodes: 0,
            }
        }
    }

    impl AstVisitor for NodeCounter {
        fn visit_node(&mut self, _id: NodeID, _src: &str) -> bool {
            self.total_nodes += 1;
            true
        }

        fn visit_function_definition(&mut self, node: &FunctionDefinition) -> bool {
            self.functions += 1;
            self.visit_node(node.id, &node.src)
        }

        fn visit_contract_definition(&mut self, node: &ContractDefinition) -> bool {
            self.contracts += 1;
            self.visit_node(node.id, &node.src)
        }

        fn visit_event_definition(&mut self, node: &EventDefinition) -> bool {
            self.events += 1;
            self.visit_node(node.id, &node.src)
        }

        fn visit_error_definition(&mut self, node: &ErrorDefinition) -> bool {
            self.errors += 1;
            self.visit_node(node.id, &node.src)
        }

        fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> bool {
            self.variables += 1;
            self.visit_node(node.id, &node.src)
        }
    }

    #[test]
    fn visitor_counts_nodes() {
        let output = load_fixture();
        let mut counter = NodeCounter::new();

        for (_path, entry) in output.sources {
            entry.ast.accept(&mut counter);
        }

        assert_eq!(counter.contracts, 43, "expected 43 ContractDefinitions");
        assert_eq!(counter.functions, 215, "expected 215 FunctionDefinitions");
        assert_eq!(counter.events, 12, "expected 12 EventDefinitions");
        assert_eq!(counter.errors, 42, "expected 42 ErrorDefinitions");
        assert!(
            counter.variables > 0,
            "expected at least some VariableDeclarations"
        );
        assert!(
            counter.total_nodes > 0,
            "expected total_nodes to be non-zero"
        );

        // Sanity: total should be much larger than the sum of specific types.
        let specific_sum = counter.contracts + counter.functions + counter.events + counter.errors;
        assert!(
            counter.total_nodes > specific_sum,
            "total_nodes ({}) should be > sum of specific types ({specific_sum})",
            counter.total_nodes
        );
    }

    #[test]
    fn cached_build_populates_typed_ast() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("poolmanager.json");
        let json = std::fs::read_to_string(&path).unwrap();
        let raw: serde_json::Value = serde_json::from_str(&json).unwrap();

        let build = crate::goto::CachedBuild::new(raw, 0);

        let typed = build
            .typed_ast
            .as_ref()
            .expect("typed_ast should be Some for poolmanager.json");

        assert_eq!(typed.len(), 45, "typed_ast should have 45 source files");

        // Verify the typed AST agrees with the raw node index
        assert_eq!(build.nodes.len(), typed.len());
    }
}
