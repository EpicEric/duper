//! Utilities for pretty-printing Duper values.

use crate::{
    DuperFloat,
    ast::{DuperIdentifier, DuperObject, DuperTemporal, DuperValue},
    format::{
        format_boolean, format_duper_bytes, format_duper_string, format_float, format_integer,
        format_key, format_null, format_temporal,
    },
    visitor::DuperVisitor,
};

/// A Duper visitor which pretty-prints the provided [`DuperValue`] with
/// line breaks, indentation, and trailing commas.
pub struct PrettyPrinter<'pp> {
    buf: String,
    strip_identifiers: bool,
    curr_indent: usize,
    indent: &'pp str,
}

impl Default for PrettyPrinter<'static> {
    fn default() -> Self {
        Self {
            buf: String::new(),
            strip_identifiers: false,
            curr_indent: 0,
            indent: "  ",
        }
    }
}

impl<'pp> PrettyPrinter<'pp> {
    /// Create a new [`PrettyPrinter`] visitor with the provided option and
    /// desired indentation.
    pub fn new(strip_identifiers: bool, indent: &'pp str) -> Result<Self, &'static str> {
        if indent.is_empty() {
            return Err("Indentation cannot be empty");
        }
        if indent.chars().any(|char| char != ' ' && char != '\t') {
            return Err("Indentation may only consist of spaces and tabs");
        }
        Ok(Self {
            buf: String::new(),
            strip_identifiers,
            curr_indent: 0,
            indent,
        })
    }

    /// Convert the [`DuperValue`] into a pretty-printed [`String`].
    pub fn pretty_print<'a>(&mut self, value: &DuperValue<'a>) -> String {
        self.buf.clear();
        value.accept(self);
        std::mem::take(&mut self.buf)
    }

    fn increase_indentation(&mut self) {
        self.curr_indent += 1;
    }

    fn decrease_indentation(&mut self) {
        self.curr_indent -= 1;
    }

    fn push_indentation(&mut self) {
        for _ in 0..self.curr_indent {
            self.buf.push_str(self.indent);
        }
    }
}

