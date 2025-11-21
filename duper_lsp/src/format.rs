use std::io::{Read, Write};

use topiary_core::{Language, Operation, TopiaryQuery, formatter};

const DUPER_QUERY: &str = include_str!("./duper.scm");

pub(crate) fn format_duper(
    mut input: impl Read,
    mut output: impl Write,
    indent: Option<String>,
) -> Result<(), topiary_core::FormatterError> {
    let grammar = tree_sitter_duper::LANGUAGE.into();
    let query = TopiaryQuery::new(&grammar, DUPER_QUERY)?;
    let language = Language {
        name: "duper".to_owned(),
        query,
        grammar,
        indent,
    };

    formatter(
        &mut input,
        &mut output,
        &language,
        Operation::Format {
            skip_idempotence: true,
            tolerate_parsing_errors: false,
        },
    )
}
