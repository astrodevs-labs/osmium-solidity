use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct FoundryJsonFile {
    pub json: serde_json::Value,
    pub file: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectCompileOutput {
    errors: Vec<CompilationError>,
}

impl ProjectCompileOutput {
    pub fn get_errors(&self) -> &Vec<CompilationError> {
        self.errors.as_ref()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompilationError {
    #[serde(rename = "sourceLocation")]
    source_location: Option<SourceLocation>,
    #[serde(rename = "type")]
    typ: String,
    component: String,
    severity: String,
    #[serde(rename = "errorCode")]
    error_code: String,
    message: String,
    #[serde(rename = "formattedMessage")]
    formatted_message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SourceLocation {
    file: String,
    start: i32,
    end: i32,
}

impl CompilationError {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_file_path(&self) -> Option<String> {
        Some(self.source_location.clone()?.file.clone())
    }

    pub fn get_start_idx(&self) -> Option<i32> {
        Some(self.source_location.clone()?.start)
    }

    pub fn get_end_idx(&self) -> Option<i32> {
        Some(self.source_location.clone()?.end)
    }

    pub fn get_start_position(&self, source_content: &str) -> Option<Position> {
        let idx = self.get_start_idx()?;
        Position::from_index(idx, source_content)
    }

    pub fn get_end_position(&self, source_content: &str) -> Option<Position> {
        let idx = self.get_end_idx()?;
        Position::from_index(idx, source_content)
    }

    pub fn get_range(&self, source_content: &str) -> Option<Range> {
        Some(Range {
            start: self.get_start_position(source_content)?,
            end: self.get_end_position(source_content)?,
        })
    }

    pub fn get_severity(&self) -> Severity {
        self.severity.clone().into()
    }
}

/**
 * Position of error, 0 based indexes
 */
#[derive(Clone, Debug)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn from_index(idx: i32, source: &str) -> Option<Self> {
        let mut idx: usize = idx as usize;
        for (i, l) in source.split('\n').enumerate() {
            let line_length = l.len() + if l.ends_with('\r') { 2 } else { 1 };
            if idx < line_length {
                return Some(Self {
                    line: i as u32,
                    column: idx as u32,
                });
            }
            idx -= line_length
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl From<String> for Severity {
    fn from(severity: String) -> Self {
        match severity {
            s if s.to_uppercase() == "ERROR" => Self::Error,
            s if s.to_uppercase() == "WARNING" => Self::Warning,
            s if s.to_uppercase() == "INFO" => Self::Info,
            _ => Self::Info,
        }
    }
}
