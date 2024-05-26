use std::{collections::HashMap, str::FromStr};

use osmium_libs_solidity_ast_extractor::{kw::function, types::SolidityAstFile};
use solc_ast_rs_types::types::*;


#[allow(dead_code)]
pub fn create_test_contract_definition() -> ContractDefinition {
    ContractDefinition {
        id: 1,
        name: "Test".to_string(),
        src: SourceLocation::from_str("29:215:1").unwrap(),
        name_location: None,
        abstract_: false,
        base_contracts: vec![],
        canonical_name: None,
        contract_dependencies: vec![],
        contract_kind: ContractDefinitionContractKind::Contract,
        documentation: None,
        fully_implemented: true,
        internal_function_i_ds: HashMap::new(),
        linearized_base_contracts: vec![],
        node_type: ContractDefinitionNodeType::ContractDefinition,
        nodes: vec![],
        scope: 0,
        used_errors: vec![],
        used_events: vec![],
    }
}

#[allow(dead_code)]
pub fn create_test_function_definition() -> FunctionDefinition {
    FunctionDefinition {
        id: 2,
        name: "notUsed".to_string(),
        src: SourceLocation::from_str("152:86:1").unwrap(),
        name_location: None,
        visibility: Visibility::Public,
        state_mutability: StateMutability::Nonpayable,
        parameters: ParameterList {
            id: 1,
            node_type: ParameterListNodeType::ParameterList,
            parameters: vec![],
            src: SourceLocation::from_str("1:1:1").unwrap(),
        },
        return_parameters: ParameterList {
            id: 1,
            node_type: ParameterListNodeType::ParameterList,
            parameters: vec![],
            src: SourceLocation::from_str("1:1:1").unwrap(),
        },
        body: None,
        kind: FunctionDefinitionKind::Function,
        node_type: FunctionDefinitionNodeType::FunctionDefinition,
        scope: 1,
        implemented: true,
        base_functions: None,
        documentation: None,
        function_selector: None,
        modifiers: vec![],
        overrides: None,
        virtual_: false,
    }
}

#[allow(dead_code)]
pub fn create_test_variable_declaration() -> VariableDeclaration {
    VariableDeclaration {
        id: 3,
        name: "number".to_string(),
        src: SourceLocation::from_str("56:21:1").unwrap(),
        visibility: Visibility::Public,
        constant: false,
        indexed: None,
        base_functions: None,
        documentation: None,
        function_selector: None,
        mutability: Mutability::Mutable,
        name_location: None,
        node_type: VariableDeclarationNodeType::VariableDeclaration,
        overrides: Some(OverrideSpecifier {
            src: SourceLocation::from_str("1:1:1").unwrap(),
            id: 1,
            node_type: OverrideSpecifierNodeType::OverrideSpecifier,
            overrides: OverrideSpecifierOverrides::UserDefinedTypeNames(vec![
                UserDefinedTypeName {
                    id:1,
                    node_type:UserDefinedTypeNameNodeType::UserDefinedTypeName,
                    referenced_declaration: 3,
                    src:SourceLocation::from_str("1:1:1").unwrap(),
                    type_descriptions:TypeDescriptions{
                        type_string: Some("uint256".to_string()),
                        type_identifier: None,
                    },
                    contract_scope: (),
                    name: Some("Test".to_string()),
                    path_node: None,
                },
            ]),
        }),
        scope: 2,
        state_variable: false,
        storage_location: StorageLocation::Default,
        type_descriptions: TypeDescriptions {
            type_string: Some("uint256".to_string()),
            type_identifier: None,
        },
        type_name: None,
        value: Some(Expression::Identifier(Identifier {
            id: 1,
            node_type: IdentifierNodeType::Identifier,
            name: "number".to_string(),
            src: SourceLocation::from_str("1:1:1").unwrap(),
            referenced_declaration: Some(3),
            type_descriptions: TypeDescriptions {
                type_string: Some("uint256".to_string()),
                type_identifier: None,
            },
            overloaded_declarations: vec![],
            argument_types: None,
        })),
    }
}

