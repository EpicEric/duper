use napi::{
    Env, JsString, JsValue, Unknown, ValueType,
    bindgen_prelude::{BigInt, FromNapiValue, JsObjectValue, Object, Uint8Array},
};
use serde_core::{
    de::{DeserializeSeed, Error, IntoDeserializer, MapAccess, SeqAccess},
    forward_to_deserialize_any,
};

pub(crate) struct DuperMetaDeserializer<'env> {
    pub(crate) env: &'env Env,
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
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let keys = self.object.get_property_names().map_err(|err| {
            serde_core::de::value::Error::custom(format!("failed to get property names: {err}"))
        })?;
        let array_length = keys.get_array_length().expect("checked array creation");
        visitor.visit_map(DuperMetaMapAccess {
            env: self.env,
            object: self.object,
            keys,
            array_length,
            curr_value: None,
            i: 0,
        })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf identifier ignored_any struct
        option unit unit_struct newtype_struct seq tuple tuple_struct enum
    }
}

struct DuperMetaMapAccess<'env> {
    env: &'env Env,
    object: Object<'env>,
    keys: Object<'env>,
    curr_value: Option<Unknown<'env>>,
    array_length: u32,
    i: u32,
}

impl<'de, 'env> MapAccess<'de> for DuperMetaMapAccess<'env> {
    type Error = serde_core::de::value::Error;

    fn size_hint(&self) -> Option<usize> {
        Some((self.array_length - self.i) as usize)
    }

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.i < self.array_length {
            let key: String = self.keys.get_element(self.i).map_err(|err| {
                serde_core::de::value::Error::custom(format!(
                    "error accessing index {} of object keys: {err:?}",
                    self.i
                ))
            })?;
            self.curr_value = Some(
                self.object
                    .get_property::<JsString, Unknown>(
                        self.env
                            .create_string(key.as_str())
                            .expect("checked string"),
                    )
                    .map_err(|err| {
                        serde_core::de::value::Error::custom(format!(
                            "error accessing property {} of object: {err:?}",
                            &key
                        ))
                    })?,
            );
            self.i += 1;
            seed.deserialize(key.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(value) = self.curr_value.take() {
            match value.get_type() {
                Ok(ValueType::Undefined) | Ok(ValueType::Null) => {
                    seed.deserialize(DuperMetaInnerNull)
                }
                Ok(ValueType::Boolean) => seed.deserialize(
                    value
                        .coerce_to_bool()
                        .expect("checked bool")
                        .into_deserializer(),
                ),
                Ok(ValueType::Number) => seed.deserialize(
                    value
                        .coerce_to_number()
                        .expect("checked number")
                        .get_double()
                        .expect("infallible conversion to f64")
                        .into_deserializer(),
                ),
                Ok(ValueType::String) => seed.deserialize(
                    value
                        .coerce_to_string()
                        .expect("checked string")
                        .into_utf8()
                        .expect("valid UTF-8")
                        .as_str()
                        .expect("valid conversion to &str")
                        .into_deserializer(),
                ),
                Ok(ValueType::BigInt) => seed.deserialize(
                    BigInt::from_unknown(value)
                        .expect("checked bigint")
                        .get_i64()
                        .0
                        .into_deserializer(),
                ),
                Ok(ValueType::Object) => {
                    if value.is_array().expect("array type check") {
                        seed.deserialize(DuperMetaInnerArrayDeserializer {
                            env: self.env,
                            array: value.coerce_to_object().expect("checked object"),
                        })
                    } else if let Ok(bytes) = Uint8Array::from_unknown(value.clone()) {
                        seed.deserialize(bytes.as_ref().into_deserializer())
                    } else {
                        seed.deserialize(DuperMetaInnerObjectDeserializer {
                            env: self.env,
                            object: value.coerce_to_object().expect("checked object"),
                        })
                    }
                }
                Ok(_) | Err(_) => Err(serde_core::de::value::Error::custom("unknown value type")),
            }
        } else {
            Err(serde_core::de::value::Error::custom("missing value"))
        }
    }
}

pub(crate) struct DuperMetaInnerObjectDeserializer<'env> {
    pub(crate) env: &'env Env,
    pub(crate) object: Object<'env>,
}

