use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::{LineColumn, Spanned};

// global
pub const RULE_ID: &str = "func-param-name-mixedcase";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Function param name must be in mixedCase";

pub struct FuncParamNameMixedCase {
    data: RuleEntry,
}

impl FuncParamNameMixedCase {
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
            same_line_ranges: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for FuncParamNameMixedCase {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts =
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            for function in
                osmium_libs_solidity_ast_extractor::retriever::retrieve_functions_nodes(&contract)
            {
                for arg in function.parameters.iter() {
                    if let Some(name) = &arg.name {
                        if !(name.as_string().chars().next().unwrap() >= 'a'
                            && name.as_string().chars().next().unwrap() <= 'z')
                            || name.as_string().contains('_')
                            || name.as_string().contains('-')
                        {
                            let span = name.span();
                            res.push(self.create_diag((span.start(), span.end()), file));
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
            description: "Function param name must be in mixedCase.".to_string(),
            category: "naming".to_string(),
            example_config: "{\"id\": \"func-param-name-mixedcase\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/naming/func_param_name_mixedcase.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/FuncParamNameMixedcase".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl FuncParamNameMixedCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = FuncParamNameMixedCase { data };
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
