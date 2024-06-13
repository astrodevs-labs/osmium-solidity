use crate::types::InteractableNode;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct UsageVisitor {
    pub id: i64,
    pub to_find: Vec<InteractableNode>,
    above_node: Option<InteractableNode>,
}

impl<'ast> Visit<'ast> for UsageVisitor {
    fn visit_type_descriptions(&mut self, type_descriptions: &'ast TypeDescriptions) {
        if self.above_node.clone().is_some_and(|node| {
            !matches!(
                node,
                InteractableNode::VariableDeclaration(_) | InteractableNode::ImportDirective(_)
            )
        }) {
            return;
        }
        if let Some(ident) = &type_descriptions.type_identifier {
            if let Some(str_id) = ident.split('$').collect::<Vec<&str>>().get(2) {
                if Ok(self.id) == str_id.parse::<i64>() {
                    self.to_find.push(self.above_node.clone().unwrap());
                }
            }
        }
    }

    fn visit_identifier_path(&mut self, _path: &'ast IdentifierPath) {
        if _path.referenced_declaration == self.id {
            self.to_find
                .push(InteractableNode::IdentifierPath(_path.clone()));
        }
        visit::visit_identifier_path(self, _path);
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        self.above_node = Some(InteractableNode::VariableDeclaration(variable.clone()));
        if let Some(type_name) = &variable.type_name {
            visit::visit_type_name(self, type_name);
        }
        if let Some(value) = &variable.value {
            visit::visit_expression(self, value);
        }
    }
    fn visit_import_directive(&mut self, import: &'ast ImportDirective) {
        self.above_node = Some(InteractableNode::ImportDirective(import.clone()));
        visit::visit_import_directive(self, import);
    }

    fn visit_user_defined_type_name(&mut self, _udt: &'ast UserDefinedTypeName) {
        if _udt.referenced_declaration == self.id {
            self.to_find
                .push(InteractableNode::UserDefinedTypeName(_udt.clone()));
        }
        self.above_node = Some(InteractableNode::UserDefinedTypeName(_udt.clone()));
        visit::visit_user_defined_type_name(self, _udt);
    }

    fn visit_identifier(&mut self, ident: &'ast Identifier) {
        let node = InteractableNode::Identifier(ident.clone());
        if ident.referenced_declaration.is_some_and(|id| self.id == id) {
            self.to_find.push(node.clone());
        }
        self.above_node = Some(node.clone());
        visit::visit_identifier(self, ident);
    }

    fn visit_member_access(&mut self, member: &'ast MemberAccess) {
        let node = InteractableNode::MemberAccess(member.clone());
        if member
            .referenced_declaration
            .is_some_and(|id| self.id == id)
        {
            self.to_find.push(node.clone());
        }
        self.above_node = Some(node.clone());
        visit::visit_member_access(self, member);
    }
}

impl UsageVisitor {
    pub fn new(id: i64) -> Self {
        UsageVisitor {
            id,
            to_find: Vec::new(),
            above_node: None,
        }
    }

    pub fn find(&mut self, ast: &SourceUnit) -> Vec<InteractableNode> {
        self.to_find = Vec::new();
        self.above_node = None;
        //eprintln!("[USAGES FINDER] Finding usages inf file: {}", ast.id);
        self.visit_source_unit(ast);
        /*
        if self.to_find.is_empty() {
            eprintln!("[USAGES FINDER] No usages found for node with id: {}", self.id);
        }
        */
        self.to_find.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::InteractableNode;
    use crate::test_utils::{create_test_ast_file_identifier, create_test_ast_file_identifier_path, create_test_ast_file_import_directive, create_test_ast_file_member_access, create_test_ast_file_user_defined_type_name};

    #[test]
    fn test_find_usages_identifier() {
        let file = create_test_ast_file_identifier();
        let id = 30;
        let mut visitor = UsageVisitor::new(id);
        let usages = visitor.find(&file.ast);
        assert_eq!(usages.len(), 2);
        if let InteractableNode::Identifier(identifier) = &usages[0] {
            assert_eq!(identifier.referenced_declaration, Some(id));
            assert_eq!(identifier.name, "number");
        } else {
            panic!("Expected IdentifierPath, got: {:?}", usages[0]);
        }
    }

    #[test]
    fn test_find_usages_identifier_path() {
        let file = create_test_ast_file_identifier_path();
        let id = 15;
        let mut visitor = UsageVisitor::new(id);
        let usages = visitor.find(&file.ast);
        assert_eq!(usages.len(), 1);
        if let InteractableNode::IdentifierPath(identifier_path) = &usages[0] {
            assert_eq!(identifier_path.referenced_declaration, id);
            assert_eq!(identifier_path.name, "IdPath");
        } else {
            panic!("Expected IdentifierPath, got: {:?}", usages[0]);
        }
    }

    #[test]
    fn test_find_usages_with_imports() {
        let file = create_test_ast_file_import_directive();
        let id = -1;
        let mut visitor = UsageVisitor::new(id);
        let usages = visitor.find(&file.ast);
        assert_eq!(usages.len(), 0);
    }

    #[test]
    fn test_find_usages_user_defined_type_name() {
        let file = create_test_ast_file_user_defined_type_name();
        let id = 5;
        let mut visitor = UsageVisitor::new(id);
        let usages = visitor.find(&file.ast);
        assert_eq!(usages.len(), 1);
        if let InteractableNode::UserDefinedTypeName(udt) = &usages[0] {
            assert_eq!(udt.referenced_declaration, id);
        } else {
            panic!("Expected UserDefinedTypeName, got: {:?}", usages[0]);
        }
    }

    #[test]
    fn test_find_usages_member_access() {
        let file = create_test_ast_file_member_access();
        let id = 3;
        let mut visitor = UsageVisitor::new(id);
        let usages = visitor.find(&file.ast);
        eprintln!("{:?}", usages);
        assert_eq!(usages.len(), 1);
        if let InteractableNode::MemberAccess(member) = &usages[0] {
            assert_eq!(member.referenced_declaration, Some(id));
        } else {
            panic!("Expected MemberAccess, got: {:?}", usages[0]);
        }
    }
}
