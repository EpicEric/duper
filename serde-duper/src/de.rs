use std::borrow::Cow;

use duper::{DuperArray, DuperKey};
use duper::{DuperInner, DuperParser, DuperValue};
use serde_core::{
    Deserialize,
    de::{self, DeserializeSeed, IntoDeserializer, Visitor},
    forward_to_deserialize_any,
};

use crate::Error;

pub struct Deserializer<'de> {
    value: Option<DuperValue<'de>>,
}

/// A structure that deserializes Duper values into Rust values.
impl<'de> Deserializer<'de> {
    /// Creates a Duper deserializer from a `&str`.
    pub fn from_string(input: &'de str) -> Result<Self, Error> {
        let value = DuperParser::parse_duper_value(input)?;
        Ok(Self { value: Some(value) })
    }

    /// Creates a Duper deserializer from a [`DuperValue`].
    pub fn from_value(value: DuperValue<'de>) -> Self {
        Self { value: Some(value) }
    }
}

/// Deserialize an instance of type `T` from a str slice of Duper text.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: Vec<u8>,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `j` is `&str`
///     let j = r#"
///         User({
///             fingerprint: b"\xF9\xBA\x14\x3B\x95\xFF\x6D\x82",
///             location: City("Menlo Park, CA"),
///         })"#;
///
///     let u: User = serde_duper::from_string(j).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a Duper object. It can also fail if the
/// structure is correct but `T`'s implementation of [`Deserialize`] decides that
/// something is wrong with the data, for example required struct fields are
/// missing from the Duper object or some number is too big to fit in the
/// expected primitive type.
pub fn from_string<'a, T>(input: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_string(input)?;
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

/// Interpret a [`DuperValue`] as an instance of type `T`.
///
/// # Example
///
/// ```
/// use std::borrow::Cow;
/// use serde::Deserialize;
/// use serde_duper::{
///     DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject,
///     DuperString, DuperValue,
/// };
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: Vec<u8>,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `d` is `serde_duper::DuperValue`
///     let d = DuperValue {
///         identifier: Some(DuperIdentifier::try_from(Cow::Borrowed("User")).unwrap()),
///         inner: DuperInner::Object(DuperObject::try_from(vec![
///             (
///                 DuperKey::from(Cow::Borrowed("fingerprint")),
///                 DuperValue {
///                     identifier: None,
///                     inner: DuperInner::Bytes(DuperBytes::from(Cow::Borrowed(
///                         &b"\xF9\xBA\x14\x3B\x95\xFF\x6D\x82"[..],
///                     ))),
///                 }
///             ),
///             (
///                 DuperKey::from(Cow::Borrowed("location")),
///                 DuperValue {
///                     identifier: Some(
///                         DuperIdentifier::try_from(Cow::Borrowed("City")).unwrap(),
///                     ),
///                     inner: DuperInner::String(DuperString::from(
///                         Cow::Borrowed("Menlo Park, CA"),
///                     )),
///                 }
///             ),
///         ]).unwrap()),
///     };
///
///     let u: User = serde_duper::from_value(d).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a Duper object. It can also fail if the
/// structure is correct but `T`'s implementation of [`Deserialize`] decides that
/// something is wrong with the data, for example required struct fields are
/// missing from the Duper object or some number is too big to fit in the
/// expected primitive type.
pub fn from_value<'a, T>(value: DuperValue<'a>) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_value(value);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = de::value::Error;

    fn is_human_readable(&self) -> bool {
        true
    }

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
                let map = MapDeserializer::new(object.into_inner());
                visitor.visit_map(map)
            }
            Some(DuperValue {
                inner: DuperInner::Array(array),
                ..
            }) => {
                let seq = SequenceDeserializer::new(array.into_inner());
                visitor.visit_seq(seq)
            }
            Some(DuperValue {
                inner: DuperInner::Tuple(tuple),
                ..
            }) if tuple.is_empty() => visitor.visit_unit(),
            Some(DuperValue {
                inner: DuperInner::Tuple(tuple),
                ..
            }) => {
                let seq = TupleDeserializer::new(tuple.into_inner());
                visitor.visit_seq(seq)
            }
            Some(DuperValue {
                inner: DuperInner::String(string),
                ..
            }) => match string.into_inner() {
                Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
                Cow::Owned(s) => visitor.visit_string(s),
            },
            Some(DuperValue {
                inner: DuperInner::Bytes(bytes),
                ..
            }) => match bytes.into_inner() {
                Cow::Borrowed(b) => visitor.visit_borrowed_bytes(b),
                Cow::Owned(b) => visitor.visit_byte_buf(b),
            },
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

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &self.value {
            Some(DuperValue {
                inner: DuperInner::Null,
                ..
            })
            | None => visitor.visit_none(),
            _ => visitor.visit_some(self),
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

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.take() {
            Some(DuperValue {
                inner: DuperInner::Array(array),
                ..
            }) if array.len() == len => {
                let seq = TupleDeserializer::new(array.into_inner());
                visitor.visit_seq(seq)
            }
            Some(DuperValue {
                inner: DuperInner::Tuple(tuple),
                ..
            }) if tuple.len() == len => {
                let seq = TupleDeserializer::new(tuple.into_inner());
                visitor.visit_seq(seq)
            }
            Some(value) => Err(de::Error::custom(format!(
                "expected tuple of len {len}, found {:?}",
                value.inner
            ))),
            None => Err(de::Error::custom("already consumed deserializer value")),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
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
            }) => visitor.visit_enum(string.as_ref().into_deserializer()),
            Some(DuperValue {
                inner: DuperInner::Object(object),
                ..
            }) if object.len() == 1 => {
                let mut object = object.into_inner();
                let pair = object.remove(0);
                visitor.visit_enum(EnumDeserializer {
                    variant: pair.0,
                    value: pair.1,
                })
            }
            Some(value) => Err(de::Error::custom(format!(
                "expected string or single-keyed object for enum, found {:?}",
                value.inner
            ))),
            None => Err(de::Error::custom("already consumed deserializer value")),
        }
    }

    // --- Others ---

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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.value = match self.value.take() {
            Some(DuperValue {
                inner: DuperInner::Bytes(bytes),
                identifier,
            }) => {
                // Ugly hack to deal with poor Serde support for bytes
                Some(DuperValue {
                    identifier,
                    inner: DuperInner::Array(DuperArray::from(
                        bytes
                            .into_inner()
                            .iter()
                            .map(|v| DuperValue {
                                identifier: None,
                                inner: DuperInner::Integer(i64::from(*v)),
                            })
                            .collect::<Vec<_>>(),
                    )),
                })
            }
            value => value,
        };
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf identifier map struct ignored_any
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

