use tower_lsp::lsp_types::{Location as LspLocation, Position as LspPosition, Range as LspRange, Url};

#[cfg(target_family = "windows")]
pub fn normalize_path(path: &str) -> String {
    let mut path = path.replace("%3A/", "://");
    path.remove(0);
    path.to_string()
}

#[cfg(not(target_family = "windows"))]
pub fn normalize_path(path: &str) -> String {
    path.to_string()
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

pub fn location_to_lsp_location(new_uri: &Url, location: &solc_references::Location) -> LspLocation {
    LspLocation {
        uri: new_uri.clone(),
        range: LspRange {
            start: LspPosition {
                line: location.start.line - 1,
                character: location.start.column - 1,
            },
            end: LspPosition {
                line: location.end.line - 1,
                character: location.end.column - 1,
            },
        },
    }
}