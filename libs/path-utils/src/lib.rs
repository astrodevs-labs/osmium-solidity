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
    slashify_path(&path)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        let path = "/c:/Users/username/Documents";
        assert_eq!(normalize_path(path), "c:/Users/username/Documents");
    }

    #[test]
    fn test_normalize_path_windows() {
        let path = "/c%3A/Users/username/Documents";
        assert_eq!(normalize_path(path), "c:/Users/username/Documents");
    }

    #[test]
    fn test_join_path() {
        let base_path = "C:/Users/username/Documents";
        let file = "file.sol";
        assert_eq!(join_path(base_path, file), "C:/Users/username/Documents/file.sol");
    }

    #[test]
    fn test_slashify_path() {
        let path = "C:\\Users\\username\\Documents";
        assert_eq!(slashify_path(path), "C:/Users/username/Documents");
    }

    #[test]
    fn test_slashify_path_double_slash() {
        let path = "C:\\Users\\\\username\\Documents";
        assert_eq!(slashify_path(path), "C:/Users/username/Documents");
    }

    #[test]
    fn test_escape_path() {
        let path = "c://Users/username/Documents";
        assert_eq!(escape_path(path), "/c%3A/Users/username/Documents");
    }

    #[test]
    fn test_escape_path_windows() {
        let path = "c://Users/username/Documents";
        assert_eq!(escape_path(path), "/c%3A/Users/username/Documents");
    }
}
