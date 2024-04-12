use std::vec;

use crate::types::InteractableNode;
use crate::utils::is_node_in_range;
use crate::Position;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct ScopeFinder {
    pub contract: Option<ContractDefinition>,
    pub spi: Vec<InteractableNode>,
    pub imports: Vec<ImportDirective>,
    position: Position,
    source: String
}

impl<'ast> Visit<'ast> for ScopeFinder {

    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if is_node_in_range(&contract.src, &self.position, &self.source) {
            self.contract = Some(contract.clone());
            visit::visit_contract_definition(self, contract);
        }
    }

    fn visit_block(&mut self,block: &'ast Block) {
        if is_node_in_range(&block.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::Block(block.clone()));
        }
        visit::visit_block(self, block);
    }

    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if is_node_in_range(&struct_def.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::StructDefinition(struct_def.clone()));
        }
        visit::visit_struct_definition(self, struct_def);
    }

    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        if is_node_in_range(&enum_def.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::EnumDefinition(enum_def.clone()));
        }
        visit::visit_enum_definition(self, enum_def);
    }

    fn visit_while(&mut self,r#while: &'ast WhileStatement) {
        if is_node_in_range(&r#while.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::WhileStatement(r#while.clone()));
        }
        visit::visit_while(self, r#while);
    }

    fn visit_do_while(&mut self, do_while: &'ast DoWhileStatement) {
        if is_node_in_range(&do_while.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::DoWhileStatement(do_while.clone()));
        }
        visit::visit_do_while(self, do_while);
    }
    fn visit_unchecked(&mut self, unchecked: &'ast UncheckedBlock) {
        if is_node_in_range(&unchecked.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::UncheckedBlock(unchecked.clone()));
        }
        visit::visit_unchecked(self, unchecked);
    }

    fn visit_if(&mut self, r#if: &'ast IfStatement) {
        if is_node_in_range(&r#if.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::IfStatement(r#if.clone()));
        }
        visit::visit_if(self, r#if);
    }

    fn visit_for(&mut self, r#for: &'ast ForStatement) {
        if is_node_in_range(&r#for.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::ForStatement(r#for.clone()));
        }
        visit::visit_for(self, r#for);
    }

    fn visit_try(&mut self, r#try: &'ast TryStatement) {
        if is_node_in_range(&r#try.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::TryStatement(r#try.clone()));
        }
        visit::visit_try(self, r#try);
    }

    fn visit_try_catch_clause(&mut self,clause: &'ast TryCatchClause) {
        if is_node_in_range(&clause.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::TryCatchClause(clause.clone()));
        }
        visit::visit_try_catch_clause(self, clause);
    }
    

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if is_node_in_range(&variable.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::VariableDeclaration(variable.clone()));
        }
        visit::visit_variable_declaration(self, variable);
    }

    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        if is_node_in_range(&event.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::EventDefinition(event.clone()));
        }
        visit::visit_event_definition(self, event);
    }

    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        if is_node_in_range(&enum_value.src, &self.position, &self.source) {
            self.spi.push(InteractableNode::EnumValue(enum_value.clone()));
        }
        visit::visit_enum_value(self, enum_value);
    }

    fn visit_import_directive(&mut self,import: &'ast ImportDirective) {
        if is_node_in_range(&import.src, &self.position, &self.source) {
            self.imports.push(import.clone());
        }
        visit::visit_import_directive(self, import);
    }

}

impl ScopeFinder {

    pub fn new(source: String, position: Position) -> Self {
        ScopeFinder {spi: vec![], contract: None, imports: vec![], position, source}
    }

    pub fn find(&mut self, src: &SourceUnit) -> (Option<ContractDefinition>, Vec<InteractableNode>, Vec<ImportDirective>){
        self.visit_source_unit(src);
        (self.contract.clone(), self.spi.clone(), self.imports.clone())
    }
}
