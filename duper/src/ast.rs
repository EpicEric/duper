//! Types for interacting with Duper's abstract syntax tree.

use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    fmt::{Debug, Display},
};

use chumsky::Parser;
use indexmap::IndexMap;

use crate::{
    parser::{self, DuperParser},
    visitor::DuperVisitor,
};

/// A Duper identifier: `MyIdentifier(...)`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperIdentifier<'a>(pub(crate) Cow<'a, str>);

/// A Duper value.
#[derive(Debug, Clone)]
pub enum DuperValue<'a> {
    /// An object: `{...}`
    Object {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the object.
        inner: DuperObject<'a>,
    },
    /// An array: `[...]`
    Array {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the array.
        inner: Vec<DuperValue<'a>>,
    },
    /// A tuple: `(...)`
    Tuple {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the tuple.
        inner: Vec<DuperValue<'a>>,
    },
    /// A string: `"..."`, `r"..."`
    String {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the string.
        inner: Cow<'a, str>,
    },
    /// A byte string: `b"..."`, `br"..."`, `b64"..."`
    Bytes {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the byte string.
        inner: Cow<'a, [u8]>,
    },
    /// A Temporal value.
    Temporal(DuperTemporal<'a>),
    /// An integer.
    Integer {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the integer.
        inner: i64,
    },
    /// A float.
    Float {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the float.
        inner: f64,
    },
    /// A boolean.
    Boolean {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
        /// The actual value of the boolean.
        inner: bool,
    },
    /// A null value.
    Null {
        /// The identifier of this value.
        identifier: Option<DuperIdentifier<'a>>,
    },
}

/// A key in a [`DuperObject`].
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DuperKey<'a>(pub(crate) Cow<'a, str>);

/// An object (or map) from [`DuperKey`]s to [`DuperValue`]s.
#[derive(Debug, Clone)]
pub struct DuperObject<'a>(pub(crate) IndexMap<DuperKey<'a>, DuperValue<'a>>);

#[derive(Debug, Clone, PartialEq)]
pub enum DuperTemporal<'a> {
    /// A Temporal Instant: `Instant('...')`
    Instant {
        /// The actual value of the Instant.
        inner: DuperTemporalInstant<'a>,
    },
    /// A Temporal ZonedDateTime: `ZonedDateTime('...')`
    ZonedDateTime {
        /// The actual value of the ZonedDateTime.
        inner: DuperTemporalZonedDateTime<'a>,
    },
    /// A Temporal PlainDate: `PlainDate('...')`
    PlainDate {
        /// The actual value of the PlainDate.
        inner: DuperTemporalPlainDate<'a>,
    },
    /// A Temporal PlainTime: `PlainTime('...')`
    PlainTime {
        /// The actual value of the PlainTime.
        inner: DuperTemporalPlainTime<'a>,
    },
    /// A Temporal PlainDateTime: `PlainDateTime('...')`
    PlainDateTime {
        /// The actual value of the PlainDateTime.
        inner: DuperTemporalPlainDateTime<'a>,
    },
    /// A Temporal PlainYearMonth: `PlainYearMonth('...')`
    PlainYearMonth {
        /// The actual value of the PlainYearMonth.
        inner: DuperTemporalPlainYearMonth<'a>,
    },
    /// A Temporal PlainMonthDay: `PlainMonthDay('...')`
    PlainMonthDay {
        /// The actual value of the PlainMonthDay.
        inner: DuperTemporalPlainMonthDay<'a>,
    },
    /// A Temporal Duration: `Duration('...')`
    Duration {
        /// The actual value of the Duper Duration.
        inner: DuperTemporalDuration<'a>,
    },
    /// An unspecified Temporal value: `'...'`, `Unknown('...')`
    Unspecified {
        /// The identifier of this value.
        identifier: Option<DuperTemporalIdentifier<'a>>,
        /// The actual value of the Temporal value.
        inner: DuperTemporalUnspecified<'a>,
    },
}

/// A fixed point, or exact time, without regard to calendar or location.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalInstant<'a>(pub(crate) Cow<'a, str>);

/// A timezone- and calendar-aware date/time object that represents a real event.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalZonedDateTime<'a>(pub(crate) Cow<'a, str>);

/// A calendar date that is not associated with a particular time or timezone.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalPlainDate<'a>(pub(crate) Cow<'a, str>);

/// A wall-clock time that is not associated with a particular date or timezone.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalPlainTime<'a>(pub(crate) Cow<'a, str>);

/// A calendar date and wall-clock time duo that does not carry any timezone information.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalPlainDateTime<'a>(pub(crate) Cow<'a, str>);

/// A date without a day component.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalPlainYearMonth<'a>(pub(crate) Cow<'a, str>);

/// A date without a year component.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalPlainMonthDay<'a>(pub(crate) Cow<'a, str>);

