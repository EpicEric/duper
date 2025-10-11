mod ast;
mod builder;
mod format;
mod parser;
mod serializer;
mod types;
mod visitor;

pub use ast::{DuperInner, DuperValue};
pub use parser::DuperParser;
pub use serializer::DuperSerializer;
pub use types::DuperTypes;
pub use visitor::DuperVisitor;

#[cfg(test)]
mod tests {
    use crate::{ast::DuperValue, parser::DuperParser, serializer::DuperSerializer};

    #[test]
    fn it_works() {
        let duper: DuperValue = DuperParser::parse_duper(
        r##"{"hello ": Array([" world", Null(null), true, b"bytes", 42, {inner: 0x12, "\uD83D\uDE00": 1.32e+20}]),r#"raw
        string"#: br#"a\b"#,}"##,
    )
    .unwrap();
        println!("{:?}", duper);
        println!("{}", DuperSerializer::new().serialize(duper));
    }
}
