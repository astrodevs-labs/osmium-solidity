
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;
use crate::types::InteractableNode;

pub struct DefinitionFinder {
    id: i64,
    node: Option<InteractableNode>,
}

impl <'ast> Visit<'ast> for DefinitionFinder {
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

impl DefinitionFinder {

    pub fn new(id: i64) -> Self {
        DefinitionFinder {
            id,
            node: None,
        }
    }
    
    pub fn find(&mut self, src: &SourceUnit) -> Option<InteractableNode> {
        self.visit_source_unit(src);
        self.node.clone()
    }
}