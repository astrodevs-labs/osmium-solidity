use std::collections::HashMap;

use osmium_libs_solidity_ast_extractor::{visit, FunctionKind, Spanned, Visibility, Visit};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "ordering";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Invalid ordering of items in the file";

struct OrderingVisitor {
    file: SolidFile,
    data: RuleEntry,
    authorized_file_items: HashMap<FileItemType, Vec<Option<FileItemType>>>,
    authorized_contract_items: HashMap<ContractItemType, Vec<Option<ContractItemType>>>,
    file_current_item: Option<FileItemType>,
    contract_current_item: Option<ContractItemType>,
    inside_contract: bool,
    reports: Vec<LintDiag>,
}

impl OrderingVisitor {
    fn new(file: SolidFile, data: RuleEntry) -> OrderingVisitor {
        let authorized_file_items: HashMap<FileItemType, Vec<Option<FileItemType>>> = [
            (FileItemType::Pragma, vec![None]),
            (
                FileItemType::Import,
                vec![None, Some(FileItemType::Pragma), Some(FileItemType::Import)],
            ),
            (
                FileItemType::Enum,
                vec![
                    None,
                    Some(FileItemType::Pragma),
                    Some(FileItemType::Import),
                    Some(FileItemType::Enum),
                ],
            ),
            (
                FileItemType::Struct,
                vec![
                    None,
                    Some(FileItemType::Pragma),
                    Some(FileItemType::Import),
                    Some(FileItemType::Enum),
                    Some(FileItemType::Struct),
                ],
            ),
            (
                FileItemType::ContractInterface,
                vec![
                    None,
                    Some(FileItemType::Pragma),
                    Some(FileItemType::Import),
                    Some(FileItemType::Enum),
                    Some(FileItemType::Struct),
                    Some(FileItemType::ContractInterface),
                ],
            ),
            (
                FileItemType::ContractLibrary,
                vec![
                    None,
                    Some(FileItemType::Pragma),
                    Some(FileItemType::Import),
                    Some(FileItemType::Enum),
                    Some(FileItemType::Struct),
                    Some(FileItemType::ContractInterface),
                    Some(FileItemType::ContractLibrary),
                ],
            ),
            (
                FileItemType::Contract,
                vec![
                    None,
                    Some(FileItemType::Pragma),
                    Some(FileItemType::Import),
                    Some(FileItemType::Enum),
                    Some(FileItemType::Struct),
                    Some(FileItemType::ContractInterface),
                    Some(FileItemType::ContractLibrary),
                    Some(FileItemType::Contract),
                ],
            ),
        ]
        .iter()
        .cloned()
        .collect();

        let authorized_contract_items: HashMap<ContractItemType, Vec<Option<ContractItemType>>> = [
            (
                ContractItemType::Udt,
                vec![None, Some(ContractItemType::Udt)],
            ),
            (
                ContractItemType::Struct,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                ],
            ),
            (
                ContractItemType::Enum,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                ],
            ),
            (
                ContractItemType::Property,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                ],
            ),
            (
                ContractItemType::Event,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                ],
            ),
            (
                ContractItemType::Modifier,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                ],
            ),
            (
                ContractItemType::Constructor,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                ],
            ),
            (
                ContractItemType::Receive,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                    Some(ContractItemType::Constructor),
                ],
            ),
            (
                ContractItemType::FallBack,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                    Some(ContractItemType::Constructor),
                    Some(ContractItemType::Receive),
                ],
            ),
            (
                ContractItemType::ExternalFunction,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                    Some(ContractItemType::Constructor),
                    Some(ContractItemType::Receive),
                    Some(ContractItemType::FallBack),
                    Some(ContractItemType::ExternalFunction),
                ],
            ),
            (
                ContractItemType::PublicFunction,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                    Some(ContractItemType::Constructor),
                    Some(ContractItemType::Receive),
                    Some(ContractItemType::FallBack),
                    Some(ContractItemType::ExternalFunction),
                    Some(ContractItemType::PublicFunction),
                ],
            ),
            (
                ContractItemType::InternalFunction,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                    Some(ContractItemType::Constructor),
                    Some(ContractItemType::Receive),
                    Some(ContractItemType::FallBack),
                    Some(ContractItemType::ExternalFunction),
                    Some(ContractItemType::PublicFunction),
                    Some(ContractItemType::InternalFunction),
                ],
            ),
            (
                ContractItemType::PrivateFunction,
                vec![
                    None,
                    Some(ContractItemType::Udt),
                    Some(ContractItemType::Struct),
                    Some(ContractItemType::Enum),
                    Some(ContractItemType::Property),
                    Some(ContractItemType::Event),
                    Some(ContractItemType::Modifier),
                    Some(ContractItemType::Constructor),
                    Some(ContractItemType::Receive),
                    Some(ContractItemType::FallBack),
                    Some(ContractItemType::ExternalFunction),
                    Some(ContractItemType::PublicFunction),
                    Some(ContractItemType::InternalFunction),
                    Some(ContractItemType::PrivateFunction),
                ],
            ),
        ]
        .iter()
        .cloned()
        .collect();
        OrderingVisitor {
            file,
            data,
            authorized_file_items,
            authorized_contract_items,
            file_current_item: None,
            contract_current_item: None,
            inside_contract: false,
            reports: Vec::new(),
        }
    }

    fn create_diag(
        &self,
        file: &SolidFile,
        location: (
            osmium_libs_solidity_ast_extractor::LineColumn,
            osmium_libs_solidity_ast_extractor::LineColumn,
        ),
    ) -> LintDiag {
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

    fn is_authorized_file_item(&self, item: FileItemType) -> bool {
        if let Some(authorized_items) = self.authorized_file_items.get(&item) {
            let res = authorized_items.contains(&self.file_current_item);
            return res;
        }
        true
    }

    fn is_authorized_contract_item(&self, item: ContractItemType) -> bool {
        if let Some(authorized_items) = self.authorized_contract_items.get(&item) {
            return authorized_items.contains(&self.contract_current_item);
        }
        true
    }
}

