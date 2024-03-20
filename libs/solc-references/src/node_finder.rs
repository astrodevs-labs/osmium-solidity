use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;
use crate::types::{InteractableNode, Position};
use crate::utils::*;

pub struct NodeVisitor {
    position: Position,
    pub node: Option<InteractableNode>,
    above_node: Option<InteractableNode>,
    source: String
}

impl <'ast> Visit<'ast> for NodeVisitor {
    fn visit_user_defined_type_name(&mut self,_udt: &'ast UserDefinedTypeName) {
        if is_node_in_range(&_udt.src, &self.position, &self.source) {
            self.node = Some(InteractableNode::UserDefinedTypeName(_udt.clone()));
        }
        self.above_node = Some(InteractableNode::UserDefinedTypeName(_udt.clone()));
        visit::visit_user_defined_type_name(self, _udt);
    }
    fn visit_contract_definition(&mut self,contract: &'ast ContractDefinition) {
        if is_node_in_range(&contract.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ContractDefinition(contract.clone()));
        }
        visit::visit_contract_definition(self, contract);
    }

    fn visit_elementary_type_name(&mut self,_elementary: &'ast ElementaryTypeName) {
        if is_node_in_range(&_elementary.src, &self.position, &self.source) {
            self.node = None;
        }
    }

    fn visit_function_definition(&mut self, function: &'ast FunctionDefinition) {
        if is_node_in_range(&function.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::FunctionDefinition(function.clone()));
        }
        visit::visit_function_definition(self, function);
    }

    fn visit_modifier_definition(&mut self, modifier: &'ast ModifierDefinition) {
        if is_node_in_range(&modifier.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ModifierDefinition(modifier.clone()));
        }
        visit::visit_modifier_definition(self, modifier);
    }

    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if is_node_in_range(&struct_def.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::StructDefinition(struct_def.clone()));
        }
        visit::visit_struct_definition(self, struct_def);
    }

    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        if is_node_in_range(&enum_def.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::EnumDefinition(enum_def.clone()));
        }
        visit::visit_enum_definition(self, enum_def);
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if is_node_in_range(&variable.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::VariableDeclaration(variable.clone()));
        }
        visit::visit_variable_declaration(self, variable);
    }

    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        if is_node_in_range(&event.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::EventDefinition(event.clone()));
        }
        visit::visit_event_definition(self, event);
    }

    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        if is_node_in_range(&enum_value.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::EnumValue(enum_value.clone()));
        }
        visit::visit_enum_value(self, enum_value);
    }

    fn visit_using_for_directive(&mut self, using_for: &'ast UsingForDirective) {
        if is_node_in_range(&using_for.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::UsingForDirective(using_for.clone()));
        }
        visit::visit_using_for_directive(self, using_for);
    }

    fn visit_import_directive(&mut self, import: &'ast ImportDirective) {
        if is_node_in_range(&import.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ImportDirective(import.clone()));
        }
        visit::visit_import_directive(self, import);
    }

    fn visit_error_definition(&mut self, error: &'ast ErrorDefinition) {
        if is_node_in_range(&error.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ErrorDefinition(error.clone()));
        }
        visit::visit_error_definition(self, error);
    }

    fn visit_function_call(&mut self, function_call: &'ast FunctionCall) {
        if is_node_in_range(&function_call.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::FunctionCall(function_call.clone()));
        }
        visit::visit_function_call(self, function_call);
    }

    fn visit_modifier_invocation(&mut self, modifier_invocation: &'ast ModifierInvocation) {
        if is_node_in_range(&modifier_invocation.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::ModifierInvocation(modifier_invocation.clone()));
        }
        visit::visit_modifier_invocation(self, modifier_invocation);
    }

    fn visit_inheritance_specifier(&mut self, inheritance_specifier: &'ast InheritanceSpecifier) {
        if is_node_in_range(&inheritance_specifier.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::InheritanceSpecifier(inheritance_specifier.clone()));
        }
        visit::visit_inheritance_specifier(self, inheritance_specifier);
    }

    fn visit_identifier(&mut self, identifier: &'ast Identifier) {
        if is_node_in_range(&identifier.src, &self.position, &self.source) {
            eprintln!("Identifier in range: {:?}", identifier);
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::Identifier(identifier.clone()));
        }
        visit::visit_identifier(self, identifier);
    }

    fn visit_member_access(&mut self, member_access: &'ast MemberAccess) {
        if is_node_in_range(&member_access.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::MemberAccess(member_access.clone()));
        }
        visit::visit_member_access(self, member_access);
    }

    fn visit_new(&mut self, new_expression: &'ast NewExpression) {
        if is_node_in_range(&new_expression.src, &self.position, &self.source) {
            self.above_node = self.node.clone();
            self.node = Some(InteractableNode::NewExpression(new_expression.clone(), Box::new(self.above_node.clone().unwrap())));
        }
        visit::visit_new(self, new_expression);
    }
}


impl NodeVisitor {
    pub fn new(position: Position, source: &String) -> Self {
        NodeVisitor {
            position,
            node: None,
            above_node: None,
            source: source.clone()
        }
    }
    pub fn find(&mut self, src: &SourceUnit) -> Option<InteractableNode> {
        self.visit_source_unit(src);
        //eprintln!("[NODE FINDER] Found node: {:?}", self.node);
        self.node.clone()
    }
}