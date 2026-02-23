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

// ── Typed declaration node ────────────────────────────────────────────────

/// A reference to any declaration-level AST node.
///
/// This enum covers the node types that hover, goto, and references care
/// about: functions, variables, contracts, events, errors, structs, enums,
/// modifiers, and user-defined value types.
///
/// Built by [`DeclIndexVisitor`] and stored in `CachedBuild` for O(1)
/// typed node lookup by ID.
#[derive(Clone, Debug)]
pub enum DeclNode {
    FunctionDefinition(FunctionDefinition),
    VariableDeclaration(VariableDeclaration),
    ContractDefinition(ContractDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    ModifierDefinition(ModifierDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
}

impl DeclNode {
    /// Node ID of the declaration.
    pub fn id(&self) -> NodeID {
        match self {
            Self::FunctionDefinition(n) => n.id,
            Self::VariableDeclaration(n) => n.id,
            Self::ContractDefinition(n) => n.id,
            Self::EventDefinition(n) => n.id,
            Self::ErrorDefinition(n) => n.id,
            Self::StructDefinition(n) => n.id,
            Self::EnumDefinition(n) => n.id,
            Self::ModifierDefinition(n) => n.id,
            Self::UserDefinedValueTypeDefinition(n) => n.id,
        }
    }

    /// Name of the declaration.
    pub fn name(&self) -> &str {
        match self {
            Self::FunctionDefinition(n) => &n.name,
            Self::VariableDeclaration(n) => &n.name,
            Self::ContractDefinition(n) => &n.name,
            Self::EventDefinition(n) => &n.name,
            Self::ErrorDefinition(n) => &n.name,
            Self::StructDefinition(n) => &n.name,
            Self::EnumDefinition(n) => &n.name,
            Self::ModifierDefinition(n) => &n.name,
            Self::UserDefinedValueTypeDefinition(n) => &n.name,
        }
    }

    /// Source location string (`"offset:length:fileId"`).
    pub fn src(&self) -> &str {
        match self {
            Self::FunctionDefinition(n) => &n.src,
            Self::VariableDeclaration(n) => &n.src,
            Self::ContractDefinition(n) => &n.src,
            Self::EventDefinition(n) => &n.src,
            Self::ErrorDefinition(n) => &n.src,
            Self::StructDefinition(n) => &n.src,
            Self::EnumDefinition(n) => &n.src,
            Self::ModifierDefinition(n) => &n.src,
            Self::UserDefinedValueTypeDefinition(n) => &n.src,
        }
    }

    /// Scope (parent) node ID, if available.
    pub fn scope(&self) -> Option<NodeID> {
        match self {
            Self::FunctionDefinition(n) => n.scope,
            Self::VariableDeclaration(n) => n.scope,
            Self::ContractDefinition(n) => n.scope,
            Self::EventDefinition(_) => None,
            Self::ErrorDefinition(_) => None,
            Self::StructDefinition(n) => n.scope,
            Self::EnumDefinition(_) => None,
            Self::ModifierDefinition(_) => None,
            Self::UserDefinedValueTypeDefinition(_) => None,
        }
    }

    /// Documentation attached to the declaration.
    pub fn documentation(&self) -> Option<&Documentation> {
        match self {
            Self::FunctionDefinition(n) => n.documentation.as_ref(),
            Self::VariableDeclaration(n) => n.documentation.as_ref(),
            Self::ContractDefinition(n) => n.documentation.as_ref(),
            Self::EventDefinition(n) => n.documentation.as_ref(),
            Self::ErrorDefinition(n) => n.documentation.as_ref(),
            Self::StructDefinition(n) => n.documentation.as_ref(),
            Self::EnumDefinition(n) => n.documentation.as_ref(),
            Self::ModifierDefinition(n) => n.documentation.as_ref(),
            Self::UserDefinedValueTypeDefinition(_) => None,
        }
    }

    /// Function selector (4-byte hex for functions/errors, 32-byte for events).
    pub fn selector(&self) -> Option<&str> {
        match self {
            Self::FunctionDefinition(n) => n.function_selector.as_deref(),
            Self::VariableDeclaration(n) => n.function_selector.as_deref(),
            Self::EventDefinition(n) => n.event_selector.as_deref(),
            Self::ErrorDefinition(n) => n.error_selector.as_deref(),
            _ => None,
        }
    }

    /// Extract a typed [`Selector`] from this declaration.
    ///
    /// Returns `Selector::Func` for functions, public variables, and errors,
    /// or `Selector::Event` for events. Equivalent to `extract_selector(&Value)`.
    pub fn extract_typed_selector(&self) -> Option<crate::types::Selector> {
        match self {
            Self::FunctionDefinition(n) => n
                .function_selector
                .as_deref()
                .map(|s| crate::types::Selector::Func(crate::types::FuncSelector::new(s))),
            Self::VariableDeclaration(n) => n
                .function_selector
                .as_deref()
                .map(|s| crate::types::Selector::Func(crate::types::FuncSelector::new(s))),
            Self::ErrorDefinition(n) => n
                .error_selector
                .as_deref()
                .map(|s| crate::types::Selector::Func(crate::types::FuncSelector::new(s))),
            Self::EventDefinition(n) => n
                .event_selector
                .as_deref()
                .map(|s| crate::types::Selector::Event(crate::types::EventSelector::new(s))),
            _ => None,
        }
    }

    /// Extract documentation text from this declaration.
    ///
    /// Returns the NatSpec text string, equivalent to the raw `extract_documentation()`.
    pub fn extract_doc_text(&self) -> Option<String> {
        self.documentation().map(|doc| match doc {
            Documentation::String(s) => s.clone(),
            Documentation::Structured(s) => s.text.clone(),
        })
    }

    /// Build a Solidity-style signature string for this declaration.
    ///
    /// Typed equivalent of `build_function_signature(&Value)` in `hover.rs`.
    /// Uses direct field access instead of `.get("field").and_then(|v| v.as_str())` chains.
    pub fn build_signature(&self) -> Option<String> {
        match self {
            Self::FunctionDefinition(n) => {
                let params = format_params_typed(&n.parameters);
                let returns = format_params_typed(&n.return_parameters);

                let mut sig = match n.kind {
                    FunctionKind::Constructor => format!("constructor({params})"),
                    FunctionKind::Receive => "receive() external payable".to_string(),
                    FunctionKind::Fallback => format!("fallback({params})"),
                    _ => format!("function {}({params})", n.name),
                };

                // Add visibility (skip for constructor and receive)
                if !matches!(n.kind, FunctionKind::Constructor | FunctionKind::Receive) {
                    if let Some(vis) = &n.visibility {
                        let vis_str = vis.to_string();
                        if !vis_str.is_empty() {
                            sig.push_str(&format!(" {vis_str}"));
                        }
                    }
                }

                // Add state mutability (skip "nonpayable")
                if !matches!(n.state_mutability, StateMutability::Nonpayable) {
                    sig.push_str(&format!(" {}", n.state_mutability));
                }

                if !returns.is_empty() {
                    sig.push_str(&format!(" returns ({returns})"));
                }
                Some(sig)
            }
            Self::ModifierDefinition(n) => {
                let params = format_params_typed(&n.parameters);
                Some(format!("modifier {}({params})", n.name))
            }
            Self::EventDefinition(n) => {
                let params = format_params_typed(&n.parameters);
                Some(format!("event {}({params})", n.name))
            }
            Self::ErrorDefinition(n) => {
                let params = format_params_typed(&n.parameters);
                Some(format!("error {}({params})", n.name))
            }
            Self::VariableDeclaration(n) => {
                let type_str = n
                    .type_descriptions
                    .type_string
                    .as_deref()
                    .unwrap_or("unknown");
                let vis = n
                    .visibility
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                let mut sig = type_str.to_string();
                if !vis.is_empty() {
                    sig.push_str(&format!(" {vis}"));
                }
                match &n.mutability {
                    Some(Mutability::Constant) => sig.push_str(" constant"),
                    Some(Mutability::Immutable) => sig.push_str(" immutable"),
                    _ => {}
                }
                sig.push_str(&format!(" {}", n.name));
                Some(sig)
            }
            Self::ContractDefinition(n) => {
                let mut sig = format!("{} {}", n.contract_kind, n.name);

                if !n.base_contracts.is_empty() {
                    let base_names: Vec<&str> = n
                        .base_contracts
                        .iter()
                        .map(|b| b.base_name.name.as_str())
                        .collect();
                    if !base_names.is_empty() {
                        sig.push_str(&format!(" is {}", base_names.join(", ")));
                    }
                }
                Some(sig)
            }
            Self::StructDefinition(n) => {
                let mut sig = format!("struct {} {{\n", n.name);
                for member in &n.members {
                    let mtype = member
                        .type_descriptions
                        .type_string
                        .as_deref()
                        .unwrap_or("?");
                    sig.push_str(&format!("    {mtype} {};\n", member.name));
                }
                sig.push('}');
                Some(sig)
            }
            Self::EnumDefinition(n) => {
                let mut sig = format!("enum {} {{\n", n.name);
                for member in &n.members {
                    sig.push_str(&format!("    {},\n", member.name));
                }
                sig.push('}');
                Some(sig)
            }
            Self::UserDefinedValueTypeDefinition(n) => {
                let underlying = type_name_to_str(&n.underlying_type);
                Some(format!("type {} is {underlying}", n.name))
            }
        }
    }

    /// Build individual parameter strings for signature help.
    ///
    /// Returns a vec of strings like `["uint256 amount", "uint16 tax"]`.
    /// Typed equivalent of `build_parameter_strings(&Value)` in `hover.rs`.
    pub fn param_strings(&self) -> Vec<String> {
        match self {
            Self::FunctionDefinition(n) => build_param_strings_typed(&n.parameters),
            Self::ModifierDefinition(n) => build_param_strings_typed(&n.parameters),
            Self::EventDefinition(n) => build_param_strings_typed(&n.parameters),
            Self::ErrorDefinition(n) => build_param_strings_typed(&n.parameters),
            _ => Vec::new(),
        }
    }

    /// Returns the `ParameterList` for this declaration's parameters, if any.
    pub fn parameters(&self) -> Option<&ParameterList> {
        match self {
            Self::FunctionDefinition(n) => Some(&n.parameters),
            Self::ModifierDefinition(n) => Some(&n.parameters),
            Self::EventDefinition(n) => Some(&n.parameters),
            Self::ErrorDefinition(n) => Some(&n.parameters),
            _ => None,
        }
    }

    /// Returns the `ParameterList` for this declaration's return parameters, if any.
    pub fn return_parameters(&self) -> Option<&ParameterList> {
        match self {
            Self::FunctionDefinition(n) => Some(&n.return_parameters),
            _ => None,
        }
    }

    /// Returns the node type string, matching the JSON `nodeType` field.
    pub fn node_type(&self) -> &'static str {
        match self {
            Self::FunctionDefinition(_) => "FunctionDefinition",
            Self::VariableDeclaration(_) => "VariableDeclaration",
            Self::ContractDefinition(_) => "ContractDefinition",
            Self::EventDefinition(_) => "EventDefinition",
            Self::ErrorDefinition(_) => "ErrorDefinition",
            Self::StructDefinition(_) => "StructDefinition",
            Self::EnumDefinition(_) => "EnumDefinition",
            Self::ModifierDefinition(_) => "ModifierDefinition",
            Self::UserDefinedValueTypeDefinition(_) => "UserDefinedValueTypeDefinition",
        }
    }

    /// Returns the type description string for this declaration, if available.
    pub fn type_string(&self) -> Option<&str> {
        match self {
            Self::VariableDeclaration(n) => n.type_descriptions.type_string.as_deref(),
            _ => None,
        }
    }

    /// Returns parameter/member names for inlay hint resolution.
    ///
    /// For functions, events, errors, modifiers: returns `parameters.parameters[].name`.
    /// For structs: returns `members[].name`.
    /// Typed equivalent of `get_parameter_names(&Value)` in `inlay_hints.rs`.
    pub fn param_names(&self) -> Option<Vec<String>> {
        match self {
            Self::FunctionDefinition(n) => Some(
                n.parameters
                    .parameters
                    .iter()
                    .map(|p| p.name.clone())
                    .collect(),
            ),
            Self::ModifierDefinition(n) => Some(
                n.parameters
                    .parameters
                    .iter()
                    .map(|p| p.name.clone())
                    .collect(),
            ),
            Self::EventDefinition(n) => Some(
                n.parameters
                    .parameters
                    .iter()
                    .map(|p| p.name.clone())
                    .collect(),
            ),
            Self::ErrorDefinition(n) => Some(
                n.parameters
                    .parameters
                    .iter()
                    .map(|p| p.name.clone())
                    .collect(),
            ),
            Self::StructDefinition(n) => Some(n.members.iter().map(|m| m.name.clone()).collect()),
            _ => None,
        }
    }

    /// For constructors, returns true if this is a constructor function.
    pub fn is_constructor(&self) -> bool {
        matches!(self, Self::FunctionDefinition(n) if matches!(n.kind, crate::solc_ast::enums::FunctionKind::Constructor))
    }
}

// ── Typed formatting helpers ──────────────────────────────────────────────────

/// Format a typed `ParameterList` into a comma-separated parameter string.
///
/// Typed equivalent of `format_parameters(Option<&Value>)` in `hover.rs`.
pub fn format_params_typed(params: &ParameterList) -> String {
    let parts: Vec<String> = params
        .parameters
        .iter()
        .map(|p| {
            let type_str = p.type_descriptions.type_string.as_deref().unwrap_or("?");
            let name = &p.name;
            let storage = p
                .storage_location
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "default".to_string());

            if name.is_empty() {
                type_str.to_string()
            } else if storage != "default" {
                format!("{type_str} {storage} {name}")
            } else {
                format!("{type_str} {name}")
            }
        })
        .collect();

