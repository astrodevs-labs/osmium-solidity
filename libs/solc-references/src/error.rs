use thiserror::Error;
use solc_wrapper::SolcWrapperError;

#[derive(Error, Debug)]
pub enum ReferencesError {
    #[error("Solc error: {0}")]
    Solc(#[from] SolcWrapperError)
}

