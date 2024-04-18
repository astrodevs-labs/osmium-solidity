mod definitions;
mod error;
mod node_finder;
mod types;
mod usages;
mod utils;
mod scoped_completion_finder;
mod imports_completion_finder;
mod inheritence_finder;
mod scope_finder;
use core::time;

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
use std::{sync::{Arc, Mutex, RwLock}, time::Instant};

use crate::{scope_finder::ScopeFinder, scoped_completion_finder::ScopedCompletionFinder, utils::get_location};

#[derive(Debug)]
pub struct ReferencesProvider {
    pub files: Arc<Mutex<Vec<SolidityAstFile>>>,
    pub base_path: RwLock<String>,
}

impl ReferencesProvider {

    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(vec![])),
            base_path: RwLock::new(String::new()),
        }
    }

    pub fn set_base_path(&self, base_path: String) {
        let mut r = self.base_path.write().unwrap();
        *r = base_path;
    }

    pub fn update_file_content(&self) -> Result<(), ReferencesError> {
        let new_files = extract_ast_from_foundry(&self.base_path.read().unwrap())?; // will always find the root foundry project
        let mut files = self.files.lock().unwrap();
        *files = new_files;
        Ok(())
    }

    fn while_inherits(&self, contract: &ContractDefinition, root_file: &SolidityAstFile, files: &Vec<SolidityAstFile>) -> Vec<CompletionItem> {
        let mut complete_finder = inheritence_finder::InheritenceFinder::new(contract.clone());
        let mut completes: Vec<CompletionItem> = vec![];
        let mut inheritences = vec![contract.clone()];
        
        while inheritences.len() > 0 {
            let current = inheritences.pop().unwrap();
            // info!("Current contract to search for inheritence: {:?}", current.name);
            for file in files {
                let (items, inheritences_res) = complete_finder.find(&file.ast, root_file.file.path == file.file.path, current.clone());
                completes.append(&mut items.clone());
                inheritences.append(&mut inheritences_res.clone());
            }
        } 
        completes
    }

    fn get_import_completes(&self, imports: Vec<ImportDirective>, files: &Vec<SolidityAstFile>) -> Vec<CompletionItem> {
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
        let files = import_finder.get_files_from_imports(files);
        for file in files {
            completes.append(&mut import_finder.find(&file.ast));
        }
        completes
    }

    pub fn get_scoped_completes(&self, uri: &str, position: Position) -> Vec<CompletionItem> {
        let files = self.files.lock().unwrap();
        if let Some(file) = files.iter().find(|file| file.file.path == uri) {

            let mut scope_finder = ScopeFinder::new(file.file.content.clone(), position);
            let (contract, spi, imports) = scope_finder.find(&file.ast);
            let mut completes: Vec<CompletionItem> = vec![];

            if let Some(contract) = contract {
                completes.append(&mut self.while_inherits(&contract, &file, &files));
            }

            let spi_finder = ScopedCompletionFinder::new(spi);
            completes.append(&mut spi_finder.inspect());

            completes.append(&mut self.get_import_completes(imports, &files));

            return completes;
        }
        vec![]
    }

    fn get_node(
        &self,
        uri: &str,
        position: Position,
        files: &Vec<SolidityAstFile>,
    ) -> Option<(SolidityAstFile, InteractableNode)> {
        let found_node: Option<InteractableNode>;
        let source_file;
        if let Some(file) = files.iter().find(|file| file.file.path == uri) {
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
        let files = self.files.lock().unwrap().clone();
        let (source_file, found_node) = match self.get_node(uri, position, &files) {
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
                        uri: join_path(&self.base_path.read().unwrap(), &import.absolute_path),
                    });
                }
                _ => {
                    return Some(get_location(&found_node, &source_file));
                }
            },
        };
        let mut def_finder = DefinitionFinder::new(ref_id);
        for file in files {
            if let Some(node) = def_finder.find(&file.ast) {
                return Some(get_location(&node, &file));
            }
        }
        None
    }

    pub fn get_references(&self, uri: &str, position: Position) -> Vec<Location> {
        let files = self.files.lock().unwrap().clone();
        let mut references: Vec<Location> = Vec::new();
        let (_, found_node) = match self.get_node(uri, position, &files) {
            Some((file, node)) => (file, node),
            None => return vec![],
        };

        let id = found_node.get_id();
        info!("Id: {:?}", id);

        let mut usages_finder = UsagesFinder::new(id);
        for file in files {
            let nodes = usages_finder.find(&file.ast);
            for node in nodes {
                references.push(get_location(&node, &file));
            }
        }
        references
    }
}
