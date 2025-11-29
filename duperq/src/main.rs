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
            let (sender, receiver) = smol::channel::bounded(1024);
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

    tasks.push(executor.spawn(async move {
        if let Some(glob) = glob {
            // Read from files
            for entry in glob {
                match entry {
                    Ok(path) => match smol::fs::read_to_string(&path).await {
                        Ok(input) => match DuperParser::parse_duper_trunk(&input) {
                            Ok(trunk) => sink.process(trunk.static_clone()).await,
                            Err(errors) => {
                                if !cli.disable_stderr {
                                    if let Ok(parse_error) = DuperParser::prettify_error(
                                        &input,
                                        &errors,
                                        Some(path.to_string_lossy().as_ref()),
                                    ) {
                                        let _ = stderr.write_all(parse_error.as_bytes()).await;
                                    }
                                }
                            }
                        },
                        Err(error) => {
                            if !cli.disable_stderr {
                                let _ = stderr.write_all(error.to_string().as_bytes()).await;
                            }
                        }
                    },
                    Err(error) => {
                        if !cli.disable_stderr {
                            let _ = stderr.write_all(error.to_string().as_bytes()).await;
                        }
                    }
                }
            }
        } else {
            // Read from stdin
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
        }
    }));

    smol::block_on(executor.run(async move { futures::future::join_all(tasks).await }));

    Ok(())
}
