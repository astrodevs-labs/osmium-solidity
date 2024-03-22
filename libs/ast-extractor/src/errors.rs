use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("Alloy extraction error: {0}")]
    Alloy(String),
    #[error("Compiler error: {0}")]
    Compiler(#[from] osmium_libs_solidity_foundry_wrapper::Error),
    #[error("Cannot read source file")]
    ReadSourceFile(std::io::Error),
    #[error("JSON parsing error: {0}")]
    AstParsing(#[from] serde_json::Error),
}
