//! Serde support for Duper. This requires the `serde` feature flag to be enabled.
//!
//! Included in this module are:
//! - (De)serialize implementations to work with Duper values directly;
//! - [`ser`] / [`de`]: (De)serializer implementations for native types;
//! - [`meta`]: Meta-(de)serialization, for wider support;
//! - [`temporal`]: Custom Serde support for Temporal values;
//! - [`error`]: A Duper-specific Serde error.

pub mod de;
pub mod error;
pub mod meta;
pub mod ser;
pub mod temporal;

use std::borrow::Cow;

use serde_core::{
    Deserializer, Serialize,
    de::{Deserialize, Error, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, SerializeTuple},
};

use crate::{
    DuperIdentifier, DuperKey, DuperObject, DuperTemporal, DuperTemporalIdentifier, DuperValue,
};

impl<'a> Serialize for DuperIdentifier<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'a> Serialize for DuperTemporalIdentifier<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'a> Serialize for DuperValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match &self {
            DuperValue::Object { inner, .. } => {
                let mut map = serializer.serialize_map(Some(inner.len()))?;
                for (key, value) in inner.iter() {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
            DuperValue::Array { inner, .. } => {
                let mut seq = serializer.serialize_seq(Some(inner.len()))?;
                for element in inner.iter() {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
            DuperValue::Tuple { inner, .. } => {
                let mut tup = serializer.serialize_tuple(inner.len())?;
                for element in inner.iter() {
                    tup.serialize_element(element)?;
                }
                tup.end()
            }
            DuperValue::String { inner, .. } => serializer.serialize_str(inner.as_ref()),
            DuperValue::Bytes { inner, .. } => serializer.serialize_bytes(inner.as_ref()),
            DuperValue::Temporal(temporal) => match temporal {
                DuperTemporal::Instant { inner } => {
                    serializer.serialize_newtype_struct("Instant", inner.as_ref())
                }
                DuperTemporal::ZonedDateTime { inner } => {
                    serializer.serialize_newtype_struct("ZonedDateTime", inner.as_ref())
                }
                DuperTemporal::PlainDate { inner } => {
                    serializer.serialize_newtype_struct("PlainDate", inner.as_ref())
                }
                DuperTemporal::PlainTime { inner } => {
                    serializer.serialize_newtype_struct("PlainTime", inner.as_ref())
                }
                DuperTemporal::PlainDateTime { inner } => {
                    serializer.serialize_newtype_struct("PlainDateTime", inner.as_ref())
                }
                DuperTemporal::PlainYearMonth { inner } => {
                    serializer.serialize_newtype_struct("PlainYearMonth", inner.as_ref())
                }
                DuperTemporal::PlainMonthDay { inner } => {
                    serializer.serialize_newtype_struct("PlainMonthDay", inner.as_ref())
                }
                DuperTemporal::Duration { inner } => {
                    serializer.serialize_newtype_struct("Duration", inner.as_ref())
                }
                DuperTemporal::Unspecified { inner, .. } => {
                    serializer.serialize_str(inner.as_ref())
                }
            },
            DuperValue::Integer { inner, .. } => serializer.serialize_i64(*inner),
            DuperValue::Float { inner, .. } => serializer.serialize_f64(*inner),
            DuperValue::Boolean { inner, .. } => serializer.serialize_bool(*inner),
            DuperValue::Null { .. } => serializer.serialize_none(),
        }
    }
}

impl<'a> Serialize for DuperKey<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for DuperValue<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DuperVisitor;

        impl<'de> Visitor<'de> for DuperVisitor {
            type Value = DuperValue<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Duper value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::Boolean {
                    identifier: None,
                    inner: v,
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::Integer {
                    identifier: None,
                    inner: v,
                })
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(v) = i64::try_from(v) {
                    Ok(DuperValue::Integer {
                        identifier: None,
                        inner: v,
                    })
                } else if let float = v as f64
                    && float as i128 == v
                {
                    Ok(DuperValue::Float {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("I128"))
                                .expect("valid identifier"),
                        ),
                        inner: float,
                    })
                } else {
                    Ok(DuperValue::String {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("I128"))
                                .expect("valid identifier"),
                        ),
                        inner: Cow::Owned(v.to_string()),
                    })
                }
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(v) = i64::try_from(v) {
                    Ok(DuperValue::Integer {
                        identifier: None,
                        inner: v,
                    })
                } else if let float = v as f64
                    && float as u64 == v
                {
                    Ok(DuperValue::Float {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("U64"))
                                .expect("valid identifier"),
                        ),
                        inner: float,
                    })
                } else {
                    Ok(DuperValue::String {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("U64"))
                                .expect("valid identifier"),
                        ),
                        inner: Cow::Owned(v.to_string()),
                    })
                }
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(v) = i64::try_from(v) {
                    Ok(DuperValue::Integer {
                        identifier: None,
                        inner: v,
                    })
                } else if let float = v as f64
                    && float as u128 == v
                {
                    Ok(DuperValue::Float {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("U128"))
                                .expect("valid identifier"),
                        ),
                        inner: float,
                    })
                } else {
                    Ok(DuperValue::String {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("U128"))
                                .expect("valid identifier"),
                        ),
                        inner: Cow::Owned(v.to_string()),
                    })
                }
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::Float {
                    identifier: None,
                    inner: v,
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_string(v.to_string())
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::String {
                    identifier: None,
                    inner: Cow::Borrowed(v),
                })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::String {
                    identifier: None,
                    inner: Cow::Owned(v),
                })
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_byte_buf(v.to_vec())
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::Bytes {
                    identifier: None,
                    inner: Cow::Borrowed(v),
                })
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::Bytes {
                    identifier: None,
                    inner: Cow::Owned(v),
                })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::Null { identifier: None })
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_any(self)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue::Tuple {
                    identifier: None,
                    inner: vec![],
                })
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_any(self)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = seq
                    .size_hint()
                    .map(|len| Vec::with_capacity(len))
                    .unwrap_or_default();
                while let Some(element) = seq.next_element()? {
                    vec.push(element);
                }
                Ok(DuperValue::Array {
                    identifier: None,
                    inner: vec,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut vec = map
                    .size_hint()
                    .map(|len| Vec::with_capacity(len))
                    .unwrap_or_default();
                while let Some(element) = map.next_entry()? {
                    vec.push(element);
                }
                Ok(DuperValue::Object {
                    identifier: None,
                    inner: DuperObject::try_from(vec).map_err(Error::custom)?,
                })
            }
        }

        deserializer.deserialize_any(DuperVisitor {})
    }
}

