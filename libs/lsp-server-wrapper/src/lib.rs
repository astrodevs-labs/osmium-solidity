mod client;
mod jsonrpc;
mod server;
mod service;

pub use crate::jsonrpc::{Error, Result};
pub use client::Client;
pub use lsp_server::RequestId;
pub use lsp_types;
use lsp_types::request::{
    GotoDeclarationParams, GotoDeclarationResponse, GotoImplementationParams,
    GotoImplementationResponse, GotoTypeDefinitionParams, GotoTypeDefinitionResponse,
};
use lsp_types::*;
use serde_json::Value;
pub use server::LspStdioServer;
pub use service::LspService;

/// Trait implemented by language server backends.
///
/// This interface allows servers adhering to the [Language Server Protocol] to be implemented in a
/// safe and easily testable way without exposing the low-level implementation details.
///
/// [Language Server Protocol]: https://microsoft.github.io/language-server-protocol/
pub trait LanguageServer {
    /// The [`initialize`] request is the first request sent from the client to the server.
    ///
    /// [`initialize`]: https://microsoft.github.io/language-server-protocol/specification#initialize
    ///
    /// This method is guaranteed to only execute once. If the client sends this request to the
    /// server again, the server will respond with JSON-RPC error code `-32600` (invalid request).
    fn initialize(&self, params: InitializeParams) -> Result<InitializeResult>;

    /// The [`initialized`] notification is sent from the client to the server after the client
    /// received the result of the initialize request but before the client sends anything else.
    ///
    /// [`initialized`]: https://microsoft.github.io/language-server-protocol/specification#initialized
    ///
    /// The server can use the `initialized` notification, for example, to dynamically register
    /// capabilities with the client.
    fn initialized(&self, params: InitializedParams) {
        let _ = params;
    }

    /// The [`shutdown`] request asks the server to gracefully shut down, but to not exit.
    ///
    /// [`shutdown`]: https://microsoft.github.io/language-server-protocol/specification#shutdown
    ///
    /// This request is often later followed by an [`exit`] notification, which will cause the
    /// server to exit immediately.
    ///
    /// [`exit`]: https://microsoft.github.io/language-server-protocol/specification#exit
    ///
    /// This method is guaranteed to only execute once. If the client sends this request to the
    /// server again, the server will respond with JSON-RPC error code `-32600` (invalid request).
    fn shutdown(&self) -> Result<()>;

    // Document Synchronization

    /// The [`textDocument/didOpen`] notification is sent from the client to the server to signal
    /// that a new text document has been opened by the client.
    ///
    /// [`textDocument/didOpen`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_didOpen
    ///
    /// The document's truth is now managed by the client and the server must not try to read the
    /// document’s truth using the document's URI. "Open" in this sense means it is managed by the
    /// client. It doesn't necessarily mean that its content is presented in an editor.
    fn did_open(&self, params: DidOpenTextDocumentParams) {
        let _ = params;
        eprintln!("Got a textDocument/didOpen notification, but it is not implemented");
    }

    /// The [`textDocument/didChange`] notification is sent from the client to the server to signal
    /// changes to a text document.
    ///
    /// [`textDocument/didChange`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_didChange
    ///
    /// This notification will contain a distinct version tag and a list of edits made to the
    /// document for the server to interpret.
    fn did_change(&self, params: DidChangeTextDocumentParams) {
        let _ = params;
        eprintln!("Got a textDocument/didChange notification, but it is not implemented");
    }

    /// The [`textDocument/willSave`] notification is sent from the client to the server before the
    /// document is actually saved.
    ///
    /// [`textDocument/willSave`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_willSave
    fn will_save(&self, params: WillSaveTextDocumentParams) {
        let _ = params;
        eprintln!("Got a textDocument/willSave notification, but it is not implemented");
    }

