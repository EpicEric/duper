use pest::{Parser as _, error::Error};

use crate::{ast::DuperValue, builder::DuperBuilder};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct DuperParser;

impl DuperParser {
    pub fn parse_duper_stream(input: &'_ str) -> Result<Vec<DuperValue<'_>>, Error<Rule>> {
        let mut pairs = Self::parse(Rule::duper_stream, input)?;
        DuperBuilder::build_duper_stream(pairs.next().unwrap())
    }

    pub fn parse_duper(input: &'_ str) -> Result<DuperValue<'_>, Error<Rule>> {
        let mut pairs = Self::parse(Rule::duper, input)?;
        DuperBuilder::build_duper(pairs.next().unwrap())
    }
}
