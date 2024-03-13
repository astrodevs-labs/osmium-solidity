use osmium_libs_solidity_ast_extractor::{retriever, LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "modifier-name-mixedcase";

// specific
const DEFAULT_MESSAGE: &str = "Modifier name must be in mixedCase";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct ModifierNameMixedcase {
    data: RuleEntry,
}

impl ModifierNameMixedcase {
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

fn is_mixed_case(name: &str) -> bool {
    let has_leading_underscore = name.starts_with('_');

    if has_leading_underscore {
        if name.chars().nth(1).unwrap().is_ascii_uppercase() {
            return false;
        }
        for c in name.chars().skip(1) {
            if c == '_' || c == '-' {
                return false;
            }
        }
        return true;
    }

    if name.chars().next().unwrap().is_ascii_uppercase() {
        return false;
    }
    if name.contains('_') || name.contains('-') {
        return false;
    }

    true
}

impl RuleType for ModifierNameMixedcase {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts.iter() {
            let functions = retriever::retrieve_functions_nodes(contract);

            for function in functions {
                if function.kind.is_modifier() && !is_mixed_case(&function.name().as_string()) {
                    let span = function.name().span();
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
            description: "Modifier name must be in mixedCase.".to_string(),
            category: "naming".to_string(),
            example_config: "{\"id\": \"modifier-name-mixedcase\", \"severity\": \"WARNING\", \"data\": []}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/naming/modifier_name_mixedcase.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/ModifierNameMixedcase".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl ModifierNameMixedcase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = ModifierNameMixedcase { data };
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
