pub mod de;
pub mod error;
pub mod meta;
pub mod ser;

use std::borrow::Cow;

use serde_core::{
    Deserializer, Serialize,
    de::{Deserialize, Error, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, SerializeTuple},
};

use crate::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTuple, DuperValue,
};

impl<'a> Serialize for DuperValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'a> Serialize for DuperIdentifier<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'a> Serialize for DuperInner<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match &self {
            DuperInner::Object(object) => {
                let mut map = serializer.serialize_map(Some(object.len()))?;
                for (key, value) in object.iter() {
                    map.serialize_entry(key.as_ref(), value)?;
                }
                map.end()
            }
            DuperInner::Array(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for element in array.iter() {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
            DuperInner::Tuple(tuple) => {
                let mut tup = serializer.serialize_tuple(tuple.len())?;
                for element in tuple.iter() {
                    tup.serialize_element(element)?;
                }
                tup.end()
            }
            DuperInner::String(string) => serializer.serialize_str(string.as_ref()),
            DuperInner::Bytes(bytes) => serializer.serialize_bytes(bytes.as_ref()),
            DuperInner::Temporal(temporal) => serializer.serialize_str(temporal.as_ref()),
            DuperInner::Integer(integer) => serializer.serialize_i64(*integer),
            DuperInner::Float(float) => serializer.serialize_f64(*float),
            DuperInner::Boolean(boolean) => serializer.serialize_bool(*boolean),
            DuperInner::Null => serializer.serialize_none(),
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
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Boolean(v),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Integer(v),
                })
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(v) = i64::try_from(v) {
                    Ok(DuperValue {
                        identifier: None,
                        inner: DuperInner::Integer(v),
                    })
                } else if let float = v as f64
                    && float as i128 == v
                {
                    Ok(DuperValue {
                        identifier: None,
                        inner: DuperInner::Float(float),
                    })
                } else {
                    Ok(DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("I128"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperInner::String(DuperString::from(v.to_string())),
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
                    Ok(DuperValue {
                        identifier: None,
                        inner: DuperInner::Integer(v),
                    })
                } else if let float = v as f64
                    && float as u64 == v
                {
                    Ok(DuperValue {
                        identifier: None,
                        inner: DuperInner::Float(float),
                    })
                } else {
                    Ok(DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("U64"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperInner::String(DuperString::from(v.to_string())),
                    })
                }
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(v) = i64::try_from(v) {
                    Ok(DuperValue {
                        identifier: None,
                        inner: DuperInner::Integer(v),
                    })
                } else if let float = v as f64
                    && float as u128 == v
                {
                    Ok(DuperValue {
                        identifier: None,
                        inner: DuperInner::Float(float),
                    })
                } else {
                    Ok(DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("U128"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperInner::String(DuperString::from(v.to_string())),
                    })
                }
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Float(v),
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
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::String(DuperString::from(Cow::Borrowed(v))),
                })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::String(DuperString::from(v)),
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
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Bytes(DuperBytes::from(Cow::Borrowed(v))),
                })
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Bytes(DuperBytes::from(v)),
                })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Null,
                })
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
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Tuple(DuperTuple::from(vec![])),
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
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Array(DuperArray::from(vec)),
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
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Object(DuperObject::try_from(vec).map_err(Error::custom)?),
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
    use insta::assert_snapshot;
    use serde::{Deserialize, Serialize};

    use crate::{
        DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
        DuperTuple, DuperValue, PrettyPrinter,
        serde::{de::Deserializer, ser::Serializer},
    };

    fn serialize_duper(value: &DuperValue<'_>) -> String {
        let ser = value
            .serialize(&mut Serializer::new())
            .expect("should serialize");
        PrettyPrinter::new(false, "  ").unwrap().pretty_print(ser)
    }

    fn deserialize_duper(value: &str) -> DuperValue<'_> {
        DuperValue::deserialize(&mut Deserializer::from_string(value).expect("should parse"))
            .expect("should deserialize")
    }

    #[test]
    fn serialize_object() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: DuperInner::Object(DuperObject(vec![(
                DuperKey::from("foo"),
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                    inner: DuperInner::Object(DuperObject(vec![])),
                },
            )])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialize_array() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray(vec![])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: DuperInner::Array(DuperArray(vec![DuperValue {
                identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                inner: DuperInner::Array(DuperArray(vec![])),
            }])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialize_tuple() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Tuple(DuperTuple(vec![])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: DuperInner::Tuple(DuperTuple(vec![DuperValue {
                identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                inner: DuperInner::Tuple(DuperTuple(vec![])),
            }])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(
            DuperValue {
                identifier: None,
                // Unfortunately, Serde deserializes non-unit tuples into arrays
                inner: DuperInner::Array(DuperArray(vec![DuperValue {
                    identifier: None,
                    inner: DuperInner::Tuple(DuperTuple(vec![])),
                }])),
            },
            deserialized,
        );
    }

    #[test]
    fn serialize_scalars() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![
                (
                    DuperKey::from("string"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::String(DuperString::from("Hello world!")),
                    },
                ),
                (
                    DuperKey::from("bytes"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Bytes(DuperBytes::from(&br"/\"[..])),
                    },
                ),
                (
                    DuperKey::from("integer"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Integer(1337),
                    },
                ),
                (
                    DuperKey::from("float"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Float(8.25),
                    },
                ),
                (
                    DuperKey::from("boolean"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Boolean(true),
                    },
                ),
                (
                    DuperKey::from("null"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Null,
                    },
                ),
            ])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("MyScalars").expect("valid identifier")),
            inner: DuperInner::Array(DuperArray(vec![
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyString").expect("valid identifier"),
                    ),
                    inner: DuperInner::String(DuperString::from("Hello world!")),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyBytes").expect("valid identifier"),
                    ),
                    inner: DuperInner::Bytes(DuperBytes::from(&br"/\"[..])),
                },
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("MyInt").expect("valid identifier")),
                    inner: DuperInner::Integer(1337),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyFloat").expect("valid identifier"),
                    ),
                    inner: DuperInner::Float(8.25),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyBool").expect("valid identifier"),
                    ),
                    inner: DuperInner::Boolean(true),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("Mysterious").expect("valid identifier"),
                    ),
                    inner: DuperInner::Null,
                },
            ])),
        };
        let serialized = serialize_duper(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_duper(&serialized);
        assert_eq!(value, deserialized);
    }
}
