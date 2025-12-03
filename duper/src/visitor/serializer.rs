//! Utilities for serializing Duper values.

use crate::{
    ast::{
        DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTemporal,
        DuperTuple, DuperValue,
    },
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
        array: &DuperArray<'a>,
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
        tuple: &DuperTuple<'a>,
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
        value: &DuperString<'a>,
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
        bytes: &DuperBytes<'a>,
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

    fn visit_temporal<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        temporal: &DuperTemporal<'a>,
    ) -> Self::Value {
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
