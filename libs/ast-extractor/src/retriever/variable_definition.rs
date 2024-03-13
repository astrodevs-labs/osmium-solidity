/**
 * variable.rs
 * Function to retrieve variable definition nodes from AST
 * author: Leon
*/
use syn_solidity::{VariableDefinition, Visit};

struct VariableDefinitionVisitor {
    variables: Vec<VariableDefinition>,
}

impl VariableDefinitionVisitor {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for VariableDefinitionVisitor {
    fn visit_variable_definition(&mut self, i: &VariableDefinition) {
        self.variables.push(i.clone());
        syn_solidity::visit::visit_variable_definition(self, i);
    }
}

pub fn retrieve_variable_definition_nodes(ast: &syn_solidity::File) -> Vec<VariableDefinition> {
    let mut visitor = VariableDefinitionVisitor::new();
    visitor.visit_file(ast);
    visitor.variables
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_variable_definition_nodes_empty() {
        let source = String::from("pragma solidity ^0.8.0;");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_variable_definition_nodes(&ast);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_retrieve_variable_definition_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("variables_definition");
        path.push("file.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_variable_definition_nodes(&ast);
        assert_eq!(res.len(), 1);
    }
}
