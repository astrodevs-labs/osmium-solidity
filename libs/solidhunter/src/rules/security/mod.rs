use crate::rules::types::{RuleEntry, RulesMap};
use std::collections::HashMap;

#[macro_use]
pub(crate) mod no_inline_assembly;
pub(crate) mod avoid_tx_origin;
pub(crate) mod func_visibility;
pub(crate) mod not_rely_on_time;
pub(crate) mod state_visibility;

// List all rules
use crate::rules::security::avoid_tx_origin::AvoidTxOrigin;
use crate::rules::security::func_visibility::FuncVisibility;
use crate::rules::security::no_inline_assembly::NoInlineAssembly;
use crate::rules::security::not_rely_on_time::NotRelyOnTime;
use crate::rules::security::state_visibility::StateVisibility;

use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        NoInlineAssembly::create_default(),
        StateVisibility::create_default(),
        NotRelyOnTime::create_default(),
        FuncVisibility::create_default(),
        AvoidTxOrigin::create_default(),
    ]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(
        no_inline_assembly::RULE_ID.to_string(),
        NoInlineAssembly::create,
    );
    rules.insert(func_visibility::RULE_ID.to_string(), FuncVisibility::create);
    rules.insert(
        state_visibility::RULE_ID.to_string(),
        StateVisibility::create,
    );
    rules.insert(avoid_tx_origin::RULE_ID.to_string(), AvoidTxOrigin::create);
    rules.insert(not_rely_on_time::RULE_ID.to_string(), NotRelyOnTime::create);

    rules
}
