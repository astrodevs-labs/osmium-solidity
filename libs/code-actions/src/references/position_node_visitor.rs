use crate::types::{InteractableNode, Position};
use crate::utils::*;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct PositionNodeVisitor {
    position: Position,
    pub node: Option<InteractableNode>,
    above_node: Option<InteractableNode>,
    source: String,
}

impl<'ast> Visit<'ast> for PositionNodeVisitor {
    fn visit_user_defined_type_name(&mut self, _udt: &'ast UserDefinedTypeName) {
        if is_node_in_range(&_udt.src, &self.position, &self.source) {
            self.node = Some(InteractableNode::UserDefinedTypeName(_udt.clone()));
        }
        self.above_node = Some(InteractableNode::UserDefinedTypeName(_udt.clone()));
        visit::visit_user_defined_type_name(self, _udt);
    }
    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if is_node_in_range(&contract.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ContractDefinition(contract.clone()));
        }
        visit::visit_contract_definition(self, contract);
    }

    fn visit_elementary_type_name(&mut self, _elementary: &'ast ElementaryTypeName) {
        if is_node_in_range(&_elementary.src, &self.position, &self.source) {
            self.node = None;
        }
    }

    fn visit_function_definition(&mut self, function: &'ast FunctionDefinition) {
        if is_node_in_range(&function.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::FunctionDefinition(function.clone()));
        }
        visit::visit_function_definition(self, function);
    }

    fn visit_modifier_definition(&mut self, modifier: &'ast ModifierDefinition) {
        if is_node_in_range(&modifier.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ModifierDefinition(modifier.clone()));
        }
        visit::visit_modifier_definition(self, modifier);
    }

    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if is_node_in_range(&struct_def.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::StructDefinition(struct_def.clone()));
        }
        visit::visit_struct_definition(self, struct_def);
    }

    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        if is_node_in_range(&enum_def.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::EnumDefinition(enum_def.clone()));
        }
        visit::visit_enum_definition(self, enum_def);
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if is_node_in_range(&variable.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::VariableDeclaration(variable.clone()));
        }
        visit::visit_variable_declaration(self, variable);
    }

    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        if is_node_in_range(&event.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::EventDefinition(event.clone()));
        }
        visit::visit_event_definition(self, event);
    }

    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        if is_node_in_range(&enum_value.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::EnumValue(enum_value.clone()));
        }
        visit::visit_enum_value(self, enum_value);
    }

    fn visit_using_for_directive(&mut self, using_for: &'ast UsingForDirective) {
        if is_node_in_range(&using_for.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::UsingForDirective(using_for.clone()));
        }
        visit::visit_using_for_directive(self, using_for);
    }

    fn visit_import_directive(&mut self, import: &'ast ImportDirective) {
        if is_node_in_range(&import.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ImportDirective(import.clone()));
        }
        visit::visit_import_directive(self, import);
    }

    fn visit_error_definition(&mut self, error: &'ast ErrorDefinition) {
        if is_node_in_range(&error.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ErrorDefinition(error.clone()));
        }
        visit::visit_error_definition(self, error);
    }

    fn visit_function_call(&mut self, function_call: &'ast FunctionCall) {
        if is_node_in_range(&function_call.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::FunctionCall(function_call.clone()));
        }
        visit::visit_function_call(self, function_call);
    }

    fn visit_modifier_invocation(&mut self, modifier_invocation: &'ast ModifierInvocation) {
        if is_node_in_range(&modifier_invocation.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ModifierInvocation(
                modifier_invocation.clone(),
            ));
        }
        visit::visit_modifier_invocation(self, modifier_invocation);
    }

    fn visit_inheritance_specifier(&mut self, inheritance_specifier: &'ast InheritanceSpecifier) {
        if is_node_in_range(&inheritance_specifier.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::InheritanceSpecifier(
                inheritance_specifier.clone(),
            ));
        }
        visit::visit_inheritance_specifier(self, inheritance_specifier);
    }

    fn visit_identifier(&mut self, identifier: &'ast Identifier) {
        if is_node_in_range(&identifier.src, &self.position, &self.source) {
            // trace!("Identifier in range: {:?}", identifier);
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::Identifier(identifier.clone()));
        }
        visit::visit_identifier(self, identifier);
    }

    fn visit_member_access(&mut self, member_access: &'ast MemberAccess) {
        if is_node_in_range(&member_access.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::MemberAccess(member_access.clone()));
        }
        visit::visit_member_access(self, member_access);
    }

    fn visit_new(&mut self, new_expression: &'ast NewExpression) {
        if is_node_in_range(&new_expression.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::NewExpression(
                new_expression.clone(),
                Box::new(self.above_node.clone().unwrap()),
            ));
        }
        visit::visit_new(self, new_expression);
    }
}

