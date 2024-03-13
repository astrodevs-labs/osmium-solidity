use osmium_libs_solidity_ast_extractor::{LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "import-on-top";

// specific
const DEFAULT_MESSAGE: &str = "Import statements must be on top";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct ImportOnTop {
    data: RuleEntry,
}

impl ImportOnTop {
    fn create_diag(&self, file: &SolidFile, location: (LineColumn, LineColumn)) -> LintDiag {
        let range = Range {
            start: Position {
                line: location.0.line,
                character: location.0.column,
            },
            end: Position {
                line: location.1.line,
                character: location.1.column,
            },
        };
        LintDiag {
            id: RULE_ID.to_string(),
            range,
            message: DEFAULT_MESSAGE.to_string(),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for ImportOnTop {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut last_import_location = 0;

        for i in 0..file.data.items.len() {
            match &file.data.items[i] {
                osmium_libs_solidity_ast_extractor::Item::Pragma(_) => {
                    continue;
                }
                osmium_libs_solidity_ast_extractor::Item::Import(_) => {
                    last_import_location = i;
                }
                _ => {
                    break;
                }
            }
        }

        for i in 0..file.data.items.len() {
            if let osmium_libs_solidity_ast_extractor::Item::Import(import) = &file.data.items[i] {
                if i > last_import_location {
                    let location = (import.span().start(), import.span().end());
                    res.push(self.create_diag(file, location));
                }
            }
        }

        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Import statements must be on top.".to_string(),
            category: "order".to_string(),
            example_config: "{\"id\": \"import-on-top\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/order/import_on_top.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/ImportOnTop".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl ImportOnTop {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = ImportOnTop { data };
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
