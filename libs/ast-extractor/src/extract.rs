/**
 * extract.rs
 * Extract AST from solidity source code
 * author: 0xMemoryGrinder
 */
mod foundry;
pub use foundry::extract_ast_from_foundry;
mod slang;
pub use slang::extract_ast_from_content;
pub use slang::extract_ast;
