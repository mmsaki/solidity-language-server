//! AST visitor trait and `Node` trait.
//!
//! This follows the official C++ `ASTConstVisitor` pattern from
//! [`libsolidity/ast/ASTVisitor.h`](https://github.com/argotorg/solidity/blob/main/libsolidity/ast/ASTVisitor.h)
//! and the `accept()` implementations from
//! [`AST_accept.h`](https://github.com/argotorg/solidity/blob/main/libsolidity/ast/AST_accept.h).
//!
//! Each `visit_*` method returns `bool`:
//! - `true` → continue visiting children
//! - `false` → skip children
//!
//! The default implementation of every `visit_*` delegates to `visit_node`,
//! and every `end_visit_*` delegates to `end_visit_node`.

use super::*;

/// Trait for visiting AST nodes.
///
/// Override only the `visit_*` / `end_visit_*` methods you care about.
/// Return `true` from `visit_*` to recurse into children, `false` to skip.
#[allow(unused_variables)]
pub trait AstVisitor {
    // ── Catch-all ──────────────────────────────────────────────────────

    fn visit_node(&mut self, id: NodeID, src: &str) -> bool {
        true
    }
    fn end_visit_node(&mut self, id: NodeID, src: &str) {}

    // ── Source unit ────────────────────────────────────────────────────

    fn visit_source_unit(&mut self, node: &SourceUnit) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_source_unit(&mut self, node: &SourceUnit) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_pragma_directive(&mut self, node: &PragmaDirective) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_import_directive(&mut self, node: &ImportDirective) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_import_directive(&mut self, node: &ImportDirective) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Contract ───────────────────────────────────────────────────────

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_contract_definition(&mut self, node: &ContractDefinition) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_using_for_directive(&mut self, node: &UsingForDirective) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_struct_definition(&mut self, node: &StructDefinition) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_enum_definition(&mut self, node: &EnumDefinition) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_enum_value(&mut self, node: &EnumValue) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_enum_value(&mut self, node: &EnumValue) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Functions & modifiers ──────────────────────────────────────────

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_function_definition(&mut self, node: &FunctionDefinition) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_parameter_list(&mut self, node: &ParameterList) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_parameter_list(&mut self, node: &ParameterList) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_override_specifier(&mut self, node: &OverrideSpecifier) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_modifier_definition(&mut self, node: &ModifierDefinition) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_modifier_invocation(&mut self, node: &ModifierInvocation) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Events & errors ────────────────────────────────────────────────

    fn visit_event_definition(&mut self, node: &EventDefinition) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_event_definition(&mut self, node: &EventDefinition) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_error_definition(&mut self, node: &ErrorDefinition) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Variables ──────────────────────────────────────────────────────

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_variable_declaration(&mut self, node: &VariableDeclaration) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Type names ─────────────────────────────────────────────────────

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_elementary_type_name(&mut self, node: &ElementaryTypeName) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_function_type_name(&mut self, node: &FunctionTypeName) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_mapping(&mut self, node: &Mapping) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_array_type_name(&mut self, node: &ArrayTypeName) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Statements ─────────────────────────────────────────────────────

