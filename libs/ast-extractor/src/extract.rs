/**
 * extract.rs
 * Extract AST from solidity source code
 * author: 0xMemoryGrinder
 */
use crate::errors::ExtractError;
use proc_macro2::TokenStream;
use std::str::FromStr;

pub fn extract_ast_from_content(content: &str) -> Result<syn_solidity::File, ExtractError> {
    let tokens = TokenStream::from_str(content)?;
    let ast = syn_solidity::parse2(tokens)?;
    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_extract_ast_from_content_good() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("good.sol");
        let source = fs::read_to_string(path).unwrap();
        let res = extract_ast_from_content(&source);
        assert!(res.is_ok());
    }

    #[test]
    fn test_extract_ast_from_content_invalid_token() {
        let source = String::from("contract test { function test() public | uint a = 1 } }");
        let result = extract_ast_from_content(&source);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Tokenization error: cannot parse string into token stream"
        );
    }

    #[test]
    fn test_extract_ast_from_content_missing_semicolumn() {
        let source = String::from("contract test { function test() public { uint a = 1 } }");
        let result = extract_ast_from_content(&source);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Parsing error");
    }
}
