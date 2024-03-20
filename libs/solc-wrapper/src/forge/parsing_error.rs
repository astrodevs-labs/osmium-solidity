use thiserror::Error;



#[derive(Error, Debug)]
pub struct ErrorLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub length: usize
}

#[derive(Error, Debug)]
pub struct ParsingError {
    pub error: String,
    pub location: ErrorLocation,
}

impl std::fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ErrorLocation in {} at line {}, column {}", self.file, self.line, self.column)
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParsingError: {}", self.error)
    }
}