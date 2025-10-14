use crate::{
    ast::{
        DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTuple, DuperValue,
    },
    format::{
        format_boolean, format_duper_bytes, format_duper_string, format_float, format_integer,
        format_key,
    },
    visitor::DuperVisitor,
};

pub struct PrettyPrinter {
    strip_identifiers: bool,
    curr_indent: usize,
    indent: usize,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self {
            strip_identifiers: false,
            curr_indent: 0,
            indent: 2,
        }
    }
}

impl PrettyPrinter {
    pub fn new(strip_identifiers: bool, indent: usize) -> Self {
        Self {
            strip_identifiers,
            curr_indent: 0,
            indent,
        }
    }

    pub fn pretty_print<'a>(&mut self, value: DuperValue<'a>) -> String {
        value.accept(self)
    }

    fn increase_indentation(&mut self) {
        self.curr_indent += self.indent;
    }

    fn decrease_indentation(&mut self) {
        self.curr_indent -= self.indent;
    }

    fn indentation(&self) -> String {
        (0..self.curr_indent).map(|_| ' ').collect()
    }
}

impl DuperVisitor for PrettyPrinter {
    type Value = String;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let mut string = String::new();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            string.push_str(identifier.as_ref());
            if object.is_empty() {
                string.push_str("({})");
            } else {
                string.push_str("({\n");
                self.increase_indentation();
                for (key, value) in object.iter() {
                    string.push_str(&self.indentation());
                    string.push_str(&format_key(key));
                    string.push_str(": ");
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push_str("})\n");
            }
        } else if object.is_empty() {
            string.push_str("{}");
        } else {
            string.push_str("{\n");
            self.increase_indentation();
            for (key, value) in object.iter() {
                string.push_str(&self.indentation());
                string.push_str(&format_key(key));
                string.push_str(": ");
                string.push_str(&value.accept(self));
                string.push_str(",\n");
            }
            self.decrease_indentation();
            string.push_str(&self.indentation());
            string.push('}');
        }

        string
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value {
        let mut string = String::new();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            string.push_str(identifier.as_ref());
            if array.is_empty() {
                string.push_str("([])");
            } else {
                string.push_str("([\n");
                self.increase_indentation();
                for value in array.iter() {
                    string.push_str(&self.indentation());
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push_str("])\n");
            }
        } else if array.is_empty() {
            string.push_str("[]");
        } else {
            string.push_str("[\n");
            self.increase_indentation();
            for value in array.iter() {
                string.push_str(&self.indentation());
                string.push_str(&value.accept(self));
                string.push_str(",\n");
            }
            self.decrease_indentation();
            string.push_str(&self.indentation());
            string.push(']');
        }

        string
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &DuperTuple<'a>,
    ) -> Self::Value {
        let mut string = String::new();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            string.push_str(identifier.as_ref());
            if tuple.is_empty() {
                string.push_str("((,))");
            } else if tuple.len() == 1 {
                string.push_str("((");
                string.push_str(&tuple.get(0).unwrap().accept(self));
                string.push_str(",))");
            } else {
                string.push_str("((\n");
                self.increase_indentation();
                for value in tuple.iter() {
                    string.push_str(&self.indentation());
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push_str("))");
            }
        } else if tuple.is_empty() {
            string.push_str("(,)");
        } else if tuple.len() == 1 {
            string.push('(');
            string.push_str(&tuple.get(0).unwrap().accept(self));
            string.push_str(",)");
        } else {
            string.push_str("(\n");
            self.increase_indentation();
            for value in tuple.iter() {
                string.push_str(&self.indentation());
                string.push_str(&value.accept(self));
                string.push_str(",\n");
            }
            self.decrease_indentation();
            string.push_str(&self.indentation());
            string.push(')');
        }

        string
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
                let mut string = String::new();
                string.push_str(identifier.as_ref());
                string.push_str("(\n");
                self.increase_indentation();
                string.push_str(&self.indentation());
                string.push_str(&value);
                string.push('\n');
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push(')');
                string
            } else {
                format!("{identifier}({value})")
            }
        } else {
            format_duper_string(value).into_owned()
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
                let mut string = String::new();
                string.push_str(identifier.as_ref());
                string.push_str("(\n");
                self.increase_indentation();
                string.push_str(&self.indentation());
                string.push_str(&bytes);
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push(')');
                string
            } else {
                format!("{identifier}({bytes})")
            }
        } else {
            format_duper_bytes(bytes).into_owned()
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
            format!("{identifier}({value})")
        } else {
            format_integer(integer)
        }
    }

    fn visit_float(&mut self, identifier: Option<&DuperIdentifier<'_>>, float: f64) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            let value = format_float(float);
            format!("{identifier}({value})")
        } else {
            format_float(float)
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
            format!("{identifier}({value})")
        } else {
            format_boolean(boolean)
        }
    }

    fn visit_null(&mut self, identifier: Option<&DuperIdentifier<'_>>) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            format!("{identifier}(null)")
        } else {
            "null".into()
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
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
    }

    #[test]
    fn empty_array() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray(vec![])),
        };
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
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
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
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
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
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
                                inner: DuperInner::String(DuperString::from(Cow::Borrowed("pest"))),
                            },
                        ])),
                    },
                ),
            ])),
        };
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
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
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
    }

    #[test]
    fn complex_object() {
        let value = DuperValue {
            identifier: Some(DuperIdentifier::from(Cow::Borrowed("Start"))),
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
                                        identifier: Some(DuperIdentifier::from(Cow::Borrowed(
                                            "Msg",
                                        ))),
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
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
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
        let pp = PrettyPrinter::new(false, 2).pretty_print(value);
        assert_snapshot!(pp);
        let _ = DuperParser::parse_duper(&pp).unwrap();
    }
}