/// A length of time, used for date/time arithmetic or differences between Temporal objects.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalDuration<'a>(pub(crate) Cow<'a, str>);

/// A Temporal object of unspecified type.
/// The inner representation may be borrowed or owned.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalUnspecified<'a>(pub(crate) Cow<'a, str>);

/// An unspecified Temporal identifier: `MyIdentifier(...)`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperTemporalIdentifier<'a>(pub(crate) DuperIdentifier<'a>);

/// Possible errors generated by [`DuperIdentifier::try_from()`].
#[derive(Debug, Clone)]
pub enum DuperIdentifierTryFromError<'a> {
    /// The identifier was empty.
    EmptyIdentifier,
    /// The identifier contained an invalid character.
    InvalidChar(Cow<'a, str>, usize),
}

/// Possible errors generated by [`DuperTemporalIdentifier::try_from()`].
#[derive(Debug, Clone)]
pub enum DuperTemporalIdentifierTryFromError<'a> {
    /// The identifier was empty.
    EmptyIdentifier,
    /// The identifier contained an invalid character.
    InvalidChar(Cow<'a, str>, usize),
    /// The identifier is reserved for typed Temporal values.
    ReservedIdentifier(Cow<'a, str>),
}

/// Possible errors generated by [`DuperObject::try_from()`].
#[derive(Debug, Clone)]
pub enum DuperObjectTryFromError<'a> {
    /// The key was duplicated.
    DuplicateKey(Cow<'a, str>),
}

/// Possible errors generated by `DuperTemporal::try_*_from()`.
#[derive(Debug, Clone)]
pub enum DuperTemporalTryFromError<'a> {
    /// The Temporal string was empty.
    EmptyTemporal,
    /// The Temporal value contained an invalid character.
    InvalidChar(Cow<'a, str>, usize),
    /// Invalid identifier.
    InvalidIdentifier(DuperTemporalIdentifierTryFromError<'a>),
}

impl<'a> DuperIdentifier<'a> {
    /// Consume this identifier and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a valid identifier from the provided [`Cow<'_, str>`], discarding
    /// any invalid characters if necessary.
    pub fn try_from_lossy(value: Cow<'a, str>) -> Result<Self, DuperIdentifierTryFromError<'a>> {
        if value.is_empty() {
            return Err(DuperIdentifierTryFromError::EmptyIdentifier);
        }
        // Eagerly try the non-lossy version first.
        if DuperIdentifier::try_from(value.as_ref()).is_ok() {
            return Ok(DuperIdentifier(value));
        }
        let invalid_char_pos = match parser::identifier_lossy()
            .parse(value.as_ref())
            .into_result()
        {
            Err(errs) => errs[0].span().start,
            Ok(identifier) => return Ok(identifier),
        };
        Err(DuperIdentifierTryFromError::InvalidChar(
            value,
            invalid_char_pos,
        ))
    }

    /// Create a clone of this `DuperIdentifier` with a static lifetime.
    pub fn static_clone(&self) -> DuperIdentifier<'static> {
        DuperIdentifier(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> Display for DuperIdentifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'a> AsRef<str> for DuperIdentifier<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperIdentifier<'a> {
    type Error = DuperIdentifierTryFromError<'a>;

    /// Create a valid identifier from the provided [`Cow<'_, str>`], returning
    /// an error if there are invalid characters.
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(DuperIdentifierTryFromError::EmptyIdentifier);
        }
        let invalid_char_pos = parser::identifier()
            .parse(value.as_ref())
            .into_result()
            .err()
            .map(|errs| errs[0].span().start);
        match invalid_char_pos {
            Some(pos) => Err(DuperIdentifierTryFromError::InvalidChar(value, pos)),
            None => Ok(Self(value)),
        }
    }
}

impl<'a> TryFrom<&'a str> for DuperIdentifier<'a> {
    type Error = DuperIdentifierTryFromError<'a>;

    /// Create a valid identifier from the provided `&str`, returning
    /// an error if there are invalid characters.
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Borrowed(value))
    }
}

impl TryFrom<String> for DuperIdentifier<'static> {
    type Error = DuperIdentifierTryFromError<'static>;

    /// Create a valid identifier from the provided [`String`], returning
    /// an error if there are invalid characters.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Owned(value))
    }
}

impl Display for DuperIdentifierTryFromError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuperIdentifierTryFromError::EmptyIdentifier => f.write_str("empty identifier"),
            DuperIdentifierTryFromError::InvalidChar(identifier, pos) => f.write_fmt(format_args!(
                "invalid character in position {pos} of identifier {identifier}"
            )),
        }
    }
}

impl std::error::Error for DuperIdentifierTryFromError<'_> {}

