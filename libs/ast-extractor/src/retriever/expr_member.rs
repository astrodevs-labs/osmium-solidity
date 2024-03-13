/**
 * expr_member.rs
 * Function to retrieve expr memners from AST
 * author: EnergyCube
 *
 * !!! UNTESTED !!!
*/
use syn_solidity::{ExprMember, Visit};

struct MemberVisitor {
    members: Vec<ExprMember>,
}

impl MemberVisitor {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for MemberVisitor {
    fn visit_expr_member(&mut self, i: &ExprMember) {
        self.members.push(i.clone());
        syn_solidity::visit::visit_expr_member(self, i);
    }
}

pub fn retrieve_expr_member_nodes(ast: &syn_solidity::File) -> Vec<ExprMember> {
    let mut visitor = MemberVisitor::new();
    visitor.visit_file(ast);
    visitor.members
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_expr_member_nodes_empty() {
        let source = String::from("pragma solidity ^0.8.0;");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_expr_member_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_expr_member_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("expr_members");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_expr_member_nodes(&ast);
        assert_eq!(res.len(), 1);
    }
}
