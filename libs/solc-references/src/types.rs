use solc_ast_rs_types::types::*;
use crate::utils::source_location_to_range;

#[derive(Debug, Clone)]
pub struct Position {
    pub line: u32,
    pub column: u32
}

impl Position {
    pub fn default() -> Position {
        Position { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    pub index: u32,
    pub length: u32
}

#[derive(Debug, Clone)]
pub struct Location {
    pub start: Position,
    pub end: Position,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub enum InteractableNode {
    ContractDefinition(ContractDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    EnumValue(EnumValue),
    UsingForDirective(UsingForDirective),
    ImportDirective(ImportDirective),
    ErrorDefinition(ErrorDefinition),
    FunctionCall(FunctionCall),
    ModifierInvocation(ModifierInvocation),
    InheritanceSpecifier(InheritanceSpecifier),
    Identifier(Identifier),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression, Box<InteractableNode>),
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}
impl InteractableNode {
    pub fn get_id(&self) -> i64 {
        match self {
            InteractableNode::ContractDefinition(node) => node.id,
            InteractableNode::FunctionDefinition(node) => node.id,
            InteractableNode::ModifierDefinition(node) => node.id,
            InteractableNode::StructDefinition(node) => node.id,
            InteractableNode::EnumDefinition(node) => node.id,
            InteractableNode::VariableDeclaration(node) => node.id,
            InteractableNode::EventDefinition(node) => node.id,
            InteractableNode::EnumValue(node) => node.id,
            InteractableNode::UsingForDirective(node) => node.id,
            InteractableNode::ImportDirective(node) => node.id,
            InteractableNode::ErrorDefinition(node) => node.id,
            InteractableNode::FunctionCall(node) => node.id,
            InteractableNode::ModifierInvocation(node) => node.id,
            InteractableNode::InheritanceSpecifier(node) => node.id,
            InteractableNode::Identifier(node) => node.id,
            InteractableNode::MemberAccess(node) => node.id,
            InteractableNode::NewExpression(node, _) => node.id,
            InteractableNode::UserDefinedTypeName(udt) => udt.id,
            InteractableNode::IdentifierPath(ip) => ip.id,
        }
    }

    pub fn get_reference_id(&self) -> Option<i64> {
        match self {
            InteractableNode::FunctionDefinition(node) => {
                if let Some(overrides) = &node.overrides {
                    match &overrides.overrides {
                        OverrideSpecifierOverrides::UserDefinedTypeNames(names) => {
                            if names.len() > 0 {
                                return Some(names[0].referenced_declaration);
                            }
                        }
                        OverrideSpecifierOverrides::IdentifierPaths(paths) => {
                            if paths.len() > 0 {
                                return Some(paths[0].referenced_declaration);
                            }
                        }
                    }
                }
                None
            },
            InteractableNode::ModifierDefinition(node) => {
                if let Some(overrides) = &node.overrides {
                    match &overrides.overrides {
                        OverrideSpecifierOverrides::UserDefinedTypeNames(names) => {
                            if names.len() > 0 {
                                return Some(names[0].referenced_declaration);
                            }
                        }
                        OverrideSpecifierOverrides::IdentifierPaths(paths) => {
                            if paths.len() > 0 {
                                return Some(paths[0].referenced_declaration);
                            }
                        }
                    
                    }
                }
                None
            },
            InteractableNode::VariableDeclaration(node) => {
                if let Some(overrides) = &node.overrides {
                    match &overrides.overrides {
                        OverrideSpecifierOverrides::UserDefinedTypeNames(names) => {
                            if names.len() > 0 {
                                return Some(names[0].referenced_declaration);
                            }
                        }
                        OverrideSpecifierOverrides::IdentifierPaths(paths) => {
                            if paths.len() > 0 {
                                return Some(paths[0].referenced_declaration);
                            }
                        }
                    
                    }
                }
                None
            },
            InteractableNode::UsingForDirective(node) => {
                if let Some(overrides) = &node.library_name {
                    match overrides {
                        UsingForDirectiveLibraryName::UserDefinedTypeName(names) => {
                            return Some(names.referenced_declaration);
                        }
                        UsingForDirectiveLibraryName::IdentifierPath(paths) => {
                            return Some(paths.referenced_declaration);
                        }
                    
                    }
                }
                None
            },
            InteractableNode::InheritanceSpecifier(node) => {
                match &node.base_name {
                    InheritanceSpecifierBaseName::UserDefinedTypeName(names) => {
                        return Some(names.referenced_declaration);
                    }
                    InheritanceSpecifierBaseName::IdentifierPath(paths) => {
                        return Some(paths.referenced_declaration);
                    }
                
                }
            },
            InteractableNode::UserDefinedTypeName(node) => {
                return Some(node.referenced_declaration);
            },
            InteractableNode::Identifier(node) => node.referenced_declaration,
            InteractableNode::MemberAccess(node) => node.referenced_declaration,
            _ => None,
        }
    }


    pub fn get_range(&self) -> Range {
        match self {
            InteractableNode::ContractDefinition(node) => source_location_to_range(
                    node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::FunctionDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::ModifierDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::StructDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::EnumDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::VariableDeclaration(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::EventDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::EnumValue(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned())
            ),
            InteractableNode::ErrorDefinition(node) => source_location_to_range(
                &node.name_location
            ),
            InteractableNode::UsingForDirective(node) => source_location_to_range(&node.src),
            InteractableNode::ImportDirective(node) => source_location_to_range(&node.src),
            InteractableNode::FunctionCall(node) => source_location_to_range(&node.src),
            InteractableNode::ModifierInvocation(node) => source_location_to_range(&node.src),
            InteractableNode::InheritanceSpecifier(node) => source_location_to_range(&node.src),
            InteractableNode::Identifier(node) => source_location_to_range(&node.src),
            InteractableNode::MemberAccess(node) => source_location_to_range(&node.src),
            InteractableNode::NewExpression(node, _) => source_location_to_range(&node.src),
            InteractableNode::UserDefinedTypeName(udt) => source_location_to_range(&udt.src),
            InteractableNode::IdentifierPath(ip) => source_location_to_range(&ip.src),
        }
    }
}