impl<'a> DuperTemporalIdentifier<'a> {
    /// Consume this identifier and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0.0
    }

    /// Create a valid identifier from the provided [`Cow<'_, str>`], discarding
    /// any invalid characters if necessary.
    pub fn try_from_lossy(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalIdentifierTryFromError<'a>> {
        let identifier = DuperIdentifier::try_from_lossy(value)?;
        Self::try_from(identifier)
    }

    /// Create a clone of this `DuperTemporalIdentifier` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalIdentifier<'static> {
        DuperTemporalIdentifier(DuperIdentifier(Cow::Owned(self.0.0.clone().into_owned())))
    }
}

impl<'a> Display for DuperTemporalIdentifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.0)
    }
}

impl<'a> AsRef<str> for DuperTemporalIdentifier<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalIdentifier<'a> {
    type Error = DuperTemporalIdentifierTryFromError<'a>;

    /// Create a valid identifier from the provided [`Cow<'_, str>`], returning
    /// an error if there are invalid characters.
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let identifier = DuperIdentifier::try_from(value)?;
        Self::try_from(identifier)
    }
}

impl<'a> TryFrom<&'a str> for DuperTemporalIdentifier<'a> {
    type Error = DuperTemporalIdentifierTryFromError<'a>;

    /// Create a valid identifier from the provided `&str`, returning
    /// an error if there are invalid characters.
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Borrowed(value))
    }
}

impl TryFrom<String> for DuperTemporalIdentifier<'static> {
    type Error = DuperTemporalIdentifierTryFromError<'static>;

    /// Create a valid identifier from the provided [`String`], returning
    /// an error if there are invalid characters.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Owned(value))
    }
}

impl<'a> TryFrom<DuperIdentifier<'a>> for DuperTemporalIdentifier<'a> {
    type Error = DuperTemporalIdentifierTryFromError<'a>;

    /// Create a valid Temporal identifier from the provided [`DuperIdentifier`],
    /// returning an error if a reserved identifier is provided.
    fn try_from(value: DuperIdentifier<'a>) -> Result<Self, Self::Error> {
        if matches!(
            value.as_ref(),
            "Instant"
                | "ZonedDateTime"
                | "PlainDate"
                | "PlainTime"
                | "PlainDateTime"
                | "PlainYearMonth"
                | "PlainMonthDay"
                | "Duration"
        ) {
            Err(DuperTemporalIdentifierTryFromError::ReservedIdentifier(
                value.into_inner(),
            ))
        } else {
            Ok(Self(value))
        }
    }
}

impl Display for DuperTemporalIdentifierTryFromError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuperTemporalIdentifierTryFromError::EmptyIdentifier => f.write_str("empty identifier"),
            DuperTemporalIdentifierTryFromError::InvalidChar(identifier, pos) => f.write_fmt(
                format_args!("invalid character in position {pos} of identifier {identifier}"),
            ),
            DuperTemporalIdentifierTryFromError::ReservedIdentifier(identifier) => f.write_fmt(
                format_args!("identifier {identifier} is reserved for typed Temporal values"),
            ),
        }
    }
}

impl std::error::Error for DuperTemporalIdentifierTryFromError<'_> {}

impl<'a> From<DuperIdentifierTryFromError<'a>> for DuperTemporalIdentifierTryFromError<'a> {
    fn from(value: DuperIdentifierTryFromError<'a>) -> Self {
        match value {
            DuperIdentifierTryFromError::EmptyIdentifier => {
                DuperTemporalIdentifierTryFromError::EmptyIdentifier
            }
            DuperIdentifierTryFromError::InvalidChar(identifier, pos) => {
                DuperTemporalIdentifierTryFromError::InvalidChar(identifier, pos)
            }
        }
    }
}

