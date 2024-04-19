use crate::{
    types::{InteractableNode, Position, Range},
    Location,
};
use log::info;
use osmium_libs_solidity_ast_extractor::types::SolidityAstFile;
use solc_ast_rs_types::types::SourceLocation;

pub fn is_node_in_range(node: &SourceLocation, position: &Position, source: &str) -> bool {
    let range = source_location_to_range(node);
    let index = position_to_index(position, source);

    if range.index <= index && range.index + range.length >= index {
        return true;
    }
    false
}

#[allow(dead_code)]
pub fn log_is_node_in_range(node: &SourceLocation, position: &Position, source: &str) -> bool {
    let range = source_location_to_range(node);
    let index = position_to_index(position, source);

    info!("Node Range: {:?}", range);
    info!("Position: {:?}", position);
    info!("Position Index: {:?}", index);
    info!("Source: {:?}", source);
    if range.index <= index && range.index + range.length >= index {
        return true;
    }
    false
}

pub fn source_location_to_range(location: &str) -> Range {
    let src = location.split(':').collect::<Vec<&str>>();
    let index = src[0].parse::<u32>().unwrap();
    let length = src[1].parse::<u32>().unwrap();
    Range { index, length }
}

pub fn position_to_index(position: &Position, source: &str) -> u32 {
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;
    for c in source.chars() {
        if (line == position.line && column == position.column) || line > position.line {
            break;
        }
        if c == '\n'{
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

pub fn get_location(node: &InteractableNode, file: &SolidityAstFile) -> Location {
    let range = node.get_range();
    let start = index_to_position(range.index, &file.file.content);
    let end = index_to_position(range.index + range.length, &file.file.content);
    Location {
        start,
        end,
        uri: file.file.path.clone(),
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn postion_to_index_when_position_not_matched() {
        let source = "pragma solidity ^0.8.0;

contract Counter {
    uint256 public number;
    uint256 public x = 2;
    uint256 public y = x;

    function setNumber(uint256 newNumber) public 
    {
        tx.origin;
        number = newNumber + y;
        
    }

    function increment() public {
        setNumber(number + 1);
    }

    function notUsed() internal {
        uint256 x = 1;
        number;
    }
}";
        let position = Position { line: 12, column: 10 };
        let index = position_to_index(&position, source);
        let expected_idx = 240;
        assert_eq!(index, expected_idx);
    }
}