    parts.join(", ")
}

/// Build individual parameter strings from a typed `ParameterList`.
///
/// Returns a vec of strings like `["uint256 amount", "uint16 tax"]`.
fn build_param_strings_typed(params: &ParameterList) -> Vec<String> {
    params
        .parameters
        .iter()
        .map(|p| {
            let type_str = p.type_descriptions.type_string.as_deref().unwrap_or("?");
            let name = &p.name;
            let storage = p
                .storage_location
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "default".to_string());

            if name.is_empty() {
                type_str.to_string()
            } else if storage != "default" {
                format!("{type_str} {storage} {name}")
            } else {
                format!("{type_str} {name}")
            }
        })
        .collect()
}

/// Extract a human-readable type string from a `TypeName` node.
pub fn type_name_to_str(tn: &TypeName) -> &str {
    match tn {
        TypeName::ElementaryTypeName(e) => e
            .type_descriptions
            .type_string
            .as_deref()
            .unwrap_or(&e.name),
        TypeName::UserDefinedTypeName(u) => u
            .type_descriptions
            .type_string
            .as_deref()
            .unwrap_or("unknown"),
        TypeName::FunctionTypeName(f) => f
            .type_descriptions
            .type_string
            .as_deref()
            .unwrap_or("function"),
        TypeName::Mapping(m) => m
            .type_descriptions
            .type_string
            .as_deref()
            .unwrap_or("mapping"),
        TypeName::ArrayTypeName(a) => a
            .type_descriptions
            .type_string
            .as_deref()
            .unwrap_or("array"),
    }
}

