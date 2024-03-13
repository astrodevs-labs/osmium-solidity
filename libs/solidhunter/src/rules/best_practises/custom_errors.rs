use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "custom-errors";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct CustomErrors {
    data: RuleEntry,
}

impl CustomErrors {
    fn create_diag(
        &self,
        file: &SolidFile,
        location: (LineColumn, LineColumn),
        diag_type: String,
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
            message: format!("Use Custom Errors instead of {} statements", diag_type),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for CustomErrors {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for contract in retriever::retrieve_contract_nodes(&file.data) {
            for stmt in retriever::retrieve_stmts_nodes(&contract) {
                if let Stmt::Revert(revert) = &stmt {
                    if let Expr::Tuple(_) = &revert.expr {
                        let location = (revert.span().start(), revert.expr.span().end());
                        res.push(self.create_diag(file, location, "revert".to_string()));
                    }
                }
                if let Stmt::Expr(expr) = &stmt {
                    if let Expr::Call(call) = &expr.expr {
                        if let Expr::Ident(ref ident) = *(call.expr) {
                            if *ident == "require" || *ident == "assert" {
                                let location = (call.span().start(), call.span().end());
                                res.push(self.create_diag(file, location, ident.to_string()));
                            }
                        }
                    }
                }
            }
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Enforces the use of Custom Errors over Require and Revert statements"
                .to_string(),
            category: "best-practises".to_string(),
            example_config: "{\"id\": \"custom-errors\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/best_practices/custom_errors.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/CustomErrors".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![
                    Example {
                        description: "Use Custom Errors".to_string(),
                        code: "revert CustomErrorFunction();".to_string(),
                    },
                    Example {
                        description: "Use of Custom Errors with arguments".to_string(),
                        code: "revert CustomErrorFunction({ msg: \"Insufficient Balance\" });"
                            .to_string(),
                    },
                ],
                bad: vec![
                    Example {
                        description: "Use of require statement".to_string(),
                        code: "require(userBalance >= availableAmount, \"Insufficient Balance\");"
                            .to_string(),
                    },
                    Example {
                        description: "Use of plain revert statement".to_string(),
                        code: "revert();".to_string(),
                    },
                    Example {
                        description: "Use of revert statement with message".to_string(),
                        code: "revert(\"Insufficient Balance\");".to_string(),
                    },
                ],
            },
        }
    }
}

impl CustomErrors {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = CustomErrors { data };
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
