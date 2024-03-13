use osmium_libs_foundry_wrapper::{CompilationError, Compiler, Error, ProjectCompileOutput};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
mod utils;
use utils::{convert_severity, get_root_path, normalized_slash_path, slashify_path};
mod affected_files_store;
use affected_files_store::AffectedFilesStore;

#[derive(Debug)]
struct State {
    compiler: Option<Compiler>,
    initialized: bool,
    affected_files: AffectedFilesStore,
}

#[derive(Debug)]
struct Backend {
    client: Client,
    state: Mutex<State>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.client
            .log_message(MessageType::INFO, "Foundry server initializing!")
            .await;
        if let Some(root_path) = get_root_path(params.clone()) {
            self.client
                .log_message(
                    MessageType::INFO,
                    &format!(
                        "Foundry server initializing with workspace path: {:?}",
                        root_path
                    ),
                )
                .await;
            let _ = self.load_workspace(root_path).await;
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    "Foundry server not initialized : no workspace path!",
                )
                .await;
        }
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Foundry server initialized!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("file opened!: {:}", params.text_document.uri),
            )
            .await;
        let _ = self
            .compile(normalized_slash_path(params.text_document.uri.path()))
            .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("file changed!: {:}", params.text_document.uri),
            )
            .await;
        let _ = self
            .compile(normalized_slash_path(params.text_document.uri.path()))
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    pub async fn load_workspace(&self, path: String) -> std::result::Result<(), ()> {
        let mut state = self.state.lock().await;
        match Compiler::new_with_executable_check() {
            Ok(compiler) => state.compiler = Some(compiler),
            Err(Error::FoundryExecutableNotFound) => {
                self.client
                    .show_message(MessageType::WARNING, "Foundry executable not found. Please install foundry and restart the extension.")
                    .await;
                return Err(());
            }
            Err(Error::InvalidFoundryVersion) => {
                self.client
                    .show_message(MessageType::WARNING, "Foundry executable version is not compatible with this extension. Please update foundry and restart the extension.")
                    .await;
                return Err(());
            }
            Err(err) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        &format!("Foundry server failed to initialize: {:?}", err),
                    )
                    .await;
                return Err(());
            }
        }
        if let Err(err) = state.compiler.as_mut().unwrap().load_workspace(path) {
            self.client
                .log_message(
                    MessageType::ERROR,
                    &format!("Foundry server failed to initialize: {:?}", err),
                )
                .await;
            return Err(());
        } else {
            state.initialized = true;
            self.client
                .log_message(MessageType::INFO, "Foundry server initialized!")
                .await;
        }
        Ok(())
    }

    /**
     * This function initializes the workspace if it is not already initialized.
     * @param {&str} filepath Filepath to compile
     * @returns {Result<(), ()>} Result of the initialization
     */
    async fn initialize_if_not(&self, filepath: &str) -> std::result::Result<(), ()> {
        let state = self.state.lock().await;

        if !state.initialized {
            drop(state); // unlock the mutex before calling load_workspace

            self.client
                .log_message(MessageType::INFO, "Foundry server initializing!")
                .await;
            let folder_path = Path::new(filepath)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            self.load_workspace(folder_path).await?
        }
        Ok(())
    }

    pub async fn compile(&self, filepath: String) -> std::result::Result<(), ()> {
        self.initialize_if_not(&filepath).await?;
        let mut state = self.state.lock().await;

        self.client
            .log_message(MessageType::INFO, "Foundry server compiling!")
            .await;

        match state.compiler.as_mut().unwrap().compile(&filepath) {
            Ok((project_path, output)) => {
                /*self.client
                .log_message(MessageType::INFO, format!("Compile errors: {:?}", output.get_errors()))
                .await;*/
                drop(state);
                self.publish_errors_diagnostics(slashify_path(&project_path), filepath, output)
                    .await;
            }
            Err(err) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("error while compiling: {:?}", err),
                    )
                    .await;
            }
        }
        Ok(())
    }

    /**
     * Generate and publish diagnostics from compilation errors
     * @param {String} project_path Project path
     * @param {String} filepath Filepath to compile
     * @param {ProjectCompileOutput} output Compilation output
     */
    pub async fn publish_errors_diagnostics(
        &self,
        project_path: String,
        filepath: String,
        output: ProjectCompileOutput,
    ) {
        let mut raised_diagnostics = HashMap::<String, Vec<Diagnostic>>::new();

        for error in output.get_errors() {
            // Generate diagnostic from compilation error
            let (affected_file, diagnostic) =
                match self.extract_diagnostic(error, &project_path).await {
                    Some(diagnostic) => diagnostic,
                    None => continue,
                };

            // Add diagnostic to the hashmap
            let url = match affected_file.to_str() {
                Some(source_path) => slashify_path(source_path),
                None => continue,
            };
            if !raised_diagnostics.contains_key(&url) {
                raised_diagnostics.insert(url.clone(), vec![diagnostic]);
            } else {
                raised_diagnostics.get_mut(&url).unwrap().push(diagnostic);
            }
        }

        self.reset_not_affected_files(project_path, filepath, &raised_diagnostics)
            .await;
        for (uri, diags) in raised_diagnostics.iter() {
            if let Ok(url) = Url::parse(&format!("file://{}", &uri)) {
                self.client
                    .publish_diagnostics(url, diags.clone(), None)
                    .await;
            } else {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("error, cannot parse file uri : {}", uri),
                    )
                    .await;
            }
        }
    }

    /**
     * Extract diagnostic from compilation error
     * @param {CompilationError} compilation_error Compilation error
     * @param {String} project_path Project path
     * @returns {Option<(PathBuf, Diagnostic)>} Diagnostic
     * @returns {None} If the diagnostic cannot be extracted
     */
    async fn extract_diagnostic(
        &self,
        compilation_error: &CompilationError,
        project_path: &str,
    ) -> Option<(PathBuf, Diagnostic)> {
        eprintln!("Compilation error: {:?}", compilation_error);
        let (source_content_filepath, range) = match self
            .extract_diagnostic_range(project_path, compilation_error)
            .await
        {
            Some((source_content_filepath, range)) => (source_content_filepath, range),
            None => return None,
        };
        let diagnostic = Diagnostic {
            range: Range {
                start: Position {
                    line: range.start.line,
                    character: range.start.column,
                },
                end: Position {
                    line: range.end.line,
                    character: range.end.column,
                },
            },
            severity: Some(convert_severity(compilation_error.get_severity())),
            code: None,
            code_description: None,
            source: Some("osmium-solidity-foundry-compiler".to_string()),
            message: compilation_error.get_message(),
            related_information: None,
            tags: None,
            data: None,
        };
        Some((source_content_filepath, diagnostic))
    }

    /**
     * Extract diagnostic range from compilation error's source location
     * Open the file and get the range from the source location
     * @param {String} project_path Project path
     * @param {CompilationError} error Compilation error
     * @returns {Option<(PathBuf, osmium_libs_foundry_wrapper::Range)>} Diagnostic range
     * @returns {None} If the diagnostic range cannot be extracted
     */
    async fn extract_diagnostic_range(
        &self,
        project_path: &str,
        error: &CompilationError,
    ) -> Option<(PathBuf, osmium_libs_foundry_wrapper::Range)> {
        let source_content_filepath = match error.get_file_path() {
            Some(source_path) => {
                let mut complete_path = Path::new(project_path).to_path_buf();
                complete_path.push(source_path);
                complete_path
            }
            None => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("error, cannot get filepath: {:?}", error),
                    )
                    .await;
                return None;
            }
        };
        let source_content = match std::fs::read_to_string(&source_content_filepath) {
            Ok(content) => content,
            Err(err) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!(
                            "error, cannot read file: {:?}, error: {:?}",
                            &source_content_filepath, err
                        ),
                    )
                    .await;
                return None;
            }
        };
        let range = match error.get_range(&source_content) {
            Some(range) => range,
            None => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("error, cannot get range: {:?}", error),
                    )
                    .await;
                return None;
            }
        };
        Some((source_content_filepath, range))
    }

    /**
     * This function resets the diagnostics of the files that are not raising an error anymore.
     * @param {String} project_path Project path
     * @param {String} filepath Filepath to compile
     * @param {HashMap<String, Vec<Diagnostic>>} raised_diagnostics Raised diagnostics
     */
    async fn reset_not_affected_files(
        &self,
        project_path: String,
        filepath: String,
        raised_diagnostics: &HashMap<String, Vec<Diagnostic>>,
    ) {
        let mut state = self.state.lock().await;

        state
            .affected_files
            .add_project_file(project_path.clone(), filepath.clone());
        let raised_files = raised_diagnostics.keys().cloned().collect::<Vec<String>>();
        let without_diagnostics = state
            .affected_files
            .fill_affected_files(raised_files, &project_path);

        self.client
            .log_message(
                MessageType::INFO,
                format!("files without diagnostic: {:?}", without_diagnostics),
            )
            .await;

        for file in without_diagnostics.iter() {
            if let Ok(url) = Url::parse(&format!("file://{}", &file)) {
                self.client.publish_diagnostics(url, vec![], None).await;
            } else {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("error, cannot parse file uri : {}", file),
                    )
                    .await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        state: Mutex::new(State {
            compiler: None,
            initialized: false,
            affected_files: AffectedFilesStore::new(),
        }),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
