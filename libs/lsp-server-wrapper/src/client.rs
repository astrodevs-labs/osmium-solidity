use crate::jsonrpc::{self};
use lsp_server::{Message, RequestId};
use lsp_types::notification::*;
use lsp_types::request::*;
use lsp_types::*;
use serde::Serialize;
use serde_json::Value;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Weak;
use tracing::error;

use crate::server::LspServer;

#[derive(Clone)]
pub(crate) struct ClientInner {
    server: Option<Weak<dyn LspServer>>,
    id: RefCell<u32>,
}

/// Handle for communicating with the language client.
///
/// This type provides a very cheap implementation of [`Clone`] so API consumers can cheaply clone
/// and pass it around as needed.
///
/// It also implements [`tower::Service`] in order to remain independent from the underlying
/// transport and to facilitate further abstraction with middleware.
#[derive(Clone)]
pub struct Client {
    inner: ClientInner,
}

impl Client {
    pub(super) fn new() -> Self {
        Client {
            inner: ClientInner {
                server: None,
                id: RefCell::new(0),
            },
        }
    }

    pub(super) fn set_server(&mut self, server: Weak<dyn LspServer>) {
        self.inner.server = Some(server);
    }
}

impl Client {
    // Lifecycle Messages

    /// Registers a new capability with the client.
    ///
    /// This corresponds to the [`client/registerCapability`] request.
    ///
    /// [`client/registerCapability`]: https://microsoft.github.io/language-server-protocol/specification#client_registerCapability
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    pub fn register_capability(
        &self,
        registrations: Vec<Registration>,
    ) -> jsonrpc::Result<RequestId> {
        self.send_request::<RegisterCapability>(RegistrationParams { registrations })
    }

    /// Unregisters a capability with the client.
    ///
    /// This corresponds to the [`client/unregisterCapability`] request.
    ///
    /// [`client/unregisterCapability`]: https://microsoft.github.io/language-server-protocol/specification#client_unregisterCapability
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    pub fn unregister_capability(
        &self,
        unregisterations: Vec<Unregistration>,
    ) -> jsonrpc::Result<RequestId> {
        self.send_request::<UnregisterCapability>(UnregistrationParams { unregisterations })
    }

    // Window Features

    /// Notifies the client to display a particular message in the user interface.
    ///
    /// This corresponds to the [`window/showMessage`] notification.
    ///
    /// [`window/showMessage`]: https://microsoft.github.io/language-server-protocol/specification#window_showMessage
    pub fn show_message<M: Display>(&self, typ: MessageType, message: M) {
        self.send_notification_unchecked::<ShowMessage>(ShowMessageParams {
            typ,
            message: message.to_string(),
        });
    }

    /// Requests the client to display a particular message in the user interface.
    ///
    /// Unlike the `show_message` notification, this request can also pass a list of actions and
    /// wait for an answer from the client.
    ///
    /// This corresponds to the [`window/showMessageRequest`] request.
    ///
    /// [`window/showMessageRequest`]: https://microsoft.github.io/language-server-protocol/specification#window_showMessageRequest
    pub fn show_message_request<M: Display>(
        &self,
        typ: MessageType,
        message: M,
        actions: Option<Vec<MessageActionItem>>,
    ) -> jsonrpc::Result<RequestId> {
        self.send_request_unchecked::<ShowMessageRequest>(ShowMessageRequestParams {
            typ,
            message: message.to_string(),
            actions,
        })
    }

    /// Notifies the client to log a particular message.
    ///
    /// This corresponds to the [`window/logMessage`] notification.
    ///
    /// [`window/logMessage`]: https://microsoft.github.io/language-server-protocol/specification#window_logMessage
    pub fn log_message<M: Display>(&self, typ: MessageType, message: M) {
        self.send_notification_unchecked::<LogMessage>(LogMessageParams {
            typ,
            message: message.to_string(),
        });
    }

    /// Asks the client to display a particular resource referenced by a URI in the user interface.
    ///
    /// Returns `Ok(true)` if the document was successfully shown, or `Ok(false)` otherwise.
    ///
    /// This corresponds to the [`window/showDocument`] request.
    ///
    /// [`window/showDocument`]: https://microsoft.github.io/language-server-protocol/specification#window_showDocument
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.
    pub fn show_document(&self, params: ShowDocumentParams) -> jsonrpc::Result<RequestId> {
        self.send_request::<ShowDocument>(params)
    }

