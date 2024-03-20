use thiserror::Error;

#[derive(Debug)]
pub enum CommandType {
    ParseFile,
    ParseStdin,
}

#[derive(Error, Debug)]
pub struct CommandError {
    pub command_type: CommandType,
    pub error: String,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CommandError: {}", self.error)
    }
}