impl<'ast> Visit<'ast> for OrderingVisitor {
    fn visit_pragma_directive(
        &mut self,
        pragma: &'ast osmium_libs_solidity_ast_extractor::PragmaDirective,
    ) {
        if !self.is_authorized_file_item(FileItemType::Pragma) {
            let location = (pragma.span().start(), pragma.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.file_current_item = Some(FileItemType::Pragma);
        }
    }

    fn visit_import_directive(
        &mut self,
        import: &'ast osmium_libs_solidity_ast_extractor::ImportDirective,
    ) {
        if !self.is_authorized_file_item(FileItemType::Import) {
            let location = (import.span().start(), import.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.file_current_item = Some(FileItemType::Import);
        }
    }

    fn visit_item_enum(&mut self, enum_def: &'ast osmium_libs_solidity_ast_extractor::ItemEnum) {
        if !self.is_authorized_file_item(FileItemType::Enum) {
            let location = (enum_def.span().start(), enum_def.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.file_current_item = Some(FileItemType::Enum);
        }
    }

    fn visit_item_struct(
        &mut self,
        struct_def: &'ast osmium_libs_solidity_ast_extractor::ItemStruct,
    ) {
        if !self.is_authorized_file_item(FileItemType::Struct) {
            let location = (struct_def.span().start(), struct_def.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.file_current_item = Some(FileItemType::Struct);
        }
    }

    fn visit_item_contract(
        &mut self,
        contract_def: &'ast osmium_libs_solidity_ast_extractor::ItemContract,
    ) {
        if contract_def.is_interface()
            && !self.is_authorized_file_item(FileItemType::ContractInterface)
            || contract_def.is_library()
                && !self.is_authorized_file_item(FileItemType::ContractLibrary)
            || !self.is_authorized_file_item(FileItemType::Contract)
        {
            let location = (contract_def.span().start(), contract_def.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.file_current_item = Some(FileItemType::Contract);
        }
        self.contract_current_item = None;
        self.inside_contract = true;
        visit::visit_item_contract(self, contract_def);
        self.inside_contract = false;
    }

    fn visit_item_udt(&mut self, udt: &'ast osmium_libs_solidity_ast_extractor::ItemUdt) {
        if !self.is_authorized_contract_item(ContractItemType::Udt) {
            let location = (udt.span().start(), udt.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.contract_current_item = Some(ContractItemType::Udt);
        }
    }

    fn visit_variable_definition(
        &mut self,
        var: &'ast osmium_libs_solidity_ast_extractor::VariableDefinition,
    ) {
        if !self.is_authorized_contract_item(ContractItemType::Property) {
            let location = (var.span().start(), var.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.contract_current_item = Some(ContractItemType::Property);
        }
    }

    fn visit_item_event(&mut self, event: &'ast osmium_libs_solidity_ast_extractor::ItemEvent) {
        if !self.is_authorized_contract_item(ContractItemType::Event) {
            let location = (event.span().start(), event.span().end());
            self.reports.push(self.create_diag(&self.file, location));
        } else {
            self.contract_current_item = Some(ContractItemType::Event);
        }
    }

    fn visit_item_function(
        &mut self,
        function: &'ast osmium_libs_solidity_ast_extractor::ItemFunction,
    ) {
        match function.kind {
            FunctionKind::Modifier(_) => {
                if !self.is_authorized_contract_item(ContractItemType::Modifier) {
                    let location = (function.span().start(), function.span().end());
                    self.reports.push(self.create_diag(&self.file, location));
                } else {
                    self.contract_current_item = Some(ContractItemType::Modifier);
                }
            }
            FunctionKind::Constructor(_) => {
                if !self.is_authorized_contract_item(ContractItemType::Constructor) {
                    let location = (function.span().start(), function.span().end());
                    self.reports.push(self.create_diag(&self.file, location));
                } else {
                    self.contract_current_item = Some(ContractItemType::Constructor);
                }
            }
            FunctionKind::Receive(_) => {
                if !self.is_authorized_contract_item(ContractItemType::Receive) {
                    let location = (function.span().start(), function.span().end());
                    self.reports.push(self.create_diag(&self.file, location));
                } else {
                    self.contract_current_item = Some(ContractItemType::Receive);
                }
            }
            FunctionKind::Fallback(_) => {
                if !self.is_authorized_contract_item(ContractItemType::FallBack) {
                    let location = (function.span().start(), function.span().end());
                    self.reports.push(self.create_diag(&self.file, location));
                } else {
                    self.contract_current_item = Some(ContractItemType::FallBack);
                }
            }
            FunctionKind::Function(_) => {
                let visibility = function.attributes.iter().find(|attr| {
                    matches!(
                        attr,
                        osmium_libs_solidity_ast_extractor::FunctionAttribute::Visibility(_)
                    )
                });
                let visibility = match visibility {
                    Some(osmium_libs_solidity_ast_extractor::FunctionAttribute::Visibility(
                        visibility,
                    )) => visibility,
                    _ => return,
                };

                match visibility {
                    Visibility::External(_) => {
                        if !self.is_authorized_contract_item(ContractItemType::ExternalFunction) {
                            let location = (function.span().start(), function.span().end());
                            self.reports.push(self.create_diag(&self.file, location));
                        } else {
                            self.contract_current_item = Some(ContractItemType::ExternalFunction);
                        }
                    }
                    Visibility::Public(_) => {
                        if !self.is_authorized_contract_item(ContractItemType::PublicFunction) {
                            let location = (function.span().start(), function.span().end());
                            self.reports.push(self.create_diag(&self.file, location));
                        } else {
                            self.contract_current_item = Some(ContractItemType::PublicFunction);
                        }
                    }
                    Visibility::Internal(_) => {
                        if !self.is_authorized_contract_item(ContractItemType::InternalFunction) {
                            let location = (function.span().start(), function.span().end());
                            self.reports.push(self.create_diag(&self.file, location));
                        } else {
                            self.contract_current_item = Some(ContractItemType::InternalFunction);
                        }
                    }
                    Visibility::Private(_) => {
                        if !self.is_authorized_contract_item(ContractItemType::PrivateFunction) {
                            let location = (function.span().start(), function.span().end());
                            self.reports.push(self.create_diag(&self.file, location));
                        } else {
                            self.contract_current_item = Some(ContractItemType::PrivateFunction);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ordering {
    data: RuleEntry,
}

impl RuleType for Ordering {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut visitor = OrderingVisitor::new(file.clone(), self.data.clone());
        visitor.visit_file(&file.data);
        visitor.reports
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            description: "Check order of elements in file and inside each contract, according to the style guide.".to_string(),
            category: "order".to_string(),
            example_config: "{\"id\": \"ordering\", \"severity\": \"WARNING\"}".to_string(),
            source_link: "https://github.com/astrodevs-labs/osmium/blob/main/toolchains/solidity/core/crates/linter-lib/src/rules/order/ordering.rs".to_string(),
            test_link: "https://github.com/astrodevs-labs/osmium/tree/main/toolchains/solidity/core/crates/linter-lib/testdata/Ordering".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl Ordering {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = Ordering { data };
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FileItemType {
    Pragma,
    Import,
    Enum,
    Struct,
    ContractInterface,
    ContractLibrary,
    Contract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContractItemType {
    Udt,
    Struct,
    Enum,
    Property,
    Event,
    Modifier,
    Constructor,
    Receive,
    FallBack,
    ExternalFunction,
    PublicFunction,
    InternalFunction,
    PrivateFunction,
}