    // TODO: Add `work_done_progress_create()` here (since 3.15.0) when supported by `tower-lsp`.
    // https://github.com/ebkalderon/tower-lsp/issues/176

    /// Notifies the client to log a telemetry event.
    ///
    /// This corresponds to the [`telemetry/event`] notification.
    ///
    /// [`telemetry/event`]: https://microsoft.github.io/language-server-protocol/specification#telemetry_event
    pub fn telemetry_event<U: Serialize>(&self, data: U) {
        match serde_json::to_value(data) {
            Err(e) => error!("invalid JSON in `telemetry/event` notification: {}", e),
            Ok(mut value) => {
                if !value.is_null() && !value.is_array() && !value.is_object() {
                    value = Value::Array(vec![value]);
                }
                self.send_notification_unchecked::<TelemetryEvent>(value);
            }
        }
    }

    /// Asks the client to refresh the code lenses currently shown in editors. As a result, the
    /// client should ask the server to recompute the code lenses for these editors.
    ///
    /// This is useful if a server detects a configuration change which requires a re-calculation
    /// of all code lenses.
    ///
    /// Note that the client still has the freedom to delay the re-calculation of the code lenses
    /// if for example an editor is currently not visible.
    ///
    /// This corresponds to the [`workspace/codeLens/refresh`] request.
    ///
    /// [`workspace/codeLens/refresh`]: https://microsoft.github.io/language-server-protocol/specification#codeLens_refresh
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.
    pub fn code_lens_refresh(&self) -> jsonrpc::Result<RequestId> {
        self.send_request::<CodeLensRefresh>(())
    }

    /// Asks the client to refresh the editors for which this server provides semantic tokens. As a
    /// result, the client should ask the server to recompute the semantic tokens for these
    /// editors.
    ///
    /// This is useful if a server detects a project-wide configuration change which requires a
    /// re-calculation of all semantic tokens. Note that the client still has the freedom to delay
    /// the re-calculation of the semantic tokens if for example an editor is currently not visible.
    ///
    /// This corresponds to the [`workspace/semanticTokens/refresh`] request.
    ///
    /// [`workspace/semanticTokens/refresh`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_semanticTokens
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.
    pub fn semantic_tokens_refresh(&self) -> jsonrpc::Result<RequestId> {
        self.send_request::<SemanticTokensRefresh>(())
    }

    /// Asks the client to refresh the inline values currently shown in editors. As a result, the
    /// client should ask the server to recompute the inline values for these editors.
    ///
    /// This is useful if a server detects a configuration change which requires a re-calculation
    /// of all inline values. Note that the client still has the freedom to delay the
    /// re-calculation of the inline values if for example an editor is currently not visible.
    ///
    /// This corresponds to the [`workspace/inlineValue/refresh`] request.
    ///
    /// [`workspace/inlineValue/refresh`]: https://microsoft.github.io/language-server-protocol/specification#workspace_inlineValue_refresh
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.
    pub fn inline_value_refresh(&self) -> jsonrpc::Result<RequestId> {
        self.send_request::<InlineValueRefreshRequest>(())
    }

    /// Asks the client to refresh the inlay hints currently shown in editors. As a result, the
    /// client should ask the server to recompute the inlay hints for these editors.
    ///
    /// This is useful if a server detects a configuration change which requires a re-calculation
    /// of all inlay hints. Note that the client still has the freedom to delay the re-calculation
    /// of the inlay hints if for example an editor is currently not visible.
    ///
    /// This corresponds to the [`workspace/inlayHint/refresh`] request.
    ///
    /// [`workspace/inlayHint/refresh`]: https://microsoft.github.io/language-server-protocol/specification#workspace_inlayHint_refresh
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.
    pub fn inlay_hint_refresh(&self) -> jsonrpc::Result<RequestId> {
        self.send_request::<InlayHintRefreshRequest>(())
    }

    /// Asks the client to refresh all needed document and workspace diagnostics.
    ///
    /// This is useful if a server detects a project wide configuration change which requires a
    /// re-calculation of all diagnostics.
    ///
    /// This corresponds to the [`workspace/diagnostic/refresh`] request.
    ///
    /// [`workspace/diagnostic/refresh`]: https://microsoft.github.io/language-server-protocol/specification#diagnostic_refresh
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.
    pub fn workspace_diagnostic_refresh(&self) -> jsonrpc::Result<RequestId> {
        self.send_request::<WorkspaceDiagnosticRefresh>(())
    }

