use super::{Range, Severity};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LintDiag {
    /// The range at which the message applies.
    pub range: Range,

    /// The diagnostic's severity.
    pub severity: Severity,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's code. Can be omitted.
    pub code: Option<NumberOrString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'.
    pub source: Option<String>,

    /// The diagnostic's message.
    pub message: String,

    pub id: String,

    pub uri: Uri,
}

impl fmt::Display for LintDiag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{}: {}\n  --> {}:{}:{}",
            self.severity,
            self.message,
            self.uri,
            self.range.start.line,
            self.range.start.character,
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

type Uri = String;
