use chumsky::prelude::*;
use duper::{
    Ansi, DuperValue, PrettyPrinter, Serializer,
    escape::unescape_str,
    parser::{identified_value, integer, object_key, quoted_string},
};
use smol::{channel, io::AsyncWrite};

use crate::{
    accessor::{
        AnyAccessor, DuperAccessor, FieldAccessor, FilterAccessor, FlattenedAccessor,
        IndexAccessor, RangeIndexAccessor, ReverseIndexAccessor, SelfAccessor,
    },
    filter::{
        AccessorFilter, AndFilter, CastFilter, CmpValue, DuperFilter, EqFilter, EqValue, GeFilter,
        GtFilter, IsFilter, IsTruthyFilter, LeFilter, LtFilter, NeFilter, NotFilter, OrFilter,
        RegexFilter, RegexIdentifierFilter, TrueFilter, TryFromDuperValueError,
    },
    formatter::{Formatter, FormatterAtom},
    processor::{FilterProcessor, OutputProcessor, Processor, SkipProcessor, TakeProcessor},
    types::DuperType,
};

pub(crate) type CreateProcessorFn<P> = Box<dyn FnOnce(P) -> Box<dyn Processor>>;

/// Parses a `duperq` query.
pub fn query<'a, O>() -> impl Parser<
    'a,
    &'a str,
    (
        Vec<CreateProcessorFn<channel::Sender<DuperValue<'static>>>>,
        CreateProcessorFn<O>,
    ),
    extra::Err<Rich<'a, char>>,
