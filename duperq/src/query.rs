// duperq 'span.tagged && span[0]name == sp0001 | "[${level}] ${span[0]time} - ${span[0]status} ${telemetry.duration:ms}"'
// duperq 'metadata.tags[created_at >= Instant(2025-11-22T00:00:00-03:00)]'

use chumsky::prelude::*;
use duper::{
    DuperInner,
    parser::{duper_value, integer, object_key},
};

use crate::{
    accessor::{
        AnyAccessor, DuperAccessor, FieldAccessor, FlattenedAccessor, IndexAccessor,
        RangeIndexAccessor, ReverseIndexAccessor,
    },
    filter::{
        AccessorFilter, AndFilter, CmpValue, DuperFilter, EqFilter, EqValue, GeFilter, GtFilter,
        IsFilter, IsTruthyFilter, LeFilter, LtFilter, NeFilter, NotFilter, OrFilter, RegexFilter,
        TryFromDuperValueError,
    },
};

fn query<'a>()
-> impl Parser<'a, &'a str, (Box<dyn DuperFilter>, Option<()>), extra::Err<Rich<'a, char>>> {
    choice((just("filter").padded().ignore_then(filter()),))
        .separated_by(just('|'))
        .collect::<AndFilter>()
        .map(|filter| Box::new(filter) as Box<dyn DuperFilter>)
        .padded()
        .then(
            just('|')
                .padded()
                .ignore_then(just("format").padded())
                .ignore_then(fmt().padded())
                .or_not(),
        )
}

fn filter<'a>() -> impl Parser<'a, &'a str, Box<dyn DuperFilter>, extra::Err<Rich<'a, char>>> + Clone
{
    recursive(|filter| {
        let atom = leaf_filter()
            .or(filter.delimited_by(just('('), just(')')))
            .padded();

        let unary = just('!')
            .ignored()
            .or(just("not").ignored())
            .padded()
            .repeated()
            .at_least(1)
            .foldr(atom.clone(), |_, rhs| {
                Box::new(NotFilter(rhs)) as Box<dyn DuperFilter>
            })
            .or(atom)
            .padded();

        let and = unary
            .clone()
            .separated_by(just("&&").or(just("and")))
            .at_least(2)
            .collect::<AndFilter>()
            .map(|filter| Box::new(filter) as Box<dyn DuperFilter>)
            .or(unary)
            .padded();

        let or = and
            .clone()
            .separated_by(just("||").or(just("or")))
            .at_least(2)
            .collect::<OrFilter>()
            .map(|filter| Box::new(filter) as Box<dyn DuperFilter>)
            .or(and)
            .padded();

        or
    })
}

