use tower_lsp::lsp_types::{
    Location as LspLocation, Position as LspPosition, Range as LspRange, Url,
};

pub fn location_to_lsp_location(
    new_uri: &Url,
    location: &osmium_libs_solidity_references::Location,
) -> LspLocation {
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
