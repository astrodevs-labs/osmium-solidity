use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "named-parameters-mapping";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct NamedParametersMapping {
    data: RuleEntry,
}

pub struct MappingsVisitor {
    mappings: Vec<TypeMapping>,
}

impl MappingsVisitor {
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for MappingsVisitor {
    fn visit_type(&mut self, t: &Type) {
        if let Type::Mapping(ty) = t {
            self.mappings.push(ty.clone());
        }
        visit::visit_type(self, t)
    }
}

impl NamedParametersMapping {
    fn create_diag(
        &self,
        location: (LineColumn, LineColumn),
        message: &str,
        file: &SolidFile,
    ) -> LintDiag {
        LintDiag {
            id: RULE_ID.to_string(),
            range: Range {
                start: Position {
                    line: location.0.line,
                    character: location.0.column,
                },
                end: Position {
                    line: location.1.line,
                    character: location.1.column,
                },
            },
            message: message.to_string(),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for NamedParametersMapping {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut visitor = MappingsVisitor::new();
        for contract in
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data)
        {
            visitor.visit_item_contract(&contract);
        }

        for mapping in visitor.mappings.iter() {
            if mapping.key_name.is_none() {
                let span = mapping.key.span();
                res.push(self.create_diag(
                    (span.start(), span.end()),
                    format!("{} parameter is not named", mapping.key).as_str(),
                    file,
                ));
            }
            if mapping.value_name.is_none() {
                let span = mapping.value.span();
                res.push(self.create_diag(
                    (span.start(), span.end()),
                    format!("{} parameter is not named", mapping.value).as_str(),
                    file,
                ));
            }
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Solidity v0.8.18 introduced named parameters on the mappings definition.".to_string(),
            category: "naming".to_string(),
            example_config: "{\"id\": \"named-parameters-mapping\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/naming/named_parameters_mapping.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/NamedParametersMapping".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![
                    Example {
                        description: "To enter \"users\" mapping the key called \"name\" is needed to get the \"balance\"".to_string(),
                        code: "mapping(string name => uint256 balance) public users;".to_string(),
                    },
                    Example {
                        description: "To enter owner token balance, the main key \"owner\" enters another mapping which its key is \"token\" to get its \"balance\"".to_string(),
                        code: "mapping(address owner => mapping(address token => uint256 balance)) public tokenBalances;".to_string(),
                    },
                    Example {
                        description: "Main key of mapping is enforced. On nested mappings other naming are not neccesary".to_string(),
                        code: "mapping(address owner => mapping(address => uint256)) public tokenBalances;".to_string(),
                    }, Example {
                        description: "Main key of the parent mapping is enforced. No naming in nested mapping uint256".to_string(),
                        code: "mapping(address owner => mapping(address token => uint256)) public tokenBalances;".to_string(),
                    }, Example {
                        description: "Main key of the parent mapping is enforced. No naming in nested mapping address".to_string(),
                        code: "mapping(address owner => mapping(address => uint256 balance)) public tokenBalances;".to_string(),
                    },
                ],
                bad: vec![
                    Example {
                        description: "No naming at all in regular mapping".to_string(),
                        code: "mapping(address => uint256)) public tokenBalances;".to_string(),
                    },
                    Example {
                        description: "Missing any variable name in regular mapping uint256".to_string(),
                        code: "mapping(address token => uint256)) public tokenBalances;".to_string(),
                    },
                    Example {
                        description: "Missing any variable name in regular mapping address".to_string(),
                        code: "mapping(address => uint256 balance)) public tokenBalances;".to_string(),
                    }, Example {
                        description: "No MAIN KEY naming in nested mapping. Other naming are not enforced".to_string(),
                        code: "mapping(address => mapping(address token => uint256 balance)) public tokenBalances;".to_string(),
                    },
                ],
            },
        }
    }
}

impl NamedParametersMapping {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NamedParametersMapping { data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: None,
        }
    }
}
