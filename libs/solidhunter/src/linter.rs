use crate::errors::SolidHunterError;
use crate::rules::create_default_rules;
use crate::rules::factory::RuleFactory;
use crate::rules::rule_impl::parse_rules;
use crate::rules::rule_impl::parse_rules_content;
use crate::rules::types::*;
use crate::types::*;
use std::fs;

use crate::ignore::get_excluded_files;
use glob::glob;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SolidFile {
    pub data: osmium_libs_solidity_ast_extractor::File,
    pub path: String,
    pub content: String,
}

pub struct SolidLinter {
    files: Vec<SolidFile>,
    rule_factory: RuleFactory,
    rules: Vec<Box<dyn RuleType>>,
    excluded_files: Vec<String>,
}

impl Default for SolidLinter {
    fn default() -> Self {
        SolidLinter::new()
    }
}

impl SolidLinter {
    pub fn new() -> Self {
        SolidLinter {
            files: Vec::new(),
            rule_factory: RuleFactory::default(),
            rules: vec![],
            excluded_files: Vec::new(),
        }
    }

    pub fn new_fileless() -> Self {
        let default_rules = create_default_rules();
        let mut linter = SolidLinter {
            files: Vec::new(),
            rule_factory: RuleFactory::default(),
            rules: Vec::new(),
            excluded_files: Vec::new(),
        };

        for rule in default_rules {
            linter.rules.push(linter.rule_factory.create_rule(rule));
        }

        linter
    }

    pub fn initialize_rules(&mut self, rules_config: &str) -> Result<(), SolidHunterError> {
        let res = parse_rules(rules_config)?;
        for rule in res.rules {
            self.rules.push(self.rule_factory.create_rule(rule));
        }
        Ok(())
    }

    pub fn get_documentation(&self) -> Vec<RuleDocumentation> {
        let mut res = Vec::new();
        for rule in &self.rules {
            res.push(rule.get_documentation())
        }
        res
    }

    pub fn initialize_excluded_files(
        &mut self,
        excluded_filepaths: Option<&Vec<String>>,
        filepaths: &Vec<String>,
    ) -> Result<(), SolidHunterError> {
        if let Some(excluded) = excluded_filepaths {
            for path in excluded {
                self.excluded_files.push(path.clone())
            }
        }
        self.excluded_files
            .append(&mut get_excluded_files(filepaths)?);
        Ok(())
    }

    pub fn initialize_rules_content(&mut self, rules_config: &str) -> Result<(), SolidHunterError> {
        let res = parse_rules_content(rules_config)?;
        for rule in res.rules {
            self.rules.push(self.rule_factory.create_rule(rule));
        }
        Ok(())
    }

    fn _file_exists(&self, path: &str) -> bool {
        for file in &self.files {
            if file.path == path {
                return true;
            }
        }
        false
    }

    fn _add_file(
        &mut self,
        path: &str,
        ast: osmium_libs_solidity_ast_extractor::File,
        content: &str,
    ) {
        if self._file_exists(path) {
            for file in &mut self.files {
                if file.path == path {
                    file.data = ast.clone();
                    file.content = String::from(content);
                }
            }
        } else {
            let file = SolidFile {
                data: ast,
                path: String::from(path),
                content: String::from(content),
            };
            self.files.push(file);
        }
    }

    pub fn parse_file(&mut self, filepath: String) -> LintResult {
        let content = fs::read_to_string(filepath.clone())?;
        if self.excluded_files.contains(&filepath) {
            return Ok(FileDiags::new(content, Vec::new()));
        }
        self.parse_content(&filepath, content.as_str())
    }

    fn _check_is_in_disable_range(&self, diag: &LintDiag, disable_ranges: &[DisableRange]) -> bool {
        let mut rules_occurrences = vec![];

        let filtered_range = disable_ranges
            .iter()
            // we only care about ranges that start before the diag
            .filter(|range| range.start_line <= diag.range.start.line)
            .map(|range| {
                if range.rule_ids.is_empty() {
                    DisableRange {
                        rule_ids: vec!["".to_string()], // empty rule means all rules
                        ..range.clone()
                    }
                } else {
                    range.clone()
                }
            })
            .collect::<Vec<DisableRange>>();

        for range in &filtered_range {
            match range.ignore_type {
                Ignore::SameLine | Ignore::NextLine => {
                    if range.start_line == diag.range.start.line
                        && (range.rule_ids.contains(&diag.id)
                            || range.rule_ids.contains(&"".to_string()))
                    {
                        return true;
                    }
                }
                Ignore::Disable => {
                    for rule in &range.rule_ids {
                        let mut found = false;
                        for (rule_id, occurences) in &mut rules_occurrences {
                            if *rule_id == rule {
                                *occurences += 1;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            rules_occurrences.push((rule, 1));
                        }
                    }
                }
                Ignore::Enable => {
                    for rule in &range.rule_ids {
                        for (rule_id, occurences) in &mut rules_occurrences {
                            if *rule_id == rule {
                                *occurences -= 1;
                                break;
                            }
                        }
                        // TODO: global disable followed by a scoped enable might not work
                    }
                }
            }
        }

        let disabled_rules = rules_occurrences
            .iter()
            .filter(|(_, occurences)| *occurences > 0)
            .map(|(rule, _)| rule.to_string())
            .collect::<Vec<String>>();

        for rule in disabled_rules {
            if rule.is_empty() || rule == diag.id {
                return true;
            }
        }
        false
    }

    fn _check_is_diag_ignored(&self, diag: &LintDiag, file: &SolidFile) -> bool {
        let ignore_comments: Vec<IgnoreComment> = file
            .content
            .lines()
            .enumerate()
            .filter_map(|(line_number, line)| IgnoreComment::from_line(line_number + 1, line))
            .collect();
        let disable_ranges = build_disable_ranges(ignore_comments);

        self._check_is_in_disable_range(diag, &disable_ranges)
    }

    pub fn parse_content(&mut self, filepath: &str, content: &str) -> LintResult {
        let res = osmium_libs_solidity_ast_extractor::extract::extract_ast_from_content(content)?;

        self._add_file(filepath, res, content);
        let mut res: Vec<_> = vec![];

        for rule in &self.rules {
            let mut diags = rule.diagnose(&self.files[self.files.len() - 1], &self.files);
            for diag in &mut diags {
                if !self._check_is_diag_ignored(diag, &self.files[self.files.len() - 1]) {
                    res.push(diag.clone());
                }
            }
        }
        Ok(FileDiags::new(content.to_string(), res))
    }

    pub fn parse_folder(&mut self, folder: &str) -> Vec<LintResult> {
        let mut result: Vec<LintResult> = Vec::new();
        if let Ok(entries) = glob(&(folder.to_owned() + "/**/*.sol")) {
            for entry in entries.flatten() {
                result.push(self.parse_file(entry.into_os_string().into_string().unwrap()));
            }
        }
        result
    }
    pub fn parse_path(&mut self, path: &str) -> Vec<LintResult> {
        if Path::new(&path).is_file() {
            vec![self.parse_file(path.to_string())]
        } else {
            self.parse_folder(path)
        }
    }

    pub fn delete_file(&mut self, path: &str) {
        loop {
            let idx = self.files.iter().position(|x| x.path == path);
            match idx {
                Some(idx) => {
                    self.files.remove(idx);
                }
                None => {
                    break;
                }
            }
        }
    }
}
