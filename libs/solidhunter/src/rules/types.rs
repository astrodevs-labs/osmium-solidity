use crate::linter::SolidFile;
use crate::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Options {
    pub description: String,
    pub default: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Example {
    pub description: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Examples {
    pub good: Vec<Example>,
    pub bad: Vec<Example>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleDocumentation {
    pub id: String,
    pub severity: Severity,
    pub description: String,
    pub category: String,
    pub example_config: String,
    pub source_link: String,
    pub test_link: String,
    pub options: Vec<Options>,
    pub examples: Examples,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleEntry {
    pub id: String,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub name: String,
    pub rules: Vec<RuleEntry>,
}

pub trait RuleType: Send + Sync + 'static {
    fn diagnose(&self, file: &SolidFile, files: &[SolidFile]) -> Vec<LintDiag>;
    fn get_documentation(&self) -> RuleDocumentation;
}

pub type RulesMap = HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>>;
