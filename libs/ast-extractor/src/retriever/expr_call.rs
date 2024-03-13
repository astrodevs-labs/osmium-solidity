/**
 * expr_call.rs
 * Function to retrieve expr calls from AST
 * author: EnergyCube
 *
 * !!! UNTESTED !!!
*/
use syn_solidity::{ExprCall, Visit};

struct CallVisitor {
    calls: Vec<ExprCall>,
}

impl CallVisitor {
    pub fn new() -> Self {
        Self { calls: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for CallVisitor {
    fn visit_expr_call(&mut self, i: &ExprCall) {
        self.calls.push(i.clone());
        syn_solidity::visit::visit_expr_call(self, i);
    }
}

pub fn retrieve_expr_call_nodes(ast: &syn_solidity::File) -> Vec<ExprCall> {
    let mut visitor = CallVisitor::new();
    visitor.visit_file(ast);
    visitor.calls
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_expr_call_nodes_empty() {
        let source = String::from("pragma solidity ^0.8.0;");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_expr_call_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_expr_call_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("expr_calls");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_expr_call_nodes(&ast);
        assert_eq!(res.len(), 1);
    }
}
