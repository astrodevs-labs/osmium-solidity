mod utils;

use crate::utils::*;

use osmium_libs_solidity_code_actions::*;
use osmium_libs_solidity_lsp_utils::log::{error, info, init_logging, warn};
use osmium_libs_solidity_path_utils::{escape_path, normalize_path};
use std::collections::HashMap;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Location as LspLocation, Position};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    code_actions_provider: Arc<CodeActionsProvider>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        init_logging(client);
        Self {
            code_actions_provider: Arc::new(CodeActionsProvider::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(workspace) = params.workspace_folders {
            self.code_actions_provider
                .set_base_path(normalize_path(workspace[0].uri.path()));
        } else {
            self.code_actions_provider
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
                rename_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("osmium-solidity-references initialized!");
        self.update().await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        eprintln!("Compile requested");
        let init_time = std::time::Instant::now();
        self.update().await;
        info!(
            "Compile and Update time: {:?}",
            init_time.elapsed().as_secs()
        );
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<LspLocation>>> {
        let uri = params.text_document_position.text_document.uri;
        let mut position = params.text_document_position.position;
        position.line += 1;
        position.character += 1;
        info!("Reference requested at position: {:?}", position);

        let locations = self.code_actions_provider.get_references(
            &normalize_path(uri.path()),
            osmium_libs_solidity_code_actions::Position {
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
        let mut uri = params.text_document_position_params.text_document.uri;
        let mut position = params.text_document_position_params.position;
        position.line += 1;
        position.character += 1;
        info!("Goto definition requested at position: {:?}", position);

        let location = self.code_actions_provider.get_definition(
            &normalize_path(uri.path()),
            osmium_libs_solidity_code_actions::Position {
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
        eprintln!("Completion requested");
        let mut position = params.text_document_position.position;
        position.line += 1;
        position.character += 1;
        let completes = self.code_actions_provider.get_completions(
            &normalize_path(params.text_document_position.text_document.uri.path()),
            osmium_libs_solidity_code_actions::Position {
                line: position.line,
                column: position.character,
            },
        );
        if completes.is_empty() {
            warn!("No completions found");
        }
        let completes = completes
            .iter()
            .map(|item| {
                let kind: i64 = item.kind.value();
                CompletionItem {
                    label: item.label.clone(),
                    kind: Some(self.completion_kind_from_i64(kind)), //TODO: transform to lsp kind
                    ..Default::default()
                }
            })
            .collect::<Vec<CompletionItem>>()
            .into_iter()
            .fold(Vec::new(), |mut acc, x| {
                if !acc.contains(&x) {
                    acc.push(x);
                }
                acc
            });
        info!("Completions found: {:?}", completes.len());
        Ok(Some(CompletionResponse::Array(completes)))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let mut position = params.text_document_position.position;
        position.line += 1;
        position.character += 1;
        let edits = self.code_actions_provider.refactor(
            &normalize_path(params.text_document_position.text_document.uri.path()), 
            osmium_libs_solidity_code_actions::Position {
                line: position.line,
                column: position.character,
            },
        );
        let mut workspace_edits = HashMap::<Url, Vec<TextEdit>>::new();
        edits.iter().for_each(|edit| {
            let new_edit = TextEdit {
                range: Range {
                    start: Position {
                        line: edit.start.line - 1,
                        character: edit.start.column - 1
                    },
                    end: Position {
                        line: edit.end.line - 1,
                        character: edit.end.column - 1
                    }
                },
                new_text: params.new_name.clone()
            };
            let url = Url::from_file_path(&edit.uri).unwrap();
            if let Some(arr) = workspace_edits.get_mut(&url) {
                arr.push(new_edit)
            }
            else {
                workspace_edits.insert(url, vec![new_edit]);
            }
        });
        return Ok(Some(WorkspaceEdit{changes:Some(workspace_edits), document_changes: None, change_annotations: None }))
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
        let ref_provider = self.code_actions_provider.clone();
        let _ = tokio::spawn(async move {
            info!("Updating references");
            if let Err(e) = ref_provider.update_file_content() {
                error!("Error updating references: {}", e);
            }
        })
        .await;
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
    Server::new(stdin, stdout, socket)
        .concurrency_level(2)
        .serve(service)
        .await;

    //Ok(())
}
