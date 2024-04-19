use crate::references::{
    definition_visitor::DefinitionVisitor, position_node_visitor::PositionNodeVisitor,
    usage_visitor::UsageVisitor,
};
use crate::types::{InteractableNode, Location, Position};
use crate::utils::get_location;
use log::{info, warn};
use osmium_libs_solidity_ast_extractor::types::SolidityAstFile;
use osmium_libs_solidity_path_utils::join_path;

pub struct ReferenceProvider {}

impl ReferenceProvider {
    pub fn new() -> Self {
        Self {}
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
            let mut node_finder = PositionNodeVisitor::new(position.clone(), &file.file.content);
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

    pub fn get_definition(
        &self,
        uri: &str,
        position: Position,
        files: &Vec<SolidityAstFile>,
        base_path: &str,
    ) -> Option<Location> {
        let (source_file, found_node) = match self.get_node(uri, position, files) {
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
                        uri: join_path(base_path, &import.absolute_path),
                    });
                }
                _ => {
                    return Some(get_location(&found_node, &source_file));
                }
            },
        };
        let mut def_finder = DefinitionVisitor::new(ref_id);
        for file in files {
            if let Some(node) = def_finder.find(&file.ast) {
                return Some(get_location(&node, &file));
            }
        }
        None
    }

    pub fn get_references(
        &self,
        uri: &str,
        position: Position,
        files: &Vec<SolidityAstFile>,
    ) -> Vec<Location> {
        let mut references: Vec<Location> = Vec::new();
        let (_, found_node) = match self.get_node(uri, position, files) {
            Some((file, node)) => (file, node),
            None => return vec![],
        };

        let id = found_node.get_id();
        info!("Id: {:?}", id);

        let mut usages_finder = UsageVisitor::new(id);
        for file in files {
            let nodes = usages_finder.find(&file.ast);
            for node in nodes {
                references.push(get_location(&node, &file));
            }
        }
        references
    }
}
