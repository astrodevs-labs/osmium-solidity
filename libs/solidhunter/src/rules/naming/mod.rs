use crate::rules::naming::const_name_snakecase::ConstNameSnakeCase;
use crate::rules::naming::contract_name_camelcase::ContractNameCamelCase;
use crate::rules::naming::event_name_camelcase::EventNameCamelCase;
use crate::rules::naming::foundry_test_functions::FoundryTestFunctions;
use crate::rules::naming::func_name_mixedcase::FuncNameMixedCase;
use crate::rules::naming::func_param_name_mixedcase::FuncParamNameMixedCase;
use crate::rules::naming::modifier_name_mixedcase::ModifierNameMixedcase;
use crate::rules::naming::named_parameters_mapping::NamedParametersMapping;
use crate::rules::naming::private_vars_leading_underscore::PrivateVarsLeadingUnderscore;
use crate::rules::naming::use_forbidden_name::UseForbiddenName;
use crate::rules::naming::var_name_mixedcase::VarNameMixedCase;
use crate::rules::types::{RuleEntry, RulesMap};
use crate::rules::RuleBuilder;
use std::collections::HashMap;

#[macro_use]
pub(crate) mod func_param_name_mixedcase;
pub(crate) mod const_name_snakecase;
pub(crate) mod contract_name_camelcase;
pub(crate) mod event_name_camelcase;
pub(crate) mod foundry_test_functions;
pub(crate) mod func_name_mixedcase;
pub(crate) mod modifier_name_mixedcase;
pub(crate) mod named_parameters_mapping;
pub(crate) mod private_vars_leading_underscore;
pub(crate) mod use_forbidden_name;
pub(crate) mod var_name_mixedcase;

// List all rules

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        ContractNameCamelCase::create_default(),
        FuncNameMixedCase::create_default(),
        FuncParamNameMixedCase::create_default(),
        UseForbiddenName::create_default(),
        EventNameCamelCase::create_default(),
        ConstNameSnakeCase::create_default(),
        PrivateVarsLeadingUnderscore::create_default(),
        VarNameMixedCase::create_default(),
        ModifierNameMixedcase::create_default(),
        NamedParametersMapping::create_default(),
        FoundryTestFunctions::create_default(),
    ]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(
        contract_name_camelcase::RULE_ID.to_string(),
        ContractNameCamelCase::create,
    );
    rules.insert(
        named_parameters_mapping::RULE_ID.to_string(),
        NamedParametersMapping::create,
    );
    rules.insert(
        func_name_mixedcase::RULE_ID.to_string(),
        FuncNameMixedCase::create,
    );
    rules.insert(
        func_param_name_mixedcase::RULE_ID.to_string(),
        FuncParamNameMixedCase::create,
    );
    rules.insert(
        use_forbidden_name::RULE_ID.to_string(),
        UseForbiddenName::create,
    );
    rules.insert(
        event_name_camelcase::RULE_ID.to_string(),
        EventNameCamelCase::create,
    );
    rules.insert(
        const_name_snakecase::RULE_ID.to_string(),
        ConstNameSnakeCase::create,
    );
    rules.insert(
        private_vars_leading_underscore::RULE_ID.to_string(),
        PrivateVarsLeadingUnderscore::create,
    );
    rules.insert(
        var_name_mixedcase::RULE_ID.to_string(),
        VarNameMixedCase::create,
    );
    rules.insert(
        modifier_name_mixedcase::RULE_ID.to_string(),
        ModifierNameMixedcase::create,
    );
    rules.insert(
        foundry_test_functions::RULE_ID.to_string(),
        FoundryTestFunctions::create,
    );

    rules
}
