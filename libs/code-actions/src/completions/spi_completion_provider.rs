use solc_ast_rs_types::types::*;

use crate::types::{CompletionItem, CompletionItemKind, InteractableNode, SPINode};




pub struct SPICompletionProvider {
    pub spi: Vec<SPINode>,
}


impl SPICompletionProvider {

    pub fn new(spi: Vec<SPINode>) -> Self {
        Self {
            spi,
        }
    }

    pub fn inspect(&self) -> Vec<CompletionItem> {
        let mut completions = vec![];

        for path in self.spi.iter() {
            let items = match path {
                SPINode::FunctionDefinition(func) => self.search_function(func),
                SPINode::ForStatement(r#for) => self.search_for_statement(r#for),
                SPINode::Block(block) => self.search_block(block),
                SPINode::UncheckedBlock(block) => self.search_unchecked_block(block),
                _ => vec![],
            };
            completions.extend(items);
        }
        completions
    }

    fn search_function(&self, func: &FunctionDefinition) -> Vec<CompletionItem> {
        let mut items = vec![];
        for param in &func.parameters.parameters {
            items.push(CompletionItem {
                label: param.name.clone(),
                kind: CompletionItemKind::VARIABLE,
            });
        }
        for param in &func.return_parameters.parameters {
            if param.name != "" {
                items.push(CompletionItem {
                    label: param.name.clone(),
                    kind: CompletionItemKind::VARIABLE,
                });
            }
        }
        items
    }

    fn search_for_statement(&self, r#for: &ForStatement) -> Vec<CompletionItem> {
        let mut items = vec![];
        if let Some(ForStatementInitializationExpression::VariableDeclarationStatement(init_var)) = &r#for.initialization_expression {
            for var_decl in &init_var.declarations {
                if let Some(var_decl) = var_decl.clone() {
                    items.push(CompletionItem {
                        label: var_decl.name.clone(),
                        kind: CompletionItemKind::VARIABLE,
                });
                }
            }
        }
        items
    }

    fn search_block(&self, block: &Block) -> Vec<CompletionItem> {
        let mut items: Vec<CompletionItem> = vec![];
        if let Some(stmts) = &block.statements {
            for statement in stmts {
                if let Statement::VariableDeclarationStatement(var_decl) = statement {
                    for var_decl in var_decl.declarations.iter().flatten() {
                        items.push(CompletionItem {
                            label: var_decl.name.clone(),
                            kind: CompletionItemKind::VARIABLE,
                        });
                    }
                }
            }
        }
        items
    }

    fn search_unchecked_block(&self, block: &UncheckedBlock) -> Vec<CompletionItem> {
        let mut items: Vec<CompletionItem> = vec![];
        for statement in &block.statements {
            if let Statement::VariableDeclarationStatement(var_decl) = statement {
                for var_decl in var_decl.declarations.iter().flatten() {
                    items.push(CompletionItem {
                        label: var_decl.name.clone(),
                        kind: CompletionItemKind::VARIABLE,
                    });
                }
            }
        }
        items
    }

}
