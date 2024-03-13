use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// TODO test output

// global
pub const RULE_ID: &str = "explicit-types";

// specific
const DEFAULT_RULE: &str = "explicit";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const EXPLICIT_TYPES: &[&str] = &[
    "uint256", "int256", "uint8", "int8", "uint16", "int16", "uint32", "int32", "uint64", "int64",
    "uint128", "int128",
];
const IMPLICIT_TYPES: &[&str] = &["uint", "int"];

pub struct ExplicitTypes {
    rule: String,
    data: RuleEntry,
}

pub struct ExplicitTypesVisitor {
    explicit: bool,
    defs: Vec<VariableDefinition>,
    decls: Vec<VariableDeclaration>,
    types: Vec<Type>,
}

impl<'ast> Visit<'ast> for ExplicitTypesVisitor {
    fn visit_variable_definition(&mut self, var: &'ast VariableDefinition) {
        if let Some((_, expr)) = &var.initializer {
            visit::visit_expr(self, expr);
        }
        if self.is_type_match(&var.ty) {
            self.defs.push(var.clone())
        }
    }

    fn visit_variable_declaration(&mut self, var: &'ast VariableDeclaration) {
        if self.is_type_match(&var.ty) {
            self.decls.push(var.clone())
        }
    }

    fn visit_type(&mut self, ty: &'ast Type) {
        if self.is_type_match(ty) {
            self.types.push(ty.clone());
        }
    }
}

impl ExplicitTypesVisitor {
    fn is_type_match(&self, ty: &Type) -> bool {
        if self.explicit {
            IMPLICIT_TYPES.iter().any(|typ| ty.to_string() == *typ)
        } else {
            EXPLICIT_TYPES.iter().any(|typ| ty.to_string() == *typ)
        }
    }
}

impl ExplicitTypes {
    fn create_diag(&self, file: &SolidFile, ty: Box<dyn Spanned>, var: Option<String>) -> LintDiag {
        LintDiag {
            range: Range {
                start: Position {
                    line: ty.span().start().line,
                    character: ty.span().start().column,
                },
                end: Position {
                    line: ty.span().end().line,
                    character: ty.span().end().column,
                },
            },
            id: RULE_ID.to_string(),
            message: format!(
                "Rule is set with {} type [var/s: {}]",
                self.rule,
                var.unwrap_or("\"\"".to_string())
            ),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for ExplicitTypes {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut visitor = ExplicitTypesVisitor {
            explicit: self.rule == "explicit",
            defs: vec![],
            decls: vec![],
            types: vec![],
        };
        visitor.visit_file(&file.data);
        for def in visitor.defs {
            res.push(self.create_diag(file, Box::new(def.ty), Some(def.name.0.to_string())));
        }
        for decl in visitor.decls {
            let name = match decl.name {
                Some(ident) => Some(ident.0.to_string()),
                _ => None,
            };
            res.push(self.create_diag(file, Box::new(decl.ty), name));
        }
        for ty in visitor.types {
            res.push(self.create_diag(file, Box::new(ty), None));
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description:
                "Forbid or enforce explicit types (like uint256) that have an alias (like uint)."
                    .to_string(),
            category: "best-practices".to_string(),
            example_config: "{\"id\": \"explicit-types\", \"severity\": \"WARNING\", \"data\": \"explicit\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/best_practices/explicit_types.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/ExplicitTypes".to_string(),
            options: vec![Options {
                description: "Options need to be one of \"explicit\", \"implicit\"".to_string(),
                default: "explicit".to_string(),
            }],
            examples: Examples {
                good: vec![
                    Example {
                        description: "If explicit is selected".to_string(),
                        code: "uint256 public variableName;".to_string(),
                    },
                    Example {
                        description: "If implicit is selected".to_string(),
                        code: "uint public variableName;".to_string(),
                    },
                    Example {
                        description: "If explicit is selected".to_string(),
                        code: "uint256 public variableName = uint256(5);".to_string(),
                    },
                ],
                bad: vec![
                    Example {
                        description: "If explicit is selected".to_string(),
                        code: "uint public variableName;".to_string(),
                    },
                    Example {
                        description: "If implicit is selected".to_string(),
                        code: "uint256 public variableName;".to_string(),
                    },
                    Example {
                        description: "At any setting".to_string(),
                        code: "uint public variableName = uint256(5);".to_string(),
                    },
                ],
            },
        }
    }
}

impl ExplicitTypes {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut value = DEFAULT_RULE.to_string();

        if let Some(data) = &data.data {
            let parsed: Result<String, serde_json::Error> = serde_json::from_value(data.clone());
            match parsed {
                Ok(val) => value = val,
                Err(_) => {
                    eprintln!("{} rule : bad config data", RULE_ID);
                }
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = ExplicitTypes { rule: value, data };
        Box::new(rule)
    }
    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(DEFAULT_RULE.into()),
        }
    }
}