impl<'a> DuperKey<'a> {
    /// Consume this key and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperKey` with a static lifetime.
    pub fn static_clone(&self) -> DuperKey<'static> {
        DuperKey(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperKey<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> From<Cow<'a, str>> for DuperKey<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for DuperKey<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for DuperKey<'static> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> DuperValue<'a> {
    /// Accepts a [`DuperVisitor`] and visits it with the current value.
    pub fn accept<V: DuperVisitor>(&self, visitor: &mut V) -> V::Value {
        match &self {
            DuperValue::Object { identifier, inner } => {
                visitor.visit_object(identifier.as_ref(), inner)
            }
            DuperValue::Array { identifier, inner } => {
                visitor.visit_array(identifier.as_ref(), inner.as_ref())
            }
            DuperValue::Tuple { identifier, inner } => {
                visitor.visit_tuple(identifier.as_ref(), inner.as_ref())
            }
            DuperValue::String { identifier, inner } => {
                visitor.visit_string(identifier.as_ref(), inner.as_ref())
            }
            DuperValue::Bytes { identifier, inner } => {
                visitor.visit_bytes(identifier.as_ref(), inner.as_ref())
            }
            DuperValue::Temporal(temporal) => visitor.visit_temporal(temporal),
            DuperValue::Integer { identifier, inner } => {
                visitor.visit_integer(identifier.as_ref(), *inner)
            }
            DuperValue::Float { identifier, inner } => {
                visitor.visit_float(identifier.as_ref(), *inner)
            }
            DuperValue::Boolean { identifier, inner } => {
                visitor.visit_boolean(identifier.as_ref(), *inner)
            }
            DuperValue::Null { identifier } => visitor.visit_null(identifier.as_ref()),
        }
    }

    /// Create a clone of this `DuperValue` with a static lifetime.
    pub fn static_clone(&self) -> DuperValue<'static> {
        match self {
            DuperValue::Object { identifier, inner } => DuperValue::Object {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: DuperObject(
                    inner
                        .0
                        .iter()
                        .map(|(key, value)| {
                            (
                                DuperKey(Cow::Owned(key.0.clone().into_owned())),
                                value.static_clone(),
                            )
                        })
                        .collect(),
                ),
            },
            DuperValue::Array { identifier, inner } => DuperValue::Array {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: inner.iter().map(|value| value.static_clone()).collect(),
            },
            DuperValue::Tuple { identifier, inner } => DuperValue::Tuple {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: inner.iter().map(|value| value.static_clone()).collect(),
            },
            DuperValue::String { identifier, inner } => DuperValue::String {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: Cow::Owned(inner.clone().into_owned()),
            },
            DuperValue::Bytes { identifier, inner } => DuperValue::Bytes {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: Cow::Owned(inner.clone().into_owned()),
            },
            DuperValue::Temporal(temporal) => DuperValue::Temporal(temporal.static_clone()),
            DuperValue::Integer { identifier, inner } => DuperValue::Integer {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: *inner,
            },
            DuperValue::Float { identifier, inner } => DuperValue::Float {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: *inner,
            },
            DuperValue::Boolean { identifier, inner } => DuperValue::Boolean {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: *inner,
            },
            DuperValue::Null { identifier } => DuperValue::Null {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
            },
        }
    }

    /// Returns a clone of the identifier associated with this Duper value.
    pub fn identifier(&self) -> Option<DuperIdentifier<'a>> {
        match self {
            DuperValue::Temporal(inner) => inner.identifier(),
            DuperValue::Object { identifier, .. }
            | DuperValue::Array { identifier, .. }
            | DuperValue::Tuple { identifier, .. }
            | DuperValue::String { identifier, .. }
            | DuperValue::Bytes { identifier, .. }
            | DuperValue::Integer { identifier, .. }
            | DuperValue::Float { identifier, .. }
            | DuperValue::Boolean { identifier, .. }
            | DuperValue::Null { identifier } => identifier.as_ref().cloned(),
        }
    }

    /// Replaces the identifier of the value, returning an error if the
    /// identifier is invalid.
    pub fn with_identifier(
        self,
        identifier: Option<DuperIdentifier<'a>>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        match self {
            DuperValue::Object { inner, .. } => Ok(DuperValue::Object { inner, identifier }),
            DuperValue::Array { inner, .. } => Ok(DuperValue::Array { inner, identifier }),
            DuperValue::Tuple { inner, .. } => Ok(DuperValue::Tuple { inner, identifier }),
            DuperValue::String { inner, .. } => Ok(DuperValue::String { inner, identifier }),
            DuperValue::Bytes { inner, .. } => Ok(DuperValue::Bytes { inner, identifier }),
            DuperValue::Temporal(temporal) => {
                Ok(DuperValue::Temporal(temporal.with_identifier(identifier)?))
            }
            DuperValue::Integer { inner, .. } => Ok(DuperValue::Integer { inner, identifier }),
            DuperValue::Float { inner, .. } => Ok(DuperValue::Float { inner, identifier }),
            DuperValue::Boolean { inner, .. } => Ok(DuperValue::Boolean { inner, identifier }),
            DuperValue::Null { .. } => Ok(DuperValue::Null { identifier }),
        }
    }

    /// Create a valid Temporal Instant from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_instant_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::Instant {
            inner: DuperTemporalInstant::try_from(value)?,
        }))
    }

    /// Create a valid Temporal ZonedDateTime from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_zoned_date_time_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::ZonedDateTime {
            inner: DuperTemporalZonedDateTime::try_from(value)?,
        }))
    }

    /// Create a valid Temporal PlainDate from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_date_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::PlainDate {
            inner: DuperTemporalPlainDate::try_from(value)?,
        }))
    }

    /// Create a valid Temporal PlainTime from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_time_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::PlainTime {
            inner: DuperTemporalPlainTime::try_from(value)?,
        }))
    }

    /// Create a valid Temporal PlainDateTime from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_date_time_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::PlainDateTime {
            inner: DuperTemporalPlainDateTime::try_from(value)?,
        }))
    }

    /// Create a valid Temporal PlainYearMonth from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_year_month_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::PlainYearMonth {
            inner: DuperTemporalPlainYearMonth::try_from(value)?,
        }))
    }

    /// Create a valid Temporal PlainMonthDay from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_month_day_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::PlainMonthDay {
            inner: DuperTemporalPlainMonthDay::try_from(value)?,
        }))
    }

    /// Create a valid Temporal Duration from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_duration_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::Duration {
            inner: DuperTemporalDuration::try_from(value)?,
        }))
    }

    /// Create a valid unspecified Temporal value from the provided [`DuperIdentifier`] and [`Cow<'_, str>`],
    /// returning an error if parsing fails or the identifier is invalid.
    pub fn try_unspecified_from(
        identifier: Option<DuperIdentifier<'a>>,
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Temporal(DuperTemporal::Unspecified {
            identifier: identifier
                .map(DuperTemporalIdentifier::try_from)
                .transpose()?,
            inner: DuperTemporalUnspecified::try_from(value)?,
        }))
    }
}

