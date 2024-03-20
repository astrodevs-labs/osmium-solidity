use thiserror::Error;

use crate::forge::error::CommandError;

#[derive(Error, Debug)]
pub enum SolcWrapperError {
    #[error("Solc error: {0}")]
    Solc(#[from] CommandError),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("No build info produced by foundry")]
    NoBuildInfo,
    #[error("Cannot read build info file")]
    ReadBuildInfo(#[from] std::io::Error),
    #[error("Cannot read source file")]
    ReadSourceFile(std::io::Error)
}