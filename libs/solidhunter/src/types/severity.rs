use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Severity {
    /// Reports an error.
    ERROR = 1,
    /// Reports a warning.
    WARNING = 2,
    /// Reports an information.
    INFO = 3,
    /// Reports a hint.
    HINT = 4,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self {
            Severity::ERROR => "error".to_string().red(),
            Severity::WARNING => "warning".to_string().yellow(),
            Severity::INFO => "info".to_string().blue(),
            Severity::HINT => "hint".to_string().green(),
        };
        write!(f, "{}", severity)
    }
}
