use crate::rules::create_rules;
use crate::rules::types::*;
use std::collections::HashMap;

pub struct RuleFactory {
    _buildables: RulesMap,
}

impl Default for RuleFactory {
    fn default() -> Self {
        RuleFactory {
            _buildables: create_rules(),
        }
    }
}

impl RuleFactory {
    pub fn new() -> RuleFactory {
        RuleFactory {
            _buildables: HashMap::new(),
        }
    }

    pub fn create_rule(&self, rule: RuleEntry) -> Box<dyn RuleType> {
        let rule_type = self._buildables.get(&rule.id);
        if rule_type.is_none() {
            panic!("Rule {} not found", &rule.id);
        }
        rule_type.unwrap()(rule)
    }
}
