/**
 * extract.rs
 * Extract AST from solidity source code
 * author: 0xMemoryGrinder
 */
use crate::errors::ExtractError;
use crate::types::*;
use osmium_libs_solidity_foundry_wrapper::{Compiler, FoundryJsonFile};
use proc_macro2::TokenStream;
use solc_ast_rs_types::types::SourceUnit;
use std::str::FromStr;
use log::error;

pub fn extract_ast_from_content(content: &str) -> Result<syn_solidity::File, ExtractError> {
    let tokens = TokenStream::from_str(content).map_err(|e| ExtractError::Alloy(e.to_string()))?;
    let ast = syn_solidity::parse2(tokens).map_err(|e| ExtractError::Alloy(e.to_string()))?;
    Ok(ast)
}

pub fn extract_ast_from_foundry(base_path: &str) -> Result<Vec<SolidityAstFile>, ExtractError> {
    let mut compiler = Compiler::new_with_executable_check()?;
    compiler.load_workspace(base_path.to_string())?;
    let (path, files) = compiler.compile_ast(base_path)?;

    get_ast_from_foundry_output(&path, files)
}

fn get_ast_from_foundry_output(
    base_path: &str,
    files: Vec<FoundryJsonFile>,
) -> Result<Vec<SolidityAstFile>, ExtractError> {
    let mut ast_files = Vec::new();
    for file in files {
        if file.file.contains("safeconsole.sol") {
            continue;
        }
        let ast: SourceUnit = serde_json::from_value(file.clone().json).map_err(|e| {
            error!(
                "Error while parsing json ast in file '{}': {:?}",
                file.file, e
            );
            e
        })?;
        let out_path = &file.file;
        ast_files.push(SolidityAstFile {
            file: SolidityFile {
                path: out_path.clone(),
                content: std::fs::read_to_string(std::path::Path::new(&base_path).join(out_path))
                    .map_err(|e| {
                    error!("Error reading compiled file : {}", e);
                    ExtractError::ReadSourceFile(e)
                })?,
            },
            ast,
        });
    }
    Ok(ast_files)
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
            "Alloy extraction error: cannot parse string into token stream"
        );
    }

    #[test]
    fn test_extract_ast_from_content_missing_semicolumn() {
        let source = String::from("contract test { function test() public { uint a = 1 } }");
        let result = extract_ast_from_content(&source);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Alloy extraction error: expected `;`"
        );
    }
}
