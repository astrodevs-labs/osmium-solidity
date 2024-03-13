/**
 * error.rs
 * Functions to retrieve error nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemError, Visit};

struct ErrorVisitor {
    errors: Vec<ItemError>,
}

impl ErrorVisitor {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ErrorVisitor {
    fn visit_item_error(&mut self, i: &ItemError) {
        self.errors.push(i.clone());
        syn_solidity::visit::visit_item_error(self, i);
    }
}

pub fn retrieve_errors_nodes(ast: &syn_solidity::ItemContract) -> Vec<ItemError> {
    let mut visitor = ErrorVisitor::new();
    visitor.visit_item_contract(ast);
    visitor.errors
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
    fn test_retrieve_error_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("errors");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_errors_nodes(&contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have error");
        }
    }

    #[test]
    fn test_retrieve_error_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("errors");
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
            let res = retrieve_errors_nodes(&contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a contract");
        }
    }
}