    /// Submits validation diagnostics for an open file with the given URI.
    ///
    /// This corresponds to the [`textDocument/publishDiagnostics`] notification.
    ///
    /// [`textDocument/publishDiagnostics`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_publishDiagnostics
    ///
    /// # Initialization
    ///
    /// This notification will only be sent if the server is initialized.
    pub fn publish_diagnostics(&self, uri: Url, diags: Vec<Diagnostic>, version: Option<i32>) {
        self.send_notification::<PublishDiagnostics>(PublishDiagnosticsParams::new(
            uri, diags, version,
        ));
    }

    // Workspace Features

    /// Fetches configuration settings from the client.
    ///
    /// The request can fetch several configuration settings in one roundtrip. The order of the
    /// returned configuration settings correspond to the order of the passed
    /// [`ConfigurationItem`]s (e.g. the first item in the response is the result for the first
    /// configuration item in the params).
    ///
    /// This corresponds to the [`workspace/configuration`] request.
    ///
    /// [`workspace/configuration`]: https://microsoft.github.io/language-server-protocol/specification#workspace_configuration
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.6.0.
    pub fn configuration(&self, items: Vec<ConfigurationItem>) -> jsonrpc::Result<RequestId> {
        self.send_request::<WorkspaceConfiguration>(ConfigurationParams { items })
    }

    /// Fetches the current open list of workspace folders.
    ///
    /// Returns `None` if only a single file is open in the tool. Returns an empty `Vec` if a
    /// workspace is open but no folders are configured.
    ///
    /// This corresponds to the [`workspace/workspaceFolders`] request.
    ///
    /// [`workspace/workspaceFolders`]: https://microsoft.github.io/language-server-protocol/specification#workspace_workspaceFolders
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.6.0.
    pub fn workspace_folders(&self) -> jsonrpc::Result<RequestId> {
        self.send_request::<WorkspaceFoldersRequest>(())
    }

    /// Requests a workspace resource be edited on the client side and returns whether the edit was
    /// applied.
    ///
    /// This corresponds to the [`workspace/applyEdit`] request.
    ///
    /// [`workspace/applyEdit`]: https://microsoft.github.io/language-server-protocol/specification#workspace_applyEdit
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    pub fn apply_edit(&self, edit: WorkspaceEdit) -> jsonrpc::Result<RequestId> {
        self.send_request::<ApplyWorkspaceEdit>(ApplyWorkspaceEditParams { edit, label: None })
    }

    /// Sends a custom notification to the client.
    ///
    /// # Initialization
    ///
    /// This notification will only be sent if the server is initialized.
    pub fn send_notification<N>(&self, params: N::Params)
    where
        N: lsp_types::notification::Notification,
    {
        self.send_notification_unchecked::<N>(params)
    }

    fn send_notification_unchecked<N>(&self, params: N::Params)
    where
        N: lsp_types::notification::Notification,
    {
        match self.inner.server.clone().unwrap().upgrade() {
            Some(server) => {
                server.send(Message::Notification(lsp_server::Notification::new(
                    N::METHOD.to_string(),
                    params,
                )));
            }
            None => {
                eprintln!("Cannot send notification, server is not initialized");
            }
        }
    }

    /// Sends a custom request to the client.
    ///
    /// # Initialization
    ///
    /// If the request is sent to the client before the server has been initialized, this will
    /// immediately return `Err` with JSON-RPC error code `-32002` ([read more]).
    ///
    /// [read more]: https://microsoft.github.io/language-server-protocol/specification#initialize
    pub fn send_request<R>(&self, params: R::Params) -> jsonrpc::Result<RequestId>
    where
        R: lsp_types::request::Request,
    {
        self.send_request_unchecked::<R>(params)
    }

    fn send_request_unchecked<R>(&self, params: R::Params) -> jsonrpc::Result<RequestId>
    where
        R: lsp_types::request::Request,
    {
        match self.inner.server.clone().unwrap().upgrade() {
            Some(server) => {
                let id = RequestId::from(self.next_request_id().to_string());
                server.send(Message::Request(lsp_server::Request::new(
                    id.clone(),
                    R::METHOD.to_string(),
                    params,
                )));
                Ok(id)
            }
            None => {
                eprintln!("Cannot send request, server is not initialized");
                Err(jsonrpc::not_initialized_error())
            }
        }
    }
}

impl Client {
    fn next_request_id(&self) -> u32 {
        let id = *self.inner.id.borrow_mut();
        if id == u32::MAX {
            *self.inner.id.borrow_mut() = 0;
        } else {
            *self.inner.id.borrow_mut() += 1;
        }
        id
    }
}
