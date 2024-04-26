use osmium_libs_solidity_ast_extractor::types::SolidityAstFile;
use solc_ast_rs_types::types::{ContractDefinition, ImportDirective};

use crate::{
    completions::{
        position_scope_visitor::PositionScopeVisitor,
        spi_completion_provider::SPICompletionProvider,
    },
    types::{self, CompletionItem},
    Position,
};

use super::{
    imports_completion_visitor::ImportCompletionVisitor,
    inheritence_completion_visitor::InheritenceCompletionVisitor,
};

pub struct AutoCompleteProvider {}

impl AutoCompleteProvider {
    pub fn new() -> Self {
        Self {}
    }

    fn while_inherits(
        &self,
        contract: &ContractDefinition,
        root_file: &SolidityAstFile,
        files: &Vec<SolidityAstFile>,
    ) -> Vec<CompletionItem> {
        let mut complete_finder = InheritenceCompletionVisitor::new(contract.clone());
        let mut completes: Vec<CompletionItem> = vec![];
        let mut inheritences = vec![contract.clone()];

        while let Some(current) = inheritences.pop() {
            // info!("Current contract to search for inheritence: {:?}", current.name);
            for file in files {
                let (items, inheritences_res) = complete_finder.find(
                    &file.ast,
                    root_file.file.path == file.file.path,
                    current.clone(),
                );
                completes.append(&mut items.clone());
                inheritences.append(&mut inheritences_res.clone());
            }
        }
        completes
    }

    fn get_import_completes(
        &self,
        imports: Vec<ImportDirective>,
        files: &Vec<SolidityAstFile>,
    ) -> Vec<CompletionItem> {
        let mut completes: Vec<CompletionItem> = vec![];
        let mut imports_to_check: Vec<ImportDirective> = vec![];
        for import in imports {
            if import.unit_alias.is_empty() && import.symbol_aliases.is_empty() {
                imports_to_check.push(import);
            } else if import.unit_alias.is_empty() {
                for symbol in import.symbol_aliases {
                    completes.push(CompletionItem {
                        label: symbol.foreign.name.clone(),
                        kind: types::CompletionItemKind::MODULE,
                    });
                }
            } else {
                completes.push(CompletionItem {
                    label: import.unit_alias.clone(),
                    kind: types::CompletionItemKind::MODULE,
                });
            }
        }
        let mut import_finder = ImportCompletionVisitor::new(imports_to_check.clone());
        let files = import_finder.get_files_from_imports(files);
        for file in files {
            completes.append(&mut import_finder.find(&file.ast));
        }
        completes
    }

    pub fn get_suggestions(
        &self,
        uri: &str,
        position: Position,
        files: &Vec<SolidityAstFile>,
    ) -> Vec<CompletionItem> {
        if let Some(file) = files.iter().find(|file| file.file.path == uri) {
            let mut scope_finder = PositionScopeVisitor::new(file.file.content.clone(), position);
            let (contract, spi, imports) = scope_finder.find(&file.ast);
            let mut completes: Vec<CompletionItem> = vec![];

            if let Some(contract) = contract {
                completes.append(&mut self.while_inherits(&contract, file, files));
            }

            let spi_finder = SPICompletionProvider::new(spi);
            completes.append(&mut spi_finder.inspect());

            completes.append(&mut self.get_import_completes(imports, files));

            return completes;
        }
        vec![]
    }
}
