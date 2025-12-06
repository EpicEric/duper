//! Functions to help with parse-based validation of Duper-encoded values.

use chumsky::Parser;

use crate::parser;

/// Check if a string parses into a valid integer.
pub fn is_valid_integer(input: &str) -> bool {
    !parser::integer().check(input).has_errors()
}

/// Check if a string parses into a valid float.
pub fn is_valid_float(input: &str) -> bool {
    !parser::float().check(input).has_errors()
}

/// Check if a string parses into a valid Temporal Instant.
pub fn is_valid_instant(input: &str) -> bool {
    !parser::temporal::instant().check(input).has_errors()
}

/// Check if a string parses into a valid Temporal ZonedDateTime.
pub fn is_valid_zoned_date_time(input: &str) -> bool {
    !parser::temporal::zoned_date_time()
        .check(input)
        .has_errors()
}

/// Check if a string parses into a valid Temporal PlainDate.
pub fn is_valid_plain_date(input: &str) -> bool {
    !parser::temporal::plain_date().check(input).has_errors()
}

/// Check if a string parses into a valid Temporal PlainTime.
pub fn is_valid_plain_time(input: &str) -> bool {
    !parser::temporal::plain_time().check(input).has_errors()
}

/// Check if a string parses into a valid Temporal PlainDateTime.
pub fn is_valid_plain_date_time(input: &str) -> bool {
    !parser::temporal::plain_date_time()
        .check(input)
        .has_errors()
}

/// Check if a string parses into a valid Temporal PlainYearMonth.
pub fn is_valid_plain_year_month(input: &str) -> bool {
    !parser::temporal::plain_year_month()
        .check(input)
        .has_errors()
}

/// Check if a string parses into a valid Temporal PlainMonthDay.
pub fn is_valid_plain_month_day(input: &str) -> bool {
    !parser::temporal::plain_month_day()
        .check(input)
        .has_errors()
}

/// Check if a string parses into a valid Temporal Duration.
pub fn is_valid_duration(input: &str) -> bool {
    !parser::temporal::duration().check(input).has_errors()
}

/// Check if a string parses into a valid unspecified Temporal value.
pub fn is_valid_unspecified_temporal(input: &str) -> bool {
    !parser::temporal::unspecified().check(input).has_errors()
}
