use crate::rules::types::{RuleEntry, RulesMap};
use std::collections::HashMap;

#[macro_use]
pub(crate) mod import_on_top;
pub(crate) mod ordering;
pub(crate) mod visibility_modifier_order;

// List all rules
use crate::rules::order::import_on_top::ImportOnTop;
use crate::rules::order::ordering::Ordering;
use crate::rules::order::visibility_modifier_order::VisibilityModiferOrder;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        ImportOnTop::create_default(),
        Ordering::create_default(),
        VisibilityModiferOrder::create_default(),
    ]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(import_on_top::RULE_ID.to_string(), ImportOnTop::create);
    rules.insert(ordering::RULE_ID.to_string(), Ordering::create);
    rules.insert(
        visibility_modifier_order::RULE_ID.to_string(),
        VisibilityModiferOrder::create,
    );

    rules
}