impl<'a> TryFrom<&'a str> for DuperValue<'a> {
    type Error = Vec<chumsky::error::Rich<'a, char>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        DuperParser::parse_duper_value(value)
    }
}

impl<'a> PartialEq for DuperValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DuperValue::Object { inner: this, .. }, DuperValue::Object { inner: that, .. }) => {
                this == that
            }
            (DuperValue::Array { inner: this, .. }, DuperValue::Array { inner: that, .. }) => {
                this == that
            }
            (DuperValue::Tuple { inner: this, .. }, DuperValue::Tuple { inner: that, .. }) => {
                this == that
            }
            (DuperValue::String { inner: this, .. }, DuperValue::String { inner: that, .. }) => {
                this == that
            }
            (DuperValue::Bytes { inner: this, .. }, DuperValue::Bytes { inner: that, .. }) => {
                this == that
            }
            (DuperValue::Temporal(this), DuperValue::Temporal(that)) => {
                this.as_ref() == that.as_ref()
            }
            (DuperValue::Integer { inner: this, .. }, DuperValue::Integer { inner: that, .. }) => {
                this == that
            }
            (DuperValue::Float { inner: this, .. }, DuperValue::Float { inner: that, .. }) => {
                this == that
            }
            (DuperValue::Boolean { inner: this, .. }, DuperValue::Boolean { inner: that, .. }) => {
                this == that
            }
            (DuperValue::Null { .. }, DuperValue::Null { .. }) => true,
            _ => false,
        }
    }
}

impl<'a> DuperObject<'a> {
    /// Consume this object and return the underlying [`IndexMap<DuperKey<'_>, DuperValue<'_>>`].
    pub fn into_inner(self) -> IndexMap<DuperKey<'a>, DuperValue<'a>> {
        self.0
    }

    /// Returns `true` if the object contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the amount of elements in this object.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over references to the (key, value) pairs in this
    /// object.
    pub fn iter(&self) -> impl Iterator<Item = (&DuperKey<'a>, &DuperValue<'a>)> {
        self.0.iter()
    }

    /// Returns the [`DuperValue`] with the given key.
    pub fn get<'b>(&'b self, key: &'b DuperKey<'_>) -> Option<&'b DuperValue<'a>> {
        self.0.get(key)
    }

    /// Create a valid object from the provided [`Vec`], dropping any duplicate keys
    /// and keeping the first one.
    pub fn from_lossy(value: Vec<(DuperKey<'a>, DuperValue<'a>)>) -> Self {
        let mut keys = std::collections::HashSet::with_capacity(value.len());
        Self(
            value
                .into_iter()
                .filter(|(key, _)| {
                    if keys.contains(key) {
                        false
                    } else {
                        keys.insert(key.clone());
                        true
                    }
                })
                .collect(),
        )
    }
}

impl<'a> From<IndexMap<DuperKey<'a>, DuperValue<'a>>> for DuperObject<'a> {
    /// Create a valid object from the provided [`HashMap`].
    fn from(value: IndexMap<DuperKey<'a>, DuperValue<'a>>) -> Self {
        Self(value)
    }
}

impl<'a> From<HashMap<DuperKey<'a>, DuperValue<'a>>> for DuperObject<'a> {
    /// Create a valid object from the provided [`HashMap`].
    fn from(value: HashMap<DuperKey<'a>, DuperValue<'a>>) -> Self {
        Self(value.into_iter().collect())
    }
}

