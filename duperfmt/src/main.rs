use std::{
    borrow::Cow,
    env,
    fmt::Display,
    fs,
    io::{self, Read, Write},
    path::PathBuf,
    time,
};

use clap::{CommandFactory, Parser};
use duperfmt::format_duper;
use miette::{Diagnostic, IntoDiagnostic, LabeledSpan, NamedSource, miette};
use thiserror::Error;
use tree_sitter::StreamingIterator;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Indentation for pretty-printing.
    #[arg(short = 'I', long, value_name = "INDENT")]
    indent: Option<String>,

    /// The file to format, or stdin if unspecified.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// The file to save to, or stdout if unspecified.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Whether the file should be overwritten.
    #[arg(short, long)]
    in_place: bool,

    /// Run in debug mode (i.e. check for formatting idempotency).
    #[arg(short, long)]
    debug: bool,
}

#[derive(Error, Debug, Diagnostic)]
struct TreeSitterReport {
    #[source_code]
    src: NamedSource<String>,
    #[label(collection)]
    reports: Vec<LabeledSpan>,
}

impl Display for TreeSitterReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Errors found while parsing Duper.")
    }
}

fn to_labeled_span(range: tree_sitter::Range, label: Option<String>) -> LabeledSpan {
    LabeledSpan::new(label, range.start_byte, range.end_byte - range.start_byte)
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    // Validate CLI options
    let mut output = cli.output.as_ref();
    if cli.in_place {
        if cli.file.is_none() {
            let mut cmd = Cli::command();
            cmd.error(
                clap::error::ErrorKind::ArgumentConflict,
                "File to edit in-place is required",
            )
            .exit();
        } else if output.is_some() {
            let mut cmd = Cli::command();
            cmd.error(
                clap::error::ErrorKind::ArgumentConflict,
                "Cannot specify output file with in-place option",
            )
            .exit();
        }
        output = cli.file.as_ref();
    }

    // Read Duper
    let (src, buf) = if let Some(file) = cli.file.as_ref() {
        (file.to_string_lossy(), fs::read(file).into_diagnostic()?)
    } else {
        let mut buf = vec![];
        io::stdin().read_to_end(&mut buf).into_diagnostic()?;
        (Cow::Borrowed("<stdin>"), buf)
    };
    let input = String::from_utf8(buf).into_diagnostic()?;

    // Parse and check for errors
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_duper::LANGUAGE.into())
        .into_diagnostic()?;
    let tree = parser.parse(&input, None).expect("parser was initialized");
    let mut reports = vec![];
    let errors_query =
        tree_sitter::Query::new(&tree_sitter_duper::LANGUAGE.into(), "(ERROR) @error")
            .expect("valid errors query");
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut captures = cursor.captures(&errors_query, tree.root_node(), input.as_bytes());
    while let Some((m, _)) = {
        captures.advance();
        captures.get()
    } {
        reports.extend(
            m.captures
                .iter()
                .map(|c| to_labeled_span(c.node.range(), Some("Error".into()))),
        );
    }
    let missing_query =
        tree_sitter::Query::new(&tree_sitter_duper::LANGUAGE.into(), "(MISSING) @missing")
            .expect("valid missing query");
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut captures = cursor.captures(&missing_query, tree.root_node(), input.as_bytes());
    while let Some((m, _)) = {
        captures.advance();
        captures.get()
    } {
        reports.extend(
            m.captures
                .iter()
                .map(|c| to_labeled_span(c.node.range(), Some("Missing".into()))),
        );
    }
    if !reports.is_empty() {
        return Err(TreeSitterReport {
            src: NamedSource::new(src, input),
            reports,
        }
        .into());
    }

    // Format and write
    let mut buf = vec![];
    format_duper(tree, &input, &mut buf, cli.indent, cli.debug)
        .map_err(|err| miette!("Failed to format Duper value: {err}"))?;

    if let Some(output) = output {
        let tmp_file = env::temp_dir().join(format!(
            "/tmp_{}.duper",
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .into_diagnostic()?
                .as_micros()
        ));
        fs::write(&tmp_file, &buf).into_diagnostic()?;
        fs::rename(&tmp_file, output).into_diagnostic()?;
    } else {
        io::stdout().write_all(&buf).into_diagnostic()?;
    }

    Ok(())
}
