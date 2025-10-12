mod ast;
mod builder;
mod format;
mod parser;
mod serde;
pub mod types;
pub mod visitor;

pub use ast::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTuple, DuperValue,
};
pub use parser::{DuperParser, Rule as DuperRule};
pub use visitor::{pretty_printer::PrettyPrinter, serializer::Serializer};

#[cfg(test)]
mod tests {
    use crate::{ast::DuperValue, parser::DuperParser, visitor::serializer::Serializer};

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