#[allow(dead_code)]
pub fn create_test_modifier_definition() -> ModifierDefinition {
    ModifierDefinition {
        id: 4,
        name: "modifier".to_string(),
        src: SourceLocation::from_str("1:1:1").unwrap(),
        name_location: None,
        visibility: Visibility::Public,
        documentation: None,
        node_type: ModifierDefinitionNodeType::ModifierDefinition,
        parameters: ParameterList {
            id: 1,
            node_type: ParameterListNodeType::ParameterList,
            parameters: vec![],
            src: SourceLocation::from_str("1:1:1").unwrap(),
        },
        virtual_: false,
        base_modifiers: None,
        body: Block {
            id: 1,
            src: SourceLocation::from_str("1:1:1").unwrap(),
            statements: None,
            node_type: BlockNodeType::Block,
            documentation: None,
        },
        overrides: None,
    }
}


#[allow(dead_code)]
pub fn create_test_struct_definition() -> StructDefinition {
    StructDefinition {
        id: 5,
        name: "TestStruct".to_string(),
        src: SourceLocation::from_str("1:1:1").unwrap(),
        name_location: None,
        documentation: None,
        node_type: StructDefinitionNodeType::StructDefinition,
        scope: 1,
        members: vec![],
        canonical_name: "TestStruct".to_string(),
        visibility: Visibility::Public,
    }
}

#[allow(dead_code)]
pub fn create_test_enum_definition() -> EnumDefinition {
    EnumDefinition {
        id: 6,
        name: "TestEnum".to_string(),
        src: SourceLocation::from_str("1:1:1").unwrap(),
        name_location: None,
        documentation: None,
        node_type: EnumDefinitionNodeType::EnumDefinition,
        members: vec![],
        canonical_name: "TestEnum".to_string(),
    }
}

#[allow(dead_code)]
pub fn create_test_event_definition() -> EventDefinition {
    EventDefinition {
        id: 7,
        name: "TestEvent".to_string(),
        src: SourceLocation::from_str("1:1:1").unwrap(),
        name_location: None,
        documentation: None,
        node_type: EventDefinitionNodeType::EventDefinition,
        parameters: ParameterList {
            id: 1,
            node_type: ParameterListNodeType::ParameterList,
            parameters: vec![],
            src: SourceLocation::from_str("1:1:1").unwrap(),
        },
        anonymous: false,
        event_selector: None,
    }
}

#[allow(dead_code)]
pub fn create_test_enum_value() -> EnumValue {
    EnumValue {
        id: 8,
        name: "TestEnumValue".to_string(),
        src: SourceLocation::from_str("1:1:1").unwrap(),
        name_location: None,
        node_type: EnumValueNodeType::EnumValue,
    }
}

#[allow(dead_code)]
pub fn create_test_using_for_directive() -> UsingForDirective {
    UsingForDirective {
        id: 9,
        library_name: None,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        node_type: UsingForDirectiveNodeType::UsingForDirective,
        type_name: None,
        function_list: vec![],
        global: None,
    }
}

#[allow(dead_code)]
pub fn create_test_import_directive() -> ImportDirective {
    ImportDirective {
        id: 10,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        node_type: ImportDirectiveNodeType::ImportDirective,
        unit_alias: "Alias".to_string(),
        absolute_path: "/home/user/test.sol".to_string(),
        file: "test.sol".to_string(),
        name_location: None,
        scope: 0,
        source_unit: 0,
        symbol_aliases: vec![],
    }
}

#[allow(dead_code)]
pub fn create_test_error_definition() -> ErrorDefinition {
    ErrorDefinition {
        id: 11,
        name: "TestError".to_string(),
        src: SourceLocation::from_str("1:1:1").unwrap(),
        name_location: "Here".to_string(),
        documentation: None,
        node_type: ErrorDefinitionNodeType::ErrorDefinition,
        error_selector: None,
        parameters: ParameterList {
            id: 1,
            node_type: ParameterListNodeType::ParameterList,
            parameters: vec![],
            src: SourceLocation::from_str("1:1:1").unwrap(),
        },
    }
}

