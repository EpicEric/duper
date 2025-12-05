use std::{cmp::Ordering, fmt::Display, str::FromStr};

use duper::{DuperTemporal, DuperValue};
use temporal_rs::{
    Duration, Instant, PlainDate, PlainDateTime, PlainMonthDay, PlainTime, PlainYearMonth,
    TemporalError, ZonedDateTime,
    options::{Disambiguation, OffsetDisambiguation},
};

use crate::{accessor::DuperAccessor, types::DuperType};

pub(crate) trait DuperFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool;
}

// Branchless filters

pub(crate) struct TrueFilter;

impl DuperFilter for TrueFilter {
    fn filter<'v>(&self, _: &DuperValue<'v>) -> bool {
        true
    }
}

// Container filters

#[derive(Default)]
pub(crate) struct AndFilter(pub(crate) Vec<Box<dyn DuperFilter>>);

impl DuperFilter for AndFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
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
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        self.0.iter().any(|inner| inner.filter(value))
    }
}

impl FromIterator<Box<dyn DuperFilter>> for OrFilter {
    fn from_iter<T: IntoIterator<Item = Box<dyn DuperFilter>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl chumsky::container::Container<Box<dyn DuperFilter>> for OrFilter {
    fn push(&mut self, item: Box<dyn DuperFilter>) {
        self.0.push(item);
    }
}

pub(crate) struct NotFilter(pub(crate) Box<dyn DuperFilter>);

impl DuperFilter for NotFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        !self.0.filter(value)
    }
}

pub(crate) struct CastFilter {
    pub(crate) typ: DuperType,
    pub(crate) filter: Box<dyn DuperFilter>,
}

impl DuperFilter for CastFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        self.typ
            .cast(value)
            .as_ref()
            .is_some_and(|value| self.filter.filter(value))
    }
}

pub(crate) struct AccessorFilter {
    pub(crate) filter: Box<dyn DuperFilter>,
    pub(crate) accessor: Box<dyn DuperAccessor>,
}

