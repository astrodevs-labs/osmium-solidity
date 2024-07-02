use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::Range;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTestsPositionsParams {
    pub file_content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestContract {
    pub name: String,
    pub range: Range,
    pub tests: Vec<Test>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    pub name: String,
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTestsPositionsResponse {
    pub contracts: Vec<TestContract>,
}
