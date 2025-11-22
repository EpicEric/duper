use std::io::Write;

use topiary_core::{Language, Operation, TopiaryQuery, formatter_tree};
use tree_sitter::Tree;

const DUPER_QUERY: &str = include_str!("./duper.scm");

pub fn format_duper(
    tree: Tree,
    input: &str,
    mut output: impl Write,
    indent: Option<String>,
    debug: bool,
) -> Result<(), topiary_core::FormatterError> {
    let language = Language {
        name: "duper".to_owned(),
        query: TopiaryQuery::new(&tree_sitter_duper::LANGUAGE.into(), DUPER_QUERY)?,
        grammar: tree_sitter_duper::LANGUAGE.into(),
        indent,
    };

    formatter_tree(
        tree.into(),
        input,
        &mut output,
        &language,
        Operation::Format {
            skip_idempotence: !debug,
            tolerate_parsing_errors: false,
        },
    )
}
