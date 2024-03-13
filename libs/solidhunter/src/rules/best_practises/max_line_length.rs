use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "max-line-length";

// specific
const DEFAULT_LENGTH: usize = 120;
const DEFAULT_SEVERITY: Severity = Severity::ERROR;

pub struct MaxLineLength {
    max_len: usize,
    data: RuleEntry,
}

impl MaxLineLength {
    fn create_diag(&self, file: &SolidFile, line_idx: usize, line: &str) -> LintDiag {
        LintDiag {
            range: Range {
                start: Position {
                    line: line_idx,
                    character: self.max_len,
                },
                end: Position {
                    line: line_idx,
                    character: line.len(),
                },
            },
            id: RULE_ID.to_string(),
            message: format!(
                "Line length must be no more than {} but current length is {}",
                self.max_len,
                line.len()
            ),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for MaxLineLength {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut line_idx = 1;

        for line in file.content.lines() {
            if line.len() > self.max_len {
                res.push(self.create_diag(file, line_idx, line));
            }
            line_idx += 1;
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Line length must be no more than maxlen.".to_string(),
            category: "best-practices".to_string(),
            example_config: "{\"id\": \"max-line-length\", \"severity\": \"WARNING\", \"data\": 80}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/best_practices/max_line_length.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/MaxLineLength".to_string(),
            options: vec![Options {
                description: "Maximum allowed number of characters per line".to_string(),
                default: "120".to_string(),
            }],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl MaxLineLength {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut max_line_length = DEFAULT_LENGTH;

        if let Some(data) = &data.data {
            let parsed: Result<usize, serde_json::Error> = serde_json::from_value(data.clone());
            match parsed {
                Ok(val) => max_line_length = val,
                Err(_) => {
                    eprintln!("{} rule : bad config data", RULE_ID);
                }
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = MaxLineLength {
            max_len: max_line_length,
            data,
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(DEFAULT_LENGTH.into()),
        }
    }
}
