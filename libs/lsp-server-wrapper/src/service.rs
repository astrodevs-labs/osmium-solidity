mod state;

pub(crate) use self::state::{ServerState, State};
pub use crate::client::Client;
use crate::{jsonrpc, LanguageServer};
use lsp_server::RequestId;
use lsp_types::request::*;
use lsp_types::*;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

pub(super) struct InnerService<S> {
    backend: Arc<S>,
    //pub(super) client: Arc<RefCell<Client>>,
}

pub struct LspService<S> {
    state: Arc<ServerState>,
    inner: InnerService<S>,
}

impl<S: LanguageServer> LspService<S> {
    pub fn new<F>(client: Rc<RefCell<Client>>, init: F) -> Self
    where
        F: FnOnce(Rc<RefCell<Client>>) -> S,
    {
        let backend = init(client);
        LspService {
            state: Arc::new(ServerState::new()),
            inner: InnerService {
                backend: Arc::new(backend),
            },
        }
    }

    pub fn call_request(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        let ret = match method {
            "initialize" => {
                self.state.set(State::Initializing);
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.initialize(params)?)
            }
            "shutdown" => {
                self.state.set(State::ShutDown);
                serde_json::to_value(self.inner.backend.shutdown()?)
            }
            "textDocument/willSaveWaitUntil" => {
                let params: WillSaveTextDocumentParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.will_save_wait_until(params)?)
            }
            "textDocument/declaration" => {
                let params: GotoDeclarationParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.goto_declaration(params)?)
            }
            "textDocument/definition" => {
                let params: GotoDefinitionParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.goto_definition(params)?)
            }
            "textDocument/typeDefinition" => {
                let params: GotoTypeDefinitionParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.goto_type_definition(params)?)
            }
            "textDocument/implementation" => {
                let params: GotoImplementationParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.goto_implementation(params)?)
            }
            "textDocument/references" => {
                let params: ReferenceParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.references(params)?)
            }
            "textDocument/prepareCallHierarchy" => {
                let params: CallHierarchyPrepareParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.prepare_call_hierarchy(params)?)
            }
            "textDocument/incomingCalls" => {
                let params: CallHierarchyIncomingCallsParams =
                    serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.incoming_calls(params)?)
            }
            "textDocument/outgoingCalls" => {
                let params: CallHierarchyOutgoingCallsParams =
                    serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.outgoing_calls(params)?)
            }
            "textDocument/prepareTypeHierarchy" => {
                let params: TypeHierarchyPrepareParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.prepare_type_hierarchy(params)?)
            }
            "textDocument/supertypes" => {
                let params: TypeHierarchySupertypesParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.supertypes(params)?)
            }
            "textDocument/subtypes" => {
                let params: TypeHierarchySubtypesParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.subtypes(params)?)
            }
            "textDocument/documentHighlight" => {
                let params: DocumentHighlightParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.document_highlight(params)?)
            }
            "textDocument/documentLink" => {
                let params: DocumentLinkParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.document_link(params)?)
            }
            "documentLink/resolve" => {
                let params: DocumentLink = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.document_link_resolve(params)?)
            }
            "textDocument/hover" => {
                let params: HoverParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.hover(params)?)
            }
            "textDocument/codeLens" => {
                let params: CodeLensParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.code_lens(params)?)
            }
            "codeLens/resolve" => {
                let params: CodeLens = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.code_lens_resolve(params)?)
            }
            "textDocument/foldingRange" => {
                let params: FoldingRangeParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.folding_range(params)?)
            }
            "textDocument/selectionRange" => {
                let params: SelectionRangeParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.selection_range(params)?)
            }
            "textDocument/documentSymbol" => {
                let params: DocumentSymbolParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.document_symbol(params)?)
            }
            "textDocument/sementicTokens/full" => {
                let params: SemanticTokensParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.semantic_tokens_full(params)?)
            }
            "textDocument/sementicTokens/full/delta" => {
                let params: SemanticTokensDeltaParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.semantic_tokens_full_delta(params)?)
            }
            "textDocument/sementicTokens/range" => {
                let params: SemanticTokensRangeParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.semantic_tokens_range(params)?)
            }
            "textDocument/inlineValue" => {
                let params: InlineValueParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.inline_value(params)?)
            }
            "textDocument/inlayHint" => {
                let params: InlayHintParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.inlay_hint(params)?)
            }
            "inlayHint/resolve" => {
                let params: InlayHint = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.inlay_hint_resolve(params)?)
            }
            "textDocument/moniker" => {
                let params: MonikerParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.moniker(params)?)
            }
            "textDocument/completion" => {
                let params: CompletionParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.completion(params)?)
            }
            "completionItem/resolve" => {
                let params: CompletionItem = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.completion_resolve(params)?)
            }
            "textDocument/diagnostic" => {
                let params: DocumentDiagnosticParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.diagnostic(params)?)
            }
            "workspace/diagnostic" => {
                let params: WorkspaceDiagnosticParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.workspace_diagnostic(params)?)
            }
            "textDocument/signatureHelp" => {
                let params: SignatureHelpParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.signature_help(params)?)
            }
            "textDocument/codeAction" => {
                let params: CodeActionParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.code_action(params)?)
            }
            "codeAction/resolve" => {
                let params: CodeAction = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.code_action_resolve(params)?)
            }
            "textDocument/documentColor" => {
                let params: DocumentColorParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.document_color(params)?)
            }
            "textDocument/colorPresentation" => {
                let params: ColorPresentationParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.color_presentation(params)?)
            }
            "textDocument/formatting" => {
                let params: DocumentFormattingParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.formatting(params)?)
            }
            "textDocument/rangeFormatting" => {
                let params: DocumentRangeFormattingParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.range_formatting(params)?)
            }
            "textDocument/onTypeFormatting" => {
                let params: DocumentOnTypeFormattingParams =
                    serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.on_type_formatting(params)?)
            }
            "textDocument/rename" => {
                let params: RenameParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.rename(params)?)
            }
            "textDocument/prepareRename" => {
                let params: TextDocumentPositionParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.prepare_rename(params)?)
            }
            "textDocument/linkedEditingRange" => {
                let params: LinkedEditingRangeParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.linked_editing_range(params)?)
            }
            "workspace/symbol" => {
                let params: WorkspaceSymbolParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.symbol(params)?)
            }
            "workspaceSymbol/resolve" => {
                let params: WorkspaceSymbol = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.symbol_resolve(params)?)
            }
            "workspace/willCreateFiles" => {
                let params: CreateFilesParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.will_create_files(params)?)
            }
            "workspace/willRenameFiles" => {
                let params: RenameFilesParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.will_rename_files(params)?)
            }
            "workspace/willDeleteFiles" => {
                let params: DeleteFilesParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.will_delete_files(params)?)
            }
            "workspace/executeCommand" => {
                let params: ExecuteCommandParams = serde_json::from_value(params).unwrap();
                serde_json::to_value(self.inner.backend.execute_command(params)?)
            }
            "exit" => {
                self.state.set(State::Exited);
                return Ok(None);
            }
            _ => {
                return Err(Box::new(jsonrpc::Error::method_not_found()).into());
            }
        }
        .map_err(|e| {
            eprintln!("Error: {}", e);
            Box::new(e)
        })?;
        Ok(Some(ret))
    }

    pub fn call_notification(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<(), Box<dyn Error>> {
        match method {
            "initialized" => {
                self.state.set(State::Initialized);
                let params: InitializedParams = serde_json::from_value(params).unwrap();
                self.inner.backend.initialized(params);
            }
            "textDocument/didOpen" => {
                let params: DidOpenTextDocumentParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_open(params);
            }
            "textDocument/didChange" => {
                let params: DidChangeTextDocumentParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_change(params);
            }
            "textDocument/willSave" => {
                let params: WillSaveTextDocumentParams = serde_json::from_value(params).unwrap();
                self.inner.backend.will_save(params);
            }
            "textDocument/didSave" => {
                let params: DidSaveTextDocumentParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_save(params);
            }
            "textDocument/didClose" => {
                let params: DidCloseTextDocumentParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_close(params);
            }
            "workspace/didChangeConfiguration" => {
                let params: DidChangeConfigurationParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_change_configuration(params);
            }
            "workspace/didChangeWorkspaceFolders" => {
                let params: DidChangeWorkspaceFoldersParams =
                    serde_json::from_value(params).unwrap();
                self.inner.backend.did_change_workspace_folders(params);
            }
            "workspace/didCreateFiles" => {
                let params: CreateFilesParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_create_files(params)
            }
            "workspace/didRenameFiles" => {
                let params: RenameFilesParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_rename_files(params)
            }
            "workspace/didDeleteFiles" => {
                let params: DeleteFilesParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_delete_files(params)
            }
            "workspace/didChangeWatchedFiles" => {
                let params: DidChangeWatchedFilesParams = serde_json::from_value(params).unwrap();
                self.inner.backend.did_change_watched_files(params)
            }
            _ => {
                return Err("Method not found".to_owned().into());
            }
        }
        Ok(())
    }

    pub fn call_response(&self, id: RequestId, result: Option<serde_json::Value>) {
        self.inner.backend.on_response(id, result)
    }
}
