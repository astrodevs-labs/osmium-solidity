use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "use-forbidden-name";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Avoid to use letters 'I', 'l', 'O' as identifiers";

pub struct UseForbiddenName {
    data: RuleEntry,
}

impl UseForbiddenName {
    fn create_diag(&self, file: &SolidFile, location: (LineColumn, LineColumn)) -> LintDiag {
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

impl RuleType for UseForbiddenName {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let blacklist = ['I', 'l', 'O'];

        let contracts = retriever::retrieve_contract_nodes(&file.data);

        // var def => contract def
        for contract in contracts.iter() {
            for node_var in contract.body.iter() {
                let var = match node_var {
                    Item::Variable(var) => var,
                    _ => continue,
                };
                if var.name.as_string().len() == 1
                    && blacklist.contains(&var.name.as_string().chars().next().unwrap())
                {
                    let location = (var.name.span().start(), var.name.span().end());
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
            description: "Avoid to use letters 'I', 'l', 'O' as identifiers.".to_string(),
            category: "naming".to_string(),
            example_config: "{\"id\": \"use-forbidden-name\", \"severity\": \"WARNING\", \"data\": []}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/naming/use_forbidden_name.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/UseForbiddenName".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl UseForbiddenName {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = UseForbiddenName { data };
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