impl<'a> From<BTreeMap<DuperKey<'a>, DuperValue<'a>>> for DuperObject<'a> {
    /// Create a valid object from the provided [`BTreeMap`].
    fn from(value: BTreeMap<DuperKey<'a>, DuperValue<'a>>) -> Self {
        Self(value.into_iter().collect())
    }
}

impl<'a> TryFrom<Vec<(DuperKey<'a>, DuperValue<'a>)>> for DuperObject<'a> {
    type Error = DuperObjectTryFromError<'a>;

    /// Create a valid object from the provided [`Vec`], returning an error if
    /// a duplicate key is found.
    fn try_from(value: Vec<(DuperKey<'a>, DuperValue<'a>)>) -> Result<Self, Self::Error> {
        let mut object = IndexMap::with_capacity(value.len());
        for (key, value) in value {
            if object.contains_key(&key) {
                return Err(DuperObjectTryFromError::DuplicateKey(key.0));
            }
            object.insert(key, value);
        }
        Ok(Self(object))
    }
}

impl<'a> PartialEq for DuperObject<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for DuperObjectTryFromError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuperObjectTryFromError::DuplicateKey(key) => {
                f.write_fmt(format_args!("duplicate key {key} in object"))
            }
        }
    }
}

impl std::error::Error for DuperObjectTryFromError<'_> {}

impl<'a> DuperTemporal<'a> {
    /// Replaces the identifier of the value, returning an error if the
    /// identifier is invalid.
    ///
    /// Only unspecified values can have their identifier updated.
    pub fn with_identifier(
        self,
        identifier: Option<DuperIdentifier<'a>>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        match self {
            DuperTemporal::Unspecified { inner, .. } => {
                if let Some(identifier) = identifier {
                    match identifier.as_ref() {
                        "Instant" => DuperTemporal::try_instant_from(inner.0),
                        "ZonedDateTime" => DuperTemporal::try_zoned_date_time_from(inner.0),
                        "PlainDate" => DuperTemporal::try_plain_date_from(inner.0),
                        "PlainTime" => DuperTemporal::try_plain_time_from(inner.0),
                        "PlainDateTime" => DuperTemporal::try_plain_date_time_from(inner.0),
                        "PlainYearMonth" => DuperTemporal::try_plain_year_month_from(inner.0),
                        "PlainMonthDay" => DuperTemporal::try_plain_month_day_from(inner.0),
                        "Duration" => DuperTemporal::try_duration_from(inner.0),
                        _ => Ok(DuperTemporal::Unspecified {
                            identifier: Some(DuperTemporalIdentifier(identifier)),
                            inner,
                        }),
                    }
                } else {
                    Ok(DuperTemporal::Unspecified {
                        identifier: None,
                        inner,
                    })
                }
            }
            other => Ok(other),
        }
    }

    /// Create a clone of this `DuperTemporal` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporal<'static> {
        match self {
            DuperTemporal::Instant { inner } => DuperTemporal::Instant {
                inner: inner.static_clone(),
            },
            DuperTemporal::ZonedDateTime { inner } => DuperTemporal::ZonedDateTime {
                inner: inner.static_clone(),
            },
            DuperTemporal::PlainDate { inner } => DuperTemporal::PlainDate {
                inner: inner.static_clone(),
            },
            DuperTemporal::PlainTime { inner } => DuperTemporal::PlainTime {
                inner: inner.static_clone(),
            },
            DuperTemporal::PlainDateTime { inner } => DuperTemporal::PlainDateTime {
                inner: inner.static_clone(),
            },
            DuperTemporal::PlainYearMonth { inner } => DuperTemporal::PlainYearMonth {
                inner: inner.static_clone(),
            },
            DuperTemporal::PlainMonthDay { inner } => DuperTemporal::PlainMonthDay {
                inner: inner.static_clone(),
            },
            DuperTemporal::Duration { inner } => DuperTemporal::Duration {
                inner: inner.static_clone(),
            },
            DuperTemporal::Unspecified { identifier, inner } => DuperTemporal::Unspecified {
                identifier: identifier
                    .as_ref()
                    .map(|identifier| identifier.static_clone()),
                inner: inner.static_clone(),
            },
        }
    }

    /// Returns a clone of identifier associated with this Duper Temporal value.
    pub fn identifier(&self) -> Option<DuperIdentifier<'a>> {
        match self {
            DuperTemporal::Instant { .. } => Some(DuperIdentifier(Cow::Borrowed("Instant"))),
            DuperTemporal::ZonedDateTime { .. } => {
                Some(DuperIdentifier(Cow::Borrowed("ZonedDateTime")))
            }
            DuperTemporal::PlainDate { .. } => Some(DuperIdentifier(Cow::Borrowed("PlainDate"))),
            DuperTemporal::PlainTime { .. } => Some(DuperIdentifier(Cow::Borrowed("PlainTime"))),
            DuperTemporal::PlainDateTime { .. } => {
                Some(DuperIdentifier(Cow::Borrowed("PlainDateTime")))
            }
            DuperTemporal::PlainYearMonth { .. } => {
                Some(DuperIdentifier(Cow::Borrowed("PlainYearMonth")))
            }
            DuperTemporal::PlainMonthDay { .. } => {
                Some(DuperIdentifier(Cow::Borrowed("PlainMonthDay")))
            }
            DuperTemporal::Duration { .. } => Some(DuperIdentifier(Cow::Borrowed("Duration"))),
            DuperTemporal::Unspecified { identifier, .. } => {
                identifier.as_ref().map(|identifier| identifier.0.clone())
            }
        }
    }

    /// Create a valid Temporal Instant from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_instant_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Instant {
            inner: DuperTemporalInstant::try_from(value)?,
        })
    }

    /// Create a valid Temporal ZonedDateTime from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_zoned_date_time_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::ZonedDateTime {
            inner: DuperTemporalZonedDateTime::try_from(value)?,
        })
    }

    /// Create a valid Temporal PlainDate from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_date_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::PlainDate {
            inner: DuperTemporalPlainDate::try_from(value)?,
        })
    }

    /// Create a valid Temporal PlainTime from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_time_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::PlainTime {
            inner: DuperTemporalPlainTime::try_from(value)?,
        })
    }

    /// Create a valid Temporal PlainDateTime from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_date_time_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::PlainDateTime {
            inner: DuperTemporalPlainDateTime::try_from(value)?,
        })
    }

    /// Create a valid Temporal PlainYearMonth from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_year_month_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::PlainYearMonth {
            inner: DuperTemporalPlainYearMonth::try_from(value)?,
        })
    }

    /// Create a valid Temporal PlainMonthDay from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_plain_month_day_from(
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::PlainMonthDay {
            inner: DuperTemporalPlainMonthDay::try_from(value)?,
        })
    }

    /// Create a valid Temporal Duration from the provided [`Cow<'_, str>`],
    /// returning an error if parsing fails.
    pub fn try_duration_from(value: Cow<'a, str>) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Duration {
            inner: DuperTemporalDuration::try_from(value)?,
        })
    }

    /// Create a valid unspecified Temporal value from the provided [`DuperIdentifier`] and [`Cow<'_, str>`],
    /// returning an error if parsing fails or the identifier is invalid.
    pub fn try_unspecified_from(
        identifier: Option<DuperIdentifier<'a>>,
        value: Cow<'a, str>,
    ) -> Result<Self, DuperTemporalTryFromError<'a>> {
        Ok(Self::Unspecified {
            identifier: identifier
                .map(DuperTemporalIdentifier::try_from)
                .transpose()?,
            inner: DuperTemporalUnspecified::try_from(value)?,
        })
    }
}

