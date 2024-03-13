use get_tests_positions::{GetTestsPositionsParams, GetTestsPositionsResponse, Test, TestContract};
use osmium_libs_solidity_ast_extractor::retriever::retrieve_functions_nodes;
use osmium_libs_solidity_ast_extractor::File;
use osmium_libs_solidity_ast_extractor::{
    extract::extract_ast_from_content, retriever::retrieve_contract_nodes,
};
use tower_lsp::jsonrpc::{self, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod get_tests_positions;
mod utils;
use utils::range_from_spanned;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    fn new(client: Client) -> Self {
        Self { client }
    }

    async fn get_tests_positions(
        &self,
        params: GetTestsPositionsParams,
    ) -> Result<GetTestsPositionsResponse> {
        self.client
            .log_message(MessageType::INFO, "Getting tests positions for file")
            .await;
        let res = extract_ast_from_content(&params.file_content);

        if let Ok(ast) = res {
            self.extract_tests_positions(ast)
        } else {
            let err = res.unwrap_err();
            eprintln!("Error: {:?}", err);
            Err(jsonrpc::Error::invalid_params(format!("Error: {:?}", err)))
        }
    }

    pub fn extract_tests_positions(&self, ast: File) -> Result<GetTestsPositionsResponse> {
        let mut res = vec![];
        let re = regex::Regex::new(r"^test.*_.+").unwrap();
        let contracts = retrieve_contract_nodes(&ast);
        for contract in contracts {
            let mut tests: Vec<Test> = vec![];
            let mut functions = retrieve_functions_nodes(&contract);
            let contract_tests = functions.iter_mut().filter(|f| {
                f.name.is_some() && re.is_match(f.name.as_ref().unwrap().as_string().as_str())
            });
            for test in contract_tests {
                let name = match &test.name {
                    Some(name) => name,
                    None => continue,
                };
                tests.push(Test {
                    name: name.as_string(),
                    range: range_from_spanned(name),
                });
            }
            res.push(TestContract {
                name: contract.name.as_string(),
                range: range_from_spanned(&contract.name),
                tests,
            });
        }
        Ok(GetTestsPositionsResponse { contracts: res })
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(Backend::new)
        .custom_method("osmium/getTestsPositions", Backend::get_tests_positions)
        .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}
