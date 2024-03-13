use crate::rules::types::{RuleEntry, RulesMap};
use std::collections::HashMap;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![]
}

pub fn create_rules() -> RulesMap {
    HashMap::new()
}
