use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::Spanned;

// global
pub const RULE_ID: &str = "func-visibility";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str =
    "Explicitly mark visibility in function (public, private, internal, external)";
pub const DEFAULT_IGNORE_CONSTRUCTORS: bool = true;

pub struct FuncVisibility {
    ignore_constructors: bool,
    data: RuleEntry,
}

impl FuncVisibility {
    fn create_diag(
        &self,
        location: (
            osmium_libs_solidity_ast_extractor::LineColumn,
            osmium_libs_solidity_ast_extractor::LineColumn,
        ),
        file: &SolidFile,
    ) -> LintDiag {
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

impl RuleType for FuncVisibility {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts =
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            for function in
                osmium_libs_solidity_ast_extractor::retriever::retrieve_functions_nodes(&contract)
            {
                if function.attributes.visibility().is_some()
                    || (function.kind.is_constructor() && self.ignore_constructors)
                {
                    continue;
                }
                if function.kind.is_function() {
                    res.push(
                        self.create_diag(
                            (function.kind.span().start(), function.span().end()),
                            file,
                        ),
                    );
                } else {
                    let span = function.kind.span();
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
            description: "Explicitly mark visibility in function.".to_string(),
            category: "security".to_string(),
            example_config: "{\"id\": \"func-visibility\", \"severity\": \"WARNING\", \"data\": {\"ignoreConstructors\": false}}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/security/func_visibility.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/FuncVisibility".to_string(),
            options: vec![Options {
                description: "A JSON object with a single property \"ignoreConstructors\" specifying if the rule should ignore constructors. (Note: This is required to be true for Solidity >=0.7.0 and false for <0.7.0)".to_string(),
                default: "{\"ignoreConstructors\":false}".to_string(),
            }],
            examples: Examples {
                good: vec![Example {
                    description: "Functions explicitly marked with visibility".to_string(),
                    code: "function b() internal { }\nfunction b() external { }\nfunction b() private { }\nfunction b() public { }\nconstructor() public { }".to_string(),
                }],
                bad: vec![Example {
                    description: "Functions without explicitly marked visibility".to_string(),
                    code: "function b() { }".to_string(),
                }],
            },
        }
    }
}

impl FuncVisibility {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut ignore_constructors = DEFAULT_IGNORE_CONSTRUCTORS;

        if let Some(data) = &data.data {
            if !data["ignoreConstructors"].is_null()
                && data["ignoreConstructors"].as_bool().is_some()
            {
                ignore_constructors = data["ignoreConstructors"].as_bool().unwrap();
            } else {
                eprintln!("{} rule : bad config data", RULE_ID);
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = FuncVisibility {
            ignore_constructors,
            data,
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(serde_json::json!({
                "ignoreConstructors": DEFAULT_IGNORE_CONSTRUCTORS,
            })),
        }
    }
}
