use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Workspace loading error: {0}")]
    InvalidRootPath(#[from] glob::PatternError),

    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),

    #[error("Executable error: foundry executable not found")]
    FoundryExecutableNotFound,

    #[error("Invalid foundry version: does not support --format-json")]
    InvalidFoundryVersion,

    #[error("Executable error: {0}")]
    ExecutableError(std::io::Error),

    #[error("No executable build-info file: {0}")]
    NoExecutableBuildInfoFile(String),

    #[error("Invalid json output: {0}")]
    InvalidJsonOutput(#[from] serde_json::Error),

    #[error("No build info file found")]
    NoBuildInfo,

    #[error("Cannot read build info file")]
    ReadBuildInfo(#[from] std::io::Error),

    #[error("filesystem error: {0}")]
    FileSystemError(std::io::Error),
}
