use std::{fmt::Display, path::PathBuf};

use chumsky::Parser as _;
use clap::Parser;
use duper::DuperParser;
use duperq::query;
use glob::glob;
use smol::{
    LocalExecutor, Unblock,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    stream::StreamExt,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Query to run.
    query: String,

    /// Glob of files to read from. If missing, defaults to stdin.
    glob: Option<String>,

    /// If set, disables logs about parsing errors from being printed to stderr.
    #[arg(short = 'E', long)]
    disable_stderr: bool,
}

enum FileReadError {
    Glob(glob::GlobError),
    Io(std::io::Error),
}

impl From<glob::GlobError> for FileReadError {
    fn from(value: glob::GlobError) -> Self {
        Self::Glob(value)
    }
}

impl From<std::io::Error> for FileReadError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for FileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileReadError::Glob(error) => error.fmt(f),
            FileReadError::Io(error) => error.fmt(f),
        }
    }
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
            }));
            (pipeline_fn)(sender)
        });

    let glob = if let Some(duper_glob) = cli.glob {
        Some(glob(&duper_glob)?)
    } else {
        None
    };

    if let Some(glob) = glob {
        let (pathbuf_sender, pathbuf_receiver) =
            smol::channel::bounded::<Result<PathBuf, FileReadError>>(128);
        let (file_sender, file_receiver) =
            smol::channel::bounded::<Result<(PathBuf, String), FileReadError>>(128);
        // Iterate over glob
        tasks.push(executor.spawn(async move {
            for entry in glob {
                match entry {
                    Err(_) if cli.disable_stderr => continue,
                    Ok(_) | Err(_) => {
                        if pathbuf_sender
                            .send(entry.map_err(|error| error.into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                }
            }
        }));
        // Read files
        tasks.extend((0..num_cpus::get()).map(|_| {
            let file_sender = file_sender.clone();
            let pathbuf_receiver = pathbuf_receiver.clone();
            executor.spawn(async move {
                while let Ok(msg) = pathbuf_receiver.recv().await {
                    match msg {
                        Ok(pathbuf) => {
                            let string = smol::fs::read_to_string(&pathbuf).await;
                            match string {
                                Err(_) if cli.disable_stderr => continue,
                                Ok(_) | Err(_) => {
                                    if file_sender
                                        .send(
                                            string
                                                .map(move |string| (pathbuf, string))
                                                .map_err(|error| error.into()),
                                        )
                                        .await
                                        .is_err()
                                    {
                                        break;
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            if file_sender.send(Err(error)).await.is_err() {
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
                                }
                            }
                        }
                    },
                    Err(error) => {
                        let _ = stderr.write_all(error.to_string().as_bytes()).await;
                    }
                }
            }
        }));
    } else {
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
        }));
    }

    smol::block_on(executor.run(async move { futures::future::join_all(tasks).await }));

    Ok(())
}
