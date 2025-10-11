mod ast;
mod builder;
mod format;
pub mod parser;
mod pretty_printer;
mod serializer;
pub mod types;
pub mod visitor;

pub use ast::{DuperInner, DuperValue};
pub use pretty_printer::PrettyPrinter;
pub use serializer::Serializer;

#[cfg(test)]
mod tests {
    use crate::{ast::DuperValue, parser::DuperParser, serializer::Serializer};

    #[test]
    fn it_works() {
        let duper: DuperValue = DuperParser::parse_duper(
        r##"{"hello ": Array([" world", Null(null), true, b"bytes", 42, {inner: 0x12, "\uD83D\uDE00": 1.32e+20}]),r#"raw
        string"#: br#"a\b"#,}"##,
    )
    .unwrap();
        println!("{:?}", duper);
        println!("{}", Serializer::new().serialize(duper));
    }
}
