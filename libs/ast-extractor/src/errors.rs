use proc_macro2::LexError;
use syn::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("Tokenization error: {0}")]
    Tokenize(#[from] LexError),
    #[error("Parsing error")]
    Parse(#[from] Error),
}
