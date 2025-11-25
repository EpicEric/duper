use std::{borrow::Cow, cmp::Ordering, str::FromStr};

use duper::{DuperInner, DuperValue};
use temporal_rs::{
    Duration, Instant, PlainDate, PlainDateTime, PlainMonthDay, PlainTime, PlainYearMonth,
    ZonedDateTime,
    options::{Disambiguation, OffsetDisambiguation},
};
use tinyvec::TinyVec;

pub(crate) trait Filter {
    fn apply<'v>(&self, value: &'v DuperValue<'_>) -> bool;
}

impl<'filter> Default for &'filter dyn Filter {
    fn default() -> Self {
        &FalseFilter
    }
}

// Branchless filters

pub(crate) struct FalseFilter;

impl Filter for FalseFilter {
    fn apply<'v>(&self, _: &'v DuperValue<'v>) -> bool {
        false
    }
}

pub(crate) struct TrueFilter;

impl Filter for TrueFilter {
    fn apply<'v>(&self, _: &'v DuperValue<'v>) -> bool {
        true
    }
}

// Container filters

#[derive(Default)]
pub(crate) struct AndFilter<'filter>(TinyVec<[&'filter dyn Filter; 4]>);

impl<'filter> Filter for AndFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        self.0.iter().all(|inner| inner.apply(value))
    }
}

impl<'filter> FromIterator<&'filter dyn Filter> for AndFilter<'filter> {
    fn from_iter<T: IntoIterator<Item = &'filter dyn Filter>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> chumsky::container::Container<&'a dyn Filter> for AndFilter<'a> {
    fn push(&mut self, item: &'a dyn Filter) {
        self.0.push(item);
    }
}

#[derive(Default)]
pub(crate) struct OrFilter<'filter>(TinyVec<[&'filter dyn Filter; 4]>);

impl<'filter> Filter for OrFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        self.0.iter().any(|inner| inner.apply(value))
    }
}

impl<'filter> FromIterator<&'filter dyn Filter> for OrFilter<'filter> {
    fn from_iter<T: IntoIterator<Item = &'filter dyn Filter>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> chumsky::container::Container<&'a dyn Filter> for OrFilter<'a> {
    fn push(&mut self, item: &'a dyn Filter) {
        self.0.push(item);
    }
}

pub(crate) struct NotFilter<'filter>(&'filter dyn Filter);

impl<'filter> Filter for NotFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        !self.0.apply(value)
    }
}

// Access filters

pub(crate) struct FieldAccessFilter<'filter>(Cow<'filter, str>, &'filter dyn Filter);

impl<'filter> Filter for FieldAccessFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'_>) -> bool {
        if let DuperInner::Object(object) = &value.inner {
            object
                .iter()
                .find(|(key, _)| key.as_ref() == self.0)
                .is_some_and(|(_, value)| self.1.apply(value))
        } else {
            false
        }
    }
}

pub(crate) struct IndexAccessFilter<'filter>(usize, &'filter dyn Filter);

impl<'filter> Filter for IndexAccessFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'_>) -> bool {
        if let DuperInner::Array(array) = &value.inner {
            array.get(self.0).is_some_and(|value| self.1.apply(value))
        } else {
            false
        }
    }
}

pub(crate) struct ReverseIndexAccessFilter<'filter>(usize, &'filter dyn Filter);

impl<'filter> Filter for ReverseIndexAccessFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'_>) -> bool {
        if let DuperInner::Array(array) = &value.inner {
            array
                .len()
                .checked_sub(self.0)
                .is_some_and(|i| array.get(i).is_some_and(|value| self.1.apply(value)))
        } else {
            false
        }
    }
}

pub(crate) struct AnyAccessFilter<'filter>(&'filter dyn Filter);

impl<'filter> Filter for AnyAccessFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'_>) -> bool {
        if let DuperInner::Array(array) = &value.inner {
            array.iter().any(|value| self.0.apply(value))
        } else {
            false
        }
    }
}

// Leaf filters