impl DuperFilter for AccessorFilter {
    fn filter<'v>(&self, value: &DuperValue<'_>) -> bool {
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
        match value {
            DuperValue::Object { .. } => Err(TryFromDuperValueError::InvalidType("Object")),
            DuperValue::Array { .. } => Err(TryFromDuperValueError::InvalidType("Array")),
            DuperValue::Tuple { inner: tuple, .. } => {
                let vec: Result<Vec<_>, _> = tuple
                    .into_iter()
                    .map(|value| EqValue::try_from_duper(value, epsilon).map(EqFilter))
                    .collect();
                Ok(EqValue::Tuple(vec?))
            }
            DuperValue::String { inner: string, .. } => Ok(EqValue::String(string.into_owned())),
            DuperValue::Bytes { inner: bytes, .. } => Ok(EqValue::Bytes(bytes.into_owned())),
            DuperValue::Temporal(temporal) => match temporal {
                DuperTemporal::Instant { inner: temporal } => Ok(EqValue::TemporalInstant(
                    Instant::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::ZonedDateTime { inner: temporal } => {
                    Ok(EqValue::TemporalZonedDateTime(ZonedDateTime::from_utf8(
                        temporal.as_ref().as_bytes(),
                        Disambiguation::Compatible,
                        OffsetDisambiguation::Prefer,
                    )?))
                }
                DuperTemporal::PlainDate { inner: temporal } => Ok(EqValue::TemporalPlainDate(
                    PlainDate::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::PlainTime { inner: temporal } => Ok(EqValue::TemporalPlainTime(
                    PlainTime::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::PlainDateTime { inner: temporal } => Ok(
                    EqValue::TemporalPlainDateTime(PlainDateTime::from_str(temporal.as_ref())?),
                ),
                DuperTemporal::PlainYearMonth { inner: temporal } => Ok(
                    EqValue::TemporalPlainYearMonth(PlainYearMonth::from_str(temporal.as_ref())?),
                ),
                DuperTemporal::PlainMonthDay { inner: temporal } => Ok(
                    EqValue::TemporalPlainMonthDay(PlainMonthDay::from_str(temporal.as_ref())?),
                ),
                DuperTemporal::Duration { inner: temporal } => Ok(EqValue::TemporalDuration(
                    Duration::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::Unspecified { .. } => {
                    Err(TryFromDuperValueError::UnspecifiedTemporal)
                }
            },
            DuperValue::Integer { inner: integer, .. } => Ok(EqValue::Integer(integer)),
            DuperValue::Float { inner: float, .. } => Ok(EqValue::Float(float, epsilon)),
            DuperValue::Boolean { inner: boolean, .. } => Ok(EqValue::Boolean(boolean)),
            DuperValue::Null { .. } => Ok(EqValue::Null),
        }
    }
}

pub(crate) struct EqFilter(pub(crate) EqValue);

impl DuperFilter for EqFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        match (&self.0, value) {
            (EqValue::Identifier(this), _) => match this {
                Some(this) => value
                    .identifier()
                    .as_ref()
                    .is_some_and(|that| this == that.as_ref()),
                None => value.identifier().is_none(),
            },
            (EqValue::Len(this), DuperValue::Object { inner: that, .. }) => *this == that.len(),
            (EqValue::Len(this), DuperValue::Array { inner: that, .. }) => *this == that.len(),
            (EqValue::Len(this), DuperValue::Tuple { inner: that, .. }) => *this == that.len(),
            (EqValue::Len(this), DuperValue::String { inner: that, .. }) => {
                *this == that.as_ref().len()
            }
            (EqValue::Len(this), DuperValue::Bytes { inner: that, .. }) => {
                *this == that.as_ref().len()
            }
            (EqValue::Tuple(this), DuperValue::Tuple { inner: that, .. }) => {
                if this.len() == that.len() {
                    this.iter()
                        .zip(that.iter())
                        .all(|(this, that)| this.filter(that))
                } else {
                    false
                }
            }
            (EqValue::String(this), DuperValue::String { inner: that, .. }) => {
                this == that.as_ref()
            }
            (EqValue::Bytes(this), DuperValue::Bytes { inner: that, .. }) => this == that.as_ref(),
            (EqValue::TemporalInstant(this), DuperValue::Temporal(that)) => {
                Instant::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalZonedDateTime(this), DuperValue::Temporal(that)) => {
                ZonedDateTime::from_utf8(
                    that.as_ref().as_bytes(),
                    Disambiguation::Compatible,
                    OffsetDisambiguation::Prefer,
                )
                .is_ok_and(|that| this.compare_instant(&that).is_eq())
            }
            (EqValue::TemporalPlainDate(this), DuperValue::Temporal(that)) => {
                PlainDate::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainTime(this), DuperValue::Temporal(that)) => {
                PlainTime::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainDateTime(this), DuperValue::Temporal(that)) => {
                PlainDateTime::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainYearMonth(this), DuperValue::Temporal(that)) => {
                PlainYearMonth::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalPlainMonthDay(this), DuperValue::Temporal(that)) => {
                PlainMonthDay::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::TemporalDuration(this), DuperValue::Temporal(that)) => {
                Duration::from_str(that.as_ref()).is_ok_and(|that| *this == that)
            }
            (EqValue::Integer(this), DuperValue::Integer { inner: that, .. }) => this == that,
            (EqValue::Float(this, epsilon), DuperValue::Float { inner: that, .. }) => {
                (this - that).abs() <= epsilon.unwrap_or(0.0).abs()
            }
            (EqValue::Integer(this), DuperValue::Float { inner: that, .. }) => {
                *this == *that as i64
            }
            (EqValue::Float(this, epsilon), DuperValue::Integer { inner: that, .. }) => {
                (this - *that as f64).abs() <= epsilon.unwrap_or(0.0).abs()
            }
            (EqValue::Boolean(this), DuperValue::Boolean { inner: that, .. }) => this == that,
            (EqValue::Null, DuperValue::Null { .. }) => true,
            _ => false,
        }
    }
}

pub(crate) struct NeFilter(pub(crate) EqValue);

impl DuperFilter for NeFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        match (&self.0, value) {
            (EqValue::Identifier(this), _) => match this {
                Some(this) => value
                    .identifier()
                    .as_ref()
                    .is_none_or(|that| this != that.as_ref()),
                None => value.identifier().is_some(),
            },
            (EqValue::Len(this), DuperValue::Object { inner: that, .. }) => *this != that.len(),
            (EqValue::Len(this), DuperValue::Array { inner: that, .. }) => *this != that.len(),
            (EqValue::Len(this), DuperValue::String { inner: that, .. }) => {
                *this != that.as_ref().len()
            }
            (EqValue::Len(this), DuperValue::Bytes { inner: that, .. }) => {
                *this != that.as_ref().len()
            }
            (EqValue::Tuple(this), DuperValue::Tuple { inner: that, .. }) => {
                if this.len() == that.len() {
                    this.iter()
                        .zip(that.iter())
                        .any(|(this, that)| !this.filter(that))
                } else {
                    true
                }
            }
            (EqValue::String(this), DuperValue::String { inner: that, .. }) => {
                this != that.as_ref()
            }
            (EqValue::Bytes(this), DuperValue::Bytes { inner: that, .. }) => this != that.as_ref(),
            (EqValue::TemporalInstant(this), DuperValue::Temporal(that)) => {
                Instant::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalZonedDateTime(this), DuperValue::Temporal(that)) => {
                ZonedDateTime::from_utf8(
                    that.as_ref().as_bytes(),
                    Disambiguation::Compatible,
                    OffsetDisambiguation::Prefer,
                )
                .ok()
                .is_none_or(|that| this.compare_instant(&that).is_ne())
            }
            (EqValue::TemporalPlainDate(this), DuperValue::Temporal(that)) => {
                PlainDate::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainTime(this), DuperValue::Temporal(that)) => {
                PlainTime::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainDateTime(this), DuperValue::Temporal(that)) => {
                PlainDateTime::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainYearMonth(this), DuperValue::Temporal(that)) => {
                PlainYearMonth::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalPlainMonthDay(this), DuperValue::Temporal(that)) => {
                PlainMonthDay::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::TemporalDuration(this), DuperValue::Temporal(that)) => {
                Duration::from_str(that.as_ref())
                    .ok()
                    .is_none_or(|that| *this != that)
            }
            (EqValue::Integer(this), DuperValue::Integer { inner: that, .. }) => this != that,
            (EqValue::Float(this, epsilon), DuperValue::Float { inner: that, .. }) => {
                (this - that).abs() > epsilon.unwrap_or(0.0).abs()
            }
            (EqValue::Integer(this), DuperValue::Float { inner: that, .. }) => {
                *this != *that as i64
            }
            (EqValue::Float(this, epsilon), DuperValue::Integer { inner: that, .. }) => {
                (this - *that as f64).abs() > epsilon.unwrap_or(0.0).abs()
            }
            (EqValue::Boolean(this), DuperValue::Boolean { inner: that, .. }) => this != that,
            (EqValue::Null, DuperValue::Null { .. }) => false,
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
        match value {
            DuperValue::Object { .. } => Err(TryFromDuperValueError::InvalidType("Object")),
            DuperValue::Array { .. } => Err(TryFromDuperValueError::InvalidType("Array")),
            DuperValue::Tuple { .. } => Err(TryFromDuperValueError::InvalidType("Tuple")),
            DuperValue::String { .. } => Err(TryFromDuperValueError::InvalidType("String")),
            DuperValue::Bytes { .. } => Err(TryFromDuperValueError::InvalidType("Bytes")),
            DuperValue::Temporal(temporal) => match temporal {
                DuperTemporal::Instant { inner: temporal } => Ok(CmpValue::TemporalInstant(
                    Instant::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::ZonedDateTime { inner: temporal } => {
                    Ok(CmpValue::TemporalZonedDateTime(ZonedDateTime::from_utf8(
                        temporal.as_ref().as_bytes(),
                        Disambiguation::Compatible,
                        OffsetDisambiguation::Prefer,
                    )?))
                }
                DuperTemporal::PlainDate { inner: temporal } => Ok(CmpValue::TemporalPlainDate(
                    PlainDate::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::PlainTime { inner: temporal } => Ok(CmpValue::TemporalPlainTime(
                    PlainTime::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::PlainDateTime { inner: temporal } => Ok(
                    CmpValue::TemporalPlainDateTime(PlainDateTime::from_str(temporal.as_ref())?),
                ),
                DuperTemporal::PlainYearMonth { inner: temporal } => Ok(
                    CmpValue::TemporalPlainYearMonth(PlainYearMonth::from_str(temporal.as_ref())?),
                ),
                DuperTemporal::PlainMonthDay { .. } => {
                    Err(TryFromDuperValueError::InvalidType("PlainMonthDay"))
                }
                DuperTemporal::Duration { inner: temporal } => Ok(CmpValue::TemporalDuration(
                    Duration::from_str(temporal.as_ref())?,
                )),
                DuperTemporal::Unspecified { .. } => {
                    Err(TryFromDuperValueError::UnspecifiedTemporal)
                }
            },
            DuperValue::Integer { inner: integer, .. } => Ok(CmpValue::Integer(integer)),
            DuperValue::Float { inner: float, .. } => Ok(CmpValue::Float(float)),
            DuperValue::Boolean { .. } => Err(TryFromDuperValueError::InvalidType("Boolean")),
            DuperValue::Null { .. } => Err(TryFromDuperValueError::InvalidType("Null")),
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
            fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
                match (&self.0, value) {
                    (CmpValue::Len(this), DuperValue::Object { inner: that, .. }) => {
                        matches!(that.len().cmp(this), $ord)
                    }
                    (CmpValue::Len(this), DuperValue::Array { inner: that, .. }) => {
                        matches!(that.len().cmp(this), $ord)
                    }
                    (CmpValue::Len(this), DuperValue::String { inner: that, .. }) => {
                        matches!(that.as_ref().len().cmp(this), $ord)
                    }
                    (CmpValue::Len(this), DuperValue::Bytes { inner: that, .. }) => {
                        matches!(that.as_ref().len().cmp(this), $ord)
                    }
                    (CmpValue::TemporalInstant(this), DuperValue::Temporal(that)) => {
                        Instant::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(that.cmp(this), $ord))
                    }
                    (CmpValue::TemporalZonedDateTime(this), DuperValue::Temporal(that)) => {
                        ZonedDateTime::from_utf8(
                            that.as_ref().as_bytes(),
                            Disambiguation::Compatible,
                            OffsetDisambiguation::Prefer,
                        )
                        .is_ok_and(|that| matches!(that.compare_instant(this), $ord))
                    }
                    (CmpValue::TemporalPlainDate(this), DuperValue::Temporal(that)) => {
                        PlainDate::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(that.compare_iso(this), $ord))
                    }
                    (CmpValue::TemporalPlainTime(this), DuperValue::Temporal(that)) => {
                        PlainTime::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(that.cmp(this), $ord))
                    }
                    (CmpValue::TemporalPlainDateTime(this), DuperValue::Temporal(that)) => {
                        PlainDateTime::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(that.compare_iso(this), $ord))
                    }
                    (CmpValue::TemporalPlainYearMonth(this), DuperValue::Temporal(that)) => {
                        PlainYearMonth::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(that.compare_iso(this), $ord))
                    }
                    (CmpValue::TemporalDuration(this), DuperValue::Temporal(that)) => {
                        Duration::from_str(that.as_ref())
                            .is_ok_and(|that| matches!(that.partial_cmp(this), Some($ord)))
                    }
                    (CmpValue::Integer(this), DuperValue::Integer { inner: that, .. }) => {
                        matches!(that.cmp(this), $ord)
                    }
                    (CmpValue::Float(this), DuperValue::Float { inner: that, .. }) => {
                        matches!(that.partial_cmp(this), Some($ord))
                    }
                    (CmpValue::Integer(this), DuperValue::Float { inner: that, .. }) => {
                        matches!((*that as i64).cmp(this), $ord)
                    }
                    (CmpValue::Float(this), DuperValue::Integer { inner: that, .. }) => {
                        matches!((*that as f64).partial_cmp(this), Some($ord))
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

#[derive(Clone)]
pub(crate) struct IsFilter(pub(crate) DuperType);

impl DuperFilter for IsFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        match (&self.0, value) {
            (DuperType::Object, DuperValue::Object { .. }) => true,
            (DuperType::Array, DuperValue::Array { .. }) => true,
            (DuperType::Tuple, DuperValue::Tuple { .. }) => true,
            (DuperType::String, DuperValue::String { .. }) => true,
            (DuperType::Bytes, DuperValue::Bytes { .. }) => true,
            (DuperType::TemporalInstant, DuperValue::Temporal(that)) => {
                Instant::from_str(that.as_ref()).is_ok()
            }
            (DuperType::TemporalZonedDateTime, DuperValue::Temporal(that)) => {
                ZonedDateTime::from_utf8(
                    that.as_ref().as_bytes(),
                    Disambiguation::Compatible,
                    OffsetDisambiguation::Prefer,
                )
                .is_ok()
            }
            (DuperType::TemporalPlainDate, DuperValue::Temporal(that)) => {
                PlainDate::from_str(that.as_ref()).is_ok()
            }
            (DuperType::TemporalPlainTime, DuperValue::Temporal(that)) => {
                PlainTime::from_str(that.as_ref()).is_ok()
            }
            (DuperType::TemporalPlainDateTime, DuperValue::Temporal(that)) => {
                PlainDateTime::from_str(that.as_ref()).is_ok()
            }
            (DuperType::TemporalPlainYearMonth, DuperValue::Temporal(that)) => {
                PlainYearMonth::from_str(that.as_ref()).is_ok()
            }
            (DuperType::TemporalPlainMonthDay, DuperValue::Temporal(that)) => {
                PlainMonthDay::from_str(that.as_ref()).is_ok()
            }
            (DuperType::TemporalDuration, DuperValue::Temporal(that)) => {
                Duration::from_str(that.as_ref()).is_ok()
            }
            (DuperType::TemporalUnspecified, DuperValue::Temporal { .. }) => true,
            (DuperType::Integer, DuperValue::Integer { .. }) => true,
            (DuperType::Float, DuperValue::Float { .. }) => true,
            (DuperType::Number, DuperValue::Integer { .. } | DuperValue::Float { .. }) => true,
            (DuperType::Boolean, DuperValue::Boolean { .. }) => true,
            (DuperType::Null, DuperValue::Null { .. }) => true,
            _ => false,
        }
    }
}

pub(crate) struct RegexFilter(pub(crate) regex::bytes::Regex);

impl DuperFilter for RegexFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        match value {
            DuperValue::String { inner: string, .. } => {
                self.0.find(string.as_ref().as_bytes()).is_some()
            }
            DuperValue::Bytes { inner: bytes, .. } => self.0.find(bytes.as_ref()).is_some(),
            DuperValue::Temporal(temporal) => self.0.find(temporal.as_ref().as_bytes()).is_some(),
            _ => false,
        }
    }
}

pub(crate) struct RegexIdentifierFilter(pub(crate) regex::Regex);

impl DuperFilter for RegexIdentifierFilter {
    fn filter<'v>(&self, value: &DuperValue<'v>) -> bool {
        value
            .identifier()
            .as_ref()
            .is_some_and(|identifier| self.0.find(identifier.as_ref()).is_some())
    }
}

pub(crate) struct IsTruthyFilter;

impl DuperFilter for IsTruthyFilter {
    fn filter<'v>(&self, value: &DuperValue<'_>) -> bool {
        match value {
            DuperValue::Object { inner: object, .. } => !object.is_empty(),
            DuperValue::Array { inner: array, .. } => !array.is_empty(),
            DuperValue::Tuple { inner: tuple, .. } => !tuple.is_empty(),
            DuperValue::String { inner: string, .. } => !string.is_empty(),
            DuperValue::Bytes { inner: bytes, .. } => !bytes.is_empty(),
            DuperValue::Temporal { .. } => true,
            DuperValue::Integer { inner: integer, .. } => *integer != 0,
            DuperValue::Float { inner: float, .. } => *float != 0.0,
            DuperValue::Boolean { inner: boolean, .. } => *boolean,
            DuperValue::Null { .. } => false,
        }
    }
}
