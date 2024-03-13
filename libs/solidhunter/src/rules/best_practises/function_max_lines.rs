use osmium_libs_solidity_ast_extractor::{FunctionBody, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "function-max-lines";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MAX_LINES: usize = 50;

pub struct FunctionMaxLines {
    number_max_lines: usize,
    data: RuleEntry,
}

impl RuleType for FunctionMaxLines {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for contract in
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&_file.data)
        {
            for function in
                osmium_libs_solidity_ast_extractor::retriever::retrieve_functions_nodes(&contract)
            {
                let report = check_function_lines(&function, self.number_max_lines);
                if let Some(report) = report {
                    let start = report.start.line;
                    let end = report.end.line;

                    res.push(LintDiag {
                        id: RULE_ID.to_string(),
                        range: report,
                        severity: self.data.severity,
                        code: None,
                        source: None,
                        message: format!(
                            "Function body contains {} lines but allowed no more than {} lines",
                            end - start,
                            self.number_max_lines
                        ),
                        uri: _file.path.clone(),
                    });
                }
            }
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description:
                "Function body contains \"count\" lines but allowed no more than maxlines."
                    .to_string(),
            category: "best-practices".to_string(),
            example_config: "{\"id\": \"function-max-lines\", \"severity\": \"WARNING\", \"data\": 20}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/best_practices/function_max_lines.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/FunctionMaxLines".to_string(),
            options: vec![Options {
                description: "Maximum allowed lines count per function	".to_string(),
                default: "50".to_string(),
            }],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

// returns a struct containing the line number of the start and end of the function if it is too long
fn check_function_lines(
    function: &osmium_libs_solidity_ast_extractor::ItemFunction,
    nb_max_line: usize,
) -> Option<Range> {
    if let FunctionBody::Block(block) = &function.body {
        let line_diff = block.span().end().line - block.span().start().line;
        let start_span = function.name.span().start();
        let end_span = block.span().end();
        if line_diff > nb_max_line {
            return Some(Range {
                start: Position {
                    line: start_span.line,
                    character: start_span.column,
                },
                end: Position {
                    line: end_span.line,
                    character: end_span.column,
                },
            });
        }
    }
    None
}

impl FunctionMaxLines {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut max_number_lines = DEFAULT_MAX_LINES;

        if let Some(data) = &data.data {
            let parsed: Result<usize, serde_json::Error> = serde_json::from_value(data.clone());
            match parsed {
                Ok(val) => max_number_lines = val,
                Err(_) => {
                    eprintln!("{} rule : bad config data", RULE_ID);
                }
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = FunctionMaxLines {
            number_max_lines: max_number_lines,
            data,
        };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(DEFAULT_MAX_LINES.into()),
        }
    }
}
