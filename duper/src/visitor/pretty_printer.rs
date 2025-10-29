//! Utilities for pretty-printing Duper values.

use crate::{
    ast::{
        DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTuple, DuperValue,
    },
    format::{
        format_boolean, format_duper_bytes, format_duper_string, format_float, format_integer,
        format_key, format_null,
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
    pub fn pretty_print<'a>(&mut self, value: DuperValue<'a>) -> String {
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
        array: &DuperArray<'a>,
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
        tuple: &DuperTuple<'a>,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.push_str(identifier.as_ref());
            if tuple.is_empty() {
                self.buf.push_str("(())");
            } else if tuple.len() == 1 {
                self.buf.push_str("((");
                tuple.get(0).unwrap().accept(self);
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
            tuple.get(0).unwrap().accept(self);
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
        value: &DuperString<'a>,
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
        bytes: &DuperBytes<'a>,
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
mod pretty_printer_tests {
    use std::borrow::Cow;

    use insta::assert_snapshot;

    use crate::{
        DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
        DuperTuple, DuperValue, PrettyPrinter, parser::DuperParser,
    };

    #[test]
    fn empty_object() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn empty_array() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray(vec![])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn single_element_object() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![(
                DuperKey::from(Cow::Borrowed("chess")),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::String(DuperString::from(Cow::Borrowed("✅"))),
                },
            )])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn single_element_array() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray(vec![DuperValue {
                identifier: None,
                inner: DuperInner::Integer(42),
            }])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn basic_object() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![
                (
                    DuperKey::from(Cow::Borrowed("zero")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Tuple(DuperTuple::from(vec![])),
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("one")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Tuple(DuperTuple::from(vec![DuperValue {
                            identifier: None,
                            inner: DuperInner::String(DuperString::from(Cow::Borrowed("Sandhole"))),
                        }])),
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("two")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Tuple(DuperTuple::from(vec![
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::String(DuperString::from(Cow::Borrowed("rust"))),
                            },
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::String(DuperString::from(Cow::Borrowed(
                                    "chumsky",
                                ))),
                            },
                        ])),
                    },
                ),
            ])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn basic_array() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray(vec![
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Bytes(DuperBytes::from(Cow::Borrowed(b"foobar".as_ref()))),
                },
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Null,
                },
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Boolean(false),
                },
            ])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn complex_object() {
        let value = DuperValue {
            identifier: Some(
                DuperIdentifier::try_from(Cow::Borrowed("Start")).expect("valid identifier"),
            ),
            inner: DuperInner::Object(DuperObject(vec![(
                DuperKey::from(Cow::Borrowed("first object")),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Object(DuperObject(vec![(
                        DuperKey::from(Cow::Borrowed("second_object")),
                        DuperValue {
                            identifier: None,
                            inner: DuperInner::Object(DuperObject(vec![
                                (
                                    DuperKey::from(Cow::Borrowed("third object")),
                                    DuperValue {
                                        identifier: Some(
                                            DuperIdentifier::try_from(Cow::Borrowed("Msg"))
                                                .expect("valid identifier"),
                                        ),
                                        inner: DuperInner::String(DuperString::from(
                                            Cow::Borrowed(
                                                "This is a very long string that will push itself into the next line.",
                                            ),
                                        )),
                                    },
                                ),
                                (
                                    DuperKey::from(Cow::Borrowed("addendum")),
                                    DuperValue {
                                        identifier: None,
                                        inner: DuperInner::Null,
                                    },
                                ),
                            ])),
                        },
                    )])),
                },
            )])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn complex_array() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray(vec![
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Array(DuperArray(vec![DuperValue {
                        identifier: None,
                        inner: DuperInner::Array(DuperArray(vec![DuperValue {
                            identifier: None,
                            inner: DuperInner::String(DuperString::from(Cow::Borrowed(
                                "So many arrays!",
                            ))),
                        }])),
                    }])),
                },
                DuperValue {
                    identifier: None,
                    inner: DuperInner::String(DuperString::from(Cow::Borrowed(
                        r#""Hello world!""#,
                    ))),
                },
            ])),
        };
        let pp = PrettyPrinter::new(false, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn strip_identifiers() {
        let value = DuperValue {
            identifier: Some(
                DuperIdentifier::try_from(Cow::Borrowed("Start")).expect("valid identifier"),
            ),
            inner: DuperInner::Object(DuperObject(vec![
                (
                    DuperKey::from(Cow::Borrowed("nested_object")),
                    DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("Nested"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperInner::Object(DuperObject(vec![
                            (
                                DuperKey::from(Cow::Borrowed("integer_field")),
                                DuperValue {
                                    identifier: Some(
                                        DuperIdentifier::try_from(Cow::Borrowed("Int"))
                                            .expect("valid identifier"),
                                    ),
                                    inner: DuperInner::Integer(42),
                                },
                            ),
                            (
                                DuperKey::from(Cow::Borrowed("string_field")),
                                DuperValue {
                                    identifier: Some(
                                        DuperIdentifier::try_from(Cow::Borrowed("Str"))
                                            .expect("valid identifier"),
                                    ),
                                    inner: DuperInner::String(DuperString::from(Cow::Borrowed(
                                        "test",
                                    ))),
                                },
                            ),
                        ])),
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("array_field")),
                    DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("Arr"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperInner::Array(DuperArray(vec![
                            DuperValue {
                                identifier: Some(
                                    DuperIdentifier::try_from(Cow::Borrowed("Float"))
                                        .expect("valid identifier"),
                                ),
                                inner: DuperInner::Float(4.2),
                            },
                            DuperValue {
                                identifier: Some(
                                    DuperIdentifier::try_from(Cow::Borrowed("Bool"))
                                        .expect("valid identifier"),
                                ),
                                inner: DuperInner::Boolean(true),
                            },
                        ])),
                    },
                ),
            ])),
        };
        let pp = PrettyPrinter::new(true, "  ").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }

    #[test]
    fn tab_indentation() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![
                (
                    DuperKey::from(Cow::Borrowed("first_level")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Object(DuperObject(vec![
                            (
                                DuperKey::from(Cow::Borrowed("second_level")),
                                DuperValue {
                                    identifier: None,
                                    inner: DuperInner::Array(DuperArray(vec![
                                        DuperValue {
                                            identifier: None,
                                            inner: DuperInner::String(DuperString::from(
                                                Cow::Borrowed("deep"),
                                            )),
                                        },
                                        DuperValue {
                                            identifier: None,
                                            inner: DuperInner::Integer(123),
                                        },
                                    ])),
                                },
                            ),
                            (
                                DuperKey::from(Cow::Borrowed("another_second_level")),
                                DuperValue {
                                    identifier: None,
                                    inner: DuperInner::Tuple(DuperTuple::from(vec![
                                        DuperValue {
                                            identifier: None,
                                            inner: DuperInner::Boolean(false),
                                        },
                                        DuperValue {
                                            identifier: None,
                                            inner: DuperInner::Null,
                                        },
                                    ])),
                                },
                            ),
                        ])),
                    },
                ),
                (
                    DuperKey::from(Cow::Borrowed("simple_field")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::String(DuperString::from(Cow::Borrowed("value"))),
                    },
                ),
            ])),
        };
        let pp = PrettyPrinter::new(false, "\t").unwrap().pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper_trunk(&pp).unwrap();
    }
}
