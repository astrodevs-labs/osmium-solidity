use crate::types::InteractableNode;
use crate::utils::is_node_in_range;
use crate::Position;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct ScopeFinder {
    pub root_scope: i64,
    pub parent_scopes: Vec<i64>,
    pub scope: i64,
    position: Position,
    source: String
}

impl<'ast> Visit<'ast> for ScopeFinder {

    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if is_node_in_range(&contract.src, &self.position, &self.source) {
            self.scope = contract.id;
            self.root_scope = self.scope;
            self.parent_scopes.push(self.scope);
        }
        visit::visit_contract_definition(self, contract);
    }

    fn visit_block(&mut self,block: &'ast Block) {
        if is_node_in_range(&block.src, &self.position, &self.source) {
            self.scope = block.id;
            self.parent_scopes.push(self.scope);
        }
        visit::visit_block(self, block);
    }

    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if is_node_in_range(&struct_def.src, &self.position, &self.source) {
            self.scope = struct_def.id;
            self.parent_scopes.push(self.scope);
        }
        visit::visit_struct_definition(self, struct_def);
    }

    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        if is_node_in_range(&enum_def.src, &self.position, &self.source) {
            self.scope = enum_def.id;
            self.parent_scopes.push(self.scope);
        }
        visit::visit_enum_definition(self, enum_def);
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if is_node_in_range(&variable.src, &self.position, &self.source) {
            self.scope = variable.id;
            self.parent_scopes.push(self.scope);
        }
        visit::visit_variable_declaration(self, variable);
    }

    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        if is_node_in_range(&event.src, &self.position, &self.source) {
            self.scope = event.id;
            self.parent_scopes.push(self.scope);
        }
        visit::visit_event_definition(self, event);
    }

    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        if is_node_in_range(&enum_value.src, &self.position, &self.source) {
            self.scope = enum_value.id;
            self.parent_scopes.push(self.scope);
        }
        visit::visit_enum_value(self, enum_value);
    }
}

impl ScopeFinder {

    pub fn new(source: String, position: Position) -> Self {
        ScopeFinder {root_scope: -1, parent_scopes: vec![], scope: -1, position, source}
    }

    pub fn find(&mut self, src: &SourceUnit) -> (i64, Vec<i64>, i64){
        self.visit_source_unit(src);
        self.parent_scopes.pop();
        (self.root_scope, self.parent_scopes.clone(), self.scope)
    }
}