    fn visit_block(&mut self, node: &Block) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_block(&mut self, node: &Block) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_unchecked_block(&mut self, node: &UncheckedBlock) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_unchecked_block(&mut self, node: &UncheckedBlock) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_placeholder_statement(&mut self, node: &PlaceholderStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_placeholder_statement(&mut self, node: &PlaceholderStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_if_statement(&mut self, node: &IfStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_if_statement(&mut self, node: &IfStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_while_statement(&mut self, node: &WhileStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_do_while_statement(&mut self, node: &DoWhileStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_do_while_statement(&mut self, node: &DoWhileStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_for_statement(&mut self, node: &ForStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_for_statement(&mut self, node: &ForStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_continue(&mut self, node: &Continue) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_continue(&mut self, node: &Continue) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_break(&mut self, node: &Break) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_break(&mut self, node: &Break) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_return(&mut self, node: &Return) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_return(&mut self, node: &Return) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_throw(&mut self, node: &Throw) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_throw(&mut self, node: &Throw) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_emit_statement(&mut self, node: &EmitStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_emit_statement(&mut self, node: &EmitStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_revert_statement(&mut self, node: &RevertStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_revert_statement(&mut self, node: &RevertStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_variable_declaration_statement(&mut self, node: &VariableDeclarationStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_expression_statement(&mut self, node: &ExpressionStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_try_statement(&mut self, node: &TryStatement) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_try_statement(&mut self, node: &TryStatement) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_try_catch_clause(&mut self, node: &TryCatchClause) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_inline_assembly(&mut self, node: &InlineAssembly) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Expressions ────────────────────────────────────────────────────

    fn visit_assignment(&mut self, node: &Assignment) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_assignment(&mut self, node: &Assignment) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_binary_operation(&mut self, node: &BinaryOperation) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_conditional(&mut self, node: &Conditional) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_elementary_type_name_expression(&mut self, node: &ElementaryTypeNameExpression) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_function_call(&mut self, node: &FunctionCall) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_function_call_options(&mut self, node: &FunctionCallOptions) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_identifier(&mut self, node: &Identifier) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_index_access(&mut self, node: &IndexAccess) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_index_range_access(&mut self, node: &IndexRangeAccess) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_literal(&mut self, node: &Literal) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_literal(&mut self, node: &Literal) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_member_access(&mut self, node: &MemberAccess) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_new_expression(&mut self, node: &NewExpression) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_tuple_expression(&mut self, node: &TupleExpression) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_unary_operation(&mut self, node: &UnaryOperation) {
        self.end_visit_node(node.id, &node.src)
    }

    // ── Misc ───────────────────────────────────────────────────────────

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_identifier_path(&mut self, node: &IdentifierPath) {
        self.end_visit_node(node.id, &node.src)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> bool {
        self.visit_node(node.id, &node.src)
    }
    fn end_visit_structured_documentation(&mut self, node: &StructuredDocumentation) {
        self.end_visit_node(node.id, &node.src)
    }
}

// ── Node trait ─────────────────────────────────────────────────────────────

/// Trait implemented by all AST nodes to support the visitor pattern.
pub trait Node {
    fn accept(&self, visitor: &mut dyn AstVisitor);
}

/// Accept a visitor for each element in a list.
pub fn list_accept(list: &[impl Node], visitor: &mut dyn AstVisitor) {
    for node in list {
        node.accept(visitor);
    }
}

/// Accept a visitor for each element in a list of optional nodes.
pub fn opt_list_accept(list: &[Option<impl Node>], visitor: &mut dyn AstVisitor) {
    for node in list.iter().flatten() {
        node.accept(visitor);
    }
}

// ── Node implementations ───────────────────────────────────────────────────
//
// Following the patterns from AST_accept.h in the official solc source.

impl Node for SourceUnit {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_source_unit(self) {
            list_accept(&self.nodes, v);
        }
        v.end_visit_source_unit(self);
    }
}

impl Node for SourceUnitNode {
    fn accept(&self, v: &mut dyn AstVisitor) {
        match self {
            SourceUnitNode::PragmaDirective(n) => n.accept(v),
            SourceUnitNode::ImportDirective(n) => n.accept(v),
            SourceUnitNode::ContractDefinition(n) => n.accept(v),
            SourceUnitNode::FunctionDefinition(n) => n.accept(v),
            SourceUnitNode::StructDefinition(n) => n.accept(v),
            SourceUnitNode::EnumDefinition(n) => n.accept(v),
            SourceUnitNode::ErrorDefinition(n) => n.accept(v),
            SourceUnitNode::UsingForDirective(n) => n.accept(v),
            SourceUnitNode::VariableDeclaration(n) => n.accept(v),
            SourceUnitNode::UserDefinedValueTypeDefinition(n) => n.accept(v),
        }
    }
}

impl Node for PragmaDirective {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_pragma_directive(self);
        v.end_visit_pragma_directive(self);
    }
}

impl Node for ImportDirective {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_import_directive(self);
        v.end_visit_import_directive(self);
    }
}

impl Node for ContractDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_contract_definition(self) {
            if let Some(Documentation::Structured(doc)) = &self.documentation {
                doc.accept(v);
            }
            list_accept(&self.base_contracts, v);
            list_accept(&self.nodes, v);
        }
        v.end_visit_contract_definition(self);
    }
}

impl Node for ContractDefinitionNode {
    fn accept(&self, v: &mut dyn AstVisitor) {
        match self {
            ContractDefinitionNode::UsingForDirective(n) => n.accept(v),
            ContractDefinitionNode::StructDefinition(n) => n.accept(v),
            ContractDefinitionNode::EnumDefinition(n) => n.accept(v),
            ContractDefinitionNode::VariableDeclaration(n) => n.accept(v),
            ContractDefinitionNode::EventDefinition(n) => n.accept(v),
            ContractDefinitionNode::ErrorDefinition(n) => n.accept(v),
            ContractDefinitionNode::FunctionDefinition(n) => n.accept(v),
            ContractDefinitionNode::ModifierDefinition(n) => n.accept(v),
            ContractDefinitionNode::UserDefinedValueTypeDefinition(n) => n.accept(v),
        }
    }
}

impl Node for InheritanceSpecifier {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_inheritance_specifier(self) {
            self.base_name.accept(v);
            if let Some(args) = &self.arguments {
                list_accept(args, v);
            }
        }
        v.end_visit_inheritance_specifier(self);
    }
}

impl Node for UsingForDirective {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_using_for_directive(self) {
            if let Some(lib) = &self.library_name {
                lib.accept(v);
            }
            if let Some(fns) = &self.function_list {
                for f in fns {
                    if let Some(func) = &f.function {
                        func.accept(v);
                    }
                    if let Some(def) = &f.definition {
                        def.accept(v);
                    }
                }
            }
            if let Some(tn) = &self.type_name {
                tn.accept(v);
            }
        }
        v.end_visit_using_for_directive(self);
    }
}

impl Node for StructDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_struct_definition(self) {
            list_accept(&self.members, v);
        }
        v.end_visit_struct_definition(self);
    }
}

impl Node for EnumDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_enum_definition(self) {
            list_accept(&self.members, v);
        }
        v.end_visit_enum_definition(self);
    }
}

impl Node for EnumValue {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_enum_value(self);
        v.end_visit_enum_value(self);
    }
}

impl Node for UserDefinedValueTypeDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_user_defined_value_type_definition(self) {
            self.underlying_type.accept(v);
        }
        v.end_visit_user_defined_value_type_definition(self);
    }
}

impl Node for FunctionDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_function_definition(self) {
            if let Some(Documentation::Structured(doc)) = &self.documentation {
                doc.accept(v);
            }
            if let Some(ov) = &self.overrides {
                ov.accept(v);
            }
            self.parameters.accept(v);
            self.return_parameters.accept(v);
            list_accept(&self.modifiers, v);
            if let Some(body) = &self.body {
                body.accept(v);
            }
        }
        v.end_visit_function_definition(self);
    }
}

