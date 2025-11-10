use std::borrow::Cow;

use chumsky::prelude::*;

use crate::{
    DuperTemporal, DuperTemporalInner,
    parser::{ascii_alphabetic, ascii_alphanumeric, whitespace_and_comments},
};

// Duper Temporal values

pub(crate) fn temporal_specified<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    choice((
        temporal_instant(),
        temporal_zoned_date_time(),
        temporal_plain_date(),
        temporal_plain_time(),
        temporal_plain_date_time(),
        temporal_plain_year_month(),
        temporal_plain_month_day(),
        temporal_duration(),
    ))
    .padded_by(whitespace_and_comments())
}

pub(crate) fn temporal_instant<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("Instant")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            instant()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|instant| DuperTemporal::Instant(DuperTemporalInner(Cow::Borrowed(instant))))
}

pub(crate) fn temporal_zoned_date_time<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("ZonedDateTime")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            zoned_date_time()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|instant| DuperTemporal::ZonedDateTime(DuperTemporalInner(Cow::Borrowed(instant))))
}

pub(crate) fn temporal_plain_date<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("PlainDate")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            plain_date()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|instant| DuperTemporal::PlainDate(DuperTemporalInner(Cow::Borrowed(instant))))
}

pub(crate) fn temporal_plain_time<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("PlainTime")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            plain_time()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|instant| DuperTemporal::PlainTime(DuperTemporalInner(Cow::Borrowed(instant))))
}

pub(crate) fn temporal_plain_date_time<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("PlainDateTime")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            plain_date_time()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|instant| DuperTemporal::PlainDateTime(DuperTemporalInner(Cow::Borrowed(instant))))
}

pub(crate) fn temporal_plain_year_month<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("PlainYearMonth")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            plain_year_month()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|instant| DuperTemporal::PlainYearMonth(DuperTemporalInner(Cow::Borrowed(instant))))
}

pub(crate) fn temporal_plain_month_day<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("PlainMonthDay")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            plain_month_day()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|instant| DuperTemporal::PlainMonthDay(DuperTemporalInner(Cow::Borrowed(instant))))
}

pub(crate) fn temporal_duration<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    just("Duration")
        .padded_by(whitespace_and_comments())
        .ignore_then(just('('))
        .ignore_then(
            duration()
                .to_slice()
                .delimited_by(just('\''), just('\''))
                .padded_by(whitespace_and_comments()),
        )
        .then_ignore(just(')'))
        .map(|duration| DuperTemporal::Duration(DuperTemporalInner(Cow::Borrowed(duration))))
}

pub(crate) fn temporal_unspecified<'a>()
-> impl Parser<'a, &'a str, DuperTemporal<'a>, extra::Err<Rich<'a, char>>> + Clone {
    unspecified()
        .to_slice()
        .delimited_by(just('\''), just('\''))
        .padded_by(whitespace_and_comments())
        .map(|unspecified| {
            DuperTemporal::Unspecified(DuperTemporalInner(Cow::Borrowed(unspecified)))
        })
}

// Inner values

pub(crate) fn zoned_date_time<'a>()
-> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    date_time()
        .then(time_offset())
        .then(timezone())
        .then(suffix_tag().repeated())
        .padded()
        .ignored()
}

pub(crate) fn non_z_zoned_date_time<'a>()
-> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    date_time()
        .then(time_num_offset())
        .then(timezone())
        .then(suffix_tag().repeated())
        .padded()
        .ignored()
}

pub(crate) fn instant<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        zoned_date_time(),
        date_time().then(time_offset()).padded().ignored(),
    ))
}

pub(crate) fn non_z_instant<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone
{
    choice((
        non_z_zoned_date_time(),
        date_time().then(time_num_offset()).padded().ignored(),
    ))
}

pub(crate) fn plain_date<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        plain_date_time(),
        date().then(suffix_tag().repeated()).padded().ignored(),
    ))
}

pub(crate) fn plain_time<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        plain_date_time(),
        time().then(suffix_tag().repeated()).padded().ignored(),
    ))
}

pub(crate) fn plain_date_time<'a>()
-> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        non_z_instant(),
        date_time().then(suffix_tag().repeated()).padded().ignored(),
    ))
}

