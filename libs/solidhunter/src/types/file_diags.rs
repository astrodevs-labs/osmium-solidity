use super::LintDiag;
use colored::Colorize;
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
        let mut formatted = format!("   {}\n", "|".to_string().cyan());
        let diag = &self.diags[idx];
        let first_line = self
            .source_file_content
            .lines()
            .nth(diag.range.start.line - 1)
            .unwrap();
        let trimmed_first_line = first_line.trim_start();
        let max_offset = first_line.len() - trimmed_first_line.len();

        // Collect all ranges to highlight for this line
        let mut ranges_to_highlight = vec![&diag.range];
        if let Some(same_line_ranges) = &diag.same_line_ranges {
            ranges_to_highlight.extend(same_line_ranges.iter());
        }

        for line_nb in diag.range.start.line..diag.range.end.line + 1 {
            let line = self.source_file_content.lines().nth(line_nb - 1).unwrap();
            let (trimmed_line, offset) = try_trim_max_offset(line, max_offset);

            // Add the line content
            formatted = format!(
                "{}{}{}{}    {}\n",
                formatted,
                line_nb.to_string().cyan(),
                compute_format_line_padding(line_nb),
                "|".to_string().cyan(),
                trimmed_line,
                "|".to_string().cyan(),
                " ".repeat(if line_nb == diag.range.start.line {
                    diag.range.start.character - offset
                } else {
                    0
                }),
                "^".repeat(higlight_length)
                    .to_string()
                    .color(diag.severity.to_color())
            );

            // Add highlights for all ranges on this line
            formatted.push_str(&format!("   {}    ", "|".to_string().cyan()));
            
            let mut highlight_line = vec![' '; trimmed_line.len()];
            
            for range in &ranges_to_highlight {
                if line_nb >= range.start.line && line_nb <= range.end.line {
                    let start_char = if line_nb == range.start.line {
                        range.start.character - offset
                    } else {
                        0
                    };
                    
                    let end_char = if line_nb == range.end.line {
                        range.end.character - offset
                    } else {
                        trimmed_line.len()
                    };

                    for i in start_char..end_char {
                        if i < highlight_line.len() {
                            highlight_line[i] = '^';
                        }
                    }
                }
            }

            formatted.push_str(&highlight_line.iter().collect::<String>().color(diag.severity.to_color()).to_string());
            formatted.push('\n');
        }
        formatted
    }
}

impl fmt::Display for FileDiags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, diag) in self.diags.iter().enumerate() {
            write!(f, "{}\n{}", diag, self.format_highlighted_lines(idx))?;
            write!(
                f,
                "   {} {}\n",
                "=".to_string().cyan(),
                diag.message.to_string().color(diag.severity.to_color())
            )?;
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