    /// The [`textDocument/willSaveWaitUntil`] request is sent from the client to the server before
    /// the document is actually saved.
    ///
    /// [`textDocument/willSaveWaitUntil`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_willSaveWaitUntil
    ///
    /// The request can return an array of `TextEdit`s which will be applied to the text document
    /// before it is saved.
    ///
    /// Please note that clients might drop results if computing the text edits took too long or if
    /// a server constantly fails on this request. This is done to keep the save fast and reliable.
    fn will_save_wait_until(
        &self,
        params: WillSaveTextDocumentParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let _ = params;
        eprintln!("Got a textDocument/willSaveWaitUntil request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/didSave`] notification is sent from the client to the server when the
    /// document was saved in the client.
    ///
    /// [`textDocument/didSave`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_didSave
    fn did_save(&self, params: DidSaveTextDocumentParams) {
        let _ = params;
        eprintln!("Got a textDocument/didSave notification, but it is not implemented");
    }

    /// The [`textDocument/didClose`] notification is sent from the client to the server when the
    /// document got closed in the client.
    ///
    /// [`textDocument/didClose`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_didClose
    ///
    /// The document's truth now exists where the document's URI points to (e.g. if the document's
    /// URI is a file URI, the truth now exists on disk).
    fn did_close(&self, params: DidCloseTextDocumentParams) {
        let _ = params;
        eprintln!("Got a textDocument/didClose notification, but it is not implemented");
    }

    // Language Features

    /// The [`textDocument/declaration`] request asks the server for the declaration location of a
    /// symbol at a given text document position.
    ///
    /// [`textDocument/declaration`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_declaration
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.14.0.
    ///
    /// The [`GotoDeclarationResponse::Link`](lsp_types::GotoDefinitionResponse::Link) return value
    /// was introduced in specification version 3.14.0 and requires client-side support in order to
    /// be used. It can be returned if the client set the following field to `true` in the
    /// [`initialize`](Self::initialize) method:
    ///
    /// ```text
    /// InitializeParams::capabilities::text_document::declaration::link_support
    /// ```
    fn goto_declaration(
        &self,
        params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/declaration request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/definition`] request asks the server for the definition location of a
    /// symbol at a given text document position.
    ///
    /// [`textDocument/definition`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_definition
    ///
    /// # Compatibility
    ///
    /// The [`GotoDefinitionResponse::Link`](lsp_types::GotoDefinitionResponse::Link) return value
    /// was introduced in specification version 3.14.0 and requires client-side support in order to
    /// be used. It can be returned if the client set the following field to `true` in the
    /// [`initialize`](Self::initialize) method:
    ///
    /// ```text
    /// InitializeParams::capabilities::text_document::definition::link_support
    /// ```
    fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/definition request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/typeDefinition`] request asks the server for the type definition location of
    /// a symbol at a given text document position.
    ///
    /// [`textDocument/typeDefinition`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_typeDefinition
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.6.0.
    ///
    /// The [`GotoTypeDefinitionResponse::Link`](lsp_types::GotoDefinitionResponse::Link) return
    /// value was introduced in specification version 3.14.0 and requires client-side support in
    /// order to be used. It can be returned if the client set the following field to `true` in the
    /// [`initialize`](Self::initialize) method:
    ///
    /// ```text
    /// InitializeParams::capabilities::text_document::type_definition::link_support
    /// ```
    fn goto_type_definition(
        &self,
        params: GotoTypeDefinitionParams,
    ) -> Result<Option<GotoTypeDefinitionResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/typeDefinition request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/implementation`] request is sent from the client to the server to resolve
    /// the implementation location of a symbol at a given text document position.
    ///
    /// [`textDocument/implementation`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_implementation
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.6.0.
    ///
    /// The [`GotoImplementationResponse::Link`](lsp_types::GotoDefinitionResponse::Link)
    /// return value was introduced in specification version 3.14.0 and requires client-side
    /// support in order to be used. It can be returned if the client set the following field to
    /// `true` in the [`initialize`](Self::initialize) method:
    ///
    /// ```text
    /// InitializeParams::capabilities::text_document::implementation::link_support
    /// ```
    fn goto_implementation(
        &self,
        params: GotoImplementationParams,
    ) -> Result<Option<GotoImplementationResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/implementation request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/references`] request is sent from the client to the server to resolve
    /// project-wide references for the symbol denoted by the given text document position.
    ///
    /// [`textDocument/references`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_references

    fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let _ = params;
        eprintln!("Got a textDocument/references request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/prepareCallHierarchy`] request is sent from the client to the server to
    /// return a call hierarchy for the language element of given text document positions.
    ///
    /// [`textDocument/prepareCallHierarchy`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_prepareCallHierarchy
    ///
    /// The call hierarchy requests are executed in two steps:
    ///
    /// 1. First, a call hierarchy item is resolved for the given text document position (this
    ///    method).
    /// 2. For a call hierarchy item, the incoming or outgoing call hierarchy items are resolved
    ///    inside [`incoming_calls`] and [`outgoing_calls`], respectively.
    ///
    /// [`incoming_calls`]: Self::incoming_calls
    /// [`outgoing_calls`]: Self::outgoing_calls
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.
    fn prepare_call_hierarchy(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Result<Option<Vec<CallHierarchyItem>>> {
        let _ = params;
        eprintln!("Got a textDocument/prepareCallHierarchy request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`callHierarchy/incomingCalls`] request is sent from the client to the server to
    /// resolve **incoming** calls for a given call hierarchy item.
    ///
    /// The request doesn't define its own client and server capabilities. It is only issued if a
    /// server registers for the [`textDocument/prepareCallHierarchy`] request.
    ///
    /// [`callHierarchy/incomingCalls`]: https://microsoft.github.io/language-server-protocol/specification#callHierarchy_incomingCalls
    /// [`textDocument/prepareCallHierarchy`]: Self::prepare_call_hierarchy
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.
    fn incoming_calls(
        &self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        let _ = params;
        eprintln!("Got a callHierarchy/incomingCalls request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`callHierarchy/outgoingCalls`] request is sent from the client to the server to
    /// resolve **outgoing** calls for a given call hierarchy item.
    ///
    /// The request doesn't define its own client and server capabilities. It is only issued if a
    /// server registers for the [`textDocument/prepareCallHierarchy`] request.
    ///
    /// [`callHierarchy/outgoingCalls`]: https://microsoft.github.io/language-server-protocol/specification#callHierarchy_outgoingCalls
    /// [`textDocument/prepareCallHierarchy`]: Self::prepare_call_hierarchy
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.
    fn outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        let _ = params;
        eprintln!("Got a callHierarchy/outgoingCalls request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/prepareTypeHierarchy`] request is sent from the client to the server to
    /// return a type hierarchy for the language element of given text document positions.
    ///
    /// [`textDocument/prepareTypeHierarchy`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_prepareTypeHierarchy
    ///
    /// Returns `Ok(None)` if the server couldn’t infer a valid type from the position.
    ///
    /// The type hierarchy requests are executed in two steps:
    ///
    /// 1. First, a type hierarchy item is prepared for the given text document position.
    /// 2. For a type hierarchy item, the supertype or subtype type hierarchy items are resolved in
    ///    [`supertypes`](Self::supertypes) and [`subtypes`](Self::subtypes), respectively.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.
    fn prepare_type_hierarchy(
        &self,
        params: TypeHierarchyPrepareParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let _ = params;
        eprintln!("Got a textDocument/prepareTypeHierarchy request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`typeHierarchy/supertypes`] request is sent from the client to the server to resolve
    /// the **supertypes** for a given type hierarchy item.
    ///
    /// Returns `Ok(None)` if the server couldn’t infer a valid type from item in `params`.
    ///
    /// The request doesn’t define its own client and server capabilities. It is only issued if a
    /// server registers for the `textDocument/prepareTypeHierarchy` request.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.
    fn supertypes(
        &self,
        params: TypeHierarchySupertypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let _ = params;
        eprintln!("Got a typeHierarchy/supertypes request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`typeHierarchy/subtypes`] request is sent from the client to the server to resolve
    /// the **subtypes** for a given type hierarchy item.
    ///
    /// Returns `Ok(None)` if the server couldn’t infer a valid type from item in `params`.
    ///
    /// The request doesn’t define its own client and server capabilities. It is only issued if a
    /// server registers for the `textDocument/prepareTypeHierarchy` request.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.
    fn subtypes(
        &self,
        params: TypeHierarchySubtypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let _ = params;
        eprintln!("Got a typeHierarchy/subtypes request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/documentHighlight`] request is sent from the client to the server to
    /// resolve appropriate highlights for a given text document position.
    ///
    /// [`textDocument/documentHighlight`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_documentHighlight
    ///
    /// For programming languages, this usually highlights all textual references to the symbol
    /// scoped to this file.
    ///
    /// This request differs slightly from `textDocument/references` in that this one is allowed to
    /// be more fuzzy.
    fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        let _ = params;
        eprintln!("Got a textDocument/documentHighlight request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/documentLink`] request is sent from the client to the server to request
    /// the location of links in a document.
    ///
    /// A document link is a range in a text document that links to an internal or external
    /// resource, like another text document or a web site.
    ///
    /// [`textDocument/documentLink`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_documentLink
    ///
    /// # Compatibility
    ///
    /// The [`DocumentLink::tooltip`] field was introduced in specification version 3.15.0 and
    /// requires client-side support in order to be used. It can be returned if the client set the
    /// following field to `true` in the [`initialize`](Self::initialize) method:
    ///
    /// ```text
    /// InitializeParams::capabilities::text_document::document_link::tooltip_support
    /// ```
    fn document_link(&self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        let _ = params;
        eprintln!("Got a textDocument/documentLink request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`documentLink/resolve`] request is sent from the client to the server to resolve the
    /// target of a given document link.
    ///
    /// [`documentLink/resolve`]: https://microsoft.github.io/language-server-protocol/specification#documentLink_resolve
    ///
    /// A document link is a range in a text document that links to an internal or external
    /// resource, like another text document or a web site.
    fn document_link_resolve(&self, params: DocumentLink) -> Result<DocumentLink> {
        let _ = params;
        eprintln!("Got a documentLink/resolve request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/hover`] request asks the server for hover information at a given text
    /// document position.
    ///
    /// [`textDocument/hover`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_hover
    ///
    /// Such hover information typically includes type signature information and inline
    /// documentation for the symbol at the given text document position.
    fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let _ = params;
        eprintln!("Got a textDocument/hover request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/codeLens`] request is sent from the client to the server to compute code
    /// lenses for a given text document.
    ///
    /// [`textDocument/codeLens`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_codeLens

    fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let _ = params;
        eprintln!("Got a textDocument/codeLens request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`codeLens/resolve`] request is sent from the client to the server to resolve the
    /// command for a given code lens item.
    ///
    /// [`codeLens/resolve`]: https://microsoft.github.io/language-server-protocol/specification#codeLens_resolve

    fn code_lens_resolve(&self, params: CodeLens) -> Result<CodeLens> {
        let _ = params;
        eprintln!("Got a codeLens/resolve request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/foldingRange`] request is sent from the client to the server to return
    /// all folding ranges found in a given text document.
    ///
    /// [`textDocument/foldingRange`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_foldingRange
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.10.0.

    fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let _ = params;
        eprintln!("Got a textDocument/foldingRange request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/selectionRange`] request is sent from the client to the server to return
    /// suggested selection ranges at an array of given positions. A selection range is a range
    /// around the cursor position which the user might be interested in selecting.
    ///
    /// [`textDocument/selectionRange`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_selectionRange
    ///
    /// A selection range in the return array is for the position in the provided parameters at the
    /// same index. Therefore `params.positions[i]` must be contained in `result[i].range`.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.15.0.

    fn selection_range(&self, params: SelectionRangeParams) -> Result<Option<Vec<SelectionRange>>> {
        let _ = params;
        eprintln!("Got a textDocument/selectionRange request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/documentSymbol`] request is sent from the client to the server to
    /// retrieve all symbols found in a given text document.
    ///
    /// [`textDocument/documentSymbol`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_documentSymbol
    ///
    /// The returned result is either:
    ///
    /// * [`DocumentSymbolResponse::Flat`] which is a flat list of all symbols found in a given
    ///   text document. Then neither the symbol’s location range nor the symbol’s container name
    ///   should be used to infer a hierarchy.
    /// * [`DocumentSymbolResponse::Nested`] which is a hierarchy of symbols found in a given text
    ///   document.

    fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/documentSymbol request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/semanticTokens/full`] request is sent from the client to the server to
    /// resolve the semantic tokens of a given file.
    ///
    /// [`textDocument/semanticTokens/full`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_semanticTokens
    ///
    /// Semantic tokens are used to add additional color information to a file that depends on
    /// language specific symbol information. A semantic token request usually produces a large
    /// result. The protocol therefore supports encoding tokens with numbers. In addition, optional
    /// support for deltas is available, i.e. [`semantic_tokens_full_delta`].
    ///
    /// [`semantic_tokens_full_delta`]: Self::semantic_tokens_full_delta
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let _ = params;
        eprintln!("Got a textDocument/semanticTokens/full request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/semanticTokens/full/delta`] request is sent from the client to the server to
    /// resolve the semantic tokens of a given file, **returning only the delta**.
    ///
    /// [`textDocument/semanticTokens/full/delta`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_semanticTokens
    ///
    /// Similar to [`semantic_tokens_full`](Self::semantic_tokens_full), except it returns a
    /// sequence of [`SemanticTokensEdit`] to transform a previous result into a new result.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn semantic_tokens_full_delta(
        &self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        let _ = params;
        eprintln!(
            "Got a textDocument/semanticTokens/full/delta request, but it is not implemented"
        );
        Err(Error::method_not_found())
    }

    /// The [`textDocument/semanticTokens/range`] request is sent from the client to the server to
    /// resolve the semantic tokens **for the visible range** of a given file.
    ///
    /// [`textDocument/semanticTokens/range`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_semanticTokens
    ///
    /// When a user opens a file, it can be beneficial to only compute the semantic tokens for the
    /// visible range (faster rendering of the tokens in the user interface). If a server can
    /// compute these tokens faster than for the whole file, it can implement this method to handle
    /// this special case.
    ///
    /// See the [`semantic_tokens_full`](Self::semantic_tokens_full) documentation for more
    /// details.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let _ = params;
        eprintln!("Got a textDocument/semanticTokens/range request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/inlineValue`] request is sent from the client to the server to compute
    /// inline values for a given text document that may be rendered in the editor at the end of
    /// lines.
    ///
    /// [`textDocument/inlineValue`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_inlineValue
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.

    fn inline_value(&self, params: InlineValueParams) -> Result<Option<Vec<InlineValue>>> {
        let _ = params;
        eprintln!("Got a textDocument/inlineValue request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/inlayHint`] request is sent from the client to the server to compute
    /// inlay hints for a given `(text document, range)` tuple that may be rendered in the editor
    /// in place with other text.
    ///
    /// [`textDocument/inlayHint`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_inlayHint
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0

    fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let _ = params;
        eprintln!("Got a textDocument/inlayHint request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`inlayHint/resolve`] request is sent from the client to the server to resolve
    /// additional information for a given inlay hint.
    ///
    /// [`inlayHint/resolve`]: https://microsoft.github.io/language-server-protocol/specification#inlayHint_resolve
    ///
    /// This is usually used to compute the tooltip, location or command properties of an inlay
    /// hint’s label part to avoid its unnecessary computation during the `textDocument/inlayHint`
    /// request.
    ///
    /// Consider a client announces the `label.location` property as a property that can be
    /// resolved lazily using the client capability:
    ///
    /// ```js
    /// textDocument.inlayHint.resolveSupport = { properties: ['label.location'] };
    /// ```
    ///
    /// then an inlay hint with a label part, but without a location, must be resolved using the
    /// `inlayHint/resolve` request before it can be used.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0

    fn inlay_hint_resolve(&self, params: InlayHint) -> Result<InlayHint> {
        let _ = params;
        eprintln!("Got a inlayHint/resolve request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/moniker`] request is sent from the client to the server to get the
    /// symbol monikers for a given text document position.
    ///
    /// [`textDocument/moniker`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_moniker
    ///
    /// An array of `Moniker` types is returned as response to indicate possible monikers at the
    /// given location. If no monikers can be calculated, `Some(vec![])` or `None` should be
    /// returned.
    ///
    /// # Concept
    ///
    /// The Language Server Index Format (LSIF) introduced the concept of _symbol monikers_ to help
    /// associate symbols across different indexes. This request adds capability for LSP server
    /// implementations to provide the same symbol moniker information given a text document
    /// position.
    ///
    /// Clients can utilize this method to get the moniker at the current location in a file the
    /// user is editing and do further code navigation queries in other services that rely on LSIF
    /// indexes and link symbols together.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn moniker(&self, params: MonikerParams) -> Result<Option<Vec<Moniker>>> {
        let _ = params;
        eprintln!("Got a textDocument/moniker request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/completion`] request is sent from the client to the server to compute
    /// completion items at a given cursor position.
    ///
    /// If computing full completion items is expensive, servers can additionally provide a handler
    /// for the completion item resolve request (`completionItem/resolve`). This request is sent
    /// when a completion item is selected in the user interface.
    ///
    /// [`textDocument/completion`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_completion
    ///
    /// # Compatibility
    ///
    /// Since 3.16.0, the client can signal that it can resolve more properties lazily. This is
    /// done using the `completion_item.resolve_support` client capability which lists all
    /// properties that can be filled in during a `completionItem/resolve` request.
    ///
    /// All other properties (usually `sort_text`, `filter_text`, `insert_text`, and `text_edit`)
    /// must be provided in the `textDocument/completion` response and must not be changed during
    /// resolve.

    fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/completion request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`completionItem/resolve`] request is sent from the client to the server to resolve
    /// additional information for a given completion item.
    ///
    /// [`completionItem/resolve`]: https://microsoft.github.io/language-server-protocol/specification#completionItem_resolve

    fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
        let _ = params;
        eprintln!("Got a completionItem/resolve request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/diagnostic`] request is sent from the client to the server to ask the
    /// server to compute the diagnostics for a given document.
    ///
    /// As with other pull requests, the server is asked to compute the diagnostics for the
    /// currently synced version of the document.
    ///
    /// The request doesn't define its own client and server capabilities. It is only issued if a
    /// server registers for the [`textDocument/diagnostic`] request.
    ///
    /// [`textDocument/diagnostic`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_diagnostic
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.

    fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let _ = params;
        eprintln!("Got a textDocument/diagnostic request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`workspace/diagnostic`] request is sent from the client to the server to ask the
    /// server to compute workspace wide diagnostics which previously where pushed from the server
    /// to the client.
    ///
    /// In contrast to the [`textDocument/diagnostic`] request, the workspace request can be
    /// long-running and is not bound to a specific workspace or document state. If the client
    /// supports streaming for the workspace diagnostic pull, it is legal to provide a
    /// `textDocument/diagnostic` report multiple times for the same document URI. The last one
    /// reported will win over previous reports.
    ///
    /// [`textDocument/diagnostic`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_diagnostic
    ///
    /// If a client receives a diagnostic report for a document in a workspace diagnostic request
    /// for which the client also issues individual document diagnostic pull requests, the client
    /// needs to decide which diagnostics win and should be presented. In general:
    ///
    /// * Diagnostics for a higher document version should win over those from a lower document
    ///   version (e.g. note that document versions are steadily increasing).
    /// * Diagnostics from a document pull should win over diagnostics from a workspace pull.
    ///
    /// The request doesn't define its own client and server capabilities. It is only issued if a
    /// server registers for the [`workspace/diagnostic`] request.
    ///
    /// [`workspace/diagnostic`]: https://microsoft.github.io/language-server-protocol/specification#workspace_diagnostic
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.

    fn workspace_diagnostic(
        &self,
        params: WorkspaceDiagnosticParams,
    ) -> Result<WorkspaceDiagnosticReportResult> {
        let _ = params;
        eprintln!("Got a workspace/diagnostic request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/signatureHelp`] request is sent from the client to the server to request
    /// signature information at a given cursor position.
    ///
    /// [`textDocument/signatureHelp`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_signatureHelp

    fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let _ = params;
        eprintln!("Got a textDocument/signatureHelp request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/codeAction`] request is sent from the client to the server to compute
    /// commands for a given text document and range. These commands are typically code fixes to
    /// either fix problems or to beautify/refactor code.
    ///
    /// The result of a [`textDocument/codeAction`] request is an array of `Command` literals which
    /// are typically presented in the user interface.
    ///
    /// [`textDocument/codeAction`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_codeAction
    ///
    /// To ensure that a server is useful in many clients, the commands specified in a code actions
    /// should be handled by the server and not by the client (see [`workspace/executeCommand`] and
    /// `ServerCapabilities::execute_command_provider`). If the client supports providing edits
    /// with a code action, then the mode should be used.
    ///
    /// When the command is selected the server should be contacted again (via the
    /// [`workspace/executeCommand`] request) to execute the command.
    ///
    /// [`workspace/executeCommand`]: https://microsoft.github.io/language-server-protocol/specification#workspace_executeCommand
    ///
    /// # Compatibility
    ///
    /// ## Since version 3.16.0
    ///
    /// A client can offer a server to delay the computation of code action properties during a
    /// `textDocument/codeAction` request. This is useful for cases where it is expensive to
    /// compute the value of a property (for example, the `edit` property).
    ///
    /// Clients signal this through the `code_action.resolve_support` client capability which lists
    /// all properties a client can resolve lazily. The server capability
    /// `code_action_provider.resolve_provider` signals that a server will offer a
    /// `codeAction/resolve` route.
    ///
    /// To help servers uniquely identify a code action in the resolve request, a code action
    /// literal may optionally carry a `data` property. This is also guarded by an additional
    /// client capability `code_action.data_support`. In general, a client should offer data
    /// support if it offers resolve support.
    ///
    /// It should also be noted that servers shouldn’t alter existing attributes of a code action
    /// in a `codeAction/resolve` request.
    ///
    /// ## Since version 3.8.0
    ///
    /// Support for [`CodeAction`] literals to enable the following scenarios:
    ///
    /// * The ability to directly return a workspace edit from the code action request.
    ///   This avoids having another server roundtrip to execute an actual code action.
    ///   However server providers should be aware that if the code action is expensive to compute
    ///   or the edits are huge it might still be beneficial if the result is simply a command and
    ///   the actual edit is only computed when needed.
    ///
    /// * The ability to group code actions using a kind. Clients are allowed to ignore that
    ///   information. However it allows them to better group code action, for example, into
    ///   corresponding menus (e.g. all refactor code actions into a refactor menu).

    fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/codeAction request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`codeAction/resolve`] request is sent from the client to the server to resolve
    /// additional information for a given code action.
    ///
    /// [`codeAction/resolve`]: https://microsoft.github.io/language-server-protocol/specification#codeAction_resolve
    ///
    /// This is usually used to compute the edit property of a [`CodeAction`] to avoid its
    /// unnecessary computation during the [`textDocument/codeAction`](Self::code_action) request.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn code_action_resolve(&self, params: CodeAction) -> Result<CodeAction> {
        let _ = params;
        eprintln!("Got a codeAction/resolve request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/documentColor`] request is sent from the client to the server to list
    /// all color references found in a given text document. Along with the range, a color value in
    /// RGB is returned.
    ///
    /// [`textDocument/documentColor`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_documentColor
    ///
    /// Clients can use the result to decorate color references in an editor. For example:
    ///
    /// * Color boxes showing the actual color next to the reference
    /// * Show a color picker when a color reference is edited
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.6.0.

    fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        let _ = params;
        eprintln!("Got a textDocument/documentColor request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/colorPresentation`] request is sent from the client to the server to
    /// obtain a list of presentations for a color value at a given location.
    ///
    /// [`textDocument/colorPresentation`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_colorPresentation
    ///
    /// Clients can use the result to:
    ///
    /// * Modify a color reference
    /// * Show in a color picker and let users pick one of the presentations
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.6.0.
    ///
    /// This request has no special capabilities and registration options since it is sent as a
    /// resolve request for the [`textDocument/documentColor`](Self::document_color) request.

    fn color_presentation(
        &self,
        params: ColorPresentationParams,
    ) -> Result<Vec<ColorPresentation>> {
        let _ = params;
        eprintln!("Got a textDocument/colorPresentation request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/formatting`] request is sent from the client to the server to format a
    /// whole document.
    ///
    /// [`textDocument/formatting`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_formatting

    fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let _ = params;
        eprintln!("Got a textDocument/formatting request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/rangeFormatting`] request is sent from the client to the server to
    /// format a given range in a document.
    ///
    /// [`textDocument/rangeFormatting`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_rangeFormatting

    fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let _ = params;
        eprintln!("Got a textDocument/rangeFormatting request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/onTypeFormatting`] request is sent from the client to the server to
    /// format parts of the document during typing.
    ///
    /// [`textDocument/onTypeFormatting`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_onTypeFormatting

    fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let _ = params;
        eprintln!("Got a textDocument/onTypeFormatting request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/rename`] request is sent from the client to the server to ask the server
    /// to compute a workspace change so that the client can perform a workspace-wide rename of a
    /// symbol.
    ///
    /// [`textDocument/rename`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_rename

    fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let _ = params;
        eprintln!("Got a textDocument/rename request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/prepareRename`] request is sent from the client to the server to setup
    /// and test the validity of a rename operation at a given location.
    ///
    /// [`textDocument/prepareRename`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_prepareRename
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.12.0.

    fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let _ = params;
        eprintln!("Got a textDocument/prepareRename request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`textDocument/linkedEditingRange`] request is sent from the client to the server to
    /// return for a given position in a document the range of the symbol at the position and all
    /// ranges that have the same content.
    ///
    /// [`textDocument/linkedEditingRange`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_linkedEditingRange
    ///
    /// Optionally a word pattern can be returned to describe valid contents.
    ///
    /// A rename to one of the ranges can be applied to all other ranges if the new content is
    /// valid. If no result-specific word pattern is provided, the word pattern from the client's
    /// language configuration is used.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn linked_editing_range(
        &self,
        params: LinkedEditingRangeParams,
    ) -> Result<Option<LinkedEditingRanges>> {
        let _ = params;
        eprintln!("Got a textDocument/linkedEditingRange request, but it is not implemented");
        Err(Error::method_not_found())
    }

    // Workspace Features

    /// The [`workspace/symbol`] request is sent from the client to the server to list project-wide
    /// symbols matching the given query string.
    ///
    /// [`workspace/symbol`]: https://microsoft.github.io/language-server-protocol/specification#workspace_symbol
    ///
    /// # Compatibility
    ///
    /// Since 3.17.0, servers can also provider a handler for [`workspaceSymbol/resolve`] requests.
    /// This allows servers to return workspace symbols without a range for a `workspace/symbol`
    /// request. Clients then need to resolve the range when necessary using the
    /// `workspaceSymbol/resolve` request.
    ///
    /// [`workspaceSymbol/resolve`]: Self::symbol_resolve
    ///
    /// Servers can only use this new model if clients advertise support for it via the
    /// `workspace.symbol.resolve_support` capability.

    fn symbol(&self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        let _ = params;
        eprintln!("Got a workspace/symbol request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`workspaceSymbol/resolve`] request is sent from the client to the server to resolve
    /// additional information for a given workspace symbol.
    ///
    /// [`workspaceSymbol/resolve`]: https://microsoft.github.io/language-server-protocol/specification#workspace_symbolResolve
    ///
    /// See the [`symbol`](Self::symbol) documentation for more details.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.17.0.

    fn symbol_resolve(&self, params: WorkspaceSymbol) -> Result<WorkspaceSymbol> {
        let _ = params;
        eprintln!("Got a workspaceSymbol/resolve request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`workspace/didChangeConfiguration`] notification is sent from the client to the server
    /// to signal the change of configuration settings.
    ///
    /// [`workspace/didChangeConfiguration`]: https://microsoft.github.io/language-server-protocol/specification#workspace_didChangeConfiguration

    fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        let _ = params;
        eprintln!("Got a workspace/didChangeConfiguration notification, but it is not implemented");
    }

    /// The [`workspace/didChangeWorkspaceFolders`] notification is sent from the client to the
    /// server to inform about workspace folder configuration changes.
    ///
    /// [`workspace/didChangeWorkspaceFolders`]: https://microsoft.github.io/language-server-protocol/specification#workspace_didChangeWorkspaceFolders
    ///
    /// The notification is sent by default if both of these boolean fields were set to `true` in
    /// the [`initialize`](Self::initialize) method:
    ///
    /// * `InitializeParams::capabilities::workspace::workspace_folders`
    /// * `InitializeResult::capabilities::workspace::workspace_folders::supported`
    ///
    /// This notification is also sent if the server has registered itself to receive this
    /// notification.

    fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        let _ = params;
        eprintln!(
            "Got a workspace/didChangeWorkspaceFolders notification, but it is not implemented"
        );
    }

    /// The [`workspace/willCreateFiles`] request is sent from the client to the server before
    /// files are actually created as long as the creation is triggered from within the client.
    ///
    /// [`workspace/willCreateFiles`]: https://microsoft.github.io/language-server-protocol/specification#workspace_willCreateFiles
    ///
    /// The request can return a [`WorkspaceEdit`] which will be applied to workspace before the
    /// files are created. Please note that clients might drop results if computing the edit took
    /// too long or if a server constantly fails on this request. This is done to keep creates fast
    /// and reliable.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn will_create_files(&self, params: CreateFilesParams) -> Result<Option<WorkspaceEdit>> {
        let _ = params;
        eprintln!("Got a workspace/willCreateFiles request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`workspace/didCreateFiles`] request is sent from the client to the server when files
    /// were created from within the client.
    ///
    /// [`workspace/didCreateFiles`]: https://microsoft.github.io/language-server-protocol/specification#workspace_didCreateFiles

    fn did_create_files(&self, params: CreateFilesParams) {
        let _ = params;
        eprintln!("Got a workspace/didCreateFiles notification, but it is not implemented");
    }

    /// The [`workspace/willRenameFiles`] request is sent from the client to the server before
    /// files are actually renamed as long as the rename is triggered from within the client.
    ///
    /// [`workspace/willRenameFiles`]: https://microsoft.github.io/language-server-protocol/specification#workspace_willRenameFiles
    ///
    /// The request can return a [`WorkspaceEdit`] which will be applied to workspace before the
    /// files are renamed. Please note that clients might drop results if computing the edit took
    /// too long or if a server constantly fails on this request. This is done to keep creates fast
    /// and reliable.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn will_rename_files(&self, params: RenameFilesParams) -> Result<Option<WorkspaceEdit>> {
        let _ = params;
        eprintln!("Got a workspace/willRenameFiles request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`workspace/didRenameFiles`] notification is sent from the client to the server when
    /// files were renamed from within the client.
    ///
    /// [`workspace/didRenameFiles`]: https://microsoft.github.io/language-server-protocol/specification#workspace_didRenameFiles

    fn did_rename_files(&self, params: RenameFilesParams) {
        let _ = params;
        eprintln!("Got a workspace/didRenameFiles notification, but it is not implemented");
    }

    /// The [`workspace/willDeleteFiles`] request is sent from the client to the server before
    /// files are actually deleted as long as the deletion is triggered from within the client
    /// either by a user action or by applying a workspace edit.
    ///
    /// [`workspace/willDeleteFiles`]: https://microsoft.github.io/language-server-protocol/specification#workspace_willDeleteFiles
    ///
    /// The request can return a [`WorkspaceEdit`] which will be applied to workspace before the
    /// files are deleted. Please note that clients might drop results if computing the edit took
    /// too long or if a server constantly fails on this request. This is done to keep deletions
    /// fast and reliable.
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.16.0.

    fn will_delete_files(&self, params: DeleteFilesParams) -> Result<Option<WorkspaceEdit>> {
        let _ = params;
        eprintln!("Got a workspace/willDeleteFiles request, but it is not implemented");
        Err(Error::method_not_found())
    }

    /// The [`workspace/didDeleteFiles`] notification is sent from the client to the server when
    /// files were deleted from within the client.
    ///
    /// [`workspace/didDeleteFiles`]: https://microsoft.github.io/language-server-protocol/specification#workspace_didDeleteFiles

    fn did_delete_files(&self, params: DeleteFilesParams) {
        let _ = params;
        eprintln!("Got a workspace/didDeleteFiles notification, but it is not implemented");
    }

    /// The [`workspace/didChangeWatchedFiles`] notification is sent from the client to the server
    /// when the client detects changes to files watched by the language client.
    ///
    /// [`workspace/didChangeWatchedFiles`]: https://microsoft.github.io/language-server-protocol/specification#workspace_didChangeWatchedFiles
    ///
    /// It is recommended that servers register for these file events using the registration
    /// mechanism. This can be done here or in the [`initialized`](Self::initialized) method using
    /// [`Client::register_capability`](crate::Client::register_capability).

    fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        let _ = params;
        eprintln!("Got a workspace/didChangeWatchedFiles notification, but it is not implemented");
    }

    /// The [`workspace/executeCommand`] request is sent from the client to the server to trigger
    /// command execution on the server.
    ///
    /// [`workspace/executeCommand`]: https://microsoft.github.io/language-server-protocol/specification#workspace_executeCommand
    ///
    /// In most cases, the server creates a [`WorkspaceEdit`] structure and applies the changes to
    /// the workspace using `Client::apply_edit()` before returning from this function.

    fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        let _ = params;
        eprintln!("Got a workspace/executeCommand request, but it is not implemented");
        Err(Error::method_not_found())
    }

    fn on_response(&self, id: RequestId, response: Option<serde_json::Value>) {
        let _ = id;
        let _ = response;
        eprintln!("Got a response, but it is not implemented");
    }

    // TODO: Add `work_done_progress_cancel()` here (since 3.15.0) when supported by `tower-lsp`.
    // https://github.com/ebkalderon/tower-lsp/issues/176
}