impl<'a> AsRef<str> for DuperTemporal<'a> {
    fn as_ref(&self) -> &str {
        match self {
            DuperTemporal::Instant { inner } => inner.as_ref(),
            DuperTemporal::ZonedDateTime { inner } => inner.as_ref(),
            DuperTemporal::PlainDate { inner } => inner.as_ref(),
            DuperTemporal::PlainTime { inner } => inner.as_ref(),
            DuperTemporal::PlainDateTime { inner } => inner.as_ref(),
            DuperTemporal::PlainYearMonth { inner } => inner.as_ref(),
            DuperTemporal::PlainMonthDay { inner } => inner.as_ref(),
            DuperTemporal::Duration { inner } => inner.as_ref(),
            DuperTemporal::Unspecified { inner, .. } => inner.as_ref(),
        }
    }
}

impl<'a> DuperTemporalInstant<'a> {
    /// Consume this Instant value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalInstant` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalInstant<'static> {
        DuperTemporalInstant(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalInstant<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalInstant<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::instant()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalInstant(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalZonedDateTime<'a> {
    /// Consume this ZonedDateTime value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalZonedDateTime` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalZonedDateTime<'static> {
        DuperTemporalZonedDateTime(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalZonedDateTime<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalZonedDateTime<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::zoned_date_time()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalZonedDateTime(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalPlainDate<'a> {
    /// Consume this PlainDate value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalPlainDate` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalPlainDate<'static> {
        DuperTemporalPlainDate(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalPlainDate<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalPlainDate<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::plain_date()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalPlainDate(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalPlainTime<'a> {
    /// Consume this PlainTime value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalPlainTime` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalPlainTime<'static> {
        DuperTemporalPlainTime(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalPlainTime<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalPlainTime<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::plain_time()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalPlainTime(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalPlainDateTime<'a> {
    /// Consume this PlainDateTime value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalPlainDateTime` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalPlainDateTime<'static> {
        DuperTemporalPlainDateTime(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalPlainDateTime<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalPlainDateTime<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::plain_date_time()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalPlainDateTime(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalPlainYearMonth<'a> {
    /// Consume this PlainYearMonth value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalPlainYearMonth` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalPlainYearMonth<'static> {
        DuperTemporalPlainYearMonth(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalPlainYearMonth<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalPlainYearMonth<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::plain_year_month()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalPlainYearMonth(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalPlainMonthDay<'a> {
    /// Consume this PlainMonthDay value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalPlainMonthDay` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalPlainMonthDay<'static> {
        DuperTemporalPlainMonthDay(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalPlainMonthDay<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalPlainMonthDay<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::plain_month_day()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalPlainMonthDay(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalDuration<'a> {
    /// Consume this Duration value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporalDuration` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalDuration<'static> {
        DuperTemporalDuration(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalDuration<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalDuration<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::duration()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalDuration(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl<'a> DuperTemporalUnspecified<'a> {
    /// Consume this Temporal value and return the underlying [`Cow<'_, str>`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Create a clone of this `DuperTemporal` with a static lifetime.
    pub fn static_clone(&self) -> DuperTemporalUnspecified<'static> {
        DuperTemporalUnspecified(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> AsRef<str> for DuperTemporalUnspecified<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DuperTemporalUnspecified<'a> {
    type Error = DuperTemporalTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DuperTemporalTryFromError::EmptyTemporal);
        }
        let result = match parser::temporal::unspecified()
            .parse(value.as_ref())
            .into_result()
        {
            Ok(_) => Ok(()),
            Err(errs) => Err(errs[0].span().start),
        };

        match result {
            Ok(_) => Ok(DuperTemporalUnspecified(value)),
            Err(invalid_char_pos) => Err(DuperTemporalTryFromError::InvalidChar(
                value,
                invalid_char_pos,
            )),
        }
    }
}

impl Display for DuperTemporalTryFromError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuperTemporalTryFromError::EmptyTemporal => f.write_str("empty Temporal value"),
            DuperTemporalTryFromError::InvalidChar(temporal, pos) => f.write_fmt(format_args!(
                "invalid character in position {pos} of Temporal value {temporal}"
            )),
            DuperTemporalTryFromError::InvalidIdentifier(error) => {
                f.write_fmt(format_args!("invalid identifier: {error}"))
            }
        }
    }
}

impl std::error::Error for DuperTemporalTryFromError<'_> {}

impl<'a> From<DuperTemporalIdentifierTryFromError<'a>> for DuperTemporalTryFromError<'a> {
    fn from(value: DuperTemporalIdentifierTryFromError<'a>) -> Self {
        DuperTemporalTryFromError::InvalidIdentifier(value)
    }
}

#[cfg(test)]
mod ast_tests {
    use std::borrow::Cow;

    use crate::DuperIdentifier;

    #[test]
    fn valid_identifiers() {
        let input = "Regular";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));

        let input = "ConcatWords";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));

        let input = "SCREAMING";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));

        let input = "Numbered123";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));

        let input = "Upper_Snake_case";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));

        let input = "Upper-Kebab-case";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));

