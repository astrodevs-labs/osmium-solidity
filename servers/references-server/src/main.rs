mod utils;

use crate::utils::*;

use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::lsp_types::Location as LspLocation;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use solc_references::*;


struct Backend {
    client: Client,
    references_provider: Arc<Mutex<ReferencesProvider>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client, references_provider: Arc::new(Mutex::new(ReferencesProvider { files: Vec::new(), base_path: String::new()})) }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(workspace) = params.workspace_folders {
            self.references_provider.lock().await.set_base_path(normalize_path(workspace[0].uri.path()));
        } else {
            self.references_provider.lock().await.set_base_path(normalize_path(&params.root_uri.unwrap().path()));
        }
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "osmium-solidity-references initialized!")
            .await;
        if let Err(e) = self.references_provider.lock().await.update_file_content() {
            self.client.log_message(MessageType::ERROR, format!("Error updating file content: {}", e)).await;
        }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        if let Err(e) = self.references_provider.lock().await.update_file_content() {
            self.client.log_message(MessageType::ERROR, format!("Error updating file content: {}", e)).await;
        }
    }
    
    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<LspLocation>>> {
        self.client.log_message(MessageType::INFO, "References requested").await;
        let uri = params.text_document_position.text_document.uri;
        let mut position = params.text_document_position.position;
        position.line += 1;
        position.character += 1;

        let locations = self.references_provider.lock().await.get_references(&normalize_path(uri.path()), solc_references::Position { line: position.line, column: position.character });
        let ret: Vec<LspLocation> = locations.iter().map(|location| {
            let mut new_uri = uri.clone();
            new_uri.set_path(&escape_path(&location.uri));
            location_to_lsp_location(&new_uri, &location)
        }).collect();
        Ok(Some(ret))
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        self.client.log_message(MessageType::INFO, "Goto definition requested").await;
        let mut uri = params.text_document_position_params.text_document.uri;
        let mut position = params.text_document_position_params.position;
        position.line += 1;
        position.character += 1;

        let location = self.references_provider.lock().await.get_definition(&normalize_path(uri.path()), solc_references::Position { line: position.line, column: position.character });
        
        if let Some(location) = location {
            uri.set_path(&escape_path(&location.uri));
            return Ok(Some(GotoDefinitionResponse::Scalar(location_to_lsp_location(&uri, &location))));
        }
        self.client.log_message(MessageType::INFO, "No definition found").await;
        Ok(None)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

}

impl Backend {
   
}

#[tokio::main]
async fn main() {
    /* 
    
    USE THIS CODE TO DEBUG
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9001").await?;
    let (stream, _) = listener.accept().await?;
    let (read, write) = tokio::io::split(stream);
    Server::new(read, write, socket).serve(service).await;
    */

    let (service, socket) = LspService::new(Backend::new);
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout, socket).serve(service).await;

    //Ok(())
}
