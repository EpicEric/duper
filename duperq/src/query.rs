// duperq 'span.tagged && span[0]name == sp0001 | "[${level}] ${span[0]time} - ${span[0]status} ${telemetry.duration:ms}"'
// duperq 'metadata.tags[created_at >= Instant(2025-11-22T00:00:00-03:00)]'

use chumsky::prelude::*;

use crate::filter::{AndFilter, Filter};

fn query<'a>() -> impl Parser<'a, &'a str, (AndFilter<'a>, Option<()>), extra::Err<Rich<'a, char>>>
{
    filter()
        .padded()
        .separated_by(just('|').padded())
        .collect::<AndFilter<'a>>()
        .then(just('|').ignore_then(fmt().padded()).or_not())
}

fn filter<'a>() -> impl Parser<'a, &'a str, &'a dyn Filter, extra::Err<Rich<'a, char>>> {
    todo!()
}

fn fmt<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    any()
        .repeated()
        .delimited_by(just('"'), just('"'))
        .ignored()
}