/// Visitor that collects all declaration nodes into a flat index.
#[derive(Default)]
pub struct DeclIndexVisitor {
    pub decls: HashMap<NodeID, DeclNode>,
}

impl DeclIndexVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AstVisitor for DeclIndexVisitor {
    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> bool {
        self.decls
            .insert(node.id, DeclNode::FunctionDefinition(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> bool {
        self.decls
            .insert(node.id, DeclNode::VariableDeclaration(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> bool {
        self.decls
            .insert(node.id, DeclNode::ContractDefinition(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> bool {
        self.decls
            .insert(node.id, DeclNode::EventDefinition(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> bool {
        self.decls
            .insert(node.id, DeclNode::ErrorDefinition(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> bool {
        self.decls
            .insert(node.id, DeclNode::StructDefinition(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> bool {
        self.decls
            .insert(node.id, DeclNode::EnumDefinition(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> bool {
        self.decls
            .insert(node.id, DeclNode::ModifierDefinition(node.decl_clone()));
        self.visit_node(node.id, &node.src)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> bool {
        self.decls.insert(
            node.id,
            DeclNode::UserDefinedValueTypeDefinition(node.decl_clone()),
        );
        self.visit_node(node.id, &node.src)
    }
}

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
    fn cached_build_populates_decl_index() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("poolmanager.json");
        let json = std::fs::read_to_string(&path).unwrap();
        let raw: serde_json::Value = serde_json::from_str(&json).unwrap();

        let build = crate::goto::CachedBuild::new(raw, 0);

        assert!(
            !build.decl_index.is_empty(),
            "decl_index should be populated"
        );

        // Count by variant
        let mut funcs = 0;
        let mut vars = 0;
        let mut contracts = 0;
        let mut events = 0;
        let mut errors = 0;
        for decl in build.decl_index.values() {
            match decl {
                DeclNode::FunctionDefinition(_) => funcs += 1,
                DeclNode::VariableDeclaration(_) => vars += 1,
                DeclNode::ContractDefinition(_) => contracts += 1,
                DeclNode::EventDefinition(_) => events += 1,
                DeclNode::ErrorDefinition(_) => errors += 1,
                _ => {}
            }
        }

        assert_eq!(
            contracts, 43,
            "expected 43 ContractDefinitions in decl_index"
        );
        assert_eq!(funcs, 215, "expected 215 FunctionDefinitions in decl_index");
        assert_eq!(events, 12, "expected 12 EventDefinitions in decl_index");
        assert_eq!(errors, 42, "expected 42 ErrorDefinitions in decl_index");
        assert!(vars > 0, "expected VariableDeclarations in decl_index");
    }

    /// Verify that `node_id_to_source_path` covers all declaration nodes.
    #[test]
    fn node_id_to_source_path_covers_decl_index() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("poolmanager.json");
        let json = std::fs::read_to_string(&path).unwrap();
        let raw: serde_json::Value = serde_json::from_str(&json).unwrap();

        let build = crate::goto::CachedBuild::new(raw, 0);

        assert!(
            !build.node_id_to_source_path.is_empty(),
            "node_id_to_source_path should be populated"
        );

        // Every contract, function, event, error at contract level should have a path
        let mut missing = Vec::new();
        for (id, decl) in &build.decl_index {
            // Skip parameter VariableDeclarations — they are nested deeper than
            // the 2-level walk in our path builder
            if matches!(decl, DeclNode::VariableDeclaration(v) if v.state_variable != Some(true)) {
                continue;
            }
            if build.node_id_to_source_path.get(id).is_none() {
                missing.push(format!(
                    "id={id} name={:?} type={}",
                    decl.name(),
                    decl.node_type()
                ));
            }
        }

        assert!(
            missing.is_empty(),
            "Declarations without source path ({}):\n{}",
            missing.len(),
            missing.join("\n"),
        );
    }
}
