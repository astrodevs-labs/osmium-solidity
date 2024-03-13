use serde::{Deserialize, Serialize};

macro_rules! define_ignore_enum {
    ($name:ident, $($variant:ident => $str:expr),* $(,)?) => {
        #[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
        pub enum $name {
            $($variant),*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => $str),*
                }
                .to_string()
            }
        }

        impl $name {
            pub fn iter() -> impl Iterator<Item = Self> {
                [$(Self::$variant),*].iter().copied()
            }
        }
    };
}

define_ignore_enum! {
    Ignore,
    NextLine => "solidhunter-disable-next-line",
    SameLine => "solidhunter-disable-line",
    Disable => "solidhunter-disable",
    Enable => "solidhunter-enable",
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnoreComment {
    pub line_number: usize,
    pub ignore_type: Ignore,
    pub rule_ids: Option<Vec<String>>,
}

impl IgnoreComment {
    pub fn from_line(line_number: usize, line: &str) -> Option<Self> {
        for ignore in Ignore::iter() {
            let ignore_str = ignore.to_string();
            if line.contains(&ignore_str) {
                let rule_ids_str = line.split(&ignore_str).nth(1);
                let rule_ids = rule_ids_str
                    .map(|s| {
                        s.split(' ')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                    })
                    .filter(|s| !s.is_empty());
                return Some(Self {
                    line_number,
                    ignore_type: ignore,
                    rule_ids,
                });
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisableRange {
    pub start_line: usize,
    pub end_line: usize,
    pub ignore_type: Ignore,
    pub rule_ids: Vec<String>,
}

pub fn build_disable_ranges(comments: Vec<IgnoreComment>) -> Vec<DisableRange> {
    let mut disable_ranges = vec![];
    let mut current_range: Option<DisableRange> = None;

    for comment in comments {
        let line_number = comment.line_number;
        let ignore_type = comment.ignore_type;
        let rule_ids = comment.rule_ids.unwrap_or(vec![]);

        match ignore_type {
            Ignore::Enable | Ignore::Disable => {
                if let Some(range) = current_range {
                    disable_ranges.push(range);
                }
                current_range = Some(DisableRange {
                    start_line: line_number,
                    end_line: line_number,
                    ignore_type,
                    rule_ids,
                });
            }
            Ignore::SameLine => {
                if let Some(range) = &mut current_range {
                    range.end_line = line_number;
                }
                disable_ranges.push(DisableRange {
                    start_line: line_number,
                    end_line: line_number,
                    ignore_type,
                    rule_ids,
                });
            }
            Ignore::NextLine => {
                if let Some(range) = &mut current_range {
                    range.end_line = line_number;
                }
                disable_ranges.push(DisableRange {
                    start_line: line_number + 1,
                    end_line: line_number + 1,
                    ignore_type,
                    rule_ids,
                });
            }
        }
    }

    if let Some(range) = current_range {
        disable_ranges.push(range);
    }

    disable_ranges
}