impl<'pp> DuperVisitor for PrettyPrinter<'pp> {
    type Value = ();

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.push_str(identifier.as_ref());
            if object.is_empty() {
                self.buf.push_str("({})");
            } else {
                self.buf.push_str("({\n");
                self.increase_indentation();
                for (key, value) in object.iter() {
                    self.push_indentation();
                    self.buf.push_str(&format_key(key));
                    self.buf.push_str(": ");
                    value.accept(self);
                    self.buf.push_str(",\n");
                }
                self.decrease_indentation();
                self.push_indentation();
                self.buf.push_str("})");
            }
        } else if object.is_empty() {
            self.buf.push_str("{}");
        } else {
            self.buf.push_str("{\n");
            self.increase_indentation();
            for (key, value) in object.iter() {
                self.push_indentation();
                self.buf.push_str(&format_key(key));
                self.buf.push_str(": ");
                value.accept(self);
                self.buf.push_str(",\n");
            }
            self.decrease_indentation();
            self.push_indentation();
            self.buf.push('}');
        }
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &[DuperValue<'a>],
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.push_str(identifier.as_ref());
            if array.is_empty() {
                self.buf.push_str("([])");
            } else {
                self.buf.push_str("([\n");
                self.increase_indentation();
                for value in array.iter() {
                    self.push_indentation();
                    value.accept(self);
                    self.buf.push_str(",\n");
                }
                self.decrease_indentation();
                self.push_indentation();
                self.buf.push_str("])");
            }
        } else if array.is_empty() {
            self.buf.push_str("[]");
        } else {
            self.buf.push_str("[\n");
            self.increase_indentation();
            for value in array.iter() {
                self.push_indentation();
                value.accept(self);
                self.buf.push_str(",\n");
            }
            self.decrease_indentation();
            self.push_indentation();
            self.buf.push(']');
        }
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[DuperValue<'a>],
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.push_str(identifier.as_ref());
            if tuple.is_empty() {
                self.buf.push_str("(())");
            } else if tuple.len() == 1 {
                self.buf.push_str("((");
                tuple
                    .first()
                    .expect("tuple contains one element")
                    .accept(self);
                self.buf.push_str("))");
            } else {
                self.buf.push_str("((\n");
                self.increase_indentation();
                for value in tuple.iter() {
                    self.push_indentation();
                    value.accept(self);
                    self.buf.push_str(",\n");
                }
                self.decrease_indentation();
                self.push_indentation();
                self.buf.push_str("))");
            }
        } else if tuple.is_empty() {
            self.buf.push_str("()");
        } else if tuple.len() == 1 {
            self.buf.push('(');
            tuple
                .first()
                .expect("tuple contains one element")
                .accept(self);
            self.buf.push(')');
        } else {
            self.buf.push_str("(\n");
            self.increase_indentation();
            for value in tuple.iter() {
                self.push_indentation();
                value.accept(self);
                self.buf.push_str(",\n");
            }
            self.decrease_indentation();
            self.push_indentation();
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
            if value.len() + self.curr_indent > 60 {
                self.buf.push_str(identifier.as_ref());
                self.buf.push_str("(\n");
                self.increase_indentation();
                self.push_indentation();
                self.buf.push_str(&value);
                self.buf.push('\n');
                self.decrease_indentation();
                self.push_indentation();
                self.buf.push(')');
            } else {
                self.buf.push_str(&format!("{identifier}({value})"));
            }
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
            if bytes.len() + self.curr_indent > 60 {
                self.buf.push_str(identifier.as_ref());
                self.buf.push_str("(\n");
                self.increase_indentation();
                self.push_indentation();
                self.buf.push_str(&bytes);
                self.decrease_indentation();
                self.push_indentation();
                self.buf.push(')');
            } else {
                self.buf.push_str(&format!("{identifier}({bytes})"));
            }
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

    fn visit_float(
        &mut self,
        identifier: Option<&DuperIdentifier<'_>>,
        float: DuperFloat,
    ) -> Self::Value {
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
mod pretty_printer_tests {
    use std::borrow::Cow;

    use indexmap::IndexMap;
    use insta::assert_snapshot;

    use super::PrettyPrinter;
    use crate::{
        DuperFloat, DuperIdentifier, DuperKey, DuperObject, DuperValue, parser::DuperParser,
    };

    #[test]
    fn empty_object() {
        let value = DuperValue::Object {
            identifier: None,
            inner: DuperObject(IndexMap::new()),
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn empty_array() {
        let value = DuperValue::Array {
            identifier: None,
            inner: vec![],
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn empty_tuple() {
        let value = DuperValue::Tuple {
            identifier: None,
            inner: vec![],
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn single_element_object() {
        let value = DuperValue::Object {
            identifier: None,
            inner: DuperObject::try_from(vec![(
                DuperKey::from(Cow::Borrowed("chess")),
                DuperValue::String {
                    identifier: None,
                    inner: Cow::Borrowed("✅"),
                },
            )])
            .unwrap(),
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn single_element_array() {
        let value = DuperValue::Array {
            identifier: None,
            inner: vec![DuperValue::Integer {
                identifier: None,
                inner: 42,
            }],
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn basic_object() {
        let value = DuperValue::Object {
            identifier: None,
            inner: DuperObject::try_from(vec![
                (
                    DuperKey::from(Cow::Borrowed("zero")),
                    DuperValue::Tuple {
                        identifier: None,
                        inner: vec![],
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("one")),
                    DuperValue::Tuple {
                        identifier: None,
                        inner: vec![DuperValue::String {
                            identifier: None,
                            inner: Cow::Borrowed("Sandhole"),
                        }],
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("two")),
                    DuperValue::Tuple {
                        identifier: None,
                        inner: vec![
                            DuperValue::String {
                                identifier: None,
                                inner: Cow::Borrowed("rust"),
                            },
                            DuperValue::String {
                                identifier: None,
                                inner: Cow::Borrowed("chumsky"),
                            },
                        ],
                    },
                ),
            ])
            .unwrap(),
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn basic_array() {
        let value = DuperValue::Array {
            identifier: None,
            inner: vec![
                DuperValue::Bytes {
                    identifier: None,
                    inner: Cow::Borrowed(b"foobar".as_ref()),
                },
                DuperValue::Null { identifier: None },
                DuperValue::Boolean {
                    identifier: None,
                    inner: false,
                },
            ],
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn complex_object() {
        let value = DuperValue::Object {
            identifier: Some(
                DuperIdentifier::try_from(Cow::Borrowed("Start")).expect("valid identifier"),
            ),
            inner: DuperObject::try_from(vec![(
                DuperKey::from(Cow::Borrowed("first object")),
                DuperValue::Object {
                    identifier: None,
                    inner: DuperObject::try_from(vec![(
                        DuperKey::from(Cow::Borrowed("second_object")),
                        DuperValue::Object {
                            identifier: None,
                            inner: DuperObject::try_from(vec![
                                (
                                    DuperKey::from(Cow::Borrowed("third object")),
                                    DuperValue::String {
                                        identifier: Some(
                                            DuperIdentifier::try_from(Cow::Borrowed("Msg"))
                                                .expect("valid identifier"),
                                        ),
                                        inner:
                                            Cow::Borrowed(
                                                "This is a very long string that will push itself into the next line.",
                                            ),
                                    },
                                ),
                                (
                                    DuperKey::from(Cow::Borrowed("addendum")),
                                    DuperValue::Null {
                                        identifier: None,
                                    },
                                ),
                            ]).unwrap(),
                        },
                    )]).unwrap(),
                },
            )]).unwrap(),
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn complex_array() {
        let value = DuperValue::Array {
            identifier: None,
            inner: vec![
                DuperValue::Array {
                    identifier: None,
                    inner: vec![DuperValue::Array {
                        identifier: None,
                        inner: vec![DuperValue::String {
                            identifier: None,
                            inner: Cow::Borrowed("So many arrays!"),
                        }],
                    }],
                },
                DuperValue::String {
                    identifier: None,
                    inner: Cow::Borrowed(r#""Hello world!""#),
                },
            ],
        };
        let pp = PrettyPrinter::new(false, "  ")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn strip_identifiers() {
        let value = DuperValue::Object {
            identifier: Some(
                DuperIdentifier::try_from(Cow::Borrowed("Start")).expect("valid identifier"),
            ),
            inner: DuperObject::try_from(vec![
                (
                    DuperKey::from(Cow::Borrowed("nested_object")),
                    DuperValue::Object {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("Nested"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperObject::try_from(vec![
                            (
                                DuperKey::from(Cow::Borrowed("integer_field")),
                                DuperValue::Integer {
                                    identifier: Some(
                                        DuperIdentifier::try_from(Cow::Borrowed("Int"))
                                            .expect("valid identifier"),
                                    ),
                                    inner: 42,
                                },
                            ),
                            (
                                DuperKey::from(Cow::Borrowed("string_field")),
                                DuperValue::String {
                                    identifier: Some(
                                        DuperIdentifier::try_from(Cow::Borrowed("Str"))
                                            .expect("valid identifier"),
                                    ),
                                    inner: Cow::Borrowed("test"),
                                },
                            ),
                        ])
                        .unwrap(),
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("array_field")),
                    DuperValue::Array {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("Arr"))
                                .expect("valid identifier"),
                        ),
                        inner: vec![
                            DuperValue::Float {
                                identifier: Some(
                                    DuperIdentifier::try_from(Cow::Borrowed("Float"))
                                        .expect("valid identifier"),
                                ),
                                inner: DuperFloat::assert(4.2),
                            },
                            DuperValue::Boolean {
                                identifier: Some(
                                    DuperIdentifier::try_from(Cow::Borrowed("Bool"))
                                        .expect("valid identifier"),
                                ),
                                inner: true,
                            },
                        ],
                    },
                ),
            ])
            .unwrap(),
        };
        let pp = PrettyPrinter::new(true, "  ").unwrap().pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn tab_indentation() {
        let value = DuperValue::Object {
            identifier: None,
            inner: DuperObject::try_from(vec![
                (
                    DuperKey::from(Cow::Borrowed("first_level")),
                    DuperValue::Object {
                        identifier: None,
                        inner: DuperObject::try_from(vec![
                            (
                                DuperKey::from(Cow::Borrowed("second_level")),
                                DuperValue::Array {
                                    identifier: None,
                                    inner: vec![
                                        DuperValue::String {
                                            identifier: None,
                                            inner: Cow::Borrowed("deep"),
                                        },
                                        DuperValue::Integer {
                                            identifier: None,
                                            inner: 123,
                                        },
                                    ],
                                },
                            ),
                            (
                                DuperKey::from(Cow::Borrowed("another_second_level")),
                                DuperValue::Tuple {
                                    identifier: None,
                                    inner: vec![
                                        DuperValue::Boolean {
                                            identifier: None,
                                            inner: false,
                                        },
                                        DuperValue::Null { identifier: None },
                                    ],
                                },
                            ),
                        ])
                        .unwrap(),
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("simple_field")),
                    DuperValue::String {
                        identifier: None,
                        inner: Cow::Borrowed("value"),
                    },
                ),
            ])
            .unwrap(),
        };
        let pp = PrettyPrinter::new(false, "\t")
            .unwrap()
            .pretty_print(&value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }
}
