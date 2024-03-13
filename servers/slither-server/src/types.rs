use serde::{Deserialize, Serialize};
use std::vec;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;
use tower_lsp::lsp_types::*;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity as Severity, Position, Range};

#[derive(Debug)]
pub struct SlitherDiag {
    pub diagnostics: Vec<Diagnostic>,
    pub uri: Url,
}

impl SlitherDiag {
    pub fn new(uri: Url, diagnostics: Vec<Diagnostic>) -> Self {
        Self { uri, diagnostics }
    }
}

#[derive(Debug)]
pub struct SlitherData {
    pub slither_processes: Vec<CancellationToken>,
    pub receiver: Option<Receiver<SlitherDiag>>,
    pub sender: Sender<SlitherDiag>,
    pub src_paths: Vec<String>,
    pub workspace: String,
}

impl SlitherData {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel::<SlitherDiag>(100);
        Self {
            src_paths: vec![],
            slither_processes: vec![],
            receiver: Some(receiver),
            sender,
            workspace: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum FoundryArrOrStr {
    Arr(Vec<String>),
    Str(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct FoundryProfile {
    pub src: Option<FoundryArrOrStr>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FoundryProfiles {
    pub default: Option<FoundryProfile>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FoundryToml {
    pub profiles: Option<FoundryProfiles>,
}

/////////////////////////
// SLITHER JSON OUTPUT //
/////////////////////////
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherResult {
    pub results: SlitherResults,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherResults {
    pub detectors: Vec<SlitherDetector>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherDetector {
    pub elements: Vec<SlitherElement>,
    pub description: String,
    pub check: String,
    pub impact: String,
    pub id: String,
    pub confidence: String,
    pub markdown: String,
    pub first_markdown_element: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherElement {
    pub source_mapping: SlitherSourceMapping,

    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub type_specific_fields: Option<SlitherTypeSpecificFields>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherSourceMapping {
    pub filename_absolute: String,
    pub filename_relative: String,
    pub filename_short: String,
    pub is_dependency: bool,
    pub lines: Vec<usize>,
    pub starting_column: usize,
    pub ending_column: usize,
    pub length: usize,
    pub start: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherTypeSpecificFields {
    pub directive: Option<Vec<String>>,
    pub signature: Option<String>,
    pub additional_fields: Option<SlitherAdditionalFields>,
    pub parent: Option<SlitherParent>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherParent {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub type_specific_fields: Option<Box<SlitherTypeSpecificFields>>,
    pub source_mapping: Option<SlitherSourceMapping>,
    pub signature: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherAdditionalFields {
    pub underlying_type: Option<String>,
    pub visibility: Option<String>,
    pub variable_name: Option<String>,
}

pub fn diag_from_json(json: SlitherDetector) -> Vec<Diagnostic> {
    let mut results: Vec<Diagnostic> = Vec::new();

    for idx in 0..json.elements.len() {
        if json.elements[idx].source_mapping.lines.is_empty()
            || json.elements[idx].type_ == "contract"
        {
            continue;
        }
        let lines = &json.elements[idx].source_mapping.lines;
        let start_col = json.elements[idx].source_mapping.starting_column;
        let end_col = json.elements[idx].source_mapping.ending_column;
        let range = Range {
            start: Position {
                line: lines[0] as u32 - 1,
                character: start_col as u32 - 1,
            },
            end: Position {
                line: lines[lines.len() - 1] as u32 - 1,
                character: end_col as u32,
            },
        };

        let severity = match json.impact.as_str() {
            "High" => Severity::ERROR,
            "Medium" => Severity::WARNING,
            "Low" => Severity::HINT,
            "Informational" => Severity::INFORMATION,
            _ => Severity::ERROR,
        };

        results.push(Diagnostic {
            range,
            severity: Some(severity),
            code: None,
            code_description: None,
            source: Some("osmium-slither".to_string()),
            message: json.description.to_string() + "\nCheck: " + &json.check,
            related_information: None,
            tags: None,
            data: None,
        });
    }

    results
}
