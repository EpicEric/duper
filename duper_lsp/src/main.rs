use std::ops::ControlFlow;
use std::{collections::HashMap, mem};

use async_lsp::{
    ClientSocket, LanguageClient, client_monitor::ClientProcessMonitorLayer,
    concurrency::ConcurrencyLayer, panic::CatchUnwindLayer, router::Router, server::LifecycleLayer,
    tracing::TracingLayer,
};
use clap::Parser as _;
use duperfmt::format_duper;
use line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
use lsp_types::{
    FileOperationFilter, FileOperationPattern, FileOperationRegistrationOptions, InitializeResult,
    OneOf, Position, PositionEncodingKind, PublishDiagnosticsParams, Range, ServerCapabilities,
    ServerInfo, TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, TextEdit, Url, WorkspaceFileOperationsServerCapabilities,
    WorkspaceServerCapabilities, notification, request,
};
use tower::ServiceBuilder;
use tracing::{Level, debug};
use tree_sitter::{InputEdit, Point, Tree};

mod diagnostics;

use crate::diagnostics::get_diagnostics;

struct ServerState {
    client: ClientSocket,
    is_utf8: bool,
    parser: tree_sitter::Parser,
    documents: HashMap<String, (TextDocumentItem, Tree)>,
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Run in debug mode (i.e. comprehensive logging and check for formatting idempotency).
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();
    let debug = cli.debug;

    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_duper::LANGUAGE;
        parser
            .set_language(&language.into())
            .expect("Error loading Duper parser");
        let mut router = Router::new(ServerState {
            client: client.clone(),
            is_utf8: false,
            parser,
            documents: HashMap::new(),
        });
        router
            .request::<request::Initialize, _>(|state, params| {
                debug!(?params, "Initializing LSP...");
                let is_utf8 = params.capabilities.general.is_some_and(|general| {
                    general
                        .position_encodings
                        .unwrap_or_default()
                        .contains(&PositionEncodingKind::UTF8)
                });
                state.is_utf8 = is_utf8;
                async move {
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
                            // hover_provider: Some(HoverProviderCapability::Simple(true)),
                            text_document_sync: Some(TextDocumentSyncCapability::Options(
                                TextDocumentSyncOptions {
                                    change: Some(TextDocumentSyncKind::INCREMENTAL),
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
                            position_encoding: is_utf8.then_some(PositionEncodingKind::UTF8),
                            ..Default::default()
                        },
                        server_info: Some(ServerInfo {
                            name: "Duper".into(),
                            version: Some(env!("CARGO_PKG_VERSION").into()),
                        }),
                    })
                }
            })
            .request::<request::HoverRequest, _>(|_, _| async move { Ok(None) })
            .request::<request::Formatting, _>(move |state, params| {
                let uri = params.text_document.uri.as_str();
                let entry = state.documents.get(uri).cloned();
                if entry.is_none() {
                    debug!(uri, "Text document not found");
                }
                async move {
                    let Some((document, tree)) = entry else {
                        return Ok(None);
                    };
                    let input = document.text;
                    if input.trim().is_empty() {
                        return Ok(None);
                    }
                    let indent: String = if params.options.insert_spaces {
                        (0..params.options.tab_size).map(|_| ' ').collect()
                    } else {
                        "\t".into()
                    };
                    let mut buf = Vec::new();
                    if let Err(err) = format_duper(tree, &input, &mut buf, Some(indent), debug) {
                        debug!(?err, "Failed to format Duper document");
                        return Ok(None);
                    }

                    let (line_count, last_line_len) = input
                        .lines()
                        .fold((0, 0), |acc, line| (acc.0 + 1, line.len()));
                    let range = Range::new(
                        Position::new(0, 0),
                        Position::new(line_count, last_line_len as u32),
                    );
                    Ok(Some(vec![TextEdit::new(
                        range,
                        String::from_utf8(buf).expect("formatting output is valid Duper"),
                    )]))
                }
            })
            .notification::<notification::Initialized>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidChangeConfiguration>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidOpenTextDocument>(|state, params| {
                let uri = params.text_document.uri.to_string();
                let tree = state
                    .parser
                    .parse(&params.text_document.text, None)
                    .expect("parser was properly initialized");
                let diagnostics = PublishDiagnosticsParams {
                    uri: params.text_document.uri.clone(),
                    diagnostics: get_diagnostics(&params.text_document.text, &tree, state.is_utf8),
                    version: Some(params.text_document.version),
                };
                let _ = state.client.publish_diagnostics(diagnostics);
                state.documents.insert(uri, (params.text_document, tree));
                ControlFlow::Continue(())
            })
            .notification::<notification::DidChangeTextDocument>(|state, params| {
                let uri = params.text_document.uri;
                let Some((document, tree)) = state.documents.get_mut(uri.as_str()) else {
                    debug!(?uri, "Text document not found");
                    return ControlFlow::Continue(());
                };
                let mut text = mem::take(&mut document.text);
                let mut new_tree = Some(tree.clone());
                for change in params.content_changes {
                    if let Some(range) = change.range {
                        let index = LineIndex::new(&text);
                        let start_line_col = if state.is_utf8 {
                            LineCol {
                                line: range.start.line,
                                col: range.start.character,
                            }
                        } else {
                            index
                                .to_utf8(
                                    WideEncoding::Utf16,
                                    WideLineCol {
                                        line: range.start.line,
                                        col: range.start.character,
                                    },
                                )
                                .expect("integer overflow")
                        };
                        let end_line_col = if state.is_utf8 {
                            LineCol {
                                line: range.end.line,
                                col: range.end.character,
                            }
                        } else {
                            index
                                .to_utf8(
                                    WideEncoding::Utf16,
                                    WideLineCol {
                                        line: range.end.line,
                                        col: range.end.character,
                                    },
                                )
                                .expect("integer overflow")
                        };

                        let start_byte = index
                            .offset(start_line_col)
                            .expect("integer overflow")
                            .into();
                        let old_end_byte =
                            index.offset(end_line_col).expect("integer overflow").into();
                        let new_end_byte = start_byte + change.text.len();

                        if let Some(new_tree) = &mut new_tree {
                            let start_position = Point::new(
                                start_line_col.line as usize,
                                start_line_col.col as usize,
                            );
                            let old_end_position =
                                Point::new(end_line_col.line as usize, end_line_col.col as usize);
                            let (line_count, last_line_len) = change
                                .text
                                .lines()
                                .fold((0, 0), |acc, line| (acc.0 + 1, line.len()));
                            let new_end_position = if line_count > 1 {
                                Point::new(
                                    (start_line_col.line as usize) + (line_count - 1),
                                    last_line_len,
                                )
                            } else {
                                Point::new(
                                    start_line_col.line as usize,
                                    (start_line_col.col as usize) + last_line_len,
                                )
                            };

                            new_tree.edit(&InputEdit {
                                start_byte,
                                old_end_byte,
                                new_end_byte,
                                start_position,
                                old_end_position,
                                new_end_position,
                            });
                        }
                        text.replace_range(start_byte..old_end_byte, &change.text);
                    } else {
                        text = change.text;
                        new_tree = None;
                    }
                }
                *tree = state
                    .parser
                    .parse(&text, new_tree.as_ref())
                    .expect("parser was properly initialized");
                *document = TextDocumentItem::new(
                    uri.clone(),
                    "duper".to_owned(),
                    params.text_document.version,
                    text,
                );
                let _ = state.client.publish_diagnostics(PublishDiagnosticsParams {
                    uri,
                    diagnostics: get_diagnostics(&document.text, &tree, state.is_utf8),
                    version: Some(params.text_document.version),
                });
                ControlFlow::Continue(())
            })
            .notification::<notification::DidCloseTextDocument>(|state, params| {
                state.documents.remove(params.text_document.uri.as_str());
                ControlFlow::Continue(())
            })
            .notification::<notification::DidChangeWatchedFiles>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidRenameFiles>(|state, params| {
                for file in params.files {
                    let Some((document, tree)) = state.documents.remove(&file.old_uri) else {
                        debug!(uri = file.old_uri, "Text document not found");
                        continue;
                    };
                    let uri = Url::parse(&file.new_uri).expect("URI is valid");
                    let diagnostics = get_diagnostics(&document.text, &tree, state.is_utf8);
                    let version = Some(document.version);
                    state.documents.insert(file.new_uri, (document, tree));
                    let _ = state.client.publish_diagnostics(PublishDiagnosticsParams {
                        uri,
                        diagnostics,
                        version,
                    });
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
        .with_max_level(if debug { Level::DEBUG } else { Level::INFO })
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();

    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().expect("should allocate stdin"),
        async_lsp::stdio::PipeStdout::lock_tokio().expect("should allocate stdout"),
    );

    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    server
        .run_buffered(stdin, stdout)
        .await
        .expect("server was shut down");
}
