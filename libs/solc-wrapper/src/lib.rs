mod forge;
mod error;
mod output;
mod utils;

pub use error::SolcWrapperError;
use forge::*;
use solc_ast_rs_types::types::SourceUnit;
use output::get_files_from_solc_output;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct SolcFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SolcJsonFile {
    pub json: serde_json::Value,
    pub file: String,
}

#[derive(Debug, Clone)]
pub struct SolcAstFile {
    pub ast: SourceUnit,
    pub file: SolcFile,
}


pub fn get_ast_for_file(base_path: String) -> Result<Vec<SolcAstFile>, SolcWrapperError> {
    
    let solc = command::ForgeCommand::default();

    eprintln!("Base path: {}", base_path.clone());
    let args = vec![String::from("build"), String::from("--build-info"), String::from("--no-cache")];
    let solc = solc.args(args);

    let init_time = SystemTime::now();

    solc.current_dir(base_path.clone()).execute()?;

    let current_time = SystemTime::now().duration_since(init_time).unwrap();
    eprintln!("Finished compiling in: {:?} seconds", current_time.as_secs());

    get_ast_from_solc_output(base_path)
}

pub fn get_ast_from_solc_output(base_path: String) -> Result<Vec<SolcAstFile>, SolcWrapperError> {
    
    let files = get_files_from_solc_output(&base_path)?;
    let init_time = SystemTime::now();
    let mut ast_files = Vec::new();
    for file in files {
        if file.file.contains("safeconsole.sol") {
            continue;
        }
        let ast: SourceUnit = serde_json::from_value(file.clone().json).map_err(|e| {
            eprintln!("Error while parsing json ast in file '{}': {:?}", file.file, e);
            e
        })?;
        let out_path = &file.file;
        ast_files.push(SolcAstFile {
            file: SolcFile {
                path: out_path.clone(),
                content: std::fs::read_to_string( std::path::Path::new(&base_path).join(&out_path)).map_err(|e| {
                    eprintln!("Error reading compiled file : {}", e);
                    SolcWrapperError::ReadSourceFile(e)
                })?,
            },
            ast,
        });
    }
    
    let current_time = SystemTime::now().duration_since(init_time).unwrap();
    eprintln!("Finished parsing ast in: {:?} seconds", current_time.as_secs());
    Ok(ast_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_file() {
        let file = "tests/single_file.json";
        let output = std::fs::read_to_string(file).unwrap();
        let res = get_files_from_solc_output(&output).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].file, "src/Counter.sol");
    }

    #[test]
    fn test_file_with_imports() {
        let file = "tests/file_with_imports.json";
        let output = std::fs::read_to_string(file).unwrap();
        let res = get_files_from_solc_output(&output).unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].file, "src/Counter.sol");
        assert_eq!(res[1].file, "src/EtherStore.sol");
    }
}