>
where
    O: AsyncWrite + Unpin + 'static,
{
    let output_processor = choice((
        just("format").padded().ignore_then(fmt().padded()),
        just("ansi").padded().map(|_| {
            Box::new(|output| {
                let mut ansi = Ansi::default();
                Box::new(OutputProcessor::new(
                    output,
                    Box::new(move |value| ansi.to_ansi(&value).unwrap_or_default()),
                )) as Box<dyn Processor>
            }) as CreateProcessorFn<O>
        }),
        just("pretty-print").padded().map(|_| {
            Box::new(|output| {
                let mut pretty_printer = PrettyPrinter::default();
                Box::new(OutputProcessor::new(
                    output,
                    Box::new(move |value| pretty_printer.pretty_print(&value).into_bytes()),
                )) as Box<dyn Processor>
            }) as CreateProcessorFn<O>
        }),
    ))
    .padded();

    choice((
        just("filter").padded().ignore_then(filter()).map(|filter| {
            Box::new(move |sender| {
                Box::new(FilterProcessor::new(sender, filter)) as Box<dyn Processor>
            }) as CreateProcessorFn<channel::Sender<DuperValue<'static>>>
        }),
        just("take")
            .padded()
            .ignore_then(integer())
            .try_map(|take, span| {
                if take > 0 {
                    Ok(Box::new(move |sender| {
                        Box::new(TakeProcessor::new(sender, take as usize)) as Box<dyn Processor>
                    })
                        as CreateProcessorFn<channel::Sender<DuperValue<'static>>>)
                } else {
                    Err(Rich::custom(
                        span,
                        "take parameter must be greater than zero",
                    ))
                }
            }),
        just("skip")
            .padded()
            .ignore_then(integer())
            .try_map(|skip, span| {
                if skip >= 0 {
                    Ok(Box::new(move |sender| {
                        Box::new(SkipProcessor::new(sender, skip as usize)) as Box<dyn Processor>
                    })
                        as CreateProcessorFn<channel::Sender<DuperValue<'static>>>)
                } else {
                    Err(Rich::custom(span, "skip parameter must be positive"))
                }
            }),
    ))
    .padded()
    .separated_by(just('|'))
    .collect::<Vec<_>>()
    .then(
        just('|')
            .padded()
            .ignore_then(output_processor.padded())
            .or_not()
            .map(|processor| {
                processor.unwrap_or_else(|| {
                    Box::new(|output| {
                        let mut serializer = Serializer::default();
                        Box::new(OutputProcessor::new(
                            output,
                            Box::new(move |value| serializer.serialize(&value).into_bytes()),
                        )) as Box<dyn Processor>
                    }) as CreateProcessorFn<O>
                })
            }),
    )
    .then_ignore(end())
}

fn filter<'a>() -> impl Parser<'a, &'a str, Box<dyn DuperFilter>, extra::Err<Rich<'a, char>>> + Clone
{
    recursive(|filter| {
        let atom = filter
            .delimited_by(just('('), just(')'))
            .or(leaf_filter(accessor()))
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

        and.clone()
            .separated_by(just("||").or(just("or")))
            .at_least(2)
            .collect::<OrFilter>()
            .map(|filter| Box::new(filter) as Box<dyn DuperFilter>)
            .or(and)
            .padded()
    })
    .boxed()
}

fn accessor<'a>()
-> impl Parser<'a, &'a str, Box<dyn DuperAccessor>, extra::Err<Rich<'a, char>>> + Clone {
    recursive(|accessor| {
        let access = choice((
            just('.').ignore_then(object_key().map(|key: duper::DuperKey<'a>| {
                Box::new(FieldAccessor(key.static_clone())) as Box<dyn DuperAccessor>
            })),
            just('.').map(|_| Box::new(SelfAccessor) as Box<dyn DuperAccessor>),
            integer()
                .or_not()
                .padded()
                .then_ignore(just(".."))
                .then(just('=').or_not())
                .then(integer().or_not().padded())
                .delimited_by(just('['), just(']'))
                .try_map(|((start, end_inclusive), end), span| match (start, end) {
                    (Some(start), _) if start < 0 => {
                        Err(Rich::custom(span, "range start must be positive"))
                    }
                    (_, Some(end)) if end < 0 => {
                        Err(Rich::custom(span, "range end must be positive"))
                    }
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
                .map(|int| {
                    if int < 0 {
                        Box::new(ReverseIndexAccessor(int.unsigned_abs() as usize))
                            as Box<dyn DuperAccessor>
                    } else {
                        Box::new(IndexAccessor(int as usize)) as Box<dyn DuperAccessor>
                    }
                }),
            leaf_filter(accessor)
                .padded()
                .delimited_by(just('['), just(']'))
                .map(|filter| Box::new(FilterAccessor(filter)) as Box<dyn DuperAccessor>),
            text::whitespace()
                .delimited_by(just('['), just(']'))
                .map(|_| Box::new(AnyAccessor) as Box<dyn DuperAccessor>),
        ));

        access
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|mut vec| {
                if vec.len() == 1 {
                    vec.remove(0)
                } else {
                    Box::new(FlattenedAccessor(vec)) as Box<dyn DuperAccessor>
                }
            })
    })
    .boxed()
}

fn leaf_filter<'a>(
    accessor: impl Parser<'a, &'a str, Box<dyn DuperAccessor>, extra::Err<Rich<'a, char>>> + Clone,
) -> impl Parser<'a, &'a str, Box<dyn DuperFilter>, extra::Err<Rich<'a, char>>> + Clone {
    type ConsumeAccessor = Box<dyn FnOnce(Box<dyn DuperFilter>) -> Box<dyn DuperFilter>>;

    let cast_accessor = just("cast")
        .padded()
        .ignore_then(
            accessor
                .clone()
                .padded()
                .then_ignore(just(','))
                .then(duper_type().padded())
                .map(|(accessor, typ)| {
                    Box::new(|filter: Box<dyn DuperFilter>| {
                        Box::new(AccessorFilter {
                            filter: Box::new(CastFilter { filter, typ }),
                            accessor,
                        }) as Box<dyn DuperFilter>
                    }) as ConsumeAccessor
                })
                .delimited_by(just('('), just(')')),
        )
        .or(accessor.map(|accessor| {
            Box::new(|filter: Box<dyn DuperFilter>| {
                Box::new(AccessorFilter { filter, accessor }) as Box<dyn DuperFilter>
            }) as ConsumeAccessor
        }));

    let eq_op = just("==").ignored().or(just('=').ignored()).padded();
    let ne_op = just("!=").ignored().or(just("<>").ignored()).padded();
    let lt_op = just("<").ignored().padded();
    let le_op = just("<=").ignored().padded();
    let gt_op = just(">").ignored().padded();
    let ge_op = just(">=").ignored().padded();
    let re_op = just("=~").ignored().padded();
    let is_op = just("is").ignored().padded();

    let len_filter = just("len")
        .ignore_then(cast_accessor.clone().delimited_by(just('('), just(')')))
        .then(choice((
            eq_op
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
        )));

    let identifier_filter = just("identifier")
        .ignore_then(cast_accessor.clone().delimited_by(just('('), just(')')))
        .then(choice((
            eq_op
                .ignore_then(
                    quoted_string()
                        .map(|identifier| Some(identifier.into_owned()))
                        .or(just("null").to(None))
                        .padded(),
                )
                .map(|value| {
                    Box::new(EqFilter(EqValue::Identifier(value))) as Box<dyn DuperFilter>
                }),
            ne_op
                .ignore_then(
                    quoted_string()
                        .map(|identifier| Some(identifier.into_owned()))
                        .or(just("null").to(None))
                        .padded(),
                )
                .map(|value| {
                    Box::new(NeFilter(EqValue::Identifier(value))) as Box<dyn DuperFilter>
                }),
            re_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| match value {
                    DuperValue::String { inner: string, .. } => regex::Regex::new(string.as_ref())
                        .map(|regex| Box::new(RegexIdentifierFilter(regex)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error)),
                    _ => Err(Rich::custom(
                        span,
                        "can only use regex operator =~ with string",
                    )),
                }),
        )));

    let exists_filter = just("exists")
        .ignore_then(cast_accessor.clone().delimited_by(just('('), just(')')))
        .map(|accessor| (accessor, Box::new(TrueFilter) as Box<dyn DuperFilter>));

    choice((
        len_filter,
        identifier_filter,
        exists_filter,
        cast_accessor.clone().then(choice((
            eq_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| {
                    EqValue::try_from_duper(value, None)
                        .map(|value| Box::new(EqFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            ne_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| {
                    EqValue::try_from_duper(value, None)
                        .map(|value| Box::new(NeFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            lt_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(LtFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            le_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(LeFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            gt_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(GtFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            ge_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| {
                    CmpValue::try_from(value)
                        .map(|value| Box::new(GeFilter(value)) as Box<dyn DuperFilter>)
                        .map_err(|error| Rich::custom(span, error))
                }),
            re_op
                .ignore_then(identified_value().padded())
                .try_map(|value, span| match value {
                    DuperValue::String { inner: string, .. } => {
                        regex::bytes::Regex::new(string.as_ref())
                            .map(|regex| Box::new(RegexFilter(regex)) as Box<dyn DuperFilter>)
                            .map_err(|error| Rich::custom(span, error))
                    }
                    _ => Err(Rich::custom(
                        span,
                        "can only use regex operator =~ with string",
                    )),
                }),
            is_op
                .ignore_then(duper_type().padded())
                .map(|typ| Box::new(IsFilter(typ)) as Box<dyn DuperFilter>),
        ))),
    ))
    .map(|(accessor, filter)| (accessor)(filter))
    .or(cast_accessor.map(|accessor| (accessor)(Box::new(IsTruthyFilter))))
}

fn duper_type<'a>() -> impl Parser<'a, &'a str, DuperType, extra::Err<Rich<'a, char>>> + Clone {
    choice((
        just("Object").to(DuperType::Object),
        just("Array").to(DuperType::Array),
        just("Tuple").to(DuperType::Tuple),
        just("String").to(DuperType::String),
        just("Bytes").to(DuperType::Bytes),
        just("Instant").to(DuperType::TemporalInstant),
        just("ZonedDateTime").to(DuperType::TemporalZonedDateTime),
        just("PlainDate").to(DuperType::TemporalPlainDate),
        just("PlainTime").to(DuperType::TemporalPlainTime),
        just("PlainDateTime").to(DuperType::TemporalPlainDateTime),
        just("PlainYearMonth").to(DuperType::TemporalPlainYearMonth),
        just("PlainMonthDay").to(DuperType::TemporalPlainMonthDay),
        just("Duration").to(DuperType::TemporalDuration),
        just("Temporal").to(DuperType::TemporalUnspecified),
        just("Integer").to(DuperType::Integer),
        just("Float").to(DuperType::Float),
        just("Number").to(DuperType::Number),
        just("Boolean").to(DuperType::Boolean),
        just("Null").to(DuperType::Null),
    ))
}

fn fmt<'a, O>() -> impl Parser<'a, &'a str, CreateProcessorFn<O>, extra::Err<Rich<'a, char>>> + Clone
where
    O: AsyncWrite + Unpin + 'static,
{
    choice((
        just('$').ignore_then(choice((
            just("cast").padded().ignore_then(
                accessor()
                    .padded()
                    .then_ignore(just(','))
                    .then(duper_type().padded())
                    .map(|(accessor, typ)| FormatterAtom::Dynamic(accessor, Some(typ)))
                    .delimited_by(just('('), just(')')),
            ),
            accessor()
                .map(|accessor| FormatterAtom::Dynamic(accessor, None))
                .padded()
                .delimited_by(just('{'), just('}')),
        ))),
        quoted_inner().try_map(|slice: &str, span| match unescape_str(slice) {
            Ok(unescaped) => Ok(FormatterAtom::Fixed(unescaped.clone().into_owned())),
            Err(error) => Err(Rich::custom(span, error.to_string())),
        }),
    ))
    .repeated()
    .collect::<Vec<_>>()
    .delimited_by(just('"'), just('"'))
    .map(|atoms| {
        Box::new(move |output| {
            let mut formatter = Formatter::new(atoms);
            Box::new(OutputProcessor::new(
                output,
                Box::new(move |value| formatter.format(value).into_bytes()),
            )) as Box<dyn Processor>
        }) as CreateProcessorFn<O>
    })
}

fn quoted_inner<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> + Clone {
    let escaped_characters = just('\\')
        .then(choice((
            one_of("\"\\/bfnrt0").to_slice(),
            just('x').then(hex_digit().repeated().exactly(2)).to_slice(),
            just('u').then(hex_digit().repeated().exactly(4)).to_slice(),
            just('U').then(hex_digit().repeated().exactly(8)).to_slice(),
        )))
        .to_slice();

    none_of("\"\\$")
        .and_is(control_character().not())
        .to_slice()
        .or(escaped_characters)
        .or(just('$').then(just('{').not()).to_slice())
        .repeated()
        .at_least(1)
        .to_slice()
}

fn hex_digit<'a>() -> impl Parser<'a, &'a str, char, extra::Err<Rich<'a, char>>> + Clone {
    choice((one_of('0'..='9'), one_of('a'..='f'), one_of('A'..='F')))
        .labelled("a hexadecimal digit")
}
fn control_character<'a>() -> impl Parser<'a, &'a str, char, extra::Err<Rich<'a, char>>> + Clone {
    choice((
        one_of('\u{0000}'..='\u{0009}'),
        one_of('\u{000b}'..='\u{001f}'),
        just('\u{007f}'),
    ))
    .labelled("a control character or tab, excluding new line")
}
