use crate::error::SlitherError;
use crate::{FoundryArrOrStr, FoundryToml, SlitherData};
use glob::glob;
use std::error::Error;
use std::process::Command as StdCommand;

pub fn is_slither_installed() -> bool {
    let output = StdCommand::new("slither").arg("--version").output();
    output.is_ok()
}

pub fn is_solc_installed() -> bool {
    let output = StdCommand::new("solc").arg("--version").output();
    output.is_ok()
}

#[cfg(target_family = "windows")]
pub fn normalize_slither_path(path: &str) -> String {
    let mut path = path.replace("%3A/", "://");
    path.remove(0);
    path.to_string()
}

#[cfg(not(target_family = "windows"))]
pub fn normalize_slither_path(path: &str) -> String {
    path.to_string()
}

fn extract_foundry_src(foundry: FoundryToml) -> Option<FoundryArrOrStr> {
    foundry.profiles?.default?.src
}

pub fn parse_foundry_toml(foundry: String, state: &mut SlitherData) {
    let foundry: FoundryToml = match toml::from_str(&foundry) {
        Ok(foundry) => foundry,
        Err(e) => {
            eprintln!("Error parsing foundry.toml: {}", e);
            return;
        }
    };
    match extract_foundry_src(foundry.clone()).unwrap_or(FoundryArrOrStr::Str("src".to_string())) {
        FoundryArrOrStr::Arr(srcs) => {
            for src in srcs {
                state.src_paths.push(src.to_string());
            }
        }
        FoundryArrOrStr::Str(src) => {
            state.src_paths.push(src);
        }
    };
}

/**
 * Find the foundry.toml config file in the given workspace using glob.
 */
pub fn find_foundry_toml_config(workspace: &str) -> Result<String, Box<dyn Error>> {
    let mut foundry_toml_path = String::new();
    for entry in glob(&format!("{}/**/foundry.toml", workspace))? {
        match entry {
            Ok(path) => {
                foundry_toml_path = path.display().to_string();
                break;
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
    if foundry_toml_path.is_empty() {
        return Err(Box::new(SlitherError::FoundryTomlNotFound));
    }
    Ok(foundry_toml_path)
}
