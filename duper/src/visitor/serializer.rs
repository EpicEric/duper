//! Utilities for serializing Duper values.

use crate::{
    DuperTemporal,
    ast::{DuperIdentifier, DuperObject, DuperValue},
    format::{
        format_boolean, format_duper_bytes, format_duper_string, format_float, format_integer,
        format_key, format_null, format_temporal,
    },
    visitor::DuperVisitor,
};

/// A Duper visitor which serializes the provided [`DuperValue`].
#[derive(Default)]
pub struct Serializer {
    buf: String,
    strip_identifiers: bool,
    minify: bool,
}

impl Serializer {
    /// Create a new [`Serializer`] visitor with the provided options.
    pub fn new(strip_identifiers: bool, minify: bool) -> Self {
        Self {
            buf: String::new(),
            strip_identifiers,
            minify,
        }
    }

    /// Convert the [`DuperValue`] into a serialized [`String`].
    pub fn serialize<'a>(&mut self, value: &DuperValue<'a>) -> String {
        self.buf.clear();
        value.accept(self);
        std::mem::take(&mut self.buf)
    }
}

impl DuperVisitor for Serializer {
    type Value = ();

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let len = object.len();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.push_str(identifier.as_ref());
            self.buf.push_str("({");
            for (i, (key, value)) in object.iter().enumerate() {
                self.buf.push_str(&format_key(key));
                if self.minify {
                    self.buf.push(':');
                } else {
                    self.buf.push_str(": ");
                }
                value.accept(self);
                if i < len - 1 {
                    if self.minify {
                        self.buf.push(',');
                    } else {
                        self.buf.push_str(", ");
                    }
                }
            }
            self.buf.push_str("})");
        } else {
            self.buf.push('{');
            for (i, (key, value)) in object.iter().enumerate() {
                self.buf.push_str(&format_key(key));
                if self.minify {
                    self.buf.push(':');
                } else {
                    self.buf.push_str(": ");
                }
                value.accept(self);
                if i < len - 1 {
                    if self.minify {
                        self.buf.push(',');
                    } else {
                        self.buf.push_str(", ");
                    }
                }
            }
            self.buf.push('}');
        }
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &[DuperValue<'a>],
    ) -> Self::Value {
        let len = array.len();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.push_str(identifier.as_ref());
            self.buf.push_str("([");
            for (i, value) in array.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    if self.minify {
                        self.buf.push(',');
                    } else {
                        self.buf.push_str(", ");
                    }
                }
            }
            self.buf.push_str("])");
        } else {
            self.buf.push('[');
            for (i, value) in array.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    if self.minify {
                        self.buf.push(',');
                    } else {
                        self.buf.push_str(", ");
                    }
                }
            }
            self.buf.push(']');
        }
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[DuperValue<'a>],
    ) -> Self::Value {
        let len = tuple.len();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.push_str(identifier.as_ref());
            self.buf.push_str("((");
            for (i, value) in tuple.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    if self.minify {
                        self.buf.push(',');
                    } else {
                        self.buf.push_str(", ");
                    }
                }
            }
            self.buf.push_str("))");
        } else {
            self.buf.push('(');
            for (i, value) in tuple.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    if self.minify {
                        self.buf.push(',');
                    } else {
                        self.buf.push_str(", ");
                    }
                }
            }
            self.buf.push(')');
        }
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        value: &'a str,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let value = format_duper_string(value);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(&format_duper_string(value));
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &'a [u8],
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let bytes = format_duper_bytes(bytes);
            self.buf.push_str(&format!("{identifier}({bytes})"));
        } else {
            self.buf.push_str(&format_duper_bytes(bytes));
        }
    }

    fn visit_temporal<'a>(&mut self, temporal: &DuperTemporal<'a>) -> Self::Value {
        let identifier = temporal.identifier();
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let value = format_temporal(temporal);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(&format_temporal(temporal));
        }
    }

    fn visit_integer(
        &mut self,
        identifier: Option<&DuperIdentifier<'_>>,
        integer: i64,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let value = format_integer(integer);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(&format_integer(integer));
        }
    }

    fn visit_float(&mut self, identifier: Option<&DuperIdentifier<'_>>, float: f64) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let value = format_float(float);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(&format_float(float));
        }
    }

    fn visit_boolean(
        &mut self,
        identifier: Option<&DuperIdentifier<'_>>,
        boolean: bool,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let value = format_boolean(boolean);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(format_boolean(boolean));
        }
    }

    fn visit_null(&mut self, identifier: Option<&DuperIdentifier<'_>>) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let value = format_null();
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(format_null());
        }
    }
}

#[cfg(test)]
mod serializer_tests {
    use std::borrow::Cow;

    use insta::assert_snapshot;

    use super::Serializer;
    use crate::{DuperIdentifier, DuperKey, DuperObject, DuperParser, DuperValue};

