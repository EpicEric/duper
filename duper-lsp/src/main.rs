use std::{collections::HashMap, ops::ControlFlow};

use async_lsp::{
    ClientSocket, LanguageServer, ResponseError,
    client_monitor::ClientProcessMonitorLayer,
    concurrency::ConcurrencyLayer,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, FileOperationFilter,
        FileOperationPattern, FileOperationPatternKind, FileOperationRegistrationOptions,
        InitializeResult, OneOf, ServerCapabilities, ServerInfo, SymbolKind, Url,
        WorkspaceFileOperationsServerCapabilities, WorkspaceFoldersServerCapabilities,
        WorkspaceServerCapabilities,
        request::{Initialize, Request},
    },
    panic::CatchUnwindLayer,
    router::Router,
    server::LifecycleLayer,
    tracing::TracingLayer,
};
use duper::{DuperParser, DuperRule};
use futures::future::BoxFuture;
use pest::{Token, iterators::Tokens};
use tower::ServiceBuilder;
use tracing::Level;

struct ServerState {
    client: ClientSocket,
    documents: HashMap<Url, String>,
}

impl LanguageServer for ServerState {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        params: <Initialize as Request>::Params,
    ) -> BoxFuture<'static, Result<InitializeResult, Self::Error>> {
        let file_operation_filers = vec![FileOperationFilter {
            scheme: Some(String::from("file")),
            pattern: FileOperationPattern {
                glob: String::from("**/*.{duper}"),
                matches: Some(FileOperationPatternKind::File),
                ..Default::default()
            },
        }];

        let file_registration_option = FileOperationRegistrationOptions {
            filters: file_operation_filers.clone(),
        };

        let workspace_capabilities =
            params
                .workspace_folders
                .map(|_| WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        ..Default::default()
                    }),

                    file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                        did_create: Some(file_registration_option.clone()),
                        did_rename: Some(file_registration_option.clone()),
                        did_delete: Some(file_registration_option.clone()),
                        ..Default::default()
                    }),
                });

        let response = InitializeResult {
            capabilities: ServerCapabilities {
                workspace: workspace_capabilities,
                document_symbol_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        };

        Box::pin(async move { Ok(response) })
    }

    fn did_change(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> ControlFlow<async_lsp::Result<()>> {
        self.documents.insert(
            params.text_document.uri,
            params.content_changes[0].text.clone(),
        );
        ControlFlow::Continue(())
    }

    fn did_save(
        &mut self,
        params: DidSaveTextDocumentParams,
    ) -> ControlFlow<async_lsp::Result<()>> {
        if let Some(text) = params.text {
            self.documents.insert(params.text_document.uri, text);
        }
        ControlFlow::Continue(())
    }

    fn did_open(
        &mut self,
        params: DidOpenTextDocumentParams,
    ) -> ControlFlow<async_lsp::Result<()>> {
        self.documents
            .insert(params.text_document.uri, params.text_document.text);
        ControlFlow::Continue(())
    }

    fn document_symbol(
        &mut self,
        params: DocumentSymbolParams,
    ) -> BoxFuture<'static, Result<Option<DocumentSymbolResponse>, Self::Error>> {
        if let Some(document) = self.documents.get(&params.text_document.uri) {
            let Ok(pairs) = DuperParser::try_parse(document) else {
                return Box::pin(async move { Ok(None) });
            };
            let symbols = parse_symbols(document, pairs.tokens())?;
            let response = DocumentSymbolResponse::Nested(symbols);
            Box::pin(async move { Ok(Some(response)) })
        } else {
            Box::pin(async move { Ok(None) })
        }
    }
}

fn parse_symbols(
    source: &str,
    tokens: &mut Tokens<'_, DuperRule>,
) -> Result<Vec<DocumentSymbol>, ResponseError> {
    let symbols = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Start { rule, pos } => match rule {
                DuperRule::COMMENT => stack.push((rule, pos)),
                DuperRule::identifier => todo!(),
                DuperRule::object => todo!(),
                DuperRule::key => todo!(),
                DuperRule::plain_key => todo!(),
                DuperRule::tuple => todo!(),
                DuperRule::array => todo!(),
                DuperRule::string => todo!(),
                DuperRule::bytes => todo!(),
                DuperRule::raw_string => todo!(),
                DuperRule::raw_bytes => todo!(),
                DuperRule::integer => todo!(),
                DuperRule::float => todo!(),
                DuperRule::boolean => todo!(),
                DuperRule::null => todo!(),
                _ => (),
            },
            Token::End { rule, pos } => match rule {
                DuperRule::COMMENT => match stack.last() {
                    Some((DuperRule::COMMENT, start)) => {
                        symbols.push(DocumentSymbol {
                            name: source.get(),
                            detail: None,
                            kind: SymbolKind::STRING,
                            tags: None,
                            deprecated: None,
                            range: todo!(),
                            selection_range: todo!(),
                            children: todo!(),
                        });
                    }
                    _ => (),
                },
                DuperRule::identifier => todo!(),
                DuperRule::object => todo!(),
                DuperRule::key => todo!(),
                DuperRule::plain_key => todo!(),
                DuperRule::tuple => todo!(),
                DuperRule::array => todo!(),
                DuperRule::string => todo!(),
                DuperRule::bytes => todo!(),
                DuperRule::raw_string => todo!(),
                DuperRule::raw_bytes => todo!(),
                DuperRule::integer => todo!(),
                DuperRule::float => todo!(),
                DuperRule::boolean => todo!(),
                DuperRule::null => todo!(),
                _ => (),
            },
        }
    }
    Ok(symbols)
}

impl ServerState {
    pub fn new_router(client: ClientSocket) -> Router<Self> {
        let router = Router::from_language_server(Self {
            client,
            documents: HashMap::new(),
        });
        router
    }
}

#[tokio::main]
async fn main() {
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client.clone()))
            .service(ServerState::new_router(client))
    });

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();

    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
        async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
    );

    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    server.run_buffered(stdin, stdout).await.unwrap();
}