enum EqValue<'filter> {
    Len(usize),
    Tuple(TinyVec<[&'filter EqFilter<'filter>; 4]>),
    String(Cow<'filter, str>),
    Bytes(Cow<'filter, [u8]>),
    TemporalInstant(Instant),
    TemporalZonedDateTime(ZonedDateTime),
    TemporalPlainDate(PlainDate),
    TemporalPlainTime(PlainTime),
    TemporalPlainDateTime(PlainDateTime),
    TemporalPlainYearMonth(PlainYearMonth),
    TemporalPlainMonthDay(PlainMonthDay),
    TemporalDuration(Duration),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl<'filter> Default for &'filter EqFilter<'filter> {
    fn default() -> Self {
        &EqFilter(EqValue::Null)
    }
}

pub(crate) struct EqFilter<'filter>(EqValue<'filter>);

impl<'filter> Filter for EqFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match (&self.0, &value.inner) {
            (EqValue::Len(this), DuperInner::Object(that)) => *this == that.len(),
            (EqValue::Len(this), DuperInner::Array(that)) => *this == that.len(),
            (EqValue::Len(this), DuperInner::String(that)) => *this == that.as_ref().len(),
            (EqValue::Len(this), DuperInner::Bytes(that)) => *this == that.as_ref().len(),
            (EqValue::Tuple(this), DuperInner::Tuple(that)) => {
                if this.len() == that.len() {
                    this.iter()
                        .zip(that.iter())
                        .all(|(this, that)| this.apply(that))
                } else {
                    false
                }
            }
            (EqValue::String(this), DuperInner::String(that)) => this.as_ref() == that.as_ref(),
            (EqValue::Bytes(this), DuperInner::Bytes(that)) => this.as_ref() == that.as_ref(),
            (EqValue::TemporalInstant(this), DuperInner::Temporal(that)) => {
                Instant::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalZonedDateTime(this), DuperInner::Temporal(that)) => {
                ZonedDateTime::from_utf8(
                    that.as_ref().as_bytes(),
                    Disambiguation::Compatible,
                    OffsetDisambiguation::Prefer,
                )
                .is_ok_and(|that| this.compare_instant(&that).is_eq())
            }
            (EqValue::TemporalPlainDate(this), DuperInner::Temporal(that)) => {
                PlainDate::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainTime(this), DuperInner::Temporal(that)) => {
                PlainTime::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainDateTime(this), DuperInner::Temporal(that)) => {
                PlainDateTime::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainYearMonth(this), DuperInner::Temporal(that)) => {
                PlainYearMonth::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainMonthDay(this), DuperInner::Temporal(that)) => {
                PlainMonthDay::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalDuration(this), DuperInner::Temporal(that)) => {
                Duration::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::Integer(this), DuperInner::Integer(that)) => this == that,
            (EqValue::Float(this), DuperInner::Float(that)) => this == that,
            (EqValue::Boolean(this), DuperInner::Boolean(that)) => this == that,
            (EqValue::Null, DuperInner::Null) => true,
            _ => false,
        }
    }
}

pub(crate) struct NeFilter<'filter>(EqValue<'filter>);

impl<'filter> Filter for NeFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match (&self.0, &value.inner) {
            (EqValue::Len(this), DuperInner::Object(that)) => *this != that.len(),
            (EqValue::Len(this), DuperInner::Array(that)) => *this != that.len(),
            (EqValue::Len(this), DuperInner::String(that)) => *this != that.as_ref().len(),
            (EqValue::Len(this), DuperInner::Bytes(that)) => *this != that.as_ref().len(),
            (EqValue::Tuple(this), DuperInner::Tuple(that)) => {
                if this.len() == that.len() {
                    this.iter()
                        .zip(that.iter())
                        .any(|(this, that)| !this.apply(that))
                } else {
                    true
                }
            }
            (EqValue::String(this), DuperInner::String(that)) => this.as_ref() != that.as_ref(),
            (EqValue::Bytes(this), DuperInner::Bytes(that)) => this.as_ref() != that.as_ref(),
            (EqValue::TemporalInstant(this), DuperInner::Temporal(that)) => {
                Instant::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalZonedDateTime(this), DuperInner::Temporal(that)) => {
                ZonedDateTime::from_utf8(
                    that.as_ref().as_bytes(),
                    Disambiguation::Compatible,
                    OffsetDisambiguation::Prefer,
                )
                .ok()
                .is_none_or(|that| this.compare_instant(&that).is_ne())
            }
            (EqValue::TemporalPlainDate(this), DuperInner::Temporal(that)) => {
                PlainDate::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainTime(this), DuperInner::Temporal(that)) => {
                PlainTime::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainDateTime(this), DuperInner::Temporal(that)) => {
                PlainDateTime::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainYearMonth(this), DuperInner::Temporal(that)) => {
                PlainYearMonth::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainMonthDay(this), DuperInner::Temporal(that)) => {
                PlainMonthDay::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalDuration(this), DuperInner::Temporal(that)) => {
                Duration::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::Integer(this), DuperInner::Integer(that)) => this != that,
            (EqValue::Float(this), DuperInner::Float(that)) => this != that,
            (EqValue::Boolean(this), DuperInner::Boolean(that)) => this != that,
            (EqValue::Null, DuperInner::Null) => false,
            _ => true,
        }
    }
}