impl Node for ParameterList {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_parameter_list(self) {
            list_accept(&self.parameters, v);
        }
        v.end_visit_parameter_list(self);
    }
}

impl Node for OverrideSpecifier {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_override_specifier(self) {
            list_accept(&self.overrides, v);
        }
        v.end_visit_override_specifier(self);
    }
}

impl Node for ModifierDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_modifier_definition(self) {
            if let Some(Documentation::Structured(doc)) = &self.documentation {
                doc.accept(v);
            }
            self.parameters.accept(v);
            if let Some(ov) = &self.overrides {
                ov.accept(v);
            }
            if let Some(body) = &self.body {
                body.accept(v);
            }
        }
        v.end_visit_modifier_definition(self);
    }
}

impl Node for ModifierInvocation {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_modifier_invocation(self) {
            self.modifier_name.accept(v);
            if let Some(args) = &self.arguments {
                list_accept(args, v);
            }
        }
        v.end_visit_modifier_invocation(self);
    }
}

impl Node for EventDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_event_definition(self) {
            if let Some(Documentation::Structured(doc)) = &self.documentation {
                doc.accept(v);
            }
            self.parameters.accept(v);
        }
        v.end_visit_event_definition(self);
    }
}

impl Node for ErrorDefinition {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_error_definition(self) {
            if let Some(Documentation::Structured(doc)) = &self.documentation {
                doc.accept(v);
            }
            self.parameters.accept(v);
        }
        v.end_visit_error_definition(self);
    }
}

impl Node for VariableDeclaration {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_variable_declaration(self) {
            if let Some(tn) = &self.type_name {
                tn.accept(v);
            }
            if let Some(ov) = &self.overrides {
                ov.accept(v);
            }
            if let Some(val) = &self.value {
                val.accept(v);
            }
        }
        v.end_visit_variable_declaration(self);
    }
}

// ── Type names ─────────────────────────────────────────────────────────────

impl Node for TypeName {
    fn accept(&self, v: &mut dyn AstVisitor) {
        match self {
            TypeName::ElementaryTypeName(n) => n.accept(v),
            TypeName::UserDefinedTypeName(n) => n.accept(v),
            TypeName::FunctionTypeName(n) => n.accept(v),
            TypeName::Mapping(n) => n.accept(v),
            TypeName::ArrayTypeName(n) => n.accept(v),
        }
    }
}

