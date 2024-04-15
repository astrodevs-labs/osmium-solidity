mod utils;

use crate::utils::*;

use osmium_libs_solidity_lsp_utils::log::{error, info, init_logging, warn};
use osmium_libs_solidity_path_utils::{escape_path, normalize_path};
use osmium_libs_solidity_references::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::Location as LspLocation;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    references_provider: Arc<Mutex<ReferencesProvider>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        init_logging(client);
        Self {
            references_provider: Arc::new(Mutex::new(ReferencesProvider {
                files: Vec::new(),
                base_path: String::new(),
            })),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(workspace) = params.workspace_folders {
            self.references_provider
                .lock()
                .await
                .set_base_path(normalize_path(workspace[0].uri.path()));
        } else {
            self.references_provider
                .lock()
                .await
                .set_base_path(normalize_path(params.root_uri.unwrap().path()));
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
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("osmium-solidity-references initialized!");
        self.update().await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.update().await;
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<LspLocation>>> {
        info!("References requested");
        let uri = params.text_document_position.text_document.uri;
        let mut position = params.text_document_position.position;
        position.line += 1;
        position.character += 1;

        let locations = self.references_provider.lock().await.get_references(
            &normalize_path(uri.path()),
            osmium_libs_solidity_references::Position {
                line: position.line,
                column: position.character,
            },
        );
        let ret: Vec<LspLocation> = locations
            .iter()
            .map(|location| {
                let mut new_uri = uri.clone();
                new_uri.set_path(&escape_path(&location.uri));
                location_to_lsp_location(&new_uri, location)
            })
            .collect();
        Ok(Some(ret))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        info!("Goto definition requested");
        let mut uri = params.text_document_position_params.text_document.uri;
        let mut position = params.text_document_position_params.position;
        position.line += 1;
        position.character += 1;

        let location = self.references_provider.lock().await.get_definition(
            &normalize_path(uri.path()),
            osmium_libs_solidity_references::Position {
                line: position.line,
                column: position.character,
            },
        );

        if let Some(location) = location {
            uri.set_path(&escape_path(&location.uri));
            return Ok(Some(GotoDefinitionResponse::Scalar(
                location_to_lsp_location(&uri, &location),
            )));
        }
        info!("No definition found");
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        info!("Completion requested");
        let mut position = params.text_document_position.position;
        position.line += 1;
        position.character += 1;
        let completes = self.references_provider.lock().await.get_scoped_completes(
            &normalize_path(&params.text_document_position.text_document.uri.path()),
            osmium_libs_solidity_references::Position {
                line: position.line,
                column: position.character,
            }
        );
        if completes.is_empty() {
            warn!("No completions found");
        }
        for complete in &completes {
            info!("Complete: {:?}", complete);
        }
        let completes = completes.iter().map(|item| {
            let kind: i64 = item.kind.value();
            CompletionItem {
                label: item.label.clone(),
                kind: Some(self.completion_kind_from_i64(kind)), //TODO: transform to lsp kind
                ..Default::default()
            }
        
        }).collect::<Vec<CompletionItem>>();
        let completes = completes.into_iter().fold(Vec::new(), |mut acc, x| {
            if !acc.contains(&x) {
                acc.push(x);
            }
            acc
        });

        Ok(Some(CompletionResponse::Array(completes)))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    fn completion_kind_from_i64(&self, value: i64) -> CompletionItemKind {
        match value {
            1 => CompletionItemKind::TEXT,
            2 => CompletionItemKind::METHOD,
            3 => CompletionItemKind::FUNCTION,
            4 => CompletionItemKind::CONSTRUCTOR,
            5 => CompletionItemKind::FIELD,
            6 => CompletionItemKind::VARIABLE,
            7 => CompletionItemKind::CLASS,
            8 => CompletionItemKind::INTERFACE,
            9 => CompletionItemKind::MODULE,
            10 => CompletionItemKind::PROPERTY,
            11 => CompletionItemKind::UNIT,
            12 => CompletionItemKind::VALUE,
            13 => CompletionItemKind::ENUM,
            14 => CompletionItemKind::KEYWORD,
            15 => CompletionItemKind::SNIPPET,
            16 => CompletionItemKind::COLOR,
            17 => CompletionItemKind::FILE,
            18 => CompletionItemKind::REFERENCE,
            19 => CompletionItemKind::FOLDER,
            20 => CompletionItemKind::ENUM_MEMBER,
            21 => CompletionItemKind::CONSTANT,
            22 => CompletionItemKind::STRUCT,
            23 => CompletionItemKind::EVENT,
            24 => CompletionItemKind::OPERATOR,
            25 => CompletionItemKind::TYPE_PARAMETER,
            _ => CompletionItemKind::TEXT,
        }

    }

    async fn update(&self) {
        if let Err(e) = self.references_provider.lock().await.update_file_content() {
            error!("Error updating file content: {}", e);
        }
    }
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
