use crate::types::InteractableNode;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct CompleteFinder {
    scope: i64,
    parent_scopes: Vec<i64>,
    contract_scope: i64,
    nodes: Vec<InteractableNode>,
    is_self: bool,
}

impl<'ast> Visit<'ast> for CompleteFinder {

    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if self.contract_scope == contract.id {
            self.nodes.push(InteractableNode::ContractDefinition(contract.clone()));
        }
        visit::visit_contract_definition(self, contract);
    }

    fn visit_function_definition(&mut self, function: &'ast FunctionDefinition) {
        if function.visibility == Visibility::Public || function.visibility == Visibility::External || self.is_self {
            self.nodes.push(InteractableNode::FunctionDefinition(function.clone()));
        }
        visit::visit_function_definition(self, function);
    }

    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if struct_def.visibility == Visibility::Public || struct_def.visibility == Visibility::External || self.is_self {
            self.nodes.push(InteractableNode::StructDefinition(struct_def.clone()));
        }
        visit::visit_struct_definition(self, struct_def);
    }

    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        self.nodes.push(InteractableNode::EnumDefinition(enum_def.clone()));
        visit::visit_enum_definition(self, enum_def);
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if variable.scope == self.scope || self.parent_scopes.contains(&variable.scope)  || variable.visibility == Visibility::Public || variable.visibility == Visibility::External || variable.scope == self.contract_scope{
            self.nodes.push(InteractableNode::VariableDeclaration(variable.clone()));
        }
        visit::visit_variable_declaration(self, variable);
    }
    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        self.nodes.push(InteractableNode::EventDefinition(event.clone()));
        visit::visit_event_definition(self, event);
    }
    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        self.nodes.push(InteractableNode::EnumValue(enum_value.clone()));
        visit::visit_enum_value(self, enum_value);
    }
}

impl CompleteFinder {
    pub fn new(scope: i64, contract_scope: i64, parent_scopes: Vec<i64>) -> Self {
        CompleteFinder {nodes: vec![], scope, contract_scope, is_self: false, parent_scopes}
    }

    pub fn find(&mut self, src: &SourceUnit, is_self: bool) -> Vec<InteractableNode> {
        self.is_self = is_self;
        self.visit_source_unit(src);
        self.nodes.clone()
    }
}
