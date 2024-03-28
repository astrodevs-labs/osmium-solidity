use glob::glob;
use std::path::PathBuf;

pub fn find_projects_paths(root_path: &str) -> Result<Vec<PathBuf>, glob::PatternError> {
    let pattern = format!("{}/**/foundry.toml", root_path);
    let filespaths = glob(&pattern)?
        .filter_map(|path| path.ok())
        .collect::<Vec<PathBuf>>();

    // remove foundry.toml at the end of the filepath
    Ok(filespaths
        .iter()
        .map(|path| path.parent().unwrap().to_path_buf())
        .collect())
}
