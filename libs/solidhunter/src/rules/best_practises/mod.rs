use crate::rules::types::{RuleEntry, RulesMap};
use std::collections::HashMap;

#[macro_use]
pub mod max_line_length;
pub mod custom_errors;
pub mod explicit_types;
pub mod function_max_lines;
pub mod max_states_count;
pub mod no_console;
pub mod no_empty_block;
pub mod no_global_import;
pub mod one_contract_per_file;
pub mod payable_fallback;
pub mod reason_string;

// List all rules
use crate::rules::best_practises::custom_errors::CustomErrors;
use crate::rules::best_practises::explicit_types::ExplicitTypes;
use crate::rules::best_practises::function_max_lines::FunctionMaxLines;
use crate::rules::best_practises::max_line_length::MaxLineLength;
use crate::rules::best_practises::max_states_count::MaxStatesCount;
use crate::rules::best_practises::no_console::NoConsole;
use crate::rules::best_practises::no_empty_block::NoEmptyBlock;
use crate::rules::best_practises::no_global_import::NoGlobalImport;
use crate::rules::best_practises::one_contract_per_file::OneContractPerFile;
use crate::rules::best_practises::payable_fallback::PayableFallback;
use crate::rules::best_practises::reason_string::ReasonString;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        MaxLineLength::create_default(),
        MaxStatesCount::create_default(),
        FunctionMaxLines::create_default(),
        ReasonString::create_default(),
        NoConsole::create_default(),
        OneContractPerFile::create_default(),
        CustomErrors::create_default(),
        NoGlobalImport::create_default(),
        NoEmptyBlock::create_default(),
        ExplicitTypes::create_default(),
        PayableFallback::create_default(),
    ]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(max_line_length::RULE_ID.to_string(), MaxLineLength::create);

    rules.insert(
        max_states_count::RULE_ID.to_string(),
        MaxStatesCount::create,
    );
    rules.insert(
        function_max_lines::RULE_ID.to_string(),
        FunctionMaxLines::create,
    );
    rules.insert(custom_errors::RULE_ID.to_string(), CustomErrors::create);
    rules.insert(reason_string::RULE_ID.to_string(), ReasonString::create);
    rules.insert(
        one_contract_per_file::RULE_ID.to_string(),
        OneContractPerFile::create,
    );
    rules.insert(
        no_global_import::RULE_ID.to_string(),
        NoGlobalImport::create,
    );
    rules.insert(no_empty_block::RULE_ID.to_string(), NoEmptyBlock::create);
    rules.insert(explicit_types::RULE_ID.to_string(), ExplicitTypes::create);
    rules.insert(no_console::RULE_ID.to_string(), NoConsole::create);
    rules.insert(
        payable_fallback::RULE_ID.to_string(),
        PayableFallback::create,
    );

    rules
}
