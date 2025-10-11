use std::borrow::Cow;

use duper::{DuperInner, DuperParser, DuperValue};
use serde_core::Deserialize;
use serde_core::de::{self, DeserializeSeed, IntoDeserializer, Visitor};

pub struct Deserializer<'de> {
    value: Option<DuperValue<'de>>,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Result<Self, String> {
        let value = DuperParser::parse_duper(input).map_err(|e| format!("Parse error: {e:?}"))?;
        Ok(Self { value: Some(value) })
    }

    pub fn from_value(value: DuperValue<'de>) -> Self {
        Self { value: Some(value) }
    }
}

pub fn from_str<'a, T>(input: &'a str) -> Result<T, String>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(input)?;
    let t =
        T::deserialize(&mut deserializer).map_err(|e| format!("Deserialization error: {e:?}"))?;
    Ok(t)
}

pub fn from_value<'a, T>(value: DuperValue<'a>) -> Result<T, String>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_value(value);
    let t =
        T::deserialize(&mut deserializer).map_err(|e| format!("Deserialization error: {e:?}"))?;
    Ok(t)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = de::value::Error;

    // --- Deserialize DuperValue ---

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.take() {
            Some(DuperValue {
                inner: DuperInner::Object(object),
                ..
            }) => {
                let map = MapDeserializer::new(object);
                visitor.visit_map(map)
            }
            Some(DuperValue {
                inner: DuperInner::Array(array),
                ..
            }) => {
                let seq = SequenceDeserializer::new(array);
                visitor.visit_seq(seq)
            }
            Some(DuperValue {
                inner: DuperInner::Tuple(tuple),
                ..
            }) => {
                let seq = SequenceDeserializer::new(tuple);
                visitor.visit_seq(seq)
            }
            Some(DuperValue {
                inner: DuperInner::String(string),
                ..
            }) => visitor.visit_str(&string),
            Some(DuperValue {
                inner: DuperInner::Bytes(bytes),
                ..
            }) => visitor.visit_bytes(&bytes),
            Some(DuperValue {
                inner: DuperInner::Integer(integer),
                ..
            }) => visitor.visit_i64(integer),
            Some(DuperValue {
                inner: DuperInner::Float(float),
                ..
            }) => visitor.visit_f64(float),
            Some(DuperValue {
                inner: DuperInner::Boolean(boolean),
                ..
            }) => visitor.visit_bool(boolean),
            Some(DuperValue {
                inner: DuperInner::Null,
                ..
            }) => visitor.visit_none(),
            None => Err(de::Error::custom("already consumed deserializer value")),
        }
    }

    // --- Known values ---

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.take() {
            Some(DuperValue {
                inner: DuperInner::Null,
                ..
            })
            | None => visitor.visit_none(),
            value => {
                self.value = value;
                visitor.visit_some(self)
            }
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.take() {
            Some(DuperValue {
                inner: DuperInner::String(string),
                ..
            }) => visitor.visit_enum(string.into_deserializer()),
            Some(DuperValue {
                inner: DuperInner::Object(mut object),
                ..
            }) if object.len() == 1 => {
                let pair = object.remove(0);
                visitor.visit_enum(EnumDeserializer {
                    variant: pair.0,
                    value: pair.1,
                })
            }
            value => Err(de::Error::custom(format!(
                "expected string or single-keyed object for enum, found {:?}",
                value
            ))),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // --- Others ---

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SequenceDeserializer<'de> {
    iter: std::vec::IntoIter<DuperValue<'de>>,
}

impl<'de> SequenceDeserializer<'de> {
    fn new(vec: Vec<DuperValue<'de>>) -> Self {
        Self {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SequenceDeserializer<'de> {
    type Error = de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed
                .deserialize(&mut Deserializer::from_value(value))
                .map(Some),
            None => Ok(None),
        }
    }
}

struct MapDeserializer<'de> {
    iter: std::vec::IntoIter<(Cow<'de, str>, DuperValue<'de>)>,
    value: Option<DuperValue<'de>>,
}

impl<'de> MapDeserializer<'de> {
    fn new(vec: Vec<(Cow<'de, str>, DuperValue<'de>)>) -> Self {
        Self {
            iter: vec.into_iter(),
            value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapDeserializer<'de> {
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(&mut Deserializer::from_value(value)),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

struct EnumDeserializer<'de> {
    variant: Cow<'de, str>,
    value: DuperValue<'de>,
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = de::value::Error;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.variant.into_deserializer())?;
        Ok((
            variant,
            VariantDeserializer {
                value: Some(self.value),
            },
        ))
    }
}

struct VariantDeserializer<'de> {
    value: Option<DuperValue<'de>>,
}

impl<'de> de::VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = de::value::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value.map(|value| value.inner) {
            Some(DuperInner::Null) => Ok(()),
            Some(_) => Err(de::Error::custom("expected null for unit variant")),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(&mut Deserializer::from_value(value)),
            None => Err(de::Error::custom("expected value for newtype variant")),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.map(|value| value.inner) {
            Some(DuperInner::Array(vec)) => {
                let seq = SequenceDeserializer::new(vec);
                visitor.visit_seq(seq)
            }
            Some(_) => Err(de::Error::custom("expected array for tuple variant")),
            None => Err(de::Error::custom("expected value for tuple variant")),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.map(|value| value.inner) {
            Some(DuperInner::Object(obj)) => {
                let map = MapDeserializer::new(obj);
                visitor.visit_map(map)
            }
            Some(_) => Err(de::Error::custom("expected object for struct variant")),
            None => Err(de::Error::custom("expected value for struct variant")),
        }
    }
}
