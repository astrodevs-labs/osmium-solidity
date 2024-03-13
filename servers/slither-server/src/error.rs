use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlitherError {
    #[error("Error while runing slither: {0}")]
    Unknown(String),
    #[error("Error while runing the slither command: {0}")]
    IoCommandError(#[from] std::io::Error),
    #[error("Error while parsing slither output: {0}")]
    ParsingFailed(#[from] serde_json::Error),
    #[error("Error when trying to find the foundry.toml file")]
    FoundryTomlNotFound,
}