impl<'de> Deserialize<'de> for DuperKey<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DuperKeyVisitor;

        impl<'de> Visitor<'de> for DuperKeyVisitor {
            type Value = DuperKey<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Duper key")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_string(v.to_string())
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperKey::from(Cow::Borrowed(v)))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperKey::from(v))
            }
        }

        deserializer.deserialize_any(DuperKeyVisitor)
    }
}

impl<'de> Deserialize<'de> for DuperIdentifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DuperIdentifierVisitor;

        impl<'de> Visitor<'de> for DuperIdentifierVisitor {
            type Value = DuperIdentifier<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Duper identifier")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_string(v.to_string())
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                DuperIdentifier::try_from_lossy(Cow::Borrowed(v)).map_err(Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                DuperIdentifier::try_from_lossy(Cow::Owned(v)).map_err(Error::custom)
            }
        }

        deserializer.deserialize_any(DuperIdentifierVisitor)
    }
}

#[cfg(test)]
mod serde_tests {
    use std::borrow::Cow;

    use indexmap::IndexMap;
    use insta::assert_snapshot;
    use serde::{Deserialize, Serialize};

    use crate::{
        DuperIdentifier, DuperKey, DuperObject, DuperValue, PrettyPrinter,
        serde::{de::Deserializer, ser::Serializer},
    };

    fn serialize_duper(value: &DuperValue<'_>) -> String {
        let ser = value
            .serialize(&mut Serializer::new())
            .expect("should serialize");
        PrettyPrinter::new(false, "  ").unwrap().pretty_print(&ser)
    }

