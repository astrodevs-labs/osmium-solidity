use osmium_libs_solidity_ast_extractor::{LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "var-name-mixedcase";

// specific
const DEFAULT_MESSAGE: &str = "Variable should be in mixedCase";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct VarNameMixedCase {
    data: RuleEntry,
}

impl VarNameMixedCase {
    fn create_diag(&self, location: (LineColumn, LineColumn), file: &SolidFile) -> LintDiag {
        LintDiag {
            id: RULE_ID.to_string(),
            range: Range {
                start: Position {
                    line: location.0.line,
                    character: location.0.column + 1,
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

impl RuleType for VarNameMixedCase {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let variables_definition =
            osmium_libs_solidity_ast_extractor::retriever::retrieve_variable_definition_nodes(
                &file.data,
            );
        for variable in variables_definition {
            if variable.name.to_string()[1..].find('_').is_some() {
                let span = variable.name.span();
                res.push(self.create_diag((span.start(), span.end()), file));
            }
        }

        let variables_declaration =
            osmium_libs_solidity_ast_extractor::retriever::retrieve_variable_declaration_nodes(
                &file.data,
            );
        for variable in variables_declaration {
            if variable.name.is_some() {
                let name = variable.name.unwrap();
                if name.to_string()[1..].find('_').is_some() {
                    let span = name.span();
                    res.push(self.create_diag((span.start(), span.end()), file));
                }
            }
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Variable name must be in mixedCase. (Does not check IMMUTABLES, use immutable-vars-naming)".to_string(),
            category: "naming".to_string(),
            example_config: "{\"id\": \"var-name-mixedcase\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/naming/var_name_mixedcase.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/VarNameMixedcase".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl VarNameMixedCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = VarNameMixedCase { data };
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
