use osmium_libs_solidity_foundry_wrapper::FoundryJsonFile;
use solc_ast_rs_types::types::SourceUnit;

pub enum SolidityJsonFile {
    Foundry(FoundryJsonFile),
}

#[derive(Debug, Clone)]
pub struct SolidityFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SolidityAstFile {
    pub ast: SourceUnit,
    pub file: SolidityFile,
}
