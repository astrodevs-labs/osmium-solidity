/**
 * struct.rs
 * Functions to retrieve struct nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemStruct, Visit};

struct StructVisitor {
    contract_structs: Vec<ItemStruct>,
    file_structs: Vec<ItemStruct>,
    inside_contract: bool,
}

impl StructVisitor {
    pub fn new() -> Self {
        Self {
            contract_structs: Vec::new(),
            file_structs: Vec::new(),
            inside_contract: false,
        }
    }
}

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_item_contract(&mut self, i: &syn_solidity::ItemContract) {
        self.inside_contract = true;
        syn_solidity::visit::visit_item_contract(self, i);
        self.inside_contract = false;
    }
    fn visit_item_struct(&mut self, i: &ItemStruct) {
        if self.inside_contract {
            self.contract_structs.push(i.clone());
        } else {
            self.file_structs.push(i.clone());
        }
        syn_solidity::visit::visit_item_struct(self, i);
    }
}

pub fn retrieve_structs_contract_nodes(ast: &syn_solidity::ItemContract) -> Vec<ItemStruct> {
    let mut visitor = StructVisitor::new();
    visitor.visit_item_contract(ast);
    visitor.contract_structs
}

pub fn retrieve_structs_file_nodes(ast: &syn_solidity::File) -> Vec<ItemStruct> {
    let mut visitor = StructVisitor::new();
    visitor.visit_file(ast);
    visitor.file_structs
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
    fn test_retrieve_struct_contract_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("structs");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_structs_contract_nodes(&contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have struct");
        }
    }

    #[test]
    fn test_retrieve_struct_contract_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("structs");
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
            let res = retrieve_structs_contract_nodes(&contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a struct");
        }
    }

    #[test]
    fn test_retrieve_struct_file_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("structs");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();

        let res = retrieve_structs_file_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_struct_file_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("structs");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();

        let res = retrieve_structs_file_nodes(&ast);
        assert_eq!(res.len(), 1);
    }
}
