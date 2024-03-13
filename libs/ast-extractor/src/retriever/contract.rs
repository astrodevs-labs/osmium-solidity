/**
 * contract.rs
 * Function to retrieve contract nodes from AST
 * author: 0xMemoryGrinder
*/
use syn_solidity::{ItemContract, Visit};

struct ContractVisitor {
    contracts: Vec<ItemContract>,
}

impl ContractVisitor {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for ContractVisitor {
    fn visit_item_contract(&mut self, i: &ItemContract) {
        self.contracts.push(i.clone());
        syn_solidity::visit::visit_item_contract(self, i);
    }
}

pub fn retrieve_contract_nodes(ast: &syn_solidity::File) -> Vec<ItemContract> {
    let mut visitor = ContractVisitor::new();
    visitor.visit_file(ast);
    visitor.contracts
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_contract_nodes_empty() {
        let source = String::from("pragma solidity ^0.8.0;");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_contract_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_contract_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("contracts");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_contract_nodes(&ast);
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_retrieve_contract_nodes_two() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("contracts");
        path.push("two.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_contract_nodes(&ast);
        assert_eq!(res.len(), 2);
    }
}
