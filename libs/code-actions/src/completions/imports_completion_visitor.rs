use crate::types::CompletionItem;
use crate::types::CompletionItemKind;
use osmium_libs_solidity_ast_extractor::types::SolidityAstFile;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct ImportCompletionVisitor {
    import_directives: Vec<ImportDirective>,
    items: Vec<CompletionItem>,
}

impl<'ast> Visit<'ast> for ImportCompletionVisitor {
    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if struct_def.visibility == Visibility::Private
            || struct_def.visibility == Visibility::Internal
        {
            return;
        }
        self.items.push(CompletionItem {
            label: struct_def.name.clone(),
            kind: CompletionItemKind::STRUCT,
        });
    }

    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        self.items.push(CompletionItem {
            label: contract.name.clone(),
            kind: CompletionItemKind::CLASS,
        });
        visit::visit_contract_definition(self, contract);
    }

    fn visit_enum_definition(&mut self, enumm: &'ast EnumDefinition) {
        self.items.push(CompletionItem {
            label: enumm.name.clone(),
            kind: CompletionItemKind::ENUM,
        });
    }

    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        self.items.push(CompletionItem {
            label: event.name.clone(),
            kind: CompletionItemKind::EVENT,
        });
    }

    fn visit_error_definition(&mut self, error: &'ast ErrorDefinition) {
        self.items.push(CompletionItem {
            label: error.name.clone(),
            kind: CompletionItemKind::CONSTANT,
        });
    }
}

impl ImportCompletionVisitor {
    pub fn new(import_directives: Vec<ImportDirective>) -> Self {
        ImportCompletionVisitor {
            import_directives,
            items: vec![],
        }
    }

    pub fn get_files_from_imports(&self, files: &Vec<SolidityAstFile>) -> Vec<SolidityAstFile> {
        let mut new_files: Vec<SolidityAstFile> = vec![];
        for file in files {
            for import in &self.import_directives {
                if file.file.path.ends_with(&import.absolute_path) {
                    new_files.push(file.clone());
                }
            }
        }
        new_files
    }

    pub fn find(&mut self, src: &SourceUnit) -> Vec<CompletionItem> {
        self.visit_source_unit(src);
        self.items.clone()
    }
}