#[allow(dead_code)]
pub fn create_test_function_call() -> FunctionCall {
    FunctionCall {
        id: 12,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        node_type: FunctionCallNodeType::FunctionCall,
        arguments: vec![],
        expression: Box::new(Expression::Identifier(Identifier {
            id: 1,
            node_type: IdentifierNodeType::Identifier,
            name: "number".to_string(),
            src: SourceLocation::from_str("1:1:1").unwrap(),
            referenced_declaration: Some(2),
            type_descriptions: TypeDescriptions {
                type_string: Some("uint256".to_string()),
                type_identifier: None,
            },
            overloaded_declarations: vec![],
            argument_types: None,
        })),
        names: vec![],
        type_descriptions: TypeDescriptions {
            type_string: Some("uint256".to_string()),
            type_identifier: None,
        },
        argument_types: None,
        is_constant: false,
        is_l_value: false,
        is_pure: false,
        kind: FunctionCallKind::FunctionCall,
        l_value_requested: false,
        name_locations: vec![],
        try_call: false,
    }
}

#[allow(dead_code)]
pub fn create_test_modifier_invocation() -> ModifierInvocation {
    ModifierInvocation {
        id: 13,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        node_type: ModifierInvocationNodeType::ModifierInvocation,
        arguments: None,
        modifier_name: ModifierInvocationModifierName::Identifier(Identifier {
            id: 1,
            node_type: IdentifierNodeType::Identifier,
            name: "modifier".to_string(),
            src: SourceLocation::from_str("1:1:1").unwrap(),
            referenced_declaration: Some(4),
            type_descriptions: TypeDescriptions {
                type_string: Some("modifier".to_string()),
                type_identifier: None,
            },
            overloaded_declarations: vec![],
            argument_types: None,
        }),
        kind: Some(ModifierInvocationKind::ModifierInvocation),
    }
}

#[allow(dead_code)]
pub fn create_test_inheritance_specifier() -> InheritanceSpecifier {
    InheritanceSpecifier {
        id: 14,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        node_type: InheritanceSpecifierNodeType::InheritanceSpecifier,
        arguments: None,
        base_name: InheritanceSpecifierBaseName::UserDefinedTypeName(UserDefinedTypeName {
            id: 1,
            node_type: UserDefinedTypeNameNodeType::UserDefinedTypeName,
            referenced_declaration: 5,
            src: SourceLocation::from_str("1:1:1").unwrap(),
            type_descriptions: TypeDescriptions {
                type_string: Some("TestStruct".to_string()),
                type_identifier: None,
            },
            contract_scope: (),
            name: Some("TestStruct".to_string()),
            path_node: None,
        }),
    }
}

#[allow(dead_code)]
pub fn create_test_identifier() -> Identifier {
    Identifier {
        id: 15,
        node_type: IdentifierNodeType::Identifier,
        name: "identifier".to_string(),
        src: SourceLocation::from_str("1:1:1").unwrap(),
        referenced_declaration: Some(3),
        type_descriptions: TypeDescriptions {
            type_string: Some("uint256".to_string()),
            type_identifier: None,
        },
        overloaded_declarations: vec![],
        argument_types: None,
    }
}

#[allow(dead_code)]
pub fn create_test_member_access() -> MemberAccess {
    MemberAccess {
        id: 16,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        expression: Box::new(Expression::Identifier(Identifier {
            id: 1,
            node_type: IdentifierNodeType::Identifier,
            name: "number".to_string(),
            src: SourceLocation::from_str("1:1:1").unwrap(),
            referenced_declaration: Some(3),
            type_descriptions: TypeDescriptions {
                type_string: Some("uint256".to_string()),
                type_identifier: None,
            },
            overloaded_declarations: vec![],
            argument_types: None,
        })),
        member_name: "member".to_string(),
        referenced_declaration: Some(3),
        type_descriptions: TypeDescriptions {
            type_string: Some("uint256".to_string()),
            type_identifier: None,
        },
        argument_types: None,
        is_constant: false,
        is_l_value: false,
        is_pure: false,
        l_value_requested: false,
        member_location: None,
        node_type: MemberAccessNodeType::MemberAccess,
    }
}

