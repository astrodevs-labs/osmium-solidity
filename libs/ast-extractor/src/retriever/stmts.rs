/**
 * contract.rs
 * Function to retrieve statements nodes from contract AST
 * author: 0xtekgrinder
*/
use syn_solidity::{Stmt, Visit};

struct SmtsVisitor {
    stmts: Vec<Stmt>,
}

impl SmtsVisitor {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for SmtsVisitor {
    fn visit_stmt(&mut self, i: &Stmt) {
        self.stmts.push(i.clone());
        syn_solidity::visit::visit_stmt(self, i);
    }
}

pub fn retrieve_stmts_nodes(ast: &syn_solidity::ItemContract) -> Vec<Stmt> {
    let mut visitor = SmtsVisitor::new();
    visitor.visit_item_contract(ast);
    visitor.stmts
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
    fn test_retrieve_stmts_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("stmts");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_stmts_nodes(&contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item is not a contract");
        }
    }

    #[test]
    fn test_retrieve_stmts_nodes_complex() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("stmts");
        path.push("complex.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_stmts_nodes(&contract);
            assert_eq!(res.len(), 7);
        } else {
            panic!("Item is not a contract");
        }
    }
}
