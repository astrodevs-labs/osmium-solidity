use osmium_libs_solidity_ast_extractor::LineColumn;

pub fn absolute_index_from_location(location: LineColumn, content: &str) -> usize {
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;

    for c in content.chars() {
        if line == location.line && column == location.column {
            return index;
        }
        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
        index += 1;
    }
    index
}
