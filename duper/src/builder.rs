use std::{borrow::Cow, collections::HashSet};

use json_escape::explicit::unescape;
use pest::{
    error::{Error, ErrorVariant},
    iterators::Pair,
};

use crate::{
    ast::{DuperInner, DuperValue},
    parser::Rule,
};

pub(crate) struct DuperBuilder;

impl DuperBuilder {
    pub(crate) fn build_duper_stream(
        pair: Pair<'_, Rule>,
    ) -> Result<Vec<DuperValue<'_>>, Error<Rule>> {
        pair.into_inner()
            .map(|inner_pair| Self::build_duper_trunk(inner_pair))
            .collect()
    }

    pub(crate) fn build_duper(pair: Pair<'_, Rule>) -> Result<DuperValue<'_>, Error<Rule>> {
        Self::build_duper_trunk(pair)
    }

    fn build_duper_trunk(pair: Pair<'_, Rule>) -> Result<DuperValue<'_>, Error<Rule>> {
        let span = pair.as_span().clone();
        let mut duper_trunk = pair.into_inner();
        let mut next = duper_trunk.next().unwrap();
        let identifier = match next.as_rule() {
            Rule::identifier => {
                let identifier = next.as_str();
                next = duper_trunk.next().unwrap();
                Some(Cow::Borrowed(identifier))
            }
            _ => None,
        };
        Ok(match next.as_rule() {
            Rule::object => DuperValue {
                identifier,
                inner: DuperInner::Object(Self::build_object(next)?),
            },
            Rule::array => DuperValue {
                identifier,
                inner: DuperInner::Array(Self::build_array(next)?),
            },
            rule => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("unexpected rule in trunk {rule:?}"),
                    },
                    span,
                ));
            }
        })
    }

    fn build_object(
        pair: Pair<'_, Rule>,
    ) -> Result<Vec<(Cow<'_, str>, DuperValue<'_>)>, Error<Rule>> {
        debug_assert!(matches!(pair.as_rule(), Rule::object));
        let span = pair.as_span().clone();
        let kv_pairs: Result<Vec<(Cow<'_, str>, DuperValue<'_>)>, _> = pair
            .into_inner()
            .map(|pair| {
                let span = pair.as_span().clone();
                let mut inner_pair = pair.into_inner();
                let key_pair = inner_pair.next().unwrap();
                let key = match key_pair.as_rule() {
                    Rule::string => {
                        Self::unescape_str(key_pair.into_inner().next().unwrap().as_str())
                    }
                    Rule::raw_string => {
                        Cow::Borrowed(key_pair.into_inner().next().unwrap().as_str())
                    }
                    Rule::plain_key => Cow::Borrowed(key_pair.as_str()),
                    rule => {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: format!("unexpected rule in object key {rule:?}"),
                            },
                            span,
                        ));
                    }
                };
                let value = Self::build_value(inner_pair.next().unwrap());
                value.map(|v| (key, v))
            })
            .collect();
        let kv_pairs = kv_pairs?;
        let unique_keys: HashSet<&Cow<'_, str>> = kv_pairs.iter().map(|(k, _)| k).collect();
        if unique_keys.len() == kv_pairs.len() {
            Ok(kv_pairs)
        } else {
            Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "duplicate keys in object".into(),
                },
                span,
            ))
        }
    }

    fn build_array(pair: Pair<'_, Rule>) -> Result<Vec<DuperValue<'_>>, Error<Rule>> {
        debug_assert!(matches!(pair.as_rule(), Rule::array | Rule::tuple));
        pair.into_inner()
            .map(|pair| Self::build_value(pair))
            .collect()
    }

    fn build_value(pair: Pair<'_, Rule>) -> Result<DuperValue<'_>, Error<Rule>> {
        let span = pair.as_span().clone();
        let mut inner_pair = pair.into_inner();
        let mut next = inner_pair.next().unwrap();
        let identifier = match next.as_rule() {
            Rule::identifier => {
                let identifier = next.as_str();
                next = inner_pair.next().unwrap();
                Some(Cow::Borrowed(identifier))
            }
            _ => None,
        };
        Ok(DuperValue {
            identifier,
            inner: match next.as_rule() {
                Rule::object => DuperInner::Object(Self::build_object(next)?),
                Rule::array => DuperInner::Array(Self::build_array(next)?),
                Rule::tuple => DuperInner::Tuple(Self::build_array(next)?),
                Rule::string => DuperInner::String(Self::unescape_str(
                    next.into_inner().next().unwrap().as_str(),
                )),
                Rule::raw_string => {
                    DuperInner::String(Cow::Borrowed(next.into_inner().next().unwrap().as_str()))
                }
                Rule::bytes => DuperInner::Bytes(Cow::Owned(
                    Self::unescape_str(next.into_inner().next().unwrap().as_str())
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
                        rule => {
                            return Err(Error::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!("unexpected rule in integer value {rule:?}"),
                                },
                                span,
                            ));
                        }
                    }
                }),
                Rule::float => DuperInner::Float(next.as_str().replace('_', "").parse().unwrap()),
                Rule::boolean => DuperInner::Boolean(next.as_str().parse().unwrap()),
                Rule::null => DuperInner::Null,
                rule => {
                    return Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!("unexpected rule in value {rule:?}"),
                        },
                        span,
                    ));
                }
            },
        })
    }

    fn unescape_str(input: &'_ str) -> Cow<'_, str> {
        unescape(input).decode_utf8().unwrap()
    }
}
