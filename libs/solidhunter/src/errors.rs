use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolidHunterError {
    // Linter errors
    #[error("SolidHunterError: Ast Extractor error occured")]
    AstError(#[from] osmium_libs_solidity_ast_extractor::errors::ExtractError),
    #[error("SolidHunterError: Something went wrong with the file during parsing")]
    ParsingError(#[from] std::io::Error),
    #[error("SolidHunterError: Serde error occured")]
    SerdeError(#[from] serde_json::Error),
    #[error("SolidHunterError: Something went wrong")]
    LinterError(String),

    // RulesError
    #[error("SolidHunterError: IO error occured with Rules")]
    IoError(std::io::Error),
}
