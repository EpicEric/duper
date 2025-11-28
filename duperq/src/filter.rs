use std::{cmp::Ordering, fmt::Display, str::FromStr};

use duper::{DuperInner, DuperValue};
use temporal_rs::{
    Duration, Instant, PlainDate, PlainDateTime, PlainMonthDay, PlainTime, PlainYearMonth,
    TemporalError, ZonedDateTime,
    options::{Disambiguation, OffsetDisambiguation},
};

use crate::accessor::DuperAccessor;

pub(crate) trait DuperFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'_>) -> bool;
}

// Branchless filters

pub(crate) struct TrueFilter;

impl DuperFilter for TrueFilter {
    fn filter<'v>(&self, _: &'v DuperValue<'v>) -> bool {
        true
    }
}

// Container filters

#[derive(Default)]
pub(crate) struct AndFilter(pub(crate) Vec<Box<dyn DuperFilter>>);

impl DuperFilter for AndFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        self.0.iter().all(|inner| inner.filter(value))
    }
}

impl FromIterator<Box<dyn DuperFilter>> for AndFilter {
    fn from_iter<T: IntoIterator<Item = Box<dyn DuperFilter>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl chumsky::container::Container<Box<dyn DuperFilter>> for AndFilter {
    fn push(&mut self, item: Box<dyn DuperFilter>) {
        self.0.push(item);
    }
}

#[derive(Default)]
pub(crate) struct OrFilter(pub(crate) Vec<Box<dyn DuperFilter>>);

impl DuperFilter for OrFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        self.0.iter().any(|inner| inner.filter(value))
    }
}

impl FromIterator<Box<dyn DuperFilter>> for OrFilter {
    fn from_iter<T: IntoIterator<Item = Box<dyn DuperFilter>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> chumsky::container::Container<Box<dyn DuperFilter>> for OrFilter {
    fn push(&mut self, item: Box<dyn DuperFilter>) {
        self.0.push(item);
    }
}

pub(crate) struct NotFilter(pub(crate) Box<dyn DuperFilter>);

impl DuperFilter for NotFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        !self.0.filter(value)
    }
}

pub(crate) struct AccessorFilter {
    pub(crate) filter: Box<dyn DuperFilter>,
    pub(crate) accessor: Box<dyn DuperAccessor>,
}

impl DuperFilter for AccessorFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'_>) -> bool {
        self.accessor
            .access(value)
            .any(|inner| self.filter.filter(inner))
    }
}

// Leaf filters

#[derive(Debug, Clone)]
pub(crate) enum TryFromDuperValueError {
    InvalidType(&'static str),
    InvalidSize(i64),
    UnspecifiedTemporal,
    TemporalError(TemporalError),
}

impl Display for TryFromDuperValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromDuperValueError::InvalidType(typ) => {
                f.write_fmt(format_args!("invalid type {typ}"))
            }
            TryFromDuperValueError::InvalidSize(size) => {
                f.write_fmt(format_args!("invalid size {size}"))
            }
            TryFromDuperValueError::UnspecifiedTemporal => f.write_str("unspecified Temporal type"),
            TryFromDuperValueError::TemporalError(inner) => inner.fmt(f),
        }
    }
}

impl From<TemporalError> for TryFromDuperValueError {
    fn from(value: TemporalError) -> Self {
        TryFromDuperValueError::TemporalError(value)
    }
}

pub(crate) enum EqValue {
    Identifier(Option<String>),
    Len(usize),
    Tuple(Vec<EqFilter>),
    String(String),
    Bytes(Vec<u8>),
    TemporalInstant(Instant),
    TemporalZonedDateTime(ZonedDateTime),
    TemporalPlainDate(PlainDate),
    TemporalPlainTime(PlainTime),
    TemporalPlainDateTime(PlainDateTime),
    TemporalPlainYearMonth(PlainYearMonth),
    TemporalPlainMonthDay(PlainMonthDay),
    TemporalDuration(Duration),
    Integer(i64),
    Float(f64, Option<f64>),
    Boolean(bool),
    Null,
}

