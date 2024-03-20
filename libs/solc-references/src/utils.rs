
use std::path::PathBuf;

use crate::{types::{InteractableNode, Position, Range}, Location};
use solc_ast_rs_types::types::SourceLocation;
use solc_wrapper::SolcAstFile;

pub fn is_node_in_range(node: &SourceLocation, position: &Position, source: &str) -> bool {
    let range = source_location_to_range(node);
    let index = position_to_index(position, source);

    if range.index <= index &&
    range.index + range.length >= index {
        return true;
    }
    false
}


pub fn source_location_to_range(location: &str) -> Range {
    let src = location.split(":").collect::<Vec<&str>>();
    let index = src[0].parse::<u32>().unwrap();
    let length = src[1].parse::<u32>().unwrap();
    Range { index: index, length }
}

pub fn position_to_index(position: &Position, source: &str) -> u32 {
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;
    for c in source.chars() {
        if line == position.line && column == position.column {
            break;
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

pub fn index_to_position(index: u32, source: &str) -> Position {
    let mut line = 1;
    let mut column = 1;
    for (i, c) in source.chars().enumerate() {
        if i == index as usize {
            break;
        }
        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    Position { line, column }
}

pub fn join_path(base_path: &str, file: &str) -> String {
    let mut path = PathBuf::from(base_path);
    path.push(file);
    path.to_str().unwrap().to_string().replace("\\", "/")
}

pub fn get_location(node: &InteractableNode, file: &SolcAstFile) -> Location {
    let range = node.get_range();
    let start = index_to_position(range.index, &file.file.content);
    let end = index_to_position(range.index + range.length, &file.file.content);
    Location {
        start,
        end,
        uri: file.file.path.clone()
    }
}
