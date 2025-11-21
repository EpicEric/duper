use std::collections::HashMap;
use std::ops::ControlFlow;

use async_lsp::{
    client_monitor::ClientProcessMonitorLayer, concurrency::ConcurrencyLayer,
    panic::CatchUnwindLayer, router::Router, server::LifecycleLayer, tracing::TracingLayer,
};
use lsp_types::{
    FileOperationFilter, FileOperationPattern, FileOperationRegistrationOptions, Hover,
    HoverContents, HoverProviderCapability, InitializeResult, MarkedString, OneOf, Position, Range,
    ServerCapabilities, ServerInfo, TextDocumentItem, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextEdit,
    WorkspaceFileOperationsServerCapabilities, WorkspaceServerCapabilities, notification, request,
};
use tower::ServiceBuilder;
use tracing::{Level, debug, info};

use crate::format::format_duper;

mod format;

struct ServerState {
    // client: ClientSocket,
    documents: HashMap<String, TextDocumentItem>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        let mut router = Router::new(ServerState {
            // client: client.clone(),
            documents: HashMap::new(),
        });
        // TO-DO: Don't .unwrap() everything
        router
            .request::<request::Initialize, _>(|_, params| async move {
                debug!(?params, "Initializing LSP...");
                let filters = vec![FileOperationFilter {
                    pattern: FileOperationPattern {
                        glob: "**/*.duper".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }];
                let operation_options = Some(FileOperationRegistrationOptions { filters });
                Ok(InitializeResult {
                    capabilities: ServerCapabilities {
                        hover_provider: Some(HoverProviderCapability::Simple(true)),
                        text_document_sync: Some(TextDocumentSyncCapability::Options(
                            TextDocumentSyncOptions {
                                change: Some(TextDocumentSyncKind::FULL),
                                open_close: Some(true),
                                ..Default::default()
                            },
                        )),
                        document_formatting_provider: Some(OneOf::Left(true)),
                        workspace: Some(WorkspaceServerCapabilities {
                            file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                                did_delete: operation_options.clone(),
                                did_create: operation_options.clone(),
                                did_rename: operation_options.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    server_info: Some(ServerInfo {
                        name: "Duper".into(),
                        version: Some(env!("CARGO_PKG_VERSION").into()),
                    }),
                })
            })
            .request::<request::HoverRequest, _>(|_, _| async move {
                Ok(Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "I am a hover text!".into(),
                    )),
                    range: None,
                }))
            })
            .request::<request::Formatting, _>(|state, params| {
                let document = state
                    .documents
                    .get(params.text_document.uri.as_str())
                    .unwrap();
                let input = document.text.clone();
                async move {
                    if input.trim().is_empty() {
                        return Ok(None);
                    }
                    let indent: String = if params.options.insert_spaces {
                        (0..params.options.tab_size).map(|_| ' ').collect()
                    } else {
                        "\t".into()
                    };
                    let mut buf = Vec::new();
                    format_duper(input.as_bytes(), &mut buf, Some(indent)).unwrap();

                    let lines = input.lines();
                    let last_line = lines.clone().last().unwrap_or("");
                    let end = Position::new(lines.count() as u32, last_line.len() as u32);
                    let range = Range::new(Position::new(0, 0), end);
                    Ok(Some(vec![TextEdit::new(
                        range,
                        String::from_utf8(buf).unwrap(),
                    )]))
                }
            })
            .notification::<notification::Initialized>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidChangeConfiguration>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidOpenTextDocument>(|state, params| {
                info!(?params, "Notification DidOpenTextDocument");
                state
                    .documents
                    .insert(params.text_document.uri.to_string(), params.text_document);
                ControlFlow::Continue(())
            })
            .notification::<notification::DidChangeTextDocument>(|state, params| {
                let uri = params.text_document.uri;
                let document = state.documents.get_mut(uri.as_str()).unwrap();
                *document = TextDocumentItem::new(
                    uri,
                    "duper".to_owned(),
                    params.text_document.version,
                    params.content_changes.into_iter().last().unwrap().text,
                );
                ControlFlow::Continue(())
            })
            .notification::<notification::DidCloseTextDocument>(|state, params| {
                state.documents.remove(params.text_document.uri.as_str());
                ControlFlow::Continue(())
            })
            .notification::<notification::DidChangeWatchedFiles>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidRenameFiles>(|state, params| {
                for file in params.files {
                    let document = state.documents.remove(&file.old_uri).unwrap();
                    state.documents.insert(file.new_uri, document);
                }
                ControlFlow::Continue(())
            })
            .notification::<notification::DidDeleteFiles>(|state, params| {
                for file in params.files {
                    state.documents.remove(&file.uri);
                }
                ControlFlow::Continue(())
            });

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client.clone()))
            .service(router)
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