pub(crate) fn plain_year_month<'a>()
-> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        plain_date(),
        year_month()
            .then(suffix_tag().repeated())
            .padded()
            .ignored(),
    ))
}

pub(crate) fn plain_month_day<'a>()
-> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        plain_date(),
        month_day().then(suffix_tag().repeated()).padded().ignored(),
    ))
}

pub(crate) fn duration<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    let fractional = text::int(10).then(just('.').then(text::digits(10).at_most(9)).or_not());

    let duration_time = one_of("Tt").then(choice((
        text::int(10)
            .then(one_of("Hh"))
            .then(
                text::int(10)
                    .then(one_of("Mm"))
                    .or_not()
                    .then(fractional.clone().then(one_of("Ss"))),
            )
            .ignored(),
        text::int(10)
            .then(one_of("Hh"))
            .then(fractional.clone().then(one_of("Mm")))
            .ignored(),
        text::int(10)
            .then(one_of("Mm"))
            .then(fractional.clone().then(one_of("Ss")))
            .ignored(),
        fractional.clone().then(one_of("Hh")).ignored(),
        fractional.clone().then(one_of("Mm")).ignored(),
        fractional.then(one_of("Ss")).ignored(),
    )));

    one_of("+-")
        .or_not()
        .then(
            one_of("Pp").then(choice((
                text::int(10)
                    .then(one_of("Yy"))
                    .then(
                        text::int(10)
                            .then(one_of("Mm"))
                            .then(text::int(10).then(one_of("Ww")).or_not())
                            .then(text::int(10).then(one_of("Dd")).or_not())
                            .or_not(),
                    )
                    .then(duration_time.clone().or_not())
                    .ignored(),
                text::int(10)
                    .then(one_of("Mm"))
                    .then(text::int(10).then(one_of("Ww")).or_not())
                    .then(text::int(10).then(one_of("Dd")).or_not())
                    .then(duration_time.clone().or_not())
                    .ignored(),
                text::int(10)
                    .then(one_of("Ww"))
                    .then(text::int(10).then(one_of("Dd")).or_not())
                    .then(duration_time.clone().or_not())
                    .ignored(),
                text::int(10)
                    .then(one_of("Dd"))
                    .then(duration_time.clone().or_not())
                    .ignored(),
                duration_time.ignored(),
            ))),
        )
        .padded()
        .ignored()
}

pub(crate) fn unspecified<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone
{
    choice((plain_year_month(), plain_month_day(), duration()))
}

// Atoms

pub(crate) fn hour<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        one_of("01").then(one_of('0'..='9')),
        just('2').then(one_of('0'..='3')),
    ))
    .ignored()
}

pub(crate) fn minute_or_second<'a>()
-> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    one_of('0'..='5').then(one_of('0'..='9')).ignored()
}

pub(crate) fn time_num_offset<'a>()
-> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    one_of("+-")
        .then(hour())
        .then(just(':').or_not().then(minute_or_second()).or_not())
        .then(
            just(':')
                .then(minute_or_second())
                .then(just('.').then(text::digits(10).at_most(9)).or_not())
                .or_not(),
        )
        .ignored()
}

pub(crate) fn time_offset<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone
{
    one_of("Zz").ignored().or(time_num_offset())
}

pub(crate) fn time<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    hour()
        .then(
            just(':')
                .then(
                    minute_or_second().then(
                        just(':')
                            .then(
                                minute_or_second()
                                    .or(just("60").ignored())
                                    .then(just('.').then(text::digits(10).at_most(9)).or_not()),
                            )
                            .ignored()
                            .or(minute_or_second()
                                .or(just("60").ignored())
                                .then(just('.').then(text::digits(10).at_most(9)).or_not())
                                .ignored())
                            .or_not(),
                    ),
                )
                .ignored()
                .or(minute_or_second()
                    .then(
                        just(':')
                            .then(
                                minute_or_second()
                                    .or(just("60").ignored())
                                    .then(just('.').then(text::digits(10).at_most(9)).or_not()),
                            )
                            .ignored()
                            .or(minute_or_second()
                                .or(just("60").ignored())
                                .then(just('.').then(text::digits(10).at_most(9)).or_not())
                                .ignored())
                            .or_not(),
                    )
                    .ignored())
                .or_not(),
        )
        .ignored()
}

