use super::LintDiag;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileDiags {
    #[serde(skip_serializing_if = "String::is_empty", rename = "sourceFileContent")]
    pub source_file_content: String,
    pub diags: Vec<LintDiag>,
}

impl FileDiags {
    pub fn new(source_file_content: String, diags: Vec<LintDiag>) -> Self {
        FileDiags {
            source_file_content,
            diags,
        }
    }

    fn format_highlighted_lines(&self, idx: usize) -> String {
        let mut formatted = "   |\n".to_string();
        let diag = &self.diags[idx];
        let first_line = self
            .source_file_content
            .lines()
            .nth(diag.range.start.line - 1)
            .unwrap();
        let trimmed_first_line = first_line.trim_start();
        let max_offset = first_line.len() - trimmed_first_line.len();

        for line_nb in diag.range.start.line..diag.range.end.line + 1 {
            let line = self.source_file_content.lines().nth(line_nb - 1).unwrap();
            let (trimmed_line, offset) = try_trim_max_offset(line, max_offset);
            let mut higlight_length = trimmed_line.len();

            if diag.range.start.line == diag.range.end.line {
                higlight_length = diag.range.end.character - diag.range.start.character;
            } else if line_nb == diag.range.start.line {
                higlight_length = trimmed_line.len() - (diag.range.start.character - offset);
            } else if line_nb == diag.range.end.line {
                higlight_length = trimmed_line.len() - (diag.range.end.character - offset) + 1;
            }

            formatted = format!(
                "{}{}{}|    {}\n   |    {}{}\n",
                formatted,
                line_nb,
                compute_format_line_padding(line_nb),
                trimmed_line,
                " ".repeat(if line_nb == diag.range.start.line {
                    diag.range.start.character - offset
                } else {
                    0
                }),
                "^".repeat(higlight_length)
            );
        }
        formatted
    }
}

impl fmt::Display for FileDiags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, diag) in self.diags.iter().enumerate() {
            write!(f, "{}\n{}", diag, self.format_highlighted_lines(idx))?;
        }
        Ok(())
    }
}

fn compute_format_line_padding(line: usize) -> String {
    let padding: String;
    if line > 99 {
        padding = " ".repeat(0);
    } else if line > 9 {
        padding = " ".to_string();
    } else {
        padding = " ".repeat(2);
    }
    padding
}

fn try_trim_max_offset(line: &str, max_offset: usize) -> (&str, usize) {
    let mut offset: usize = 0;

    for (i, c) in line.chars().enumerate() {
        if i >= max_offset {
            break;
        }
        if c.is_whitespace() {
            offset += 1;
        }
    }
    (&line[offset..], offset)
}