enum CmpValue {
    Len(usize),
    TemporalInstant(Instant),
    TemporalZonedDateTime(ZonedDateTime),
    TemporalPlainDate(PlainDate),
    TemporalPlainTime(PlainTime),
    TemporalPlainDateTime(PlainDateTime),
    TemporalPlainYearMonth(PlainYearMonth),
    TemporalPlainMonthDay(PlainMonthDay),
    TemporalDuration(Duration),
    Integer(i64),
    Float(f64),
}

macro_rules! cmp_filter {
    (
        $filter:ident,
        $ord:pat
    ) => {
        struct $filter(CmpValue);

        impl Filter for $filter {
            fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
                match (&self.0, &value.inner) {
                    (CmpValue::Len(this), DuperInner::Object(that)) => {
                        matches!(this.cmp(&that.len()), $ord)
                    }
                    (CmpValue::Len(this), DuperInner::Array(that)) => {
                        matches!(this.cmp(&that.len()), $ord)
                    }
                    (CmpValue::Len(this), DuperInner::String(that)) => {
                        matches!(this.cmp(&that.as_ref().len()), $ord)
                    }
                    (CmpValue::Len(this), DuperInner::Bytes(that)) => {
                        matches!(this.cmp(&that.as_ref().len()), $ord)
                    }
                    (CmpValue::TemporalInstant(this), DuperInner::Temporal(that)) => {
                        Instant::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(this.cmp(&that), $ord))
                    }
                    (CmpValue::TemporalZonedDateTime(this), DuperInner::Temporal(that)) => {
                        ZonedDateTime::from_utf8(
                            that.as_ref().as_bytes(),
                            Disambiguation::Compatible,
                            OffsetDisambiguation::Prefer,
                        )
                        .is_ok_and(|that| matches!(this.compare_instant(&that), $ord))
                    }
                    (CmpValue::TemporalPlainDate(this), DuperInner::Temporal(that)) => {
                        PlainDate::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(this.compare_iso(&that), $ord))
                    }
                    (CmpValue::TemporalPlainTime(this), DuperInner::Temporal(that)) => {
                        PlainTime::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(this.cmp(&that), $ord))
                    }
                    (CmpValue::TemporalPlainDateTime(this), DuperInner::Temporal(that)) => {
                        PlainDateTime::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(this.compare_iso(&that), $ord))
                    }
                    (CmpValue::TemporalPlainYearMonth(this), DuperInner::Temporal(that)) => {
                        PlainYearMonth::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(this.compare_iso(&that), $ord))
                    }
                    (CmpValue::TemporalDuration(this), DuperInner::Temporal(that)) => {
                        Duration::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(this.partial_cmp(&that), Some($ord)))
                    }
                    (CmpValue::Integer(this), DuperInner::Integer(that)) => {
                        matches!(this.cmp(that), $ord)
                    }
                    (CmpValue::Float(this), DuperInner::Float(that)) => {
                        matches!(this.partial_cmp(that), Some($ord))
                    }
                    _ => false,
                }
            }
        }
    };
}

