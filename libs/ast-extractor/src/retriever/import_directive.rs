/**
 * import_directive.rs
 * Function to retrieve import directive from AST
 * author: EnergyCube
*/
use syn_solidity::{ImportDirective, Visit};

struct HeaderVisitor {
    import: Vec<ImportDirective>,
}

impl HeaderVisitor {
    pub fn new() -> Self {
        Self { import: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for HeaderVisitor {
    fn visit_import_directive(&mut self, i: &ImportDirective) {
        self.import.push(i.clone());
        syn_solidity::visit::visit_import_directive(self, i);
    }
}

pub fn retrieve_import_directive_nodes(ast: &syn_solidity::File) -> Vec<ImportDirective> {
    let mut visitor = HeaderVisitor::new();
    visitor.visit_file(ast);
    visitor.import
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_header_nodes_empty() {
        let source = String::from("pragma solidity ^0.8.0;");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_import_directive_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_header_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("header");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_import_directive_nodes(&ast);
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_retrieve_header_nodes_ten() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("header");
        path.push("ten.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_import_directive_nodes(&ast);
        assert_eq!(res.len(), 10);
    }
}
