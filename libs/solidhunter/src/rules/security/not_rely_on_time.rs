use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::LineColumn;

// global
pub const RULE_ID: &str = "not-rely-on-time";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Avoid making time-based decisions in your business logic";

pub struct NotRelyOnTime {
    data: RuleEntry,
}

impl NotRelyOnTime {
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

impl RuleType for NotRelyOnTime {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut i = 1;

        for line in file.content.lines() {
            if let Some(index) = line.find("now") {
                res.push(self.create_diag(
                    (
                        LineColumn {
                            line: i,
                            column: index,
                        },
                        LineColumn {
                            line: i,
                            column: index + 3,
                        },
                    ),
                    file,
                ));
            }
            if let Some(index) = line.find("block.timestamp") {
                res.push(self.create_diag(
                    (
                        LineColumn {
                            line: i,
                            column: index,
                        },
                        LineColumn {
                            line: i,
                            column: index + 15,
                        },
                    ),
                    file,
                ));
            }
            i += 1;
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Avoid making time-based decisions in your business logic.".to_string(),
            category: "security".to_string(),
            example_config: "{\"id\": \"not-rely-on-time\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/security/not_rely_on_time.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/NotRelyOnTime".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl NotRelyOnTime {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NotRelyOnTime { data };
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