impl Node for ElementaryTypeName {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_elementary_type_name(self);
        v.end_visit_elementary_type_name(self);
    }
}

impl Node for UserDefinedTypeName {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_user_defined_type_name(self)
            && let Some(path) = &self.path_node
        {
            path.accept(v);
        }
        v.end_visit_user_defined_type_name(self);
    }
}

impl Node for FunctionTypeName {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_function_type_name(self) {
            self.parameter_types.accept(v);
            self.return_parameter_types.accept(v);
        }
        v.end_visit_function_type_name(self);
    }
}

impl Node for Mapping {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_mapping(self) {
            self.key_type.accept(v);
            self.value_type.accept(v);
        }
        v.end_visit_mapping(self);
    }
}

impl Node for ArrayTypeName {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_array_type_name(self) {
            self.base_type.accept(v);
            if let Some(len) = &self.length {
                len.accept(v);
            }
        }
        v.end_visit_array_type_name(self);
    }
}

// ── Statements ─────────────────────────────────────────────────────────────

impl Node for Statement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        match self {
            Statement::Block(n) => n.accept(v),
            Statement::UncheckedBlock(n) => n.accept(v),
            Statement::PlaceholderStatement(n) => n.accept(v),
            Statement::IfStatement(n) => n.accept(v),
            Statement::WhileStatement(n) => n.accept(v),
            Statement::DoWhileStatement(n) => n.accept(v),
            Statement::ForStatement(n) => n.accept(v),
            Statement::Continue(n) => n.accept(v),
            Statement::Break(n) => n.accept(v),
            Statement::Return(n) => n.accept(v),
            Statement::Throw(n) => n.accept(v),
            Statement::EmitStatement(n) => n.accept(v),
            Statement::RevertStatement(n) => n.accept(v),
            Statement::VariableDeclarationStatement(n) => n.accept(v),
            Statement::ExpressionStatement(n) => n.accept(v),
            Statement::TryStatement(n) => n.accept(v),
            Statement::InlineAssembly(n) => n.accept(v),
        }
    }
}

impl Node for Block {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_block(self) {
            list_accept(&self.statements, v);
        }
        v.end_visit_block(self);
    }
}

impl Node for UncheckedBlock {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_unchecked_block(self) {
            list_accept(&self.statements, v);
        }
        v.end_visit_unchecked_block(self);
    }
}

impl Node for PlaceholderStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_placeholder_statement(self);
        v.end_visit_placeholder_statement(self);
    }
}

impl Node for IfStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_if_statement(self) {
            self.condition.accept(v);
            self.true_body.accept(v);
            if let Some(fb) = &self.false_body {
                fb.accept(v);
            }
        }
        v.end_visit_if_statement(self);
    }
}

impl Node for WhileStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_while_statement(self) {
            self.condition.accept(v);
            self.body.accept(v);
        }
        v.end_visit_while_statement(self);
    }
}

impl Node for DoWhileStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_do_while_statement(self) {
            self.condition.accept(v);
            self.body.accept(v);
        }
        v.end_visit_do_while_statement(self);
    }
}

impl Node for ForStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_for_statement(self) {
            if let Some(init) = &self.initialization_expression {
                init.accept(v);
            }
            if let Some(cond) = &self.condition {
                cond.accept(v);
            }
            if let Some(lp) = &self.loop_expression {
                lp.accept(v);
            }
            self.body.accept(v);
        }
        v.end_visit_for_statement(self);
    }
}

impl Node for Continue {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_continue(self);
        v.end_visit_continue(self);
    }
}

impl Node for Break {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_break(self);
        v.end_visit_break(self);
    }
}

impl Node for Return {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_return(self)
            && let Some(expr) = &self.expression
        {
            expr.accept(v);
        }
        v.end_visit_return(self);
    }
}

impl Node for Throw {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_throw(self);
        v.end_visit_throw(self);
    }
}

impl Node for EmitStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_emit_statement(self) {
            self.event_call.accept(v);
        }
        v.end_visit_emit_statement(self);
    }
}

impl Node for RevertStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_revert_statement(self) {
            self.error_call.accept(v);
        }
        v.end_visit_revert_statement(self);
    }
}

impl Node for VariableDeclarationStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_variable_declaration_statement(self) {
            opt_list_accept(&self.declarations, v);
            if let Some(init) = &self.initial_value {
                init.accept(v);
            }
        }
        v.end_visit_variable_declaration_statement(self);
    }
}

impl Node for ExpressionStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_expression_statement(self) {
            self.expression.accept(v);
        }
        v.end_visit_expression_statement(self);
    }
}

