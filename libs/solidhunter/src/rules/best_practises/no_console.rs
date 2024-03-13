use osmium_libs_solidity_ast_extractor::*;

use crate::linter::SolidFile;
use crate::rules::types::{Example, Examples, RuleDocumentation, RuleEntry, RuleType};
use crate::types::{LintDiag, Position, Range, Severity};

// global
pub const RULE_ID: &str = "no-console";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct NoConsole {
    data: RuleEntry,
}

impl NoConsole {
    fn create_diag(
        &self,
        file: &SolidFile,
        location: (LineColumn, LineColumn),
        message: String,
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
            message,
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for NoConsole {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res: Vec<LintDiag> = Vec::new();

        // Check functions calls
        for expr_member in retriever::retrieve_expr_member_nodes(&file.data) {
            if let Expr::Ident(expr_ident) = *expr_member.expr {
                if expr_ident == "console" {
                    if let Expr::Call(expr_call) = *expr_member.member {
                        if let Expr::Ident(expr_ident) = *expr_call.expr {
                            if expr_ident.as_string().starts_with("log") {
                                let diag = self.create_diag(
                                    file,
                                    (expr_ident.span().start(), expr_ident.span().end()),
                                    "Unexpected console statement".to_string(),
                                );
                                res.push(diag);
                            }
                        }
                    }
                }
            }
        }

        // Check imports
        let blacklist: Vec<String> = vec![
            "hardhat/console.sol".to_string(),
            "forge-std/console".to_string(), // console?.sol (easier to blacklist this way)
        ];

        for header in retriever::retrieve_import_directive_nodes(&file.data) {
            if let ImportPath::Plain(pathplain) = &header.path {
                for test in &pathplain.path.values {
                    for refused in blacklist.clone() {
                        if test.value().contains(&refused) {
                            let diag = self.create_diag(
                                file,
                                (header.span().start(), header.span().end()),
                                "Unexpected import of console file".to_string(),
                            );
                            res.push(diag);
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
            description: "No console.log/logInt/logBytesX/logString/etc & No hardhat and forge-std console.sol import statements.".to_string(),
            category: "best-practices".to_string(),
            example_config: "{\"id\": \"no-console\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/best_practices/no_console.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/NoConsole".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![Example{description: "No console.logX statements".to_string(),
                    code: "console.log('test');".to_string()},
                    Example{description: "No hardhat/console.sol import statements".to_string(),
                    code: "import 'hardhat/console.sol';".to_string()},
                    Example{description: "No forge-std console.sol & console2.sol import statements".to_string(),
                    code: "import 'forge-std/consoleN.sol';".to_string()},
                ],
            },
        }
    }
}

impl NoConsole {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NoConsole { data };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: None,
        }
    }
}
