/**
 * enum.rs
 * Functions to retrieve enum nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemEnum, Visit};

struct EnumVisitor {
    contract_enums: Vec<ItemEnum>,
    file_enums: Vec<ItemEnum>,
    inside_contract: bool,
}

impl EnumVisitor {
    pub fn new() -> Self {
        Self {
            contract_enums: Vec::new(),
            file_enums: Vec::new(),
            inside_contract: false,
        }
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_item_contract(&mut self, i: &syn_solidity::ItemContract) {
        self.inside_contract = true;
        syn_solidity::visit::visit_item_contract(self, i);
        self.inside_contract = false;
    }

    fn visit_item_enum(&mut self, i: &ItemEnum) {
        if self.inside_contract {
            self.contract_enums.push(i.clone());
        } else {
            self.file_enums.push(i.clone());
        }
        syn_solidity::visit::visit_item_enum(self, i);
    }
}

pub fn retrieve_enums_contract_nodes(ast: &syn_solidity::ItemContract) -> Vec<ItemEnum> {
    let mut visitor = EnumVisitor::new();
    visitor.visit_item_contract(ast);
    visitor.contract_enums
}

pub fn retrieve_enums_file_nodes(ast: &syn_solidity::File) -> Vec<ItemEnum> {
    let mut visitor = EnumVisitor::new();
    visitor.visit_file(ast);
    visitor.file_enums
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
    fn test_retrieve_enum_contract_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("enums");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_enums_contract_nodes(&contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item shouldn't have enum");
        }
    }

    #[test]
    fn test_retrieve_enum_contract_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("enums");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let contract = ast
            .items
            .iter()
            .find(|i| matches!(i, Item::Contract(_)))
            .unwrap()
            .clone();

        if let Item::Contract(contract) = contract {
            let res = retrieve_enums_contract_nodes(&contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a contract");
        }
    }

    #[test]
    fn test_retrieve_enum_file_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("enums");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();

        let res = retrieve_enums_file_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_enum_file_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("enums");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();

        let res = retrieve_enums_file_nodes(&ast);
        assert_eq!(res.len(), 1);
    }
}
