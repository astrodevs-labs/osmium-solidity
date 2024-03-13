use osmium_libs_solidity_ast_extractor::{LineColumn, Spanned};
use tower_lsp::lsp_types::{Position, Range};

pub fn range_from_span(start: LineColumn, end: LineColumn) -> Range {
    Range {
        start: Position {
            line: start.line as u32,
            character: start.column as u32,
        },
        end: Position {
            line: end.line as u32,
            character: end.column as u32,
        },
    }
}

pub fn range_from_spanned<T: Spanned>(spanned: &T) -> Range {
    range_from_span(spanned.span().start(), spanned.span().end())
}