fn accessor<'a>()
-> impl Parser<'a, &'a str, Box<dyn DuperAccessor>, extra::Err<Rich<'a, char>>> + Clone {
    let access = just('.').or_not().ignore_then(choice((
        object_key().padded().map(|key: duper::DuperKey<'a>| {
            Box::new(FieldAccessor(key.as_ref().into())) as Box<dyn DuperAccessor>
        }),
        integer()
            .or_not()
            .padded()
            .then_ignore(just(".."))
            .then(just('=').or_not())
            .then(integer().or_not().padded())
            .delimited_by(just('['), just(']'))
            .padded()
            .try_map(|((start, end_inclusive), end), span| match (start, end) {
                (Some(start), _) if start < 0 => {
                    Err(Rich::custom(span, "range start must be positive"))
                }
                (_, Some(end)) if end < 0 => Err(Rich::custom(span, "range end must be positive")),
                (None, None) => Ok(RangeIndexAccessor {
                    start: std::ops::Bound::Unbounded,
                    end: std::ops::Bound::Unbounded,
                }),
                (Some(start), None) => Ok(RangeIndexAccessor {
                    start: std::ops::Bound::Included(start as usize),
                    end: std::ops::Bound::Unbounded,
                }),
                (None, Some(end)) => Ok(RangeIndexAccessor {
                    start: std::ops::Bound::Unbounded,
                    end: if end_inclusive.is_some() {
                        std::ops::Bound::Included(end as usize)
                    } else {
                        std::ops::Bound::Excluded(end as usize)
                    },
                }),
                (Some(start), Some(end)) => Ok(RangeIndexAccessor {
                    start: std::ops::Bound::Included(start as usize),
                    end: if end_inclusive.is_some() {
                        std::ops::Bound::Included(end as usize)
                    } else {
                        std::ops::Bound::Excluded(end as usize)
                    },
                }),
            })
            .map(|accessor| Box::new(accessor) as Box<dyn DuperAccessor>),
        integer()
            .padded()
            .delimited_by(just('['), just(']'))
            .padded()
            .map(|int| {
                if int < 0 {
                    Box::new(ReverseIndexAccessor(int.unsigned_abs() as usize))
                        as Box<dyn DuperAccessor>
                } else {
                    Box::new(IndexAccessor(int as usize)) as Box<dyn DuperAccessor>
                }
            }),
        text::whitespace()
            .delimited_by(just('['), just(']'))
            .padded()
            .map(|_| Box::new(AnyAccessor) as Box<dyn DuperAccessor>),
    )));

    access
        .clone()
        .repeated()
        .at_least(2)
        .collect::<Vec<_>>()
        .map(|vec| Box::new(FlattenedAccessor(vec)) as Box<dyn DuperAccessor>)
        .or(access)
}

