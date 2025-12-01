use napi::{
    JsValue,
    bindgen_prelude::{JsObjectValue, Object},
};
use serde_core::{
    de::{DeserializeSeed, Error, MapAccess},
    forward_to_deserialize_any,
};

pub(crate) struct DuperMetaDeserializer<'env> {
    pub(crate) object: Object<'env>,
}

impl<'de, 'env> serde_core::Deserializer<'de> for DuperMetaDeserializer<'env> {
    type Error = serde_core::de::value::Error;

    fn is_human_readable(&self) -> bool {
        true
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let keys = self.object.get_property_names().map_err(|err| {
            serde_core::de::value::Error::custom(format!("Failed to get property names: {err}"))
        })?;
        let array_length = keys.get_array_length().map_err(|err| {
            serde_core::de::value::Error::custom("Error accessing array length of object keys")
        })?;
        visitor.visit_map(DuperMetaMapAccess {
            object: self.object,
            keys,
            array_length,
            curr_value: None,
            i: 0,
        })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let len = fields.len();
        if name == "DuperValue" && len == 3 {
            self.deserialize_map(visitor)
        } else {
            Err(serde_core::de::value::Error::custom(format!(
                "expected DuperValue struct with 3 fields, found {name} with {len} field(s)"
            )))
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf identifier ignored_any
        option unit unit_struct newtype_struct seq tuple tuple_struct enum
    }
}

struct DuperMetaMapAccess<'env> {
    object: Object<'env>,
    keys: Object<'env>,
    array_length: u32,
    curr_value: Option<Object<'env>>,
    i: u32,
}

impl<'de, 'env> MapAccess<'de> for DuperMetaMapAccess<'env> {
    type Error = serde_core::de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let array_length = self.keys.get_array_length().map_err(|err| {
            serde_core::de::value::Error::custom("Error accessing array length of object keys")
        })?;
        if self.i < array_length {
            let key: String = self.keys.get_element(self.i).map_err(|err| {
                serde_core::de::value::Error::custom(format!(
                    "Error accessing index {} of object keys",
                    self.i
                ))
            })?;
            self.curr_value = self
                .object
                .get_property(key.as_str().into())
                .map_err(|err| {
                    serde_core::de::value::Error::custom(format!(
                        "Error accessing property {} of object",
                        &key
                    ))
                })?;
            seed.deserialize(deserializer).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // It doesn't make a difference whether the colon is parsed at the end
        // of `next_key_seed` or at the beginning of `next_value_seed`. In this
        // case the code is a bit simpler having it here.
        if self.de.next_char()? != ':' {
            return Err(Error::ExpectedMapColon);
        }
        // Deserialize a map value.
        seed.deserialize(&mut *self.de)
    }
}

pub(crate) struct DuperMetaInnerDeserializer<'env> {
    pub(crate) object: Object<'env>,
}

impl<'de, 'env> serde_core::Deserializer<'de> for DuperMetaInnerDeserializer<'env> {
    type Error = serde_core::de::value::Error;

    fn is_human_readable(&self) -> bool {
        true
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let keys = self.object.get_property_names().map_err(|err| {
            serde_core::de::value::Error::custom(format!("Failed to get property names: {err}"))
        })?;
        visitor.visit_map(DuperMetaMapAccess {
            object: self.object,
            keys,
            i: 0,
        })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let len = fields.len();
        if name == "DuperValue" && len == 3 {
            self.deserialize_map(visitor)
        } else {
            Err(serde_core::de::value::Error::custom(format!(
                "expected DuperValue struct with 3 fields, found {name} with {len} field(s)"
            )))
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf identifier ignored_any
        option unit unit_struct newtype_struct seq tuple tuple_struct enum
    }
}
