use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractError {
    //#[error("Tokenization error: {0}")]
    //Tokenize(#[from] proc_macro2::LexError),
    //#[error("Parsing error")]
    //Parse(#[from] syn::Error),
    #[error("Alloy extraction error: {0}")]
    Alloy(String),
    #[error("Compiler error: {0}")]
    Compiler(#[from] osmium_libs_solidity_foundry_wrapper::Error),
    #[error("Cannot read source file")]
    ReadSourceFile(std::io::Error),
    #[error("JSON parsing error: {0}")]
    AstParsing(#[from] serde_json::Error),
}