struct TupleDeserializer<'de> {
    iter: std::vec::IntoIter<DuperValue<'de>>,
    len: usize,
}

impl<'de> TupleDeserializer<'de> {
    fn new(vec: Vec<DuperValue<'de>>) -> Self {
        let len = vec.len();
        Self {
            iter: vec.into_iter(),
            len,
        }
    }
}

impl<'de> de::SeqAccess<'de> for TupleDeserializer<'de> {
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

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapDeserializer<'de> {
    iter: std::vec::IntoIter<(DuperKey<'de>, DuperValue<'de>)>,
    value: Option<DuperValue<'de>>,
}

impl<'de> MapDeserializer<'de> {
    fn new(vec: Vec<(DuperKey<'de>, DuperValue<'de>)>) -> Self {
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
                seed.deserialize(key.as_ref().into_deserializer()).map(Some)
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
    variant: DuperKey<'de>,
    value: DuperValue<'de>,
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = de::value::Error;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.variant.as_ref().into_deserializer())?;
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
            Some(DuperInner::Tuple(vec)) if vec.is_empty() => Ok(()),
            Some(DuperInner::Null) => Ok(()),
            Some(value) => Err(de::Error::custom(format!(
                "expected null for unit variant, found {value:?}"
            ))),
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
                let seq = SequenceDeserializer::new(vec.into_inner());
                visitor.visit_seq(seq)
            }
            Some(DuperInner::Tuple(vec)) => {
                let seq = TupleDeserializer::new(vec.into_inner());
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
                let map = MapDeserializer::new(obj.into_inner());
                visitor.visit_map(map)
            }
            Some(_) => Err(de::Error::custom("expected object for struct variant")),
            None => Err(de::Error::custom("expected value for struct variant")),
        }
    }
}
