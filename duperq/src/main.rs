use std::path::PathBuf;

use chumsky::Parser as _;
use clap::Parser;
use duper::DuperParser;
use duperq::query;
use smol::{
    LocalExecutor, Unblock,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    stream::StreamExt,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// If set, disables logs about parsing errors from being printed to stderr.
    #[arg(short = 'E', long)]
    disable_stderr: bool,

    /// Query to run.
    query: String,

    /// Files to read from. If missing, defaults to stdin.
    #[arg(name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut stderr = Unblock::new(std::io::stderr());
    let (pipeline_fns, output) = match query().parse(&cli.query).into_result() {
        Ok(pipeline) => pipeline,
        Err(errors) => {
            return Err(anyhow::anyhow!(DuperParser::prettify_error(
                &cli.query, &errors, None
            )?));
        }
    };

    let executor = LocalExecutor::new();

    let mut tasks = Vec::with_capacity(pipeline_fns.len());
    let mut sink = pipeline_fns
        .into_iter()
        .rfold(output, |mut output, pipeline_fn| {
            let (sender, receiver) = smol::channel::bounded(128);
            tasks.push(executor.spawn(async move {
                while let Ok(value) = receiver.recv().await {
                    output.process(value).await;
                }
                output.close().await;
            }));
            (pipeline_fn)(sender)
        });

    let files = cli.files;

    if files.is_empty() {
        // Read from stdin
        tasks.push(executor.spawn(async move {
            let stdin = BufReader::new(Unblock::new(std::io::stdin()));
            let mut lines = stdin.lines();
            while let Some(Ok(line)) = lines.next().await {
                match DuperParser::parse_duper_trunk(&line) {
                    Ok(trunk) => sink.process(trunk.static_clone()).await,
                    Err(errors) => {
                        if !cli.disable_stderr {
                            if let Ok(parse_error) =
                                DuperParser::prettify_error(&line, &errors, None)
                            {
                                let _ = stderr.write_all(parse_error.as_bytes()).await;
                            }
                        }
                    }
                }
            }
            sink.close().await;
        }));
    } else {
        let (pathbuf_sender, pathbuf_receiver) = smol::channel::bounded::<PathBuf>(128);
        let (file_sender, file_receiver) =
            smol::channel::bounded::<Result<(PathBuf, String), std::io::Error>>(128);
        // Iterate over files
        tasks.push(executor.spawn(async move {
            for entry in files {
                if pathbuf_sender.send(entry).await.is_err() {
                    break;
                }
            }
        }));
        // Read files
        tasks.extend((0..num_cpus::get()).map(|_| {
            let file_sender = file_sender.clone();
            let pathbuf_receiver = pathbuf_receiver.clone();
            executor.spawn(async move {
                while let Ok(pathbuf) = pathbuf_receiver.recv().await {
                    let string = smol::fs::read_to_string(&pathbuf).await;
                    match string {
                        Err(_) if cli.disable_stderr => continue,
                        Ok(_) | Err(_) => {
                            if file_sender
                                .send(string.map(move |string| (pathbuf, string)))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                }
            })
        }));
        // Parse and process values
        tasks.push(executor.spawn(async move {
            while let Ok(input) = file_receiver.recv().await {
                match input {
                    Ok((pathbuf, string)) => match DuperParser::parse_duper_trunk(&string) {
                        Ok(trunk) => sink.process(trunk.static_clone()).await,
                        Err(errors) => {
                            if !cli.disable_stderr {
                                if let Ok(parse_error) = DuperParser::prettify_error(
                                    &string,
                                    &errors,
                                    Some(pathbuf.to_string_lossy().as_ref()),
                                ) {
                                    let _ = stderr.write_all(parse_error.as_bytes()).await;
                                    let _ = stderr.flush().await;
                                }
                            }
                        }
                    },
                    Err(error) => {
                        let _ = stderr.write_all(error.to_string().as_bytes()).await;
                        let _ = stderr.flush().await;
                    }
                }
            }
            sink.close().await;
        }));
    }

    smol::block_on(executor.run(async move { futures::future::join_all(tasks).await }));

    Ok(())
}
