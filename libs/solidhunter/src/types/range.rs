use super::Position;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    // Compute the number of characters between the start and end of the range
    pub fn compute_length(&self, content: &str) -> usize {
        if self.start.line == self.end.line {
            self.end.character - self.start.character
        } else {
            let mut length = 0;
            let mut line = self.start.line;
            let mut character = self.start.character;
            while line < self.end.line {
                let line_content = content.lines().nth(line - 1).unwrap();
                length += line_content.len() + 1 - character;
                line += 1;
                character = 0;
            }
            length += self.end.character - character;
            length
        }
    }
}
