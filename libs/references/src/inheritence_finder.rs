use crate::types::CompletionItem;
use crate::types::CompletionItemKind;
use osmium_libs_solidity_ast_extractor::types::SolidityAstFile;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;


pub struct InheritenceFinder {
    contract: ContractDefinition,
    is_self: bool,
    items: Vec<CompletionItem>,
    inheritences: Vec<ContractDefinition>,
}


impl<'ast> Visit<'ast> for InheritenceFinder {

    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if self.contract.contract_dependencies.contains(&contract.id) || self.is_self {
            if !self.is_self{
                self.inheritences.push(contract.clone());
            }
            visit::visit_contract_definition(self, contract);
        }
    }

    fn visit_function_definition(&mut self, function: &'ast FunctionDefinition) {
        if function.visibility != Visibility::Private || self.is_self {
            self.items.push(CompletionItem{
                label: function.name.clone(),
                kind: CompletionItemKind::FUNCTION
            });
        }
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if variable.visibility != Visibility::Private || self.is_self {
            self.items.push(CompletionItem{
                label: variable.name.clone(),
                kind: CompletionItemKind::VARIABLE
            });
        }
    }

}

impl InheritenceFinder {


    pub fn new(contract: ContractDefinition) -> Self {
        InheritenceFinder {contract, is_self: false, items: vec![], inheritences: vec![]}
    }

    pub fn find(&mut self, src: &SourceUnit, is_self: bool, current_contract: ContractDefinition) -> (Vec<CompletionItem>, Vec<ContractDefinition>) {
        self.contract = current_contract;
        self.is_self = is_self;
        self.visit_source_unit(src);
        (self.items.clone(), self.inheritences.clone())
    }

}