    fn deserialize_duper(value: &str) -> DuperValue<'_> {
        DuperValue::deserialize(&mut Deserializer::from_string(value).expect("should parse"))
            .expect("should deserialize")
    }

    #[test]
    fn serialize_object() {
        let value = DuperValue::Object {
            identifier: None,
            inner: DuperObject(IndexMap::new()),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);

        let value = DuperValue::Object {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: DuperObject::try_from(vec![(
                DuperKey::from("foo"),
                DuperValue::Object {
                    identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                    inner: DuperObject(IndexMap::new()),
                },
            )])
            .unwrap(),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialize_array() {
        let value = DuperValue::Array {
            identifier: None,
            inner: vec![],
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);

        let value = DuperValue::Array {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: vec![DuperValue::Array {
                identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                inner: vec![],
            }],
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialize_tuple() {
        let value = DuperValue::Tuple {
            identifier: None,
            inner: vec![],
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);

        let value = DuperValue::Tuple {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: vec![DuperValue::Tuple {
                identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                inner: vec![],
            }],
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(
            // Unfortunately, Serde deserializes non-unit tuples into arrays
            DuperValue::Array {
                identifier: None,
                inner: vec![DuperValue::Tuple {
                    identifier: None,
                    inner: vec![],
                }],
            },
            deserialized,
        );
    }

    #[test]
    fn serialize_scalars() {
        let value = DuperValue::Object {
            identifier: None,
            inner: DuperObject::try_from(vec![
                (
                    DuperKey::from("string"),
                    DuperValue::String {
                        identifier: None,
                        inner: Cow::Borrowed("Hello world!"),
                    },
                ),
                (
                    DuperKey::from("bytes"),
                    DuperValue::Bytes {
                        identifier: None,
                        inner: Cow::Borrowed(&br"/\"[..]),
                    },
                ),
                (
                    DuperKey::from("temporal"),
                    DuperValue::try_plain_time_from(Cow::Borrowed("16:20:00"))
                        .expect("valid PlainTime"),
                ),
                (
                    DuperKey::from("integer"),
                    DuperValue::Integer {
                        identifier: None,
                        inner: 1337,
                    },
                ),
                (
                    DuperKey::from("float"),
                    DuperValue::Float {
                        identifier: None,
                        inner: 8.25,
                    },
                ),
                (
                    DuperKey::from("boolean"),
                    DuperValue::Boolean {
                        identifier: None,
                        inner: true,
                    },
                ),
                (
                    DuperKey::from("null"),
                    DuperValue::Null { identifier: None },
                ),
            ])
            .unwrap(),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(
            deserialized,
            DuperValue::Object {
                identifier: None,
                inner: DuperObject::try_from(vec![
                    (
                        DuperKey::from("string"),
                        DuperValue::String {
                            identifier: None,
                            inner: Cow::Borrowed("Hello world!"),
                        },
                    ),
                    (
                        DuperKey::from("bytes"),
                        DuperValue::Bytes {
                            identifier: None,
                            inner: Cow::Borrowed(&br"/\"[..]),
                        },
                    ),
                    (
                        DuperKey::from("temporal"),
                        DuperValue::String {
                            identifier: None,
                            inner: Cow::Borrowed("16:20:00")
                        }
                    ),
                    (
                        DuperKey::from("integer"),
                        DuperValue::Integer {
                            identifier: None,
                            inner: 1337,
                        },
                    ),
                    (
                        DuperKey::from("float"),
                        DuperValue::Float {
                            identifier: None,
                            inner: 8.25,
                        },
                    ),
                    (
                        DuperKey::from("boolean"),
                        DuperValue::Boolean {
                            identifier: None,
                            inner: true,
                        },
                    ),
                    (
                        DuperKey::from("null"),
                        DuperValue::Null { identifier: None },
                    ),
                ])
                .unwrap(),
            }
        );

        let value = DuperValue::Array {
            identifier: Some(DuperIdentifier::try_from("MyScalars").expect("valid identifier")),
            inner: vec![
                DuperValue::String {
                    identifier: Some(
                        DuperIdentifier::try_from("MyString").expect("valid identifier"),
                    ),
                    inner: Cow::Borrowed("Hello world!"),
                },
                DuperValue::Bytes {
                    identifier: Some(
                        DuperIdentifier::try_from("MyBytes").expect("valid identifier"),
                    ),
                    inner: Cow::Borrowed(&br"/\"[..]),
                },
                DuperValue::try_unspecified_from(
                    Some(DuperIdentifier(Cow::Borrowed("MyTemporal"))),
                    Cow::Borrowed("2012-12-21"),
                )
                .unwrap(),
                DuperValue::Integer {
                    identifier: Some(DuperIdentifier::try_from("MyInt").expect("valid identifier")),
                    inner: 1337,
                },
                DuperValue::Float {
                    identifier: Some(
                        DuperIdentifier::try_from("MyFloat").expect("valid identifier"),
                    ),
                    inner: 8.25,
                },
                DuperValue::Boolean {
                    identifier: Some(
                        DuperIdentifier::try_from("MyBool").expect("valid identifier"),
                    ),
                    inner: true,
                },
                DuperValue::Null {
                    identifier: Some(
                        DuperIdentifier::try_from("Mysterious").expect("valid identifier"),
                    ),
                },
            ],
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(
            deserialized,
            DuperValue::Array {
                identifier: Some(DuperIdentifier::try_from("MyScalars").expect("valid identifier")),
                inner: vec![
                    DuperValue::String {
                        identifier: None,
                        inner: Cow::Borrowed("Hello world!"),
                    },
                    DuperValue::Bytes {
                        identifier: None,
                        inner: Cow::Borrowed(&br"/\"[..]),
                    },
                    DuperValue::String {
                        identifier: None,
                        inner: Cow::Borrowed("2012-12-21")
                    },
                    DuperValue::Integer {
                        identifier: None,
                        inner: 1337,
                    },
                    DuperValue::Float {
                        identifier: None,
                        inner: 8.25,
                    },
                    DuperValue::Boolean {
                        identifier: None,
                        inner: true,
                    },
                    DuperValue::Null { identifier: None },
                ],
            }
        );
    }
}
