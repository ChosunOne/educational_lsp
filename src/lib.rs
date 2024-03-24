use state::State;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub mod state;

#[derive(Debug)]
pub struct Backend {
    client: Client,
    state: State,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            state: Default::default(),
        }
    }
    fn capabilities(&self) -> ServerCapabilities {
        ServerCapabilities {
            text_document_sync: TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL).into(),
            hover_provider: HoverProviderCapability::Simple(true).into(),
            definition_provider: Some(OneOf::Left(true)),
            code_action_provider: CodeActionProviderCapability::Simple(true).into(),
            completion_provider: CompletionOptions::default().into(),
            ..Default::default()
        }
    }

    fn server_info(&self) -> ServerInfo {
        ServerInfo {
            name: "Educational LSP in Rust".into(),
            version: "0.1.0".to_owned().into(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let server_info = self.server_info().into();
        let capabilities = self.capabilities();

        let result = InitializeResult {
            server_info,
            capabilities,
        };

        Ok(result)
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let diagnostics = self
            .state
            .open_document(params.text_document.uri.clone(), params.text_document.text);
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        for change in params.content_changes {
            let diagnostics = self.state.update_document(uri.clone(), change.text);
            self.client
                .publish_diagnostics(uri.clone(), diagnostics, None)
                .await;
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let response = self.state.hover(
            &params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        );
        Ok(response.into())
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let response = self.state.definition(
            &params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        );
        Ok(response.into())
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let response = self.state.code_action(&params.text_document.uri);
        Ok(response.into())
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let response = self
            .state
            .completion(&params.text_document_position.text_document.uri);
        Ok(response.into())
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
