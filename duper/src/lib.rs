use std::borrow::Cow;

use json_escape::explicit::unescape;
use pest::{Parser, error::Error, iterators::Pair};
use pest_derive::Parser;

#[derive(Debug)]
struct DuperStream<'a>(Vec<DuperTrunk<'a>>);

#[derive(Debug)]
enum DuperTrunk<'a> {
    Object {
        identifier: Option<&'a str>,
        value: Vec<(Cow<'a, str>, DuperValue<'a>)>,
    },
    Array {
        identifier: Option<&'a str>,
        value: Vec<DuperValue<'a>>,
    },
}

#[derive(Debug)]
struct DuperValue<'a> {
    identifier: Option<&'a str>,
    value: DuperInner<'a>,
}

#[derive(Debug)]
enum DuperInner<'a> {
    Object(Vec<(Cow<'a, str>, DuperValue<'a>)>),
    Array(Vec<DuperValue<'a>>),
    String(Cow<'a, str>),
    Bytes(Cow<'a, [u8]>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl<'a> Into<DuperValue<'a>> for DuperTrunk<'a> {
    fn into(self) -> DuperValue<'a> {
        match self {
            DuperTrunk::Object { identifier, value } => DuperValue {
                identifier,
                value: DuperInner::Object(value),
            },
            DuperTrunk::Array { identifier, value } => DuperValue {
                identifier,
                value: DuperInner::Array(value),
            },
        }
    }
}

#[derive(Parser)]
#[grammar = "duper.pest"]
struct DuperParser;

fn parse_duper_stream(input: &str) -> Result<DuperStream<'_>, Error<Rule>> {
    let duper = DuperParser::parse(Rule::duper_stream, input)?
        .next()
        .unwrap();

    Ok(DuperStream(
        duper.into_inner().map(parse_duper_trunk).collect(),
    ))
}

fn parse_duper(input: &str) -> Result<DuperTrunk<'_>, Error<Rule>> {
    let duper = DuperParser::parse(Rule::duper, input)?.next().unwrap();

    Ok(parse_duper_trunk(duper).into())
}

fn parse_duper_trunk(pair: Pair<'_, Rule>) -> DuperTrunk<'_> {
    let mut duper_trunk = pair.into_inner();
    let mut next = duper_trunk.next().unwrap();
    let identifier = match next.as_rule() {
        Rule::identifier => {
            let identifier = next.as_str();
            next = duper_trunk.next().unwrap();
            Some(identifier)
        }
        _ => None,
    };
    match next.as_rule() {
        Rule::object => DuperTrunk::Object {
            identifier,
            value: parse_object(next),
        },
        Rule::array => DuperTrunk::Array {
            identifier,
            value: parse_array(next),
        },
        _ => unreachable!(),
    }
}

fn parse_object(pair: Pair<'_, Rule>) -> Vec<(Cow<'_, str>, DuperValue<'_>)> {
    debug_assert!(matches!(pair.as_rule(), Rule::object));
    pair.into_inner()
        .map(|pair| {
            let mut inner_pair = pair.into_inner();
            let key_pair = inner_pair.next().unwrap();
            let key = match key_pair.as_rule() {
                Rule::string => unescape_str(key_pair.into_inner().next().unwrap().as_str()),
                Rule::raw_string => Cow::Borrowed(key_pair.into_inner().next().unwrap().as_str()),
                Rule::plain_key => Cow::Borrowed(key_pair.as_str()),
                _ => unreachable!(),
            };
            let value = parse_value(inner_pair.next().unwrap());
            (key, value)
        })
        .collect()
}

fn parse_array(pair: Pair<'_, Rule>) -> Vec<DuperValue<'_>> {
    debug_assert!(matches!(pair.as_rule(), Rule::array));
    pair.into_inner().map(|pair| parse_value(pair)).collect()
}

fn parse_value(pair: Pair<'_, Rule>) -> DuperValue<'_> {
    let mut inner_pair = pair.into_inner();
    let mut next = inner_pair.next().unwrap();
    let identifier = match next.as_rule() {
        Rule::identifier => {
            let identifier = next.as_str();
            next = inner_pair.next().unwrap();
            Some(identifier)
        }
        _ => None,
    };
    DuperValue {
        identifier,
        value: match next.as_rule() {
            Rule::object => DuperInner::Object(parse_object(next)),
            Rule::array => DuperInner::Array(parse_array(next)),
            Rule::string => {
                DuperInner::String(unescape_str(next.into_inner().next().unwrap().as_str()))
            }
            Rule::raw_string => {
                DuperInner::String(Cow::Borrowed(next.into_inner().next().unwrap().as_str()))
            }
            Rule::bytes => DuperInner::Bytes(Cow::Owned(
                unescape_str(next.into_inner().next().unwrap().as_str())
                    .as_bytes()
                    .to_vec(),
            )),
            Rule::raw_bytes => DuperInner::Bytes(Cow::Borrowed(
                next.into_inner().next().unwrap().as_str().as_bytes(),
            )),
            Rule::integer => DuperInner::Integer({
                let integer_inner = next.into_inner().next().unwrap();
                match integer_inner.as_rule() {
                    Rule::decimal_integer => {
                        i64::from_str_radix(integer_inner.as_str(), 10).unwrap()
                    }
                    Rule::hex_integer => {
                        i64::from_str_radix(integer_inner.as_str().split_at(2).1, 16).unwrap()
                    }
                    Rule::octal_integer => {
                        i64::from_str_radix(integer_inner.as_str().split_at(2).1, 8).unwrap()
                    }
                    Rule::binary_integer => {
                        i64::from_str_radix(integer_inner.as_str().split_at(2).1, 2).unwrap()
                    }
                    _ => unreachable!(),
                }
            }),
            Rule::float => DuperInner::Float(next.as_str().replace('_', "").parse().unwrap()),
            Rule::boolean => DuperInner::Boolean(next.as_str().parse().unwrap()),
            Rule::null => DuperInner::Null,
            _ => unreachable!(),
        },
    }
}

fn unescape_str(input: &str) -> Cow<'_, str> {
    unescape(input).decode_utf8().unwrap()
}

#[test]
fn it_works() {
    let duper: DuperTrunk = parse_duper(
        r##"{"hello ": Array([" world", Null(null), true, b"bytes", 42, {inner: 0x12, "\uD83D\uDE00": 1.32e+20}]),r#"raw
        string"#: br#"a\b"#,}"##,
    )
    .unwrap();
    println!("{:?}", duper);
}
