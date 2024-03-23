use osmium_libs_solidity_ast_extractor::errors::ExtractError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReferencesError {
    #[error("Extract error: {0}")]
    Solc(#[from] ExtractError),
}
