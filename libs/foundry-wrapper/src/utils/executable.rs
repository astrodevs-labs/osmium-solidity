use crate::Error;
use std::path::PathBuf;

pub fn find_forge_executable() -> Result<PathBuf, Error> {
    which::which("forge").map_err(|_| Error::FoundryExecutableNotFound)
}

pub fn check_executable_argument(executable_path: &str) -> Result<(), Error> {
    let output = std::process::Command::new(executable_path)
        .arg("compile")
        .arg("--format-json")
        .output()
        .map_err(Error::ExecutableError)?;

    let stderr_str = String::from_utf8_lossy(&output.stderr);
    if stderr_str.contains("unexpected argument '--format-json'") {
        return Err(Error::InvalidFoundryVersion);
    }
    Ok(())
}
