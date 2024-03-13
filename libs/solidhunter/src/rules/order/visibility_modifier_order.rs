use osmium_libs_solidity_ast_extractor::retriever::{
    retrieve_contract_nodes, retrieve_functions_nodes,
};
use osmium_libs_solidity_ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "visibility-modifier-order";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Visibility modifier must be first in list of modifiers";

pub struct VisibilityModiferOrder {
    data: RuleEntry,
}

impl RuleType for VisibilityModiferOrder {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let reports = check_visibility_modifier_order(_file);
        for report in reports {
            res.push(LintDiag {
                id: RULE_ID.to_string(),
                range: report,
                severity: self.data.severity,
                code: None,
                source: None,
                message: DEFAULT_MESSAGE.to_string(),
                uri: _file.path.clone(),
            });
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Visibility modifier must be first in list of modifiers.".to_string(),
            category: "order".to_string(),
            example_config: "{\"id\": \"visibility-modifier-order\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/order/visibility_modifier_order.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/VisibilityModifierOrder".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

fn check_visibility_modifier_order(file: &SolidFile) -> Vec<Range> {
    let mut reports = Vec::new();

    let contracts = retrieve_contract_nodes(&file.data);
    for contract in contracts {
        let functions = retrieve_functions_nodes(&contract);
        for function in functions {
            let mut is_attributes = false;
            function.attributes.iter().for_each(|attributes| {
                if attributes.modifier().is_some() || attributes.mutability().is_some() {
                    is_attributes = true;
                }
                if attributes.visibility().is_some() && is_attributes {
                    reports.push(Range {
                        start: Position {
                            line: attributes.span().start().line,
                            character: attributes.span().start().column,
                        },
                        end: Position {
                            line: attributes.span().end().line,
                            character: attributes.span().end().column,
                        },
                    });
                }
            });
        }
    }
    reports
}

impl VisibilityModiferOrder {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = VisibilityModiferOrder { data };
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