#[allow(dead_code)]
pub fn create_test_new_expression() -> NewExpression {
    NewExpression {
        id: 17,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        node_type: NewExpressionNodeType::NewExpression,
        type_name: TypeName::UserDefinedTypeName(UserDefinedTypeName {
            id: 1,
            node_type: UserDefinedTypeNameNodeType::UserDefinedTypeName,
            referenced_declaration: 5,
            src: SourceLocation::from_str("1:1:1").unwrap(),
            type_descriptions: TypeDescriptions {
                type_string: Some("TestStruct".to_string()),
                type_identifier: None,
            },
            contract_scope: (),
            name: Some("TestStruct".to_string()),
            path_node: None,
        }),
        argument_types: None,
        is_constant: false,
        is_l_value: Some(false),
        is_pure: false,
        l_value_requested: false,
        type_descriptions: TypeDescriptions {
            type_string: Some("TestStruct".to_string()),
            type_identifier: None,
        },
    }
}

#[allow(dead_code)]
pub fn create_test_user_defined_type_name() -> UserDefinedTypeName {
    UserDefinedTypeName {
        id: 18,
        node_type: UserDefinedTypeNameNodeType::UserDefinedTypeName,
        referenced_declaration: 5,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        type_descriptions: TypeDescriptions {
            type_string: Some("TestStruct".to_string()),
            type_identifier: None,
        },
        contract_scope: (),
        name: Some("TestStruct".to_string()),
        path_node: None,
    }
}

#[allow(dead_code)]
pub fn create_test_identifier_path() -> IdentifierPath {
    IdentifierPath {
        id: 19,
        node_type: IdentifierPathNodeType::IdentifierPath,
        name: "IdPath".to_string(),
        name_locations: vec![],
        referenced_declaration: 15,
        src: SourceLocation::from_str("1:1:1").unwrap(),
    }
}

#[allow(dead_code)]
pub fn create_test_ast_file() -> SolidityAstFile {
    let source = "pragma solidity ^0.8.0;
    
    contract Counter {
        uint256 public number;
        uint256 public x = 2;
        uint256 public y = x;
    
        function notUsed() internal {
            uint256 x = 1;
            number;
        }














































        
    }";
    let path = "test.sol";

    let mut function = create_test_function_definition();
    function.modifiers.push(create_test_modifier_invocation().into());
    function.body = Some(Block {
        documentation: None,
        id: 30,
        node_type: BlockNodeType::Block,
        src: SourceLocation::from_str("1:1:1").unwrap(),
        statements: Some([
            Statement::ExpressionStatement(ExpressionStatement {
                expression: create_test_function_call().into(),
                id: 100,
                node_type: ExpressionStatementNodeType::ExpressionStatement,
                src: SourceLocation::from_str("1:1:1").unwrap(),
                documentation: None,
            }),
            Statement::ExpressionStatement(ExpressionStatement {
                expression: create_test_member_access().into(),
                id: 102,
                node_type: ExpressionStatementNodeType::ExpressionStatement,
                src: SourceLocation::from_str("1:1:1").unwrap(),
                documentation: None,
            }),
            Statement::ExpressionStatement(ExpressionStatement {
                expression: create_test_new_expression().into(),
                id: 103,
                node_type: ExpressionStatementNodeType::ExpressionStatement,
                src: SourceLocation::from_str("1:1:1").unwrap(),
                documentation: None,
            }),
        ].iter().cloned().collect()),
    });
    let mut contract = create_test_contract_definition();
    contract.nodes.push(function.into());
    contract.nodes.push(create_test_variable_declaration().into());
    contract.nodes.push(create_test_enum_definition().into());
    contract.nodes.push(create_test_struct_definition().into());
    contract.nodes.push(create_test_event_definition().into());
    contract.nodes.push(create_test_using_for_directive().into());
    contract.nodes.push(create_test_error_definition().into());
    
    contract.base_contracts.push(create_test_inheritance_specifier().into());
    SolidityAstFile {
        file: osmium_libs_solidity_ast_extractor::types::SolidityFile {
            path: path.to_string(),
            content: source.to_string(),
        },
        ast: SourceUnit {
            id: 0,
            nodes: vec![
                contract.into(),
                create_test_function_definition().into(),
                create_test_variable_declaration().into(),
            ],
            src: SourceLocation::from_str("0:0:0").unwrap(),
            absolute_path: "/home/user/test.sol".to_string(),
            experimental_solidity: None,
            exported_symbols: None,
            license: None,
            node_type: SourceUnitNodeType::SourceUnit,
        },
    }
}
