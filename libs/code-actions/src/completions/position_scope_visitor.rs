use std::vec;

use crate::types::SPINode;
use crate::utils::is_node_in_range;
use crate::Position;
use solc_ast_rs_types::types::*;
use solc_ast_rs_types::visit;
use solc_ast_rs_types::visit::*;

pub struct PositionScopeVisitor {
    pub contract: Option<ContractDefinition>,
    pub spi: Vec<SPINode>,
    pub imports: Vec<ImportDirective>,
    position: Position,
    source: String,
}

impl<'ast> Visit<'ast> for PositionScopeVisitor {
    fn visit_contract_definition(&mut self, contract: &'ast ContractDefinition) {
        if is_node_in_range(&contract.src, &self.position, &self.source) {
            self.contract = Some(contract.clone());
            visit::visit_contract_definition(self, contract);
        }
    }

    fn visit_block(&mut self, block: &'ast Block) {
        if is_node_in_range(&block.src, &self.position, &self.source) {
            self.spi.push(SPINode::Block(block.clone()));
            visit::visit_block(self, block);
        }
    }

    fn visit_struct_definition(&mut self, struct_def: &'ast StructDefinition) {
        if is_node_in_range(&struct_def.src, &self.position, &self.source) {
            visit::visit_struct_definition(self, struct_def);
        }
    }

    fn visit_enum_definition(&mut self, enum_def: &'ast EnumDefinition) {
        if is_node_in_range(&enum_def.src, &self.position, &self.source) {
            visit::visit_enum_definition(self, enum_def);
        }
    }

    fn visit_unchecked(&mut self, unchecked: &'ast UncheckedBlock) {
        if is_node_in_range(&unchecked.src, &self.position, &self.source) {
            self.spi.push(SPINode::UncheckedBlock(unchecked.clone()));
        }
        visit::visit_unchecked(self, unchecked);
    }

    fn visit_for(&mut self, r#for: &'ast ForStatement) {
        if let ForStatementBody::Block(block) = &r#for.body {
            if is_node_in_range(&block.src, &self.position, &self.source) {
                self.spi.push(SPINode::Block(block.clone()));
                visit::visit_for(self, r#for);
                return;
            }
        }
        if is_node_in_range(&r#for.src, &self.position, &self.source) {
            self.spi.push(SPINode::ForStatement(r#for.clone()));
        }
    }

    fn visit_function_definition(&mut self, function: &'ast FunctionDefinition) {
        if is_node_in_range(&function.src, &self.position, &self.source) {
            self.spi.push(SPINode::FunctionDefinition(function.clone()));
            visit::visit_function_definition(self, function);
        } else if let Some(body) = &function.body {
            if is_node_in_range(&body.src, &self.position, &self.source) {
                self.spi.push(SPINode::FunctionDefinition(function.clone()));
                visit::visit_function_definition(self, function);
            }
        }
    }

    fn visit_try(&mut self, r#try: &'ast TryStatement) {
        if is_node_in_range(&r#try.src, &self.position, &self.source) {
            self.spi.push(SPINode::TryStatement(r#try.clone()));
        } else {
            for clause in &r#try.clauses {
                if is_node_in_range(&clause.src, &self.position, &self.source) {
                    self.spi.push(SPINode::TryStatement(r#try.clone()));
                    self.spi.push(SPINode::TryCatchClause(clause.clone()));
                    visit::visit_try(self, r#try);
                    return;
                }
            }
        }
    }

    fn visit_variable_declaration(&mut self, variable: &'ast VariableDeclaration) {
        if is_node_in_range(&variable.src, &self.position, &self.source) {
            self.spi
                .push(SPINode::VariableDeclaration(variable.clone()));
            visit::visit_variable_declaration(self, variable);
        }
    }

    fn visit_event_definition(&mut self, event: &'ast EventDefinition) {
        if is_node_in_range(&event.src, &self.position, &self.source) {
            visit::visit_event_definition(self, event);
        }
    }

    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        if is_node_in_range(&enum_value.src, &self.position, &self.source) {
            visit::visit_enum_value(self, enum_value);
        }
    }

    fn visit_import_directive(&mut self, import: &'ast ImportDirective) {
        self.imports.push(import.clone());
        visit::visit_import_directive(self, import);
    }
}

impl PositionScopeVisitor {
    pub fn new(source: String, position: Position) -> Self {
        PositionScopeVisitor {
            spi: vec![],
            contract: None,
            imports: vec![],
            position,
            source,
        }
    }

    pub fn find(
        &mut self,
        src: &SourceUnit,
    ) -> (
        Option<ContractDefinition>,
        Vec<SPINode>,
        Vec<ImportDirective>,
    ) {
        self.visit_source_unit(src);
        (
            self.contract.clone(),
            self.spi.clone(),
            self.imports.clone(),
        )
    }
}
