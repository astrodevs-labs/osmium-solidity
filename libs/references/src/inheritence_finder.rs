use crate::types::CompletionItem;
use crate::types::CompletionItemKind;
use crate::types::InteractableNode;
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
        visit::visit_function_definition(self, function);
    }

    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if struct_def.visibility != Visibility::Private || self.is_self {
        }
        visit::visit_struct_definition(self, struct_def);
    }

    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        visit::visit_enum_definition(self, enum_def);
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if variable.visibility != Visibility::Private {
        }
        visit::visit_variable_declaration(self, variable);
    }
    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        visit::visit_event_definition(self, event);
    }
    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        visit::visit_enum_value(self, enum_value);
    }
}

impl InheritenceFinder {


    pub fn new(contract: ContractDefinition, is_self: bool) -> Self {
        InheritenceFinder {contract, is_self: false, items: vec![], inheritences: vec![]}
    }

    pub fn find(&mut self, src: &SourceUnit, is_self: bool) -> (Vec<CompletionItem>, Vec<ContractDefinition>) {
        self.is_self = is_self;
        self.visit_source_unit(src);
        (self.items.clone(), self.inheritences.clone())
    }
}