fn leaf_filter<'a>()
-> impl Parser<'a, &'a str, Box<dyn DuperFilter>, extra::Err<Rich<'a, char>>> + Clone {
    let accessor = accessor();

    let eq_op = just("==").ignored().or(just('=').ignored()).padded();
    let ne_op = just("!=").ignored().or(just("<>").ignored()).padded();
    let lt_op = just("<").ignored().padded();
    let le_op = just("<=").ignored().padded();
    let gt_op = just(">").ignored().padded();
    let ge_op = just(">=").ignored().padded();
    let re_op = just("=~").ignored().padded();
    let is_op = just("is").ignored().padded();

    just("len")
        .ignore_then(accessor.clone().delimited_by(just('('), just(')')))
        .then(choice((
            eq_op
                .clone()
                .ignore_then(integer().padded())
                .try_map(|value, span| {
                    if value >= 0 {
                        Ok(Box::new(EqFilter(EqValue::Len(value as usize)))
                            as Box<dyn DuperFilter>)
                    } else {
                        Err(Rich::custom(
                            span,
                            TryFromDuperValueError::InvalidSize(value),
                        ))
                    }
                }),
            ne_op
                .clone()
                .ignore_then(integer().padded())
                .try_map(|value, span| {
                    if value >= 0 {
                        Ok(Box::new(NeFilter(EqValue::Len(value as usize)))
                            as Box<dyn DuperFilter>)
                    } else {
                        Err(Rich::custom(
                            span,
                            TryFromDuperValueError::InvalidSize(value),
                        ))
                    }
                }),
            lt_op
                .clone()
                .ignore_then(integer().padded())
                .try_map(|value, span| {
                    if value >= 0 {
                        Ok(Box::new(LtFilter(CmpValue::Len(value as usize)))
                            as Box<dyn DuperFilter>)
                    } else {
                        Err(Rich::custom(
                            span,
                            TryFromDuperValueError::InvalidSize(value),
                        ))
                    }
                }),
            le_op
                .clone()
                .ignore_then(integer().padded())
                .try_map(|value, span| {
                    if value >= 0 {
                        Ok(Box::new(LeFilter(CmpValue::Len(value as usize)))
                            as Box<dyn DuperFilter>)
                    } else {
                        Err(Rich::custom(
                            span,
                            TryFromDuperValueError::InvalidSize(value),
                        ))
                    }
                }),
            gt_op
                .clone()
                .ignore_then(integer().padded())
                .try_map(|value, span| {
                    if value >= 0 {
                        Ok(Box::new(GtFilter(CmpValue::Len(value as usize)))
                            as Box<dyn DuperFilter>)
                    } else {
                        Err(Rich::custom(
                            span,
                            TryFromDuperValueError::InvalidSize(value),
                        ))
                    }
                }),
            ge_op
                .clone()
                .ignore_then(integer().padded())
                .try_map(|value, span| {
                    if value >= 0 {
                        Ok(Box::new(GeFilter(CmpValue::Len(value as usize)))
                            as Box<dyn DuperFilter>)
                    } else {
                        Err(Rich::custom(
                            span,
                            TryFromDuperValueError::InvalidSize(value),
                        ))
                    }
                }),
        )))
        .or(accessor.clone().then(choice((
            eq_op
                .ignore_then(duper_value().padded())
                .try_map(|value, span| {
                    EqValue::try_from_duper(value, None)
                        .map(|value| Box::new(EqFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            ne_op
                .ignore_then(duper_value().padded())
                .try_map(|value, span| {
                    EqValue::try_from_duper(value, None)
                        .map(|value| Box::new(NeFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            lt_op
                .ignore_then(duper_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(LtFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            le_op
                .ignore_then(duper_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(LeFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            gt_op
                .ignore_then(duper_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(GtFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            ge_op
                .ignore_then(duper_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(GeFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            re_op
                .ignore_then(duper_value().padded())
                .try_map(|value, span| match value.inner {
                    DuperInner::String(string) => regex::bytes::Regex::new(string.as_ref())
                        .map(|regex| Box::new(RegexFilter(regex)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error)),
                    _ => Err(Rich::custom(
                        span,
                        "can only use regex operator =~ with string",
                    )),
                }),
            is_op
                .ignore_then(
                    choice((
                        just("Object").map(|_| IsFilter::Object),
                        just("Array").map(|_| IsFilter::Array),
                        just("Tuple").map(|_| IsFilter::Tuple),
                        just("String").map(|_| IsFilter::String),
                        just("Bytes").map(|_| IsFilter::Bytes),
                        just("Instant").map(|_| IsFilter::TemporalInstant),
                        just("ZonedDateTime").map(|_| IsFilter::TemporalZonedDateTime),
                        just("PlainDate").map(|_| IsFilter::TemporalPlainDate),
                        just("PlainTime").map(|_| IsFilter::TemporalPlainTime),
                        just("PlainDateTime").map(|_| IsFilter::TemporalPlainDateTime),
                        just("PlainYearMonth").map(|_| IsFilter::TemporalPlainYearMonth),
                        just("PlainMonthDay").map(|_| IsFilter::TemporalPlainMonthDay),
                        just("Duration").map(|_| IsFilter::TemporalDuration),
                        just("Temporal").map(|_| IsFilter::TemporalUnspecified),
                        just("Integer").map(|_| IsFilter::Integer),
                        just("Float").map(|_| IsFilter::Float),
                        just("Number").map(|_| IsFilter::Number),
                        just("Boolean").map(|_| IsFilter::Boolean),
                        just("Null").map(|_| IsFilter::Null),
                    ))
                    .padded(),
                )
                .map(|value| Box::new(value) as Box<dyn DuperFilter>),
        ))))
        .map(|(accessor, filter)| {
            Box::new(AccessorFilter { accessor, filter }) as Box<dyn DuperFilter>
        })
        .or(accessor.map(|accessor| {
            Box::new(AccessorFilter {
                accessor,
                filter: Box::new(IsTruthyFilter),
            }) as Box<dyn DuperFilter>
        }))
}

fn fmt<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    any()
        .repeated()
        .delimited_by(just('"'), just('"'))
        .ignored()
}