impl Node for TryStatement {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_try_statement(self) {
            self.external_call.accept(v);
            list_accept(&self.clauses, v);
        }
        v.end_visit_try_statement(self);
    }
}

impl Node for TryCatchClause {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_try_catch_clause(self) {
            if let Some(params) = &self.parameters {
                params.accept(v);
            }
            self.block.accept(v);
        }
        v.end_visit_try_catch_clause(self);
    }
}

impl Node for InlineAssembly {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_inline_assembly(self);
        v.end_visit_inline_assembly(self);
    }
}

// ── Expressions ────────────────────────────────────────────────────────────

impl Node for Expression {
    fn accept(&self, v: &mut dyn AstVisitor) {
        match self {
            Expression::Assignment(n) => n.accept(v),
            Expression::BinaryOperation(n) => n.accept(v),
            Expression::Conditional(n) => n.accept(v),
            Expression::ElementaryTypeNameExpression(n) => n.accept(v),
            Expression::FunctionCall(n) => n.accept(v),
            Expression::FunctionCallOptions(n) => n.accept(v),
            Expression::Identifier(n) => n.accept(v),
            Expression::IndexAccess(n) => n.accept(v),
            Expression::IndexRangeAccess(n) => n.accept(v),
            Expression::Literal(n) => n.accept(v),
            Expression::MemberAccess(n) => n.accept(v),
            Expression::NewExpression(n) => n.accept(v),
            Expression::TupleExpression(n) => n.accept(v),
            Expression::UnaryOperation(n) => n.accept(v),
        }
    }
}

impl Node for Assignment {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_assignment(self) {
            self.left_hand_side.accept(v);
            self.right_hand_side.accept(v);
        }
        v.end_visit_assignment(self);
    }
}

impl Node for BinaryOperation {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_binary_operation(self) {
            self.left_expression.accept(v);
            self.right_expression.accept(v);
        }
        v.end_visit_binary_operation(self);
    }
}

impl Node for Conditional {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_conditional(self) {
            self.condition.accept(v);
            self.true_expression.accept(v);
            self.false_expression.accept(v);
        }
        v.end_visit_conditional(self);
    }
}

impl Node for ElementaryTypeNameExpression {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_elementary_type_name_expression(self);
        v.end_visit_elementary_type_name_expression(self);
    }
}

impl Node for FunctionCall {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_function_call(self) {
            self.expression.accept(v);
            list_accept(&self.arguments, v);
        }
        v.end_visit_function_call(self);
    }
}

impl Node for FunctionCallOptions {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_function_call_options(self) {
            self.expression.accept(v);
            list_accept(&self.options, v);
        }
        v.end_visit_function_call_options(self);
    }
}

impl Node for Identifier {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_identifier(self);
        v.end_visit_identifier(self);
    }
}

impl Node for IndexAccess {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_index_access(self) {
            self.base_expression.accept(v);
            if let Some(idx) = &self.index_expression {
                idx.accept(v);
            }
        }
        v.end_visit_index_access(self);
    }
}

impl Node for IndexRangeAccess {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_index_range_access(self) {
            self.base_expression.accept(v);
            if let Some(s) = &self.start_expression {
                s.accept(v);
            }
            if let Some(e) = &self.end_expression {
                e.accept(v);
            }
        }
        v.end_visit_index_range_access(self);
    }
}

impl Node for Literal {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_literal(self);
        v.end_visit_literal(self);
    }
}

impl Node for MemberAccess {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_member_access(self) {
            self.expression.accept(v);
        }
        v.end_visit_member_access(self);
    }
}

impl Node for NewExpression {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_new_expression(self) {
            self.type_name.accept(v);
        }
        v.end_visit_new_expression(self);
    }
}

impl Node for TupleExpression {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_tuple_expression(self) {
            opt_list_accept(&self.components, v);
        }
        v.end_visit_tuple_expression(self);
    }
}

impl Node for UnaryOperation {
    fn accept(&self, v: &mut dyn AstVisitor) {
        if v.visit_unary_operation(self) {
            self.sub_expression.accept(v);
        }
        v.end_visit_unary_operation(self);
    }
}

// ── Misc ───────────────────────────────────────────────────────────────────

impl Node for IdentifierPath {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_identifier_path(self);
        v.end_visit_identifier_path(self);
    }
}

impl Node for StructuredDocumentation {
    fn accept(&self, v: &mut dyn AstVisitor) {
        v.visit_structured_documentation(self);
        v.end_visit_structured_documentation(self);
    }
}
