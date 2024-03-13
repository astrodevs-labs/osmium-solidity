/**
 * event.rs
 * Functions to retrieve event nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemEvent, Visit};

struct EventVisitor {
    contract_events: Vec<ItemEvent>,
    file_events: Vec<ItemEvent>,
    inside_contract: bool,
}

impl EventVisitor {
    pub fn new() -> Self {
        Self {
            contract_events: Vec::new(),
            file_events: Vec::new(),
            inside_contract: false,
        }
    }
}

impl<'ast> Visit<'ast> for EventVisitor {
    fn visit_item_contract(&mut self, i: &syn_solidity::ItemContract) {
        self.inside_contract = true;
        syn_solidity::visit::visit_item_contract(self, i);
        self.inside_contract = false;
    }

    fn visit_item_event(&mut self, i: &ItemEvent) {
        if self.inside_contract {
            self.contract_events.push(i.clone());
        } else {
            self.file_events.push(i.clone());
        }
        syn_solidity::visit::visit_item_event(self, i);
    }
}

pub fn retrieve_events_contract_nodes(ast: &syn_solidity::ItemContract) -> Vec<ItemEvent> {
    let mut visitor = EventVisitor::new();
    visitor.visit_item_contract(ast);
    visitor.contract_events
}

pub fn retrieve_events_file_nodes(ast: &syn_solidity::File) -> Vec<ItemEvent> {
    let mut visitor = EventVisitor::new();
    visitor.visit_file(ast);
    visitor.file_events
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn_solidity::Item;

    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_event_contract_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("event");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_events_contract_nodes(&contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have event");
        }
    }

    #[test]
    fn test_retrieve_event_contract_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("event");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast
            .items
            .iter()
            .find(|i| matches!(i, Item::Contract(_)))
            .unwrap()
            .clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_events_contract_nodes(&contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a event");
        }
    }

    #[test]
    fn test_retrieve_event_file_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("event");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();

        let res = retrieve_events_file_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_event_file_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("event");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();

        let res = retrieve_events_file_nodes(&ast);
        assert_eq!(res.len(), 1);
    }
}
