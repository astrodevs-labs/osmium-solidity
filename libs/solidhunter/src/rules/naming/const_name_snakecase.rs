use osmium_libs_solidity_ast_extractor::{retriever, Item, LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "const-name-snakecase";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Constant name must be in capitalized SNAKE_CASE";

pub struct ConstNameSnakeCase {
    data: RuleEntry,
}

impl ConstNameSnakeCase {
    fn create_diag(&self, location: (LineColumn, LineColumn), file: &SolidFile) -> LintDiag {
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
            message: DEFAULT_MESSAGE.to_string(),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

fn is_snake_case(name: &str) -> bool {
    for c in name.chars() {
        if c != '_' && !c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}

impl RuleType for ConstNameSnakeCase {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts.iter() {
            for node_var in contract.body.iter() {
                if let Item::Variable(var) = node_var {
                    if !var.attributes.has_constant() {
                        continue;
                    }
                    if !is_snake_case(&var.name.as_string()) {
                        let span = var.name.span();
                        res.push(self.create_diag((span.start(), span.end()), file));
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
            description: "Constant name must be in capitalized SNAKE_CASE. (Does not check IMMUTABLES, use immutable-vars-naming)".to_string(),
            category: "naming".to_string(),
            example_config: " {\"id\": \"const-name-snakecase\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/naming/const_name_snakecase.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/ConstNameSnakecase".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl ConstNameSnakeCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = ConstNameSnakeCase { data };
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
