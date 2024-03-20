mod types;
mod error;
mod definitions;
mod usages;
mod node_finder;
mod utils;

use error::ReferencesError;
use node_finder::NodeVisitor;
use solc_wrapper::{get_ast_for_file, SolcAstFile};
pub use solc_wrapper::SolcFile;
pub use solc_ast_rs_types::types::*;
pub use types::{Location, Position};
use types::InteractableNode;
use usages::UsagesFinder;
use definitions::DefinitionFinder;
use utils::join_path;

use crate::utils::get_location;

#[derive(Debug)]
pub struct ReferencesProvider {
    pub files: Vec<SolcAstFile>,
    pub base_path: String
}

impl ReferencesProvider {
    pub fn set_base_path(&mut self, base_path: String) {
        self.base_path = base_path;
    }

    pub fn update_file_content(&mut self) -> Result<(), ReferencesError> {
        self.files = get_ast_for_file(self.base_path.clone())?;
        Ok(())
    }

    fn get_node(&self, uri: &str, position: Position) -> Option<(SolcAstFile, InteractableNode)> {
        let found_node: Option<InteractableNode>;
        let source_file;
        if let Some(file) = self.files.iter().find(|file| file.file.path == uri) {
            let mut node_finder = NodeVisitor::new(position.clone(), &file.file.content);
            source_file = file;
            found_node = node_finder.find(&file.ast);
        }
        else {
            eprintln!("No file found at uri: {}", uri);
            return None;
        }
        if found_node.is_none() {
            eprintln!("[NODE FINDER] No node found at position: {:?}", &position);
            return None;
        }
        Some((source_file.clone(), found_node.unwrap()))
    }

    pub fn get_definition(&self, uri: &str, position: Position) -> Option<Location> {
        let (source_file, found_node) = match self.get_node(uri, position) {
            Some((file, node)) => (file, node),
            None => return None
        };

        let ref_id = match found_node.get_reference_id() {
            Some(id) => id,
            None => {
                match found_node {
                    InteractableNode::ImportDirective(import) => {
                        return Some(Location {
                            start: Position::default(),
                            end: Position::default(),
                            uri: join_path(&self.base_path, &import.absolute_path)
                        });
                    },
                    _ => {
                        return Some(get_location(&found_node, &source_file));
                    }
                }
            }
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
            None => return vec![]
        };

        let id = found_node.get_id();
        eprintln!("Id: {:?}", id);

        let mut usages_finder = UsagesFinder::new(id);
        for file in &self.files {
            let nodes = usages_finder.find(&file.ast);
            for node in nodes {
                references.push(get_location(&node, &file));
            }
        }
        references
    }
}