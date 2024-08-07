use log::error;
use osmium_libs_solidity_foundry_wrapper::{Compiler, FoundryJsonFile};
use solc_ast_rs_types::types::SourceUnit;
use crate::errors::ExtractError;
use crate::types::*;


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
        let ast: Result<SourceUnit, serde_json::Error> = serde_json::from_value(file.clone().json);
        if let Err(e) = &ast {
            error!(
                "Error while parsing json ast in file '{}': {:?}",
                file.file, e
            );
            continue;
        }
        let ast = ast.unwrap();
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