impl<'de, 'env> serde_core::Deserializer<'de> for DuperMetaInnerObjectDeserializer<'env> {
    type Error = serde_core::de::value::Error;

    fn is_human_readable(&self) -> bool {
        true
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let keys = self.object.get_property_names().map_err(|err| {
            serde_core::de::value::Error::custom(format!("failed to get property names: {err}"))
        })?;
        let array_length = keys.get_array_length().expect("checked array creation");
        visitor.visit_map(DuperMetaInnerObjectMapAccess {
            env: self.env,
            object: self.object,
            keys,
            array_length,
            curr_value: None,
            i: 0,
        })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf identifier ignored_any struct
        option unit unit_struct newtype_struct seq tuple tuple_struct enum
    }
}

struct DuperMetaInnerObjectMapAccess<'env> {
    env: &'env Env,
    object: Object<'env>,
    keys: Object<'env>,
    curr_value: Option<Unknown<'env>>,
    array_length: u32,
    i: u32,
}

impl<'de, 'env> MapAccess<'de> for DuperMetaInnerObjectMapAccess<'env> {
    type Error = serde_core::de::value::Error;

    fn size_hint(&self) -> Option<usize> {
        Some((self.array_length - self.i) as usize)
    }

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.i < self.array_length {
            let key: String = self.keys.get_element(self.i).map_err(|err| {
                serde_core::de::value::Error::custom(format!(
                    "error accessing index {} of object keys: {err:?}",
                    self.i
                ))
            })?;
            self.curr_value = Some(
                self.object
                    .get_property::<JsString, Unknown>(
                        self.env.create_string(key.as_str()).map_err(|err| {
                            serde_core::de::value::Error::custom(format!(
                                "error converting property {} to string: {err:?}",
                                &key
                            ))
                        })?,
                    )
                    .map_err(|err| {
                        serde_core::de::value::Error::custom(format!(
                            "error accessing property {} of object: {err:?}",
                            &key
                        ))
                    })?,
            );
            self.i += 1;
            seed.deserialize(key.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(value) = self.curr_value.take() {
            seed.deserialize(DuperMetaDeserializer {
                env: self.env,
                object: value.coerce_to_object().map_err(|err| {
                    serde_core::de::value::Error::custom(format!(
                        "error converting map value to object: {err:?}",
                    ))
                })?,
            })
        } else {
            Err(serde_core::de::value::Error::custom("missing value"))
        }
    }
}

pub(crate) struct DuperMetaInnerArrayDeserializer<'env> {
    pub(crate) env: &'env Env,
    pub(crate) array: Object<'env>,
}

impl<'de, 'env> serde_core::Deserializer<'de> for DuperMetaInnerArrayDeserializer<'env> {
    type Error = serde_core::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let array_length = self.array.get_array_length().expect("checked array");
        visitor.visit_seq(DuperMetaInnerArraySeqAccess {
            env: self.env,
            array: self.array,
            array_length,
            i: 0,
        })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf identifier ignored_any map struct
        option unit unit_struct newtype_struct tuple tuple_struct enum
    }
}

struct DuperMetaInnerArraySeqAccess<'env> {
    env: &'env Env,
    array: Object<'env>,
    array_length: u32,
    i: u32,
}

impl<'de, 'env> SeqAccess<'de> for DuperMetaInnerArraySeqAccess<'env> {
    type Error = serde_core::de::value::Error;

    fn size_hint(&self) -> Option<usize> {
        Some((self.array_length - self.i) as usize)
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.i < self.array_length {
            let object = self.array.get_element::<Object>(self.i).map_err(|err| {
                serde_core::de::value::Error::custom(format!(
                    "failed to get object from array: {err}"
                ))
            })?;
            self.i += 1;
            Ok(Some(seed.deserialize(DuperMetaDeserializer {
                env: self.env,
                object,
            })?))
        } else {
            Ok(None)
        }
    }
}

struct DuperMetaInnerNull;

impl<'de> serde_core::Deserializer<'de> for DuperMetaInnerNull {
    type Error = serde_core::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        visitor.visit_none()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf identifier ignored_any map struct
        option unit unit_struct newtype_struct seq tuple tuple_struct enum
    }
}
