use pest::{Parser, error::Error, iterators::Pair};
use pest_derive::Parser;

#[derive(Debug)]
struct DuperStream<'a>(Vec<&'a DuperTrunk<'a>>);

#[derive(Debug)]
struct Duper<'a>(DuperTrunk<'a>);

#[derive(Debug)]
enum DuperTrunk<'a> {
    Object {
        identifier: Option<&'a str>,
        value: Vec<(&'a str, DuperDescribedValue<'a>)>,
    },
    Array {
        identifier: Option<&'a str>,
        value: Vec<DuperDescribedValue<'a>>,
    },
}

#[derive(Debug)]
struct DuperDescribedValue<'a> {
    identifier: Option<&'a str>,
    value: DuperValue<'a>,
}

#[derive(Debug)]
enum DuperValue<'a> {
    Object(Vec<(&'a str, DuperDescribedValue<'a>)>),
    Array(Vec<DuperDescribedValue<'a>>),
    String(&'a str),
    Bytes(&'a [u8]),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Parser)]
#[grammar = "duper.pest"]
struct DuperParser;

fn parse_duper(input: &str) -> Result<Duper<'_>, Error<Rule>> {
    let duper = DuperParser::parse(Rule::duper, input)?.next().unwrap();

    let mut duper_trunk = duper.into_inner();
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
        Rule::object => Ok(Duper(DuperTrunk::Object {
            identifier,
            value: parse_object(next),
        })),
        Rule::array => Ok(Duper(DuperTrunk::Array {
            identifier,
            value: parse_array(next),
        })),
        _ => unreachable!(),
    }
}

fn parse_object(pair: Pair<'_, Rule>) -> Vec<(&str, DuperDescribedValue<'_>)> {
    debug_assert!(matches!(pair.as_rule(), Rule::object));
    pair.into_inner()
        .map(|pair| {
            let mut inner_pair = pair.into_inner();
            let key_pair = inner_pair.next().unwrap();
            let key = match key_pair.as_rule() {
                Rule::string => key_pair.into_inner().next().unwrap().as_str(),
                Rule::plain_key => key_pair.as_str(),
                _ => unreachable!(),
            };
            let value = parse_value(inner_pair.next().unwrap());
            (key, value)
        })
        .collect()
}

fn parse_array(pair: Pair<'_, Rule>) -> Vec<DuperDescribedValue<'_>> {
    debug_assert!(matches!(pair.as_rule(), Rule::array));
    pair.into_inner().map(|pair| parse_value(pair)).collect()
}

fn parse_value(pair: Pair<'_, Rule>) -> DuperDescribedValue<'_> {
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
    DuperDescribedValue {
        identifier,
        value: match next.as_rule() {
            Rule::object => DuperValue::Object(parse_object(next)),
            Rule::array => DuperValue::Array(parse_array(next)),
            Rule::string => DuperValue::String(next.into_inner().next().unwrap().as_str()),
            Rule::bytes => DuperValue::Bytes(next.into_inner().next().unwrap().as_str().as_bytes()),
            Rule::integer => DuperValue::Integer({
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
            Rule::float => DuperValue::Float(next.as_str().replace('_', "").parse().unwrap()),
            Rule::boolean => DuperValue::Boolean(next.as_str().parse().unwrap()),
            Rule::null => DuperValue::Null,
            _ => unreachable!(),
        },
    }
}

#[test]
fn it_works() {
    let duper: Duper = parse_duper(
        r#"{"hello ": Array([" world", Null(null), true, b"bytes", 42, {inner: 0x12, some_thing: 1.32e+20}]),}"#,
    )
    .unwrap();
    println!("{:?}", duper);
}