pub(crate) fn month_day<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    let days_31 = one_of('0'..='2')
        .then(one_of('0'..='9'))
        .ignored()
        .or(just('3').then(one_of("01")).ignored());
    let days_30 = one_of('0'..='2')
        .then(one_of('0'..='9'))
        .ignored()
        .or(just("30").ignored());
    let days_29 = one_of('0'..='1')
        .then(one_of('0'..='9'))
        .ignored()
        .or(just('2').then(one_of('0'..='9')).ignored());

    choice((
        just("--").ignored(),
        text::digits(10)
            .exactly(4)
            .then(just('-').or_not())
            .ignored(),
        one_of("+-")
            .then(text::digits(10).exactly(6))
            .then(just('-').or_not())
            .ignored(),
    ))
    .or_not()
    .ignore_then(
        choice((
            just("01"),
            just("03"),
            just("05"),
            just("07"),
            just("08"),
            just("10"),
            just("12"),
        ))
        .then(just('-').ignore_then(days_31.clone()).or(days_31))
        .ignored()
        .or(choice((just("04"), just("06"), just("09"), just("11")))
            .then(just('-').ignore_then(days_30.clone()).or(days_30))
            .ignored())
        .or(just("02")
            .then(just('-').ignore_then(days_29.clone()).or(days_29))
            .ignored()),
    )
}

pub(crate) fn year_month<'a>()
-> impl Parser<'a, &'a str, (u32, u32), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        text::digits(10).exactly(4).to_slice(),
        one_of("+-").then(text::digits(10).exactly(6)).to_slice(),
    ))
    .from_str::<u32>()
    .unwrapped()
    .then(
        just('-')
            .ignore_then(
                just('0')
                    .then(one_of('0'..='9'))
                    .to_slice()
                    .or(just('1').then(one_of('0'..='2')).to_slice())
                    .from_str::<u32>()
                    .unwrapped(),
            )
            .or(just('0')
                .then(one_of('0'..='9'))
                .to_slice()
                .or(just('1').then(one_of('0'..='2')).to_slice())
                .from_str::<u32>()
                .unwrapped()),
    )
}

pub(crate) fn date<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    let days_28 = one_of('0'..='1')
        .then(one_of('0'..='9'))
        .ignored()
        .or(just('2').then(one_of('0'..='8')).ignored());

    let day = choice((
        days_28.ignored(),
        just("29")
            .contextual()
            .configure(|_, ctx: &(u32, u32)| {
                ctx.1 != 2 || (ctx.0 % 4 == 0 && (ctx.0 % 100 != 0 || ctx.0 % 400 == 0))
            })
            .ignored(),
        just("30")
            .contextual()
            .configure(|_, ctx: &(u32, u32)| ctx.1 != 2)
            .ignored(),
        just("31")
            .contextual()
            .configure(|_, ctx: &(u32, u32)| matches!(ctx.1, 1 | 3 | 5 | 7 | 8 | 10 | 12))
            .ignored(),
    ));

    year_month()
        .then_ignore(just('-'))
        .ignore_with_ctx(day.clone())
        .or(year_month().ignore_with_ctx(day))
}

pub(crate) fn date_time<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    date().then(one_of("tT ")).then(time()).ignored()
}

pub(crate) fn timezone<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    let timezone_part = choice((ascii_alphabetic(), one_of("._")))
        .then(choice((ascii_alphanumeric(), one_of("._-+"))).repeated())
        .and_is(choice((just('.').ignored(), just("..").ignored())).not());

    just('!')
        .or_not()
        .then(
            timezone_part
                .clone()
                .then(just('/').then(timezone_part).repeated())
                .ignored()
                .or(time_num_offset()),
        )
        .delimited_by(just('['), just(']'))
        .ignored()
}

pub(crate) fn suffix_tag<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    let suffix_key = choice((one_of('a'..='z'), just('_')))
        .then(choice((one_of('a'..='z'), one_of('0'..='9'), one_of("_-"))).repeated());
    let suffix_value = ascii_alphanumeric().repeated().at_least(1);

    just('!')
        .or_not()
        .then(
            suffix_key.then(just('=')).then(
                suffix_value
                    .clone()
                    .then(just('-').then(suffix_value).repeated()),
            ),
        )
        .delimited_by(just('['), just(']'))
        .ignored()
}
