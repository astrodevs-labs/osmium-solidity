mod definitions;
mod error;
mod node_finder;
mod types;
mod usages;
mod utils;
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

    pub fn get_scoped_completes(&self, uri: &str, position: Position) -> Vec<CompletionItem> {
        if let Some(file) = self.files.iter().find(|file| file.file.path == uri) {
            let scope_finder = ScopeFinder::new(file.file.content.clone(), position);
            /*let mut complete_finder = inheritence_finder::InheritenceFinder::new(
                scope_finder.scope,
                scope_finder.root_scope,
                scope_finder.parent_scopes,
            );
            let mut completes: Vec<InteractableNode> = vec![];
            for file in &self.files {
                let src = &file.ast;
                let is_self = uri.contains(&file.file.path);
                completes.append(&mut complete_finder.find(src, is_self));
            }
            return completes
                .iter()
                .map(|node| CompletionItem {
                    label: node.get_name(),
                    kind: node.get_kind(),
                } )
                .collect();
        */
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