impl PositionNodeVisitor {
    pub fn new(position: Position, source: &str) -> Self {
        PositionNodeVisitor {
            position,
            node: None,
            above_node: None,
            source: source.to_owned(),
        }
    }
    pub fn find(&mut self, src: &SourceUnit) -> Option<InteractableNode> {
        self.visit_source_unit(src);
        self.node.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::{
        create_test_ast_file_contract_definition, create_test_ast_file_enum_definition,
        create_test_ast_file_enum_value, create_test_ast_file_error_definition,
        create_test_ast_file_event_definition, create_test_ast_file_function_call,
        create_test_ast_file_function_definition, create_test_ast_file_identifier,
        create_test_ast_file_inheritance_specifier, create_test_ast_file_member_access,
        create_test_ast_file_modifier_definition, create_test_ast_file_modifier_invocation,
        create_test_ast_file_new_expression, create_test_ast_file_struct_definition,
        create_test_ast_file_user_defined_type_name, create_test_ast_file_using_for_directive,
        create_test_ast_file_variable_declaration,
    };

    #[test]
    fn test_find_contract_definition() {
        let file = create_test_ast_file_contract_definition();
        let position = Position {
            line: 3,
            column: 10,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::ContractDefinition(contract)) = node {
            assert_eq!(contract.name, "Test");
        } else {
            panic!("Expected ContractDefinition, got {:?}", node);
        }
    }

    #[test]
    fn test_find_contract_definition_wrong_position() {
        let file = create_test_ast_file_contract_definition();
        let position = Position { line: 1, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_function_definition() {
        let file = create_test_ast_file_function_definition();
        let position = Position {
            line: 3,
            column: 10,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::FunctionDefinition(function)) = node {
            assert_eq!(function.name, "notUsed");
        } else {
            panic!("Expected FunctionDefinition, got {:?}", node);
        }
    }

    #[test]
    fn test_find_function_definition_wrong_position() {
        let file = create_test_ast_file_function_definition();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_variable_declaration() {
        let file = create_test_ast_file_variable_declaration();
        let position = Position {
            line: 3,
            column: 21,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::VariableDeclaration(variable)) = node {
            assert_eq!(variable.name, "number");
        } else {
            panic!("Expected VariableDeclaration, got {:?}", node);
        }
    }

    #[test]
    fn test_find_variable_declaration_wrong_position() {
        let file = create_test_ast_file_variable_declaration();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_enum_definition() {
        let file = create_test_ast_file_enum_definition();
        let position = Position {
            line: 3,
            column: 12,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::EnumDefinition(enum_def)) = node {
            assert_eq!(enum_def.name, "TestEnum");
        } else {
            panic!("Expected EnumDefinition, got {:?}", node);
        }
    }

    #[test]
    fn test_find_enum_definition_wrong_position() {
        let file = create_test_ast_file_enum_definition();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_enum_value() {
        let file = create_test_ast_file_enum_value();
        let position = Position {
            line: 4,
            column: 14,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::EnumValue(enum_value)) = node {
            assert_eq!(enum_value.name, "TestEnumValue");
        } else {
            panic!("Expected EnumValue, got {:?}", node);
        }
    }

    #[test]
    fn test_find_enum_value_wrong_position() {
        let file = create_test_ast_file_enum_value();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_struct_definition() {
        let file = create_test_ast_file_struct_definition();
        let position = Position {
            line: 4,
            column: 14,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::StructDefinition(struct_def)) = node {
            assert_eq!(struct_def.name, "TestStruct");
        } else {
            panic!("Expected StructDefinition, got {:?}", node);
        }
    }

    #[test]
    fn test_find_struct_definition_wrong_position() {
        let file = create_test_ast_file_struct_definition();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_event_definition() {
        let file = create_test_ast_file_event_definition();
        let position = Position {
            line: 4,
            column: 18,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::EventDefinition(event)) = node {
            assert_eq!(event.name, "TestEvent");
        } else {
            panic!("Expected EventDefinition, got {:?}", node);
        }
    }

    #[test]
    fn test_find_event_definition_wrong_position() {
        let file = create_test_ast_file_event_definition();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_using_for_directive() {
        let file = create_test_ast_file_using_for_directive();
        let position = Position {
            line: 4,
            column: 19,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::UsingForDirective(_)) = node {
            assert!(true)
        } else {
            panic!("Expected UsingForDirective, got {:?}", node);
        }
    }

    #[test]
    fn test_find_using_for_directive_wrong_position() {
        let file = create_test_ast_file_using_for_directive();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_error_defintion() {
        let file = create_test_ast_file_error_definition();
        let position = Position {
            line: 3,
            column: 15,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::ErrorDefinition(error)) = node {
            assert_eq!(error.name, "TestError");
        } else {
            panic!("Expected ErrorDefinition, got {:?}", node);
        }
    }

    #[test]
    fn test_find_error_defintion_wrong_position() {
        let file = create_test_ast_file_error_definition();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_function_call() {
        let file = create_test_ast_file_function_call();
        let position = Position {
            line: 5,
            column: 15,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::FunctionCall(_)) = node {
            assert!(true);
        } else {
            panic!("Expected FunctionCall, got {:?}", node);
        }
    }

    #[test]
    fn test_find_function_call_wrong_position() {
        let file = create_test_ast_file_function_call();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_modifier_definition() {
        let file = create_test_ast_file_modifier_definition();
        let position = Position {
            line: 4,
            column: 18,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::ModifierDefinition(modifier)) = node {
            assert_eq!(modifier.name, "modifier");
        } else {
            panic!("Expected ModifierDefinition, got {:?}", node);
        }
    }

    #[test]
    fn test_find_modifier_definition_wrong_position() {
        let file = create_test_ast_file_modifier_definition();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_modifier_invocation() {
        let file = create_test_ast_file_modifier_invocation();
        let position = Position {
            line: 4,
            column: 29,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::ModifierInvocation(_)) = node {
            assert!(true);
        } else {
            panic!("Expected ModifierInvocation, got {:?}", node);
        }
    }

    #[test]
    fn test_find_modifier_invocation_wrong_position() {
        let file = create_test_ast_file_modifier_invocation();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_inheritance_specifier() {
        let file = create_test_ast_file_inheritance_specifier();
        let position = Position {
            line: 3,
            column: 25,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::InheritanceSpecifier(_)) = node {
            assert!(true);
        } else {
            panic!("Expected InheritanceSpecifier, got {:?}", node);
        }
    }

    #[test]
    fn test_find_inheritance_specifier_wrong_position() {
        let file = create_test_ast_file_inheritance_specifier();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_identifier() {
        let file = create_test_ast_file_identifier();
        let position = Position {
            line: 6,
            column: 15,
        };

        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::Identifier(identifier)) = node {
            assert_eq!(identifier.name, "number");
        } else {
            panic!("Expected Identifier, got {:?}", node);
        }
    }

    #[test]
    fn test_find_indentifier_wrong_position() {
        let file = create_test_ast_file_identifier();
        let position = Position { line: 2, column: 1 };

        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_member_access() {
        let file = create_test_ast_file_member_access();
        let position = Position {
            line: 6,
            column: 22,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::MemberAccess(member_access)) = node {
            assert_eq!(member_access.member_name, "member");
        } else {
            panic!("Expected MemberAccess, got {:?}", node);
        }
    }

    #[test]
    fn test_find_member_access_wrong_position() {
        let file = create_test_ast_file_member_access();
        let position = Position { line: 2, column: 1 };

        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_new_expression() {
        let file = create_test_ast_file_new_expression();
        let position = Position {
            line: 6,
            column: 20,
        };

        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::NewExpression(new_expression, _)) = node {
            assert_eq!(
                new_expression.node_type,
                NewExpressionNodeType::NewExpression
            );
        } else {
            panic!("Expected NewExpression, got {:?}", node);
        }
    }

    #[test]
    fn test_find_new_expression_wrong_position() {
        let file = create_test_ast_file_new_expression();
        let position = Position { line: 2, column: 1 };

        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_user_defined_type_name() {
        let file = create_test_ast_file_user_defined_type_name();
        let position = Position {
            line: 6,
            column: 45,
        };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::UserDefinedTypeName(udt)) = node {
            assert_eq!(udt.name, Some("TestStruct".to_string()));
        } else {
            panic!("Expected UserDefinedTypeName, got {:?}", node);
        }
    }

    #[test]
    fn test_find_user_defined_type_name_wrong_position() {
        let file = create_test_ast_file_user_defined_type_name();
        let position = Position { line: 2, column: 1 };
        let mut visitor = PositionNodeVisitor::new(position, &file.file.content);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }
}
