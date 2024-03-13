use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "avoid-tx-origin";

// specific
const DEFAULT_MESSAGE: &str = "Avoid to use tx.origin";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

struct ExprVisitor {
    exprs: Vec<ExprMember>,
}

impl ExprVisitor {
    pub fn new() -> Self {
        Self { exprs: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ExprVisitor {
    fn visit_expr_member(&mut self, i: &'ast ExprMember) {
        self.exprs.push(i.clone());
        visit::visit_expr_member(self, i);
    }
}

pub struct AvoidTxOrigin {
    data: RuleEntry,
}

impl AvoidTxOrigin {
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

impl RuleType for AvoidTxOrigin {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut visitor = ExprVisitor::new();
        for contract in
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data)
        {
            visitor.visit_item_contract(&contract);
        }

        for expr in visitor.exprs {
            if let Expr::Ident(ident) = &*expr.expr {
                if let Expr::Ident(ident2) = &*expr.member {
                    if ident == "tx" && ident2 == "origin" {
                        let location = (expr.span().start(), expr.span().end());
                        res.push(self.create_diag(location, file));
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
            description: "Avoid to use tx.origin.".to_string(),
            category: "security".to_string(),
            example_config: " {\"id\": \"avoid-tx-origin\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/security/avoid_tx_origin.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/AvoidTxOrigin".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl AvoidTxOrigin {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = AvoidTxOrigin { data };
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
