use chumsky::Parser;

use crate::parser;

pub fn is_valid_integer(input: &str) -> bool {
    !parser::integer().parse(input).has_errors()
}

pub fn is_valid_float(input: &str) -> bool {
    !parser::float().parse(input).has_errors()
}

pub fn is_valid_instant(input: &str) -> bool {
    !parser::temporal::instant().parse(input).has_errors()
}

pub fn is_valid_zoned_date_time(input: &str) -> bool {
    !parser::temporal::zoned_date_time()
        .parse(input)
        .has_errors()
}

pub fn is_valid_plain_date(input: &str) -> bool {
    !parser::temporal::plain_date().parse(input).has_errors()
}

pub fn is_valid_plain_time(input: &str) -> bool {
    !parser::temporal::plain_time().parse(input).has_errors()
}

pub fn is_valid_plain_date_time(input: &str) -> bool {
    !parser::temporal::plain_date_time()
        .parse(input)
        .has_errors()
}

pub fn is_valid_plain_year_month(input: &str) -> bool {
    !parser::temporal::plain_year_month()
        .parse(input)
        .has_errors()
}

pub fn is_valid_plain_month_day(input: &str) -> bool {
    !parser::temporal::plain_month_day()
        .parse(input)
        .has_errors()
}

pub fn is_valid_duration(input: &str) -> bool {
    !parser::temporal::duration().parse(input).has_errors()
}

pub fn is_valid_unspecified_temporal(input: &str) -> bool {
    !parser::temporal::unspecified().parse(input).has_errors()
}
