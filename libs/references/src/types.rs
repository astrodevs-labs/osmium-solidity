use crate::utils::source_location_to_range;
use solc_ast_rs_types::types::*;

#[derive(Debug, Clone)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Default for Position {
    fn default() -> Position {
        Position { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    pub index: u32,
    pub length: u32,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub start: Position,
    pub end: Position,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct CompletionItemKind(i64);

#[derive(Debug, Clone)]
pub struct CompletionItem {

    pub label: String,

    pub kind: CompletionItemKind,

    // TODO
    // pub documentation: Option<Documentation>,

}

impl CompletionItemKind {
    pub const TEXT: CompletionItemKind = CompletionItemKind(1);
    pub const METHOD: CompletionItemKind = CompletionItemKind(2);
    pub const FUNCTION: CompletionItemKind = CompletionItemKind(3);
    pub const CONSTRUCTOR: CompletionItemKind = CompletionItemKind(4);
    pub const FIELD: CompletionItemKind = CompletionItemKind(5);
    pub const VARIABLE: CompletionItemKind = CompletionItemKind(6);
    pub const CLASS: CompletionItemKind = CompletionItemKind(7);
    pub const INTERFACE: CompletionItemKind = CompletionItemKind(8);
    pub const MODULE: CompletionItemKind = CompletionItemKind(9);
    pub const PROPERTY: CompletionItemKind = CompletionItemKind(10);
    pub const UNIT: CompletionItemKind = CompletionItemKind(11);
    pub const VALUE: CompletionItemKind = CompletionItemKind(12);
    pub const ENUM: CompletionItemKind = CompletionItemKind(13);
    pub const KEYWORD: CompletionItemKind = CompletionItemKind(14);
    pub const SNIPPET: CompletionItemKind = CompletionItemKind(15);
    pub const COLOR: CompletionItemKind = CompletionItemKind(16);
    pub const FILE: CompletionItemKind = CompletionItemKind(17);
    pub const REFERENCE: CompletionItemKind = CompletionItemKind(18);
    pub const FOLDER: CompletionItemKind = CompletionItemKind(19);
    pub const ENUM_MEMBER: CompletionItemKind = CompletionItemKind(20);
    pub const CONSTANT: CompletionItemKind = CompletionItemKind(21);
    pub const STRUCT: CompletionItemKind = CompletionItemKind(22);
    pub const EVENT: CompletionItemKind = CompletionItemKind(23);
    pub const OPERATOR: CompletionItemKind = CompletionItemKind(24);
    pub const TYPE_PARAMETER: CompletionItemKind = CompletionItemKind(25);

    pub fn value(&self) -> i64 {
        self.0
    }
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
    Block(Block),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    DoWhileStatement(DoWhileStatement),
    UncheckedBlock(UncheckedBlock),
    TryCatchClause(TryCatchClause),
    TryStatement(TryStatement),
}
impl InteractableNode {

    pub fn get_name(&self) -> String
    {
        match self {
            InteractableNode::ContractDefinition(node) => node.name.clone(),
            InteractableNode::FunctionDefinition(node) => node.name.clone(),
            InteractableNode::ModifierDefinition(node) => node.name.clone(),
            InteractableNode::StructDefinition(node) => node.name.clone(),
            InteractableNode::EnumDefinition(node) => node.name.clone(),
            InteractableNode::VariableDeclaration(node) => node.name.clone(),
            InteractableNode::EventDefinition(node) => node.name.clone(),
            InteractableNode::EnumValue(node) => node.name.clone(),
            InteractableNode::UsingForDirective(node) => "".to_string(),
            InteractableNode::ImportDirective(node) => "".to_string(),
            InteractableNode::ErrorDefinition(node) => node.name.clone(),
            InteractableNode::FunctionCall(node) => "".to_string(),
            InteractableNode::ModifierInvocation(node) => "".to_string(),
            InteractableNode::InheritanceSpecifier(node) => "".to_string(),
            InteractableNode::Identifier(node) => node.name.clone(),
            InteractableNode::MemberAccess(node) => node.member_name.clone(),
            InteractableNode::NewExpression(node, _) => "".to_string(),
            InteractableNode::UserDefinedTypeName(node) => node.name.clone().unwrap(),
            InteractableNode::IdentifierPath(node) => "".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_scope(&self) -> i64 {
        match self {
            InteractableNode::ContractDefinition(node) => node.scope,
            InteractableNode::FunctionDefinition(node) => node.scope,
            InteractableNode::StructDefinition(node) => node.scope,
            InteractableNode::VariableDeclaration(node) => node.scope,
            InteractableNode::ImportDirective(node) => node.scope,
            _ => -1,
        }
    }
    
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
            _ => -1,
        }
    }

    pub fn get_reference_id(&self) -> Option<i64> {
        match self {
            InteractableNode::FunctionDefinition(node) => {
                if let Some(overrides) = &node.overrides {
                    match &overrides.overrides {
                        OverrideSpecifierOverrides::UserDefinedTypeNames(names) => {
                            if !names.is_empty() {
                                return Some(names[0].referenced_declaration);
                            }
                        }
                        OverrideSpecifierOverrides::IdentifierPaths(paths) => {
                            if !paths.is_empty() {
                                return Some(paths[0].referenced_declaration);
                            }
                        }
                    }
                }
                None
            }
            InteractableNode::ModifierDefinition(node) => {
                if let Some(overrides) = &node.overrides {
                    match &overrides.overrides {
                        OverrideSpecifierOverrides::UserDefinedTypeNames(names) => {
                            if !names.is_empty() {
                                return Some(names[0].referenced_declaration);
                            }
                        }
                        OverrideSpecifierOverrides::IdentifierPaths(paths) => {
                            if !paths.is_empty() {
                                return Some(paths[0].referenced_declaration);
                            }
                        }
                    }
                }
                None
            }
            InteractableNode::VariableDeclaration(node) => {
                if let Some(overrides) = &node.overrides {
                    match &overrides.overrides {
                        OverrideSpecifierOverrides::UserDefinedTypeNames(names) => {
                            if !names.is_empty() {
                                return Some(names[0].referenced_declaration);
                            }
                        }
                        OverrideSpecifierOverrides::IdentifierPaths(paths) => {
                            if !paths.is_empty() {
                                return Some(paths[0].referenced_declaration);
                            }
                        }
                    }
                }
                None
            }
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
            }
            InteractableNode::InheritanceSpecifier(node) => match &node.base_name {
                InheritanceSpecifierBaseName::UserDefinedTypeName(names) => {
                    Some(names.referenced_declaration)
                }
                InheritanceSpecifierBaseName::IdentifierPath(paths) => {
                    Some(paths.referenced_declaration)
                }
            },
            InteractableNode::UserDefinedTypeName(node) => Some(node.referenced_declaration),
            InteractableNode::Identifier(node) => node.referenced_declaration,
            InteractableNode::MemberAccess(node) => node.referenced_declaration,
            _ => None,
        }
    }

    pub fn get_range(&self) -> Range {
        match self {
            InteractableNode::ContractDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::FunctionDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::ModifierDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::StructDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::EnumDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::VariableDeclaration(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::EventDefinition(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::EnumValue(node) => source_location_to_range(
                node.name_location.as_ref().unwrap_or(&node.src.to_owned()),
            ),
            InteractableNode::ErrorDefinition(node) => {
                source_location_to_range(&node.name_location)
            }
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
            InteractableNode::Block(block) => source_location_to_range(&block.src),
            _ => Range {
                index: 0,
                length: 0,
            },
        }
    }

    pub fn get_kind(&self) -> CompletionItemKind {
        match self {
            InteractableNode::ContractDefinition(_) => CompletionItemKind::CLASS,
            InteractableNode::FunctionDefinition(_) => CompletionItemKind::FUNCTION,
            InteractableNode::ModifierDefinition(_) => CompletionItemKind::METHOD,
            InteractableNode::StructDefinition(_) => CompletionItemKind::STRUCT,
            InteractableNode::EnumDefinition(_) => CompletionItemKind::ENUM,
            InteractableNode::VariableDeclaration(_) => CompletionItemKind::VARIABLE,
            InteractableNode::EventDefinition(_) => CompletionItemKind::EVENT,
            InteractableNode::EnumValue(_) => CompletionItemKind::ENUM_MEMBER,
            InteractableNode::UsingForDirective(_) => CompletionItemKind::REFERENCE,
            InteractableNode::ImportDirective(_) => CompletionItemKind::REFERENCE,
            InteractableNode::ErrorDefinition(_) => CompletionItemKind::STRUCT,
            InteractableNode::FunctionCall(_) => CompletionItemKind::FUNCTION,
            InteractableNode::ModifierInvocation(_) => CompletionItemKind::METHOD,
            InteractableNode::InheritanceSpecifier(_) => CompletionItemKind::REFERENCE,
            InteractableNode::Identifier(_) => CompletionItemKind::VARIABLE,
            InteractableNode::MemberAccess(_) => CompletionItemKind::VARIABLE,
            InteractableNode::NewExpression(_, _) => CompletionItemKind::CONSTRUCTOR,
            InteractableNode::UserDefinedTypeName(_) => CompletionItemKind::TYPE_PARAMETER,
            InteractableNode::IdentifierPath(_) => CompletionItemKind::VARIABLE,
            InteractableNode::Block(_) => CompletionItemKind::UNIT,
            _ => CompletionItemKind::TEXT,
        }
    }
}
