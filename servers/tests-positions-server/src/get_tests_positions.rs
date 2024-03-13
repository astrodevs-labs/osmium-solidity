use osmium_libs_lsp_server_wrapper::lsp_types::{request::Request, Range};
use serde::{Deserialize, Serialize};

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

pub struct GetTestsPositionsRequest {}

impl Request for GetTestsPositionsRequest {
    type Params = GetTestsPositionsParams;
    type Result = GetTestsPositionsResponse;
    const METHOD: &'static str = "osmium/getTestsPositions";
}
