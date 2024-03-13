/**
 * using.rs
 * Functions to retrieve using nodes from contract AST
 * author: Leon
*/
use syn_solidity::{UsingDirective, Visit};

struct UsingVisitor {
    usings: Vec<UsingDirective>,
}

impl UsingVisitor {
    pub fn new() -> Self {
        Self { usings: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for UsingVisitor {
    fn visit_using_directive(&mut self, i: &UsingDirective) {
        self.usings.push(i.clone());
        syn_solidity::visit::visit_using_directive(self, i);
    }
}

pub fn retrieve_usings_nodes(ast: &syn_solidity::ItemContract) -> Vec<UsingDirective> {
    let mut visitor = UsingVisitor::new();
    visitor.visit_item_contract(ast);
    visitor.usings
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
    fn test_retrieve_using_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("using");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_usings_nodes(&contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have any using directive");
        }
    }

    #[test]
    fn test_retrieve_using_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("using");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast
            .items
            .iter()
            .find(|i| match i {
                Item::Contract(ctr) => ctr.name == "Wallet",
                _ => false,
            })
            .unwrap()
            .clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_usings_nodes(&contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a a using directive");
        }
    }
}