    fn example_value() -> DuperValue<'static> {
        DuperValue::Object {
            identifier: Some(DuperIdentifier(Cow::Borrowed("Product"))),
            inner: DuperObject::try_from(vec![
                (
                    DuperKey(Cow::Borrowed("product_id")),
                    DuperValue::String {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Uuid"))),
                        inner: Cow::Borrowed("1dd7b7aa-515e-405f-85a9-8ac812242609"),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("name")),
                    DuperValue::String {
                        identifier: None,
                        inner: Cow::Borrowed("Wireless Bluetooth Headphones"),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("brand")),
                    DuperValue::String {
                        identifier: None,
                        inner: Cow::Borrowed("AudioTech"),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("price")),
                    DuperValue::String {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Decimal"))),
                        inner: Cow::Borrowed("129.99"),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("dimensions")),
                    DuperValue::Tuple {
                        identifier: None,
                        inner: vec![
                            DuperValue::Float {
                                identifier: None,
                                inner: 18.5,
                            },
                            DuperValue::Float {
                                identifier: None,
                                inner: 15.2,
                            },
                            DuperValue::Float {
                                identifier: None,
                                inner: 7.8,
                            },
                        ],
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("weight")),
                    DuperValue::Float {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Weight"))),
                        inner: 0.285,
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("in_stock")),
                    DuperValue::Boolean {
                        identifier: None,
                        inner: true,
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("specifications")),
                    DuperValue::Object {
                        identifier: None,
                        inner: DuperObject::try_from(vec![
                            (
                                DuperKey(Cow::Borrowed("battery_life")),
                                DuperValue::String {
                                    identifier: Some(DuperIdentifier(Cow::Borrowed("Duration"))),
                                    inner: Cow::Borrowed("30h"),
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("noise_cancellation")),
                                DuperValue::Boolean {
                                    identifier: None,
                                    inner: true,
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("connectivity")),
                                DuperValue::Array {
                                    identifier: None,
                                    inner: vec![
                                        DuperValue::String {
                                            identifier: None,
                                            inner: Cow::Borrowed("Bluetooth 5.0"),
                                        },
                                        DuperValue::String {
                                            identifier: None,
                                            inner: Cow::Borrowed("3.5mm Jack"),
                                        },
                                    ],
                                },
                            ),
                        ])
                        .unwrap(),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("image_thumbnail")),
                    DuperValue::Bytes {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Png"))),
                        inner: Cow::Borrowed(
                            b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x64",
                        ),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("tags")),
                    DuperValue::Array {
                        identifier: None,
                        inner: vec![
                            DuperValue::String {
                                identifier: None,
                                inner: Cow::Borrowed("electronics"),
                            },
                            DuperValue::String {
                                identifier: None,
                                inner: Cow::Borrowed("audio"),
                            },
                            DuperValue::String {
                                identifier: None,
                                inner: Cow::Borrowed("wireless"),
                            },
                        ],
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("release_date")),
                    DuperValue::String {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Date"))),
                        inner: Cow::Borrowed("2023-11-15"),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("warranty_period")),
                    DuperValue::Null { identifier: None },
                ),
                (
                    DuperKey(Cow::Borrowed("customer_ratings")),
                    DuperValue::Object {
                        identifier: None,
                        inner: DuperObject::try_from(vec![
                            (
                                DuperKey(Cow::Borrowed("latest_review")),
                                DuperValue::String {
                                    identifier: None,
                                    inner: Cow::Borrowed(r#"Absolutely ""astounding""!! 😎"#),
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("average")),
                                DuperValue::Float {
                                    identifier: None,
                                    inner: 4.5,
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("count")),
                                DuperValue::Integer {
                                    identifier: None,
                                    inner: 127,
                                },
                            ),
                        ])
                        .unwrap(),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("created_at")),
                    DuperValue::String {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("DateTime"))),
                        inner: Cow::Borrowed("2023-11-17T21:50:43+00:00"),
                    },
                ),
            ])
            .unwrap(),
        }
    }

    #[test]
    fn default() {
        let mut serializer = Serializer::new(false, false);
        let value = serializer.serialize(&example_value());
        println!("{}", value);
        assert_snapshot!(value);
        let _ = DuperParser::parse_duper_trunk(&value).unwrap();
    }

    #[test]
    fn strip_identifiers() {
        let mut serializer = Serializer::new(true, false);
        let value = serializer.serialize(&example_value());
        println!("{}", value);
        assert_snapshot!(value);
        let _ = DuperParser::parse_duper_trunk(&value).unwrap();
    }

    #[test]
    fn minify() {
        let mut serializer = Serializer::new(false, true);
        let value = serializer.serialize(&example_value());
        println!("{}", value);
        assert_snapshot!(value);
        let _ = DuperParser::parse_duper_trunk(&value).unwrap();
    }

    #[test]
    fn compact() {
        let mut serializer = Serializer::new(true, true);
        let value = serializer.serialize(&example_value());
        println!("{}", value);
        assert_snapshot!(value);
        let _ = DuperParser::parse_duper_trunk(&value).unwrap();
    }
}
