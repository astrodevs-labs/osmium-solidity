use std::{collections::HashMap, str::FromStr};

use crate::{
    types::{InteractableNode, Position, Range},
    Location,
};
use log::info;
use osmium_libs_solidity_ast_extractor::{types::SolidityAstFile, Type};
use solc_ast_rs_types::types::*;

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
    use std::str::FromStr;

    use crate::{test_utils::create_test_ast_file, utils::index_to_position};

    pub use super::*;

    #[test]
    fn test_get_location() {
        let file = create_test_ast_file();
        let node = file.ast.nodes[0].clone();
        if let SourceUnitNodesItem::ContractDefinition(node) = node {
            let node = InteractableNode::ContractDefinition(node);
            let location = get_location(&node, &file);
            let expected_location = Location {
                start: Position { line: 3, column: 5 },
                end: Position { line: 12, column: 6 },
                uri: "test.sol".to_string(),
            };
            assert_eq!(location, expected_location);
        } else {
            panic!("Expected ContractDefinition");
        }

    }

    #[test]
    fn test_log_is_node_in_range() {
        let file = create_test_ast_file();
        let node = file.ast.nodes[0].clone();
        if let SourceUnitNodesItem::ContractDefinition(contract) = node {
            let position = Position {
                line: 3,
                column: 5,
            };
            let source = &file.file.content;
            let is_in_range = log_is_node_in_range(&contract.src, &position, source);
            assert_eq!(is_in_range, true);
        } else {
            panic!("Expected ContractDefinition");
        }
    }

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
        let position = Position {
            line: 12,
            column: 10,
        };
        let index = position_to_index(&position, source);
        let expected_idx = 240;
        assert_eq!(index, expected_idx);
    }

    #[test]
    fn postion_to_index_when_position_matched() {
        let source = "pragma solidity ^0.8.0;

contract Counter {
    uint256 public number;
    uint256 public x = 2;
    uint256 public y = x;

    function setNumber(uint256 newNumber) public 
    {
        tx.origin;
        number = newNumber + y;
        d
    }

    function increment() public {
        setNumber(number + 1);
    }

    function notUsed() internal {
        uint256 x = 1;
        number;
    }
}";
        let position = Position {
            line: 12,
            column: 10,
        };
        let index = position_to_index(&position, source);
        let expected_idx = 240;
        assert_eq!(index, expected_idx);
    }
    
    #[test]
    fn test_index_to_position() {
        let source = "pragma solidity ^0.8.0;

contract Counter {
    uint256 public number;
    uint256 public x = 2;
    uint256 public y = x;

    function setNumber(uint256 newNumber) public 
    {
        tx.origin;
        number = newNumber + y;
        d
    }

    function increment() public {
        setNumber(number + 1);
    }

    function notUsed() internal {
        uint256 x = 1;
        number;
    }
}";
        let index = 240;
        let position = index_to_position(index, source);
        let expected_position = Position {
            line: 12,
            column: 10,
        };
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_source_location_to_range() {
        let location = "240:1";
        let range = source_location_to_range(location);
        let expected_range = Range {
            index: 240,
            length: 1,
        };
        assert_eq!(range, expected_range);
    }

    #[test]
    fn test_is_node_in_range() {
        let source = "pragma solidity ^0.8.0;

contract Counter {
    uint256 public number;
    uint256 public x = 2;
    uint256 public y = x;

    function setNumber(uint256 newNumber) public 
    {
        tx.origin;
        number = newNumber + y;
        d
    }

    function increment() public {
        setNumber(number + 1);
    }

    function notUsed() internal {
        uint256 x = 1;
        number;
    }
}";
        let position = Position {
            line: 12,
            column: 10,
        };
        let location = "240:1:1";
        let src_location = SourceLocation::from_str(location);
        let is_in_range = is_node_in_range(&src_location.unwrap(), &position, source);
        assert_eq!(is_in_range, true);
    }

    #[test]
    fn test_is_node_in_range_when_not_in_range() {
        let source = "pragma solidity ^0.8.0;

contract Counter {
    uint256 public number;
    uint256 public x = 2;
    uint256 public y = x;

    function setNumber(uint256 newNumber) public 
    {
        tx.origin;
        number = newNumber + y;
        d
    }

    function increment() public {
        setNumber(number + 1);
    }

    function notUsed() internal {
        uint256 x = 1;
        number;
    }
}";

        let position = Position {
            line: 12,
            column: 10,
        };
        let location = "210:1:1";
        let src_location = SourceLocation::from_str(location);
        let is_in_range = is_node_in_range(&src_location.unwrap(), &position, source);
        assert_eq!(is_in_range, false);
    }
    
    #[test]
    fn test_is_node_in_range_when_not_in_range_with_empty_source() {
        let source = "";
        let position = Position {
            line: 12,
            column: 10,
        };
        let location = "210:1:1";
        let src_location = SourceLocation::from_str(location);
        let is_in_range = is_node_in_range(&src_location.unwrap(), &position, source);
        assert_eq!(is_in_range, false);
    }

    #[test]
    fn test_is_node_in_range_when_not_in_range_with_empty_location() {
        let source = "pragma solidity ^0.8.0;

contract Counter {
    uint256 public number;
    uint256 public x = 2;
    uint256 public y = x;

    function setNumber(uint256 newNumber) public 
    {
        tx.origin;
        number = newNumber + y;
        d
    }

    function increment() public {
        setNumber(number + 1);
    }

    function notUsed() internal {
        uint256 x = 1;
        number;
    }
}";
        let position = Position {
            line: 12,
            column: 10,
        };
        let location = "0:0:0";
        let src_location = SourceLocation::from_str(location);
        let is_in_range = is_node_in_range(&src_location.unwrap(), &position, source);
        assert_eq!(is_in_range, false);
    }

    #[test]
    fn test_is_node_in_range_when_not_in_range_with_empty_position() {
        let source = "pragma solidity ^0.8.0;

contract Counter {
    uint256 public number;
    uint256 public x = 2;
    uint256 public y = x;

    function setNumber(uint256 newNumber) public 
    {
        tx.origin;
        number = newNumber + y;
        d
    }

    function increment() public {
        setNumber(number + 1);
    }

    function notUsed() internal {
        uint256 x = 1;
        number;
    }
}";
        let position = Position {
            line: 0,
            column: 0,
        };
        let location = "240:1:1";
        let src_location = SourceLocation::from_str(location);
        let is_in_range = is_node_in_range(&src_location.unwrap(), &position, source);
        assert_eq!(is_in_range, false);
    }

}
