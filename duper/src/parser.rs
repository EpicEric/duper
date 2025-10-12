use pest::{Parser as _, error::Error};

use crate::{ast::DuperValue, builder::DuperBuilder};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct DuperParser;

impl DuperParser {
    pub fn parse_duper_stream(input: &'_ str) -> Result<Vec<DuperValue<'_>>, Box<Error<Rule>>> {
        let mut pairs = Self::parse(Rule::duper_stream, input)?;
        DuperBuilder::build_duper_stream(pairs.next().unwrap())
    }

    pub fn parse_duper(input: &'_ str) -> Result<DuperValue<'_>, Box<Error<Rule>>> {
        let mut pairs = Self::parse(Rule::duper, input)?;
        DuperBuilder::build_duper(pairs.next().unwrap())
    }

    pub fn parse_duper_value(input: &'_ str) -> Result<DuperValue<'_>, Box<Error<Rule>>> {
        let mut pairs = Self::parse(Rule::duper_value, input)?;
        DuperBuilder::build_duper_value(pairs.next().unwrap())
    }
}

#[cfg(test)]
mod duper_parser_tests {
    use crate::{DuperInner, DuperParser};

    #[test]
    fn duper_stream() {
        let input = r#"---
        ["foobar", true]
        ---
        {duper: 1337}
        ---
        ---"#;
        let duper_stream = DuperParser::parse_duper_stream(input).unwrap();
        assert_eq!(duper_stream.len(), 2);
        assert!(matches!(duper_stream[0].inner, DuperInner::Array(_)));
        assert!(matches!(duper_stream[1].inner, DuperInner::Object(_)));
    }

    #[test]
    fn duper() {
        let input = r#"
        {duper: 1337}
        "#;
        let duper = DuperParser::parse_duper(input).unwrap();
        assert!(matches!(duper.inner, DuperInner::Object(_)));
    }

    #[test]
    fn duper_value() {
        let input = r#"
        "hello"
        "#;
        let duper = DuperParser::parse_duper_value(input).unwrap();
        assert!(matches!(duper.inner, DuperInner::String(_)));
    }
}
