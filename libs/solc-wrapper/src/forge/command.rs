use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;

use super::error::{CommandError, CommandType};

pub struct ForgeCommand {
    args: Vec<String>,
    bin_path : PathBuf,
    current_dir: String
}

impl Default for ForgeCommand {
    fn default() -> Self {
        ForgeCommand::new("forge")
    }
}

impl ForgeCommand {

    pub fn new(path: impl Into<PathBuf>) -> Self {
        ForgeCommand {
            args: Vec::new(),
            bin_path: path.into(),
            current_dir: String::from(".")
        }
    }

    pub fn current_dir(mut self, current_dir: String) -> Self {
        self.current_dir = current_dir;
        self
    }

    pub fn arg<T: Into<String>>(mut self, arg: T) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self = self.arg(arg);
        }
        self
    }

    pub fn execute(&self) -> Result<(), CommandError> {
        Command::new(&self.bin_path)
            .current_dir(&self.current_dir)
            .args(&self.args)
            .stdout(Stdio::piped())
            .output()
            .map_err(|e| {
                eprintln!("Forge Command Error: {}", e.to_string());
                CommandError { error: e.to_string(), command_type: CommandType::ParseFile }
        })?;
        Ok(())
    }
}