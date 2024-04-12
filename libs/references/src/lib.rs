mod definitions;
mod error;
mod node_finder;
mod types;
mod usages;
mod utils;
mod imports_completion_finder;
mod inheritence_finder;
mod scope_finder;
use definitions::DefinitionFinder;
use error::ReferencesError;
use log::{info, warn};
use node_finder::NodeVisitor;
use osmium_libs_solidity_ast_extractor::extract::extract_ast_from_foundry;
use osmium_libs_solidity_ast_extractor::types::SolidityAstFile;
use osmium_libs_solidity_path_utils::join_path;
pub use solc_ast_rs_types::types::*;
use types::{CompletionItem, InteractableNode};
pub use types::{Location, Position};
use usages::UsagesFinder;

use crate::{scope_finder::ScopeFinder, utils::get_location};

#[derive(Debug)]
pub struct ReferencesProvider {
    pub files: Vec<SolidityAstFile>,
    pub base_path: String,
}

impl ReferencesProvider {
    pub fn set_base_path(&mut self, base_path: String) {
        self.base_path = base_path;
    }

    pub fn update_file_content(&mut self) -> Result<(), ReferencesError> {
        self.files = extract_ast_from_foundry(&self.base_path)?; // will always find the root foundry project
        Ok(())
    }

    fn while_inherits(&self, contract: &ContractDefinition, self_path: &str) -> Vec<CompletionItem> {
        let mut complete_finder = inheritence_finder::InheritenceFinder::new(contract.clone());
        let mut completes: Vec<CompletionItem> = vec![];
        let mut inheritences = vec![contract.clone()];
        while inheritences.len() > 0 {
            for file in &self.files {
                let (items, inheritences_res) = complete_finder.find(&file.ast, file.file.path == self_path, inheritences.last().unwrap().clone());
                completes.append(&mut items.clone());
                inheritences.pop();
                inheritences.append(&mut inheritences_res.clone());
            }
        }
        completes
    }

    fn get_import_completes(&self, imports: Vec<ImportDirective>) -> Vec<CompletionItem> {
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
        let mut import_finder = imports_completion_finder::ImportCompletionFinder::new(imports_to_check.clone());
        let mut files = import_finder.get_files_from_imports(&self.files);
        for file in files {
            completes.append(&mut import_finder.find(&file.ast));
        }
        completes
    }

    pub fn get_scoped_completes(&self, uri: &str, position: Position) -> Vec<CompletionItem> {
        if let Some(file) = self.files.iter().find(|file| file.file.path == uri) {

            let mut scope_finder = ScopeFinder::new(file.file.content.clone(), position);
            let (contract, spi, imports) = scope_finder.find(&file.ast);
            
            let mut completes: Vec<CompletionItem> = vec![];

            if let Some(contract) = contract {
                completes.append(&mut self.while_inherits(&contract, &file.file.path));
            }

            completes.append(&mut self.get_import_completes(imports));

            return completes;
        }
        vec![]
    }

    fn get_node(
        &self,
        uri: &str,
        position: Position,
    ) -> Option<(SolidityAstFile, InteractableNode)> {
        let found_node: Option<InteractableNode>;
        let source_file;
        if let Some(file) = self.files.iter().find(|file| file.file.path == uri) {
            let mut node_finder = NodeVisitor::new(position.clone(), &file.file.content);
            source_file = file;
            found_node = node_finder.find(&file.ast);
        } else {
            warn!("No file found at uri: {}", uri);
            return None;
        }
        if found_node.is_none() {
            info!("[NODE FINDER] No node found at position: {:?}", &position);
            return None;
        }
        Some((source_file.clone(), found_node.unwrap()))
    }

    pub fn get_definition(&self, uri: &str, position: Position) -> Option<Location> {
        let (source_file, found_node) = match self.get_node(uri, position) {
            Some((file, node)) => (file, node),
            None => return None,
        };

        let ref_id = match found_node.get_reference_id() {
            Some(id) => id,
            None => match found_node {
                InteractableNode::ImportDirective(import) => {
                    return Some(Location {
                        start: Position::default(),
                        end: Position::default(),
                        uri: join_path(&self.base_path, &import.absolute_path),
                    });
                }
                _ => {
                    return Some(get_location(&found_node, &source_file));
                }
            },
        };
        let mut def_finder = DefinitionFinder::new(ref_id);
        for file in &self.files {
            if let Some(node) = def_finder.find(&file.ast) {
                return Some(get_location(&node, file));
            }
        }
        None
    }

    pub fn get_references(&self, uri: &str, position: Position) -> Vec<Location> {
        let mut references: Vec<Location> = Vec::new();
        let (_, found_node) = match self.get_node(uri, position) {
            Some((file, node)) => (file, node),
            None => return vec![],
        };

        let id = found_node.get_id();
        info!("Id: {:?}", id);

        let mut usages_finder = UsagesFinder::new(id);
        for file in &self.files {
            let nodes = usages_finder.find(&file.ast);
            for node in nodes {
                references.push(get_location(&node, file));
            }
        }
        references
    }
}
