#[cfg(feature = "serde")]
use crate::{DuperArray, DuperBytes, DuperInner, DuperKey, DuperObject, DuperString, DuperValue};
#[cfg(feature = "serde")]
use serde_core::de::Visitor;
#[cfg(feature = "serde")]
use std::borrow::Cow;

#[cfg(feature = "serde")]
impl<'a> serde_core::Serialize for DuperValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match &self.inner {
            crate::DuperInner::Object(object) => {
                use serde_core::ser::SerializeMap;

                let mut map = serializer.serialize_map(Some(object.0.len()))?;
                for (key, value) in object.0.iter() {
                    use serde_core::ser::SerializeMap;
                    map.serialize_entry(key.as_ref(), value)?;
                }
                map.end()
            }
            crate::DuperInner::Array(array) => {
                use serde_core::ser::SerializeSeq;

                let mut seq = serializer.serialize_seq(Some(array.0.len()))?;
                for value in array.0.iter() {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            crate::DuperInner::Tuple(tuple) => {
                use serde_core::ser::SerializeTuple;

                let mut seq = serializer.serialize_tuple(tuple.0.len())?;
                for value in tuple.0.iter() {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            crate::DuperInner::String(string) => serializer.serialize_str(string.as_ref()),
            crate::DuperInner::Bytes(bytes) => serializer.serialize_bytes(bytes.as_ref()),
            crate::DuperInner::Integer(integer) => serializer.serialize_i64(*integer),
            crate::DuperInner::Float(float) => serializer.serialize_f64(*float),
            crate::DuperInner::Boolean(boolean) => serializer.serialize_bool(*boolean),
            crate::DuperInner::Null => serializer.serialize_none(),
        }
    }
}

#[cfg(feature = "serde")]
struct DuperValueDeserializerVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for DuperValueDeserializerVisitor {
    type Value = DuperValue<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid Duper value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Boolean(v),
        })
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v),
        })
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        if let Ok(i) = i64::try_from(v) {
            self.visit_i64(i)
        } else {
            self.visit_f64(v as f64)
        }
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_f64(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Float(v),
        })
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_str(&v.to_string())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::String(DuperString::from(Cow::Borrowed(v))),
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::String(DuperString::from(Cow::Owned(v))),
        })
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_byte_buf(v.to_vec())
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Bytes(DuperBytes::from(Cow::Borrowed(v))),
        })
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Bytes(DuperBytes::from(Cow::Owned(v))),
        })
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Null,
        })
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        serde_core::Deserialize::deserialize(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_none()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        serde_core::Deserialize::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::SeqAccess<'de>,
    {
        let mut values = Vec::new();

        while let Some(value) = seq.next_element()? {
            values.push(value);
        }

        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray::from(values)),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::MapAccess<'de>,
    {
        let mut entries = Vec::new();

        while let Some((key, value)) = map.next_entry()? {
            entries.push((DuperKey::from(Cow::Owned(key)), value));
        }

        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject::from(entries)),
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> serde_core::Deserialize<'de> for DuperValue<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        deserializer.deserialize_any(DuperValueDeserializerVisitor)
    }
}
