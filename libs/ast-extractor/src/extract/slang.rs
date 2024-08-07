use crate::errors::ExtractError;
use semver::Version;
use slang_solidity::parse_output::ParseOutput;
use slang_solidity::kinds::NonterminalKind;
use slang_solidity::language::Language;
use std::fs;
use std::path::PathBuf;

pub fn extract_ast_from_content(content: &str) -> Result<ParseOutput, ExtractError> {
  let pragma_version_regex = regex::Regex::new(r"pragma solidity (\^)?(\d+\.\d+\.\d+);").map_err(|_| ExtractError::ReadVersion)?;
  let version: String = pragma_version_regex
      .captures(content)
      .map(|c| c.get(2).unwrap().as_str().to_string())
      .unwrap_or_else(|| "0.8.0".to_string());
  let language = Language::new(Version::parse(&version).map_err(|_| ExtractError::ReadVersion)?).map_err(|_| ExtractError::ReadVersion)?;
  let parse_output = language.parse(NonterminalKind::SourceUnit, content);
  
  Ok(parse_output)
}

pub fn extract_ast(filepath: impl Into<PathBuf>) -> Result<ParseOutput, ExtractError> {
  let path = filepath.into();
  let content = fs::read_to_string(&path).map_err(ExtractError::ReadSourceFile)?;
  extract_ast_from_content(&content)
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
        let source = String::from("pragma solidity 0.8.0; contract test { function test() public | uint a = 1 } }");
        let result = extract_ast_from_content(&source);
        assert!(result.is_ok());
        assert!(result.unwrap().errors().len() > 0);
    }

    #[test]
    fn test_extract_ast_from_content_missing_semicolumn() {
        let source = String::from("pragma solidity 0.8.0; contract test { function test() public { uint a = 1 } }");
        let result = extract_ast_from_content(&source);
        eprintln!("{:?}", result);
        assert!(result.is_ok());
        assert!(result.unwrap().errors().len() > 0);
    }
}