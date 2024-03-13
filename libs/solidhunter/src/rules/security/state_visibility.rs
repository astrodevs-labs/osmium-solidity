use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "state-visibility";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Explicitly mark visibility of state";

pub struct StateVisibility {
    data: RuleEntry,
}

impl StateVisibility {
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

impl RuleType for StateVisibility {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts.iter() {
            for node_var in contract.body.iter() {
                if let Item::Variable(var) = node_var {
                    if var.attributes.visibility().is_none() {
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
            description: "Explicitly mark visibility of state.".to_string(),
            category: "security".to_string(),
            example_config: "{\"id\": \"state-visibility\", \"severity\": \"WARNING\", \"data\": []}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/security/state_visibility.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![Example {
                    description: "State explicitly marked with visibility".to_string(),
                    code: "uint public data;".to_string(),
                }],
                bad: vec![Example {
                    description: "Functions without explicitly marked visibility".to_string(),
                    code: "uint data;".to_string(),
                }],
            },
        }
    }
}

impl StateVisibility {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = StateVisibility { data };
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