        let input = "IPv4Address";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));

        let input = "ISO639-3";
        assert!(
            matches!(DuperIdentifier::try_from(Cow::Borrowed(input)).unwrap(),
                DuperIdentifier(Cow::Borrowed(str)) if str == input
            )
        );
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Borrowed(str)) if str == input
        ));
    }

    #[test]
    fn lossy_identifiers() {
        let input = "NoÜnicodeÇharacters";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Owned(str)) if str == "Nonicodeharacters"
        ));

        let input = "noStartingLowercase";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Owned(str)) if str == "NoStartingLowercase"
        ));

        let input = "No Space";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Owned(str)) if str == "NoSpace"
        ));

        let input = "NoEndingHyphen-";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Owned(str)) if str == "NoEndingHyphen"
        ));

        let input = "NoEndingUnderscore_";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Owned(str)) if str == "NoEndingUnderscore"
        ));

        let input = "No-_HyphenThenUnderscore";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Owned(str)) if str == "No-HyphenThenUnderscore"
        ));

        let input = "No_-UnderscoreThenHyphen";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(matches!(
            DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).unwrap(),
            DuperIdentifier(Cow::Owned(str)) if str == "No_UnderscoreThenHyphen"
        ));
    }

    #[test]
    fn invalid_identifiers() {
        let input = "";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).is_err());

        let input = "1NoStartingNumber";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).is_err());

        let input = "-NoStartingHyphen";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).is_err());

        let input = "_NoStartingUnderscore";
        assert!(DuperIdentifier::try_from(Cow::Borrowed(input)).is_err());
        assert!(DuperIdentifier::try_from_lossy(Cow::Borrowed(input)).is_err());
    }
}