cmp_filter!(GeFilter, Ordering::Greater | Ordering::Equal);
cmp_filter!(GtFilter, Ordering::Greater);
cmp_filter!(LeFilter, Ordering::Less | Ordering::Equal);
cmp_filter!(LtFilter, Ordering::Less);

enum IsFilter {
    Object,
    Array,
    Tuple,
    String,
    Bytes,
    TemporalInstant,
    TemporalZonedDateTime,
    TemporalPlainDate,
    TemporalPlainTime,
    TemporalPlainDateTime,
    TemporalPlainYearMonth,
    TemporalPlainMonthDay,
    TemporalDuration,
    TemporalUnspecified,
    Integer,
    Float,
    Number,
    Boolean,
    Null,
}

impl Filter for IsFilter {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match (&self, &value.inner) {
            (IsFilter::Object, DuperInner::Object(_)) => true,
            (IsFilter::Array, DuperInner::Array(_)) => true,
            (IsFilter::Tuple, DuperInner::Tuple(_)) => true,
            (IsFilter::String, DuperInner::String(_)) => true,
            (IsFilter::Bytes, DuperInner::Bytes(_)) => true,
            (IsFilter::TemporalInstant, DuperInner::Temporal(that)) => {
                Instant::from_str(that.as_ref()).is_ok()
            }
            (IsFilter::TemporalZonedDateTime, DuperInner::Temporal(that)) => {
                ZonedDateTime::from_utf8(
                    that.as_ref().as_bytes(),
                    Disambiguation::Compatible,
                    OffsetDisambiguation::Prefer,
                )
                .is_ok()
            }
            (IsFilter::TemporalPlainDate, DuperInner::Temporal(that)) => {
                PlainDate::from_str(that.as_ref()).is_ok()
            }
            (IsFilter::TemporalPlainTime, DuperInner::Temporal(that)) => {
                PlainTime::from_str(that.as_ref()).is_ok()
            }
            (IsFilter::TemporalPlainDateTime, DuperInner::Temporal(that)) => {
                PlainDateTime::from_str(that.as_ref()).is_ok()
            }
            (IsFilter::TemporalPlainYearMonth, DuperInner::Temporal(that)) => {
                PlainYearMonth::from_str(that.as_ref()).is_ok()
            }
            (IsFilter::TemporalPlainMonthDay, DuperInner::Temporal(that)) => {
                PlainMonthDay::from_str(that.as_ref()).is_ok()
            }
            (IsFilter::TemporalDuration, DuperInner::Temporal(that)) => {
                Duration::from_str(that.as_ref()).is_ok()
            }
            (IsFilter::TemporalUnspecified, DuperInner::Temporal(_)) => true,
            (IsFilter::Integer, DuperInner::Integer(_)) => true,
            (IsFilter::Float, DuperInner::Float(_)) => true,
            (IsFilter::Number, DuperInner::Integer(_) | DuperInner::Float(_)) => true,
            (IsFilter::Boolean, DuperInner::Boolean(_)) => true,
            (IsFilter::Null, DuperInner::Null) => true,
            _ => false,
        }
    }
}

pub(crate) struct RegexFilter(regex::Regex);

impl Filter for RegexFilter {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match &value.inner {
            DuperInner::String(string) => self.0.find(string.as_ref()).is_some(),
            DuperInner::Temporal(temporal) => self.0.find(temporal.as_ref()).is_some(),
            _ => false,
        }
    }
}

pub(crate) struct RegexBytesFilter(regex::bytes::Regex);

impl Filter for RegexBytesFilter {
    fn apply<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match &value.inner {
            DuperInner::Bytes(bytes) => self.0.find(bytes.as_ref()).is_some(),
            _ => false,
        }
    }
}

pub(crate) struct FieldExistsFilter<'filter>(Cow<'filter, str>);

impl<'filter> Filter for FieldExistsFilter<'filter> {
    fn apply<'v>(&self, value: &'v DuperValue<'_>) -> bool {
        if let DuperInner::Object(object) = &value.inner {
            object
                .iter()
                .find(|(key, _)| key.as_ref() == self.0)
                .is_some()
        } else {
            false
        }
    }
}
