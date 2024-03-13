use osmium_libs_lsp_server_wrapper::lsp_types::request::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentRequestParams {
    pub uri: String,
}

pub struct ContentRequest {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentResponse {
    pub content: String,
}

impl Request for ContentRequest {
    type Params = ContentRequestParams;
    type Result = ContentResponse;
    const METHOD: &'static str = "osmium/getContent";
}
