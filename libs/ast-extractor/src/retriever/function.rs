/**
 * contract.rs
 * Function to retrieve function nodes from contract AST
 * author: 0xMemoryGrinder
*/
use syn_solidity::{ItemFunction, Visit};

struct FunctionVisitor {
    functions: Vec<ItemFunction>,
}

impl FunctionVisitor {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_function(&mut self, i: &ItemFunction) {
        self.functions.push(i.clone());
        syn_solidity::visit::visit_item_function(self, i);
    }
}

pub fn retrieve_functions_nodes(ast: &syn_solidity::ItemContract) -> Vec<ItemFunction> {
    let mut visitor = FunctionVisitor::new();
    visitor.visit_item_contract(ast);
    visitor.functions
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
    fn test_retrieve_function_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("functions");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_functions_nodes(&contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item is not a contract");
        }
    }

    #[test]
    fn test_retrieve_function_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("functions");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_functions_nodes(&contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item is not a contract");
        }
    }

    #[test]
    fn test_retrieve_function_nodes_with_modifier() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("functions");
        path.push("modifier.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_functions_nodes(&contract);
            assert_eq!(res.len(), 2);
        } else {
            panic!("Item is not a contract");
        }
    }
}
