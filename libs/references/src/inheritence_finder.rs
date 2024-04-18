use std::vec;

use crate::types::CompletionItem;
use crate::types::CompletionItemKind;
use crate::types::InteractableNode;
use log::info;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit::*;


pub struct InheritenceFinder {
    contract: ContractDefinition,
    is_self: bool,
    items: Vec<CompletionItem>,
    inheritences: Vec<ContractDefinition>,
    ids_visited: Vec<i64>
}


impl<'ast> Visit<'ast> for InheritenceFinder {

    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if !self.ids_visited.contains(&contract.id) && (self.contract.base_contracts.iter().find(|elem| {
            InteractableNode::InheritanceSpecifier(elem.to_owned().clone()).get_reference_id().is_some_and(|id| id == contract.id)
        }).is_some() || self.is_self ){
            if !self.is_self{
                self.inheritences.push(contract.clone());
            }
            contract.nodes.iter().for_each(|node| {
                match node {
                    ContractDefinitionNodesItem::VariableDeclaration(var) => self.items.push(CompletionItem{
                        label: var.name.clone(),
                        kind: CompletionItemKind::VARIABLE
                    }),
                    ContractDefinitionNodesItem::FunctionDefinition(func) => self.items.push(CompletionItem{
                        label: func.name.clone(),                                                                                                                                                                                                                                                           
                        kind: CompletionItemKind::FUNCTION
                    }),
                    _ => {}
                }
            });
            self.ids_visited.push(contract.id);
        }
    }

}

impl InheritenceFinder {


    pub fn new(contract: ContractDefinition) -> Self {
        InheritenceFinder {contract, is_self: false, items: vec![], inheritences: vec![], ids_visited: vec![]}
    }

    pub fn find(&mut self, src: &SourceUnit, is_self: bool, current_contract: ContractDefinition) -> (Vec<CompletionItem>, Vec<ContractDefinition>) {
        self.contract = current_contract;
        self.is_self = is_self;
        self.inheritences = vec![];
        self.items = vec![];
        self.visit_source_unit(src);
        (self.items.clone(), self.inheritences.clone())
    }

}