impl EqValue {
    pub(crate) fn try_from_duper(
        value: DuperValue<'_>,
        epsilon: Option<f64>,
    ) -> Result<Self, TryFromDuperValueError> {
        match value.inner {
            DuperInner::Object(_) => Err(TryFromDuperValueError::InvalidType("Object")),
            DuperInner::Array(_) => Err(TryFromDuperValueError::InvalidType("Array")),
            DuperInner::Tuple(tuple) => {
                let vec: Result<Vec<_>, _> = tuple
                    .into_inner()
                    .into_iter()
                    .map(|value| {
                        EqValue::try_from_duper(value, epsilon).map(|value| EqFilter(value))
                    })
                    .collect();
                Ok(EqValue::Tuple(vec?))
            }
            DuperInner::String(string) => Ok(EqValue::String(string.into_inner().into_owned())),
            DuperInner::Bytes(bytes) => Ok(EqValue::Bytes(bytes.into_inner().into_owned())),
            DuperInner::Temporal(temporal) => match value.identifier {
                Some(identifier) if identifier.as_ref() == "Instant" => Ok(
                    EqValue::TemporalInstant(Instant::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "ZonedDateTime" => {
                    Ok(EqValue::TemporalZonedDateTime(ZonedDateTime::from_utf8(
                        temporal.as_ref().as_bytes(),
                        Disambiguation::Compatible,
                        OffsetDisambiguation::Prefer,
                    )?))
                }
                Some(identifier) if identifier.as_ref() == "PlainDate" => Ok(
                    EqValue::TemporalPlainDate(PlainDate::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainTime" => Ok(
                    EqValue::TemporalPlainTime(PlainTime::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainDateTime" => Ok(
                    EqValue::TemporalPlainDateTime(PlainDateTime::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainYearMonth" => Ok(
                    EqValue::TemporalPlainYearMonth(PlainYearMonth::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainMonthDay" => Ok(
                    EqValue::TemporalPlainMonthDay(PlainMonthDay::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "Duration" => Ok(
                    EqValue::TemporalDuration(Duration::from_str(temporal.as_ref())?),
                ),
                Some(_) | None => Err(TryFromDuperValueError::UnspecifiedTemporal),
            },
            DuperInner::Integer(integer) => Ok(EqValue::Integer(integer)),
            DuperInner::Float(float) => Ok(EqValue::Float(float, epsilon)),
            DuperInner::Boolean(boolean) => Ok(EqValue::Boolean(boolean)),
            DuperInner::Null => Ok(EqValue::Null),
        }
    }
}

pub(crate) struct EqFilter(pub(crate) EqValue);

impl DuperFilter for EqFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match (&self.0, &value.inner) {
            (EqValue::Identifier(this), _) => match this {
                Some(this) => value
                    .identifier
                    .as_ref()
                    .is_some_and(|that| this == that.as_ref()),
                None => value.identifier.is_none(),
            },
            (EqValue::Len(this), DuperInner::Object(that)) => *this == that.len(),
            (EqValue::Len(this), DuperInner::Array(that)) => *this == that.len(),
            (EqValue::Len(this), DuperInner::String(that)) => *this == that.as_ref().len(),
            (EqValue::Len(this), DuperInner::Bytes(that)) => *this == that.as_ref().len(),
            (EqValue::Tuple(this), DuperInner::Tuple(that)) => {
                if this.len() == that.len() {
                    this.iter()
                        .zip(that.iter())
                        .all(|(this, that)| this.filter(that))
                } else {
                    false
                }
            }
            (EqValue::String(this), DuperInner::String(that)) => this == that.as_ref(),
            (EqValue::Bytes(this), DuperInner::Bytes(that)) => this == that.as_ref(),
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
            (EqValue::Float(this, epsilon), DuperInner::Float(that)) => {
                (this - that).abs() <= epsilon.unwrap_or(0.0).abs()
            }
            (EqValue::Boolean(this), DuperInner::Boolean(that)) => this == that,
            (EqValue::Null, DuperInner::Null) => true,
            _ => false,
        }
    }
}

pub(crate) struct NeFilter(pub(crate) EqValue);

impl DuperFilter for NeFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match (&self.0, &value.inner) {
            (EqValue::Identifier(this), _) => match this {
                Some(this) => value
                    .identifier
                    .as_ref()
                    .is_none_or(|that| this != that.as_ref()),
                None => value.identifier.is_some(),
            },
            (EqValue::Len(this), DuperInner::Object(that)) => *this != that.len(),
            (EqValue::Len(this), DuperInner::Array(that)) => *this != that.len(),
            (EqValue::Len(this), DuperInner::String(that)) => *this != that.as_ref().len(),
            (EqValue::Len(this), DuperInner::Bytes(that)) => *this != that.as_ref().len(),
            (EqValue::Tuple(this), DuperInner::Tuple(that)) => {
                if this.len() == that.len() {
                    this.iter()
                        .zip(that.iter())
                        .any(|(this, that)| !this.filter(that))
                } else {
                    true
                }
            }
            (EqValue::String(this), DuperInner::String(that)) => this != that.as_ref(),
            (EqValue::Bytes(this), DuperInner::Bytes(that)) => this != that.as_ref(),
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
            (EqValue::Float(this, epsilon), DuperInner::Float(that)) => {
                (this - that).abs() > epsilon.unwrap_or(0.0).abs()
            }
            (EqValue::Boolean(this), DuperInner::Boolean(that)) => this != that,
            (EqValue::Null, DuperInner::Null) => false,
            _ => true,
        }
    }
}

pub(crate) enum CmpValue {
    Len(usize),
    TemporalInstant(Instant),
    TemporalZonedDateTime(ZonedDateTime),
    TemporalPlainDate(PlainDate),
    TemporalPlainTime(PlainTime),
    TemporalPlainDateTime(PlainDateTime),
    TemporalPlainYearMonth(PlainYearMonth),
    TemporalDuration(Duration),
    Integer(i64),
    Float(f64),
}

impl TryFrom<DuperValue<'_>> for CmpValue {
    type Error = TryFromDuperValueError;

    fn try_from(value: DuperValue<'_>) -> Result<Self, Self::Error> {
        match value.inner {
            DuperInner::Object(_) => Err(TryFromDuperValueError::InvalidType("Object")),
            DuperInner::Array(_) => Err(TryFromDuperValueError::InvalidType("Array")),
            DuperInner::Tuple(_) => Err(TryFromDuperValueError::InvalidType("Tuple")),
            DuperInner::String(_) => Err(TryFromDuperValueError::InvalidType("String")),
            DuperInner::Bytes(_) => Err(TryFromDuperValueError::InvalidType("Bytes")),
            DuperInner::Temporal(temporal) => match value.identifier {
                Some(identifier) if identifier.as_ref() == "Instant" => Ok(
                    CmpValue::TemporalInstant(Instant::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "ZonedDateTime" => {
                    Ok(CmpValue::TemporalZonedDateTime(ZonedDateTime::from_utf8(
                        temporal.as_ref().as_bytes(),
                        Disambiguation::Compatible,
                        OffsetDisambiguation::Prefer,
                    )?))
                }
                Some(identifier) if identifier.as_ref() == "PlainDate" => Ok(
                    CmpValue::TemporalPlainDate(PlainDate::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainTime" => Ok(
                    CmpValue::TemporalPlainTime(PlainTime::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainDateTime" => Ok(
                    CmpValue::TemporalPlainDateTime(PlainDateTime::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainYearMonth" => Ok(
                    CmpValue::TemporalPlainYearMonth(PlainYearMonth::from_str(temporal.as_ref())?),
                ),
                Some(identifier) if identifier.as_ref() == "PlainMonthDay" => {
                    Err(TryFromDuperValueError::InvalidType("PlainMonthDay"))
                }
                Some(identifier) if identifier.as_ref() == "Duration" => Ok(
                    CmpValue::TemporalDuration(Duration::from_str(temporal.as_ref())?),
                ),
                Some(_) | None => Err(TryFromDuperValueError::UnspecifiedTemporal),
            },
            DuperInner::Integer(integer) => Ok(CmpValue::Integer(integer)),
            DuperInner::Float(float) => Ok(CmpValue::Float(float)),
            DuperInner::Boolean(_) => Err(TryFromDuperValueError::InvalidType("Boolean")),
            DuperInner::Null => Err(TryFromDuperValueError::InvalidType("Null")),
        }
    }
}

macro_rules! cmp_filter {
    (
        $filter:ident,
        $ord:pat
    ) => {
        pub(crate) struct $filter(pub(crate) CmpValue);

        impl DuperFilter for $filter {
            fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
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

pub(crate) enum IsFilter {
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

impl DuperFilter for IsFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
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

pub(crate) struct RegexFilter(pub(crate) regex::bytes::Regex);

impl DuperFilter for RegexFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        match &value.inner {
            DuperInner::String(string) => self.0.find(string.as_ref().as_bytes()).is_some(),
            DuperInner::Bytes(bytes) => self.0.find(bytes.as_ref()).is_some(),
            DuperInner::Temporal(temporal) => self.0.find(temporal.as_ref().as_bytes()).is_some(),
            _ => false,
        }
    }
}

pub(crate) struct RegexIdentifierFilter(pub(crate) regex::Regex);

impl DuperFilter for RegexIdentifierFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'v>) -> bool {
        value
            .identifier
            .as_ref()
            .is_some_and(|identifier| self.0.find(identifier.as_ref()).is_some())
    }
}

pub(crate) struct IsTruthyFilter;

impl DuperFilter for IsTruthyFilter {
    fn filter<'v>(&self, value: &'v DuperValue<'_>) -> bool {
        match &value.inner {
            DuperInner::Object(object) => !object.is_empty(),
            DuperInner::Array(array) => !array.is_empty(),
            DuperInner::Tuple(tuple) => !tuple.is_empty(),
            DuperInner::String(string) => !string.is_empty(),
            DuperInner::Bytes(bytes) => !bytes.is_empty(),
            DuperInner::Temporal(_) => true,
            DuperInner::Integer(integer) => *integer != 0,
            DuperInner::Float(float) => *float != 0.0,
            DuperInner::Boolean(boolean) => *boolean,
            DuperInner::Null => false,
        }
    }
}
