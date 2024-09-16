use crate::types::InteractableNode;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct DefinitionVisitor {
    id: i64,
    node: Option<InteractableNode>,
}

impl<'ast> Visit<'ast> for DefinitionVisitor {
    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if contract.id == self.id {
            self.node = Some(InteractableNode::ContractDefinition(contract.clone()));
        } else {
            visit::visit_contract_definition(self, contract);
        }
    }
    fn visit_function_definition(&mut self, function: &'ast FunctionDefinition) {
        if function.id == self.id {
            self.node = Some(InteractableNode::FunctionDefinition(function.clone()));
        } else {
            visit::visit_function_definition(self, function);
        }
    }
    fn visit_modifier_definition(&mut self, modifier: &'ast ModifierDefinition) {
        if modifier.id == self.id {
            self.node = Some(InteractableNode::ModifierDefinition(modifier.clone()));
        } else {
            visit::visit_modifier_definition(self, modifier);
        }
    }
    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if struct_def.id == self.id {
            self.node = Some(InteractableNode::StructDefinition(struct_def.clone()));
        } else {
            visit::visit_struct_definition(self, struct_def);
        }
    }
    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        if enum_def.id == self.id {
            self.node = Some(InteractableNode::EnumDefinition(enum_def.clone()));
        } else {
            visit::visit_enum_definition(self, enum_def);
        }
    }
    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if variable.id == self.id {
            self.node = Some(InteractableNode::VariableDeclaration(variable.clone()));
        } else {
            visit::visit_variable_declaration(self, variable);
        }
    }
    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        if event.id == self.id {
            self.node = Some(InteractableNode::EventDefinition(event.clone()));
        } else {
            visit::visit_event_definition(self, event);
        }
    }
    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        if enum_value.id == self.id {
            self.node = Some(InteractableNode::EnumValue(enum_value.clone()));
        } else {
            visit::visit_enum_value(self, enum_value);
        }
    }
}

impl DefinitionVisitor {
    pub fn new(id: i64) -> Self {
        DefinitionVisitor { id, node: None }
    }

    pub fn find(&mut self, src: &SourceUnit) -> Option<InteractableNode> {
        self.visit_source_unit(src);
        self.node.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::{
        create_test_ast_file_contract_definition, create_test_ast_file_enum_definition,
        create_test_ast_file_enum_value, create_test_ast_file_event_definition,
        create_test_ast_file_function_definition, create_test_ast_file_modifier_definition,
        create_test_ast_file_struct_definition, create_test_ast_file_variable_declaration,
    };

    use super::*;

    #[test]
    fn test_find_contract_definition() {
        let id = 1;
        let file = create_test_ast_file_contract_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::ContractDefinition(contract)) = node {
            assert_eq!(contract.id, id);
        } else {
            panic!("Expected ContractDefinition node");
        }
    }

    #[test]
    fn test_find_contract_definition_not_found() {
        let id = 0;
        let file = create_test_ast_file_contract_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_function_definition() {
        let id = 2;
        let file = create_test_ast_file_function_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::FunctionDefinition(function)) = node {
            assert_eq!(function.id, id);
        } else {
            panic!("Expected FunctionDefinition node");
        }
    }

    #[test]
    fn test_find_function_definition_not_found() {
        let id = 0;
        let file = create_test_ast_file_function_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_modifier_definition() {
        let id = 4;
        let file = create_test_ast_file_modifier_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::ModifierDefinition(modifier)) = node {
            assert_eq!(modifier.id, id);
        } else {
            panic!("Expected ModifierDefinition node");
        }
    }

    #[test]
    fn test_find_modifier_definition_not_found() {
        let id = 0;
        let file = create_test_ast_file_modifier_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_struct_definition() {
        let id = 5;
        let file = create_test_ast_file_struct_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::StructDefinition(struct_def)) = node {
            assert_eq!(struct_def.id, id);
        } else {
            panic!("Expected StructDefinition node");
        }
    }

    #[test]
    fn test_find_struct_definition_not_found() {
        let id = 0;
        let file = create_test_ast_file_struct_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_enum_definition() {
        let id = 6;
        let file = create_test_ast_file_enum_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::EnumDefinition(enum_def)) = node {
            assert_eq!(enum_def.id, id);
        } else {
            panic!("Expected EnumDefinition node");
        }
    }

    #[test]
    fn test_find_enum_definition_not_found() {
        let id = 0;
        let file = create_test_ast_file_enum_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_variable_declaration() {
        let id = 3;
        let file = create_test_ast_file_variable_declaration();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::VariableDeclaration(variable)) = node {
            assert_eq!(variable.id, id);
        } else {
            panic!("Expected VariableDeclaration node");
        }
    }

    #[test]
    fn test_find_variable_declaration_not_found() {
        let id = 0;
        let file = create_test_ast_file_variable_declaration();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_enum_value() {
        let id = 8;
        let file = create_test_ast_file_enum_value();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::EnumValue(enum_def)) = node {
            assert_eq!(enum_def.id, id);
        } else {
            panic!("Expected EnumDefinition node");
        }
    }

    #[test]
    fn test_find_enum_value_not_found() {
        let id = 0;
        let file = create_test_ast_file_enum_value();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }

    #[test]
    fn test_find_event_definition() {
        let id = 7;
        let file = create_test_ast_file_event_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_some());
        if let Some(InteractableNode::EventDefinition(event)) = node {
            assert_eq!(event.id, id);
        } else {
            panic!("Expected EventDefinition node");
        }
    }

    #[test]
    fn test_find_event_definition_not_found() {
        let id = 0;
        let file = create_test_ast_file_event_definition();
        let mut visitor = DefinitionVisitor::new(id);
        let node = visitor.find(&file.ast);
        assert!(node.is_none());
    }
}
