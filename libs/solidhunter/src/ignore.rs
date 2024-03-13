use crate::errors::SolidHunterError;
use glob::glob;
use std::path::Path;

fn parse_line(line: &str, path: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let line = line.replace("./", "");
    if let Some(parent) = path.parent() {
        if let Some(filepath) = parent.join(line).to_str() {
            if let Ok(entries) = glob(filepath) {
                for entry in entries.flatten() {
                    files.push(entry.into_os_string().into_string().unwrap())
                }
            }
        }
    }

    files
}

fn parse_solihunterignore(filepath: &String) -> Result<Vec<String>, SolidHunterError> {
    let mut excluded_files = Vec::new();
    let content = std::fs::read_to_string(filepath)?;

    for line in content.lines() {
        excluded_files.append(&mut parse_line(line, Path::new(filepath)));
    }

    Ok(excluded_files)
}

fn get_solidhunterignore_paths(filepath: &String) -> Result<Vec<String>, SolidHunterError> {
    let mut ignored_files = Vec::new();

    if let Ok(entries) = glob(&format!("{}/**/.solidhunterignore", filepath)) {
        for entry in entries.flatten() {
            ignored_files.push(entry.into_os_string().into_string().unwrap())
        }
    }

    Ok(ignored_files)
}

pub fn get_excluded_files(filepaths: &Vec<String>) -> Result<Vec<String>, SolidHunterError> {
    let mut excluded_files = Vec::new();

    for filepath in filepaths {
        let path = Path::new(filepath);

        if path.is_file() {
            continue;
        }

        let solidhunterignore_paths = get_solidhunterignore_paths(filepath)?;

        for solidhunterignore_path in solidhunterignore_paths {
            if let Ok(mut excluded) = parse_solihunterignore(&solidhunterignore_path) {
                excluded_files.append(&mut excluded);
            }
        }
    }
    Ok(excluded_files)
}
