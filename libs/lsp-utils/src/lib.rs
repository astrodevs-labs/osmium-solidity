use osmium_libs_solidity_path_utils::normalize_path;
use tower_lsp::lsp_types::InitializeParams;

#[cfg(feature = "log")]
pub mod log;

/**
 * This function returns the first workspace path from the InitializeParams.
 * If there is no workspace path, it returns the root path.
 * @returns {Option<String>} Normalized path
 */
pub fn get_root_path(params: InitializeParams) -> Option<String> {
    if let Some(folder) = params.workspace_folders?.first() {
        return Some(normalize_path(folder.uri.path()));
    } else if let Some(root_uri) = params.root_uri {
        return Some(normalize_path(root_uri.path()));
    }
    None
}
