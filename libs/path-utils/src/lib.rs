use std::path::PathBuf;

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
    slashify_path(path)
}

/**
 * This function normalizes the path for Linux and MacOS. Nothing to do.
 * @param {&str} path Path to normalize
 * @returns {String} Normalized path
 */
#[cfg(not(target_family = "windows"))]
pub fn normalize_path(path: &str) -> String {
    slashify_path(path)
}

pub fn join_path(base_path: &str, file: &str) -> String {
    let mut path = PathBuf::from(base_path);
    path.push(file);
    slashify_path(&path.to_str().unwrap().to_string().replace('\\', "/"))
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

#[cfg(target_family = "windows")]
pub fn escape_path(path: &str) -> String {
    let mut path = path.replace("://", "%3A/");
    path.insert(0, '/');
    path.to_string()
}

#[cfg(not(target_family = "windows"))]
pub fn escape_path(path: &str) -> String {
    path.to_string()
}
