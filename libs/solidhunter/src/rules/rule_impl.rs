use crate::errors::SolidHunterError;
use crate::rules::create_default_rules;
use crate::rules::types::*;

pub fn create_rules_file(path: &str) {
    let rules = Rules {
        name: "solidhunter".to_string(),
        rules: create_default_rules(),
    };
    let serialized = serde_json::to_string_pretty(&rules).unwrap();

    let _ = std::fs::write(path, serialized);
}

pub fn parse_rules(path: &str) -> Result<Rules, SolidHunterError> {
    if !std::path::Path::new(&path).is_file() {
        return Err(SolidHunterError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to load a solidhunter's config file",
        )));
    }
    let file = std::fs::read_to_string(path)?;
    let parsed: Rules = serde_json::from_str(&file)?;

    Ok(parsed)
}

pub fn parse_rules_content(content: &str) -> Result<Rules, SolidHunterError> {
    let parsed: Rules = serde_json::from_str(content)?;

    Ok(parsed)
}
