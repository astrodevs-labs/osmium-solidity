use osmium_libs_foundry_wrapper::Severity;
use tower_lsp::lsp_types::{DiagnosticSeverity, InitializeParams};

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

pub fn convert_severity(severity: Severity) -> DiagnosticSeverity {
    match severity {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
        Severity::Info => DiagnosticSeverity::INFORMATION,
    }
}

/**
 * This function normalizes the path for Windows.
 * VSCode send the path starting with /%3A/ instead of {letter}:/
 * @param {&str} path Path to normalize
 * @returns {String} Normalized path
 */
#[cfg(target_family = "windows")]
pub fn normalize_path(path: &str) -> String {
    let mut path = path.replace("%3A/", "://");
    path.remove(0);
    path.to_string()
}

/**
 * This function normalizes the path for Linux and MacOS. Nothing to do.
 * @param {&str} path Path to normalize
 * @returns {String} Normalized path
 */
#[cfg(not(target_family = "windows"))]
pub fn normalize_path(path: &str) -> String {
    path.to_string()
}

/**
 * This function replaces all backslashes and double-slahes with slashes.
 * This is useful for Windows paths.
 * @param {&str} path Path to slashify
 * @returns {String} Slashified path
 */
pub fn slashify_path(path: &str) -> String {
    path.replace('\\', "/")
        .replace("\\\\", "/")
        .replace("//", "/")
}

pub fn normalized_slash_path(path: &str) -> String {
    slashify_path(&normalize_path(path))
}
