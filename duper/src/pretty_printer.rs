use std::borrow::Cow;

use crate::{
    ast::DuperValue,
    format::{
        format_boolean, format_bytes, format_float, format_integer, format_key, format_string,
    },
    visitor::DuperVisitor,
};

pub struct PrettyPrinter {
    indent: usize,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    pub fn serialize<'a>(&mut self, value: DuperValue<'a>) -> String {
        value.accept(self)
    }

    fn increase_indentation(&mut self) {
        self.indent += 2
    }

    fn decrease_indentation(&mut self) {
        self.indent -= 2
    }

    fn indentation(&self) -> String {
        (0..self.indent).map(|_| ' ').collect()
    }
}

impl DuperVisitor for PrettyPrinter {
    type Value = String;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&Cow<'a, str>>,
        object: &Vec<(Cow<'a, str>, DuperValue<'a>)>,
    ) -> Self::Value {
        let mut string = String::new();

        if let Some(identifier) = identifier {
            string.push_str(identifier.as_ref());
            if object.is_empty() {
                string.push_str("({})");
            } else {
                string.push_str("({\n");
                self.increase_indentation();
                for (key, value) in object.into_iter() {
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
        } else {
            if object.is_empty() {
                string.push_str("{}");
            } else {
                string.push_str("{\n");
                self.increase_indentation();
                for (key, value) in object.into_iter() {
                    string.push_str(&format_key(key));
                    string.push_str(": ");
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push('}');
            }
        }

        string
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&Cow<'a, str>>,
        array: &Vec<DuperValue<'a>>,
    ) -> Self::Value {
        let mut string = String::new();

        if let Some(identifier) = identifier {
            string.push_str(identifier.as_ref());
            if array.is_empty() {
                string.push_str("([])");
            } else {
                string.push_str("([");
                self.increase_indentation();
                for value in array.into_iter() {
                    string.push_str(&self.indentation());
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push_str("])\n");
            }
        } else {
            if array.is_empty() {
                string.push_str("[]");
            } else {
                string.push('[');
                self.increase_indentation();
                for value in array.into_iter() {
                    string.push_str(&self.indentation());
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push(']');
            }
        }

        string
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&Cow<'a, str>>,
        tuple: &Vec<DuperValue<'a>>,
    ) -> Self::Value {
        let mut string = String::new();

        if let Some(identifier) = identifier {
            string.push_str(identifier.as_ref());
            if tuple.is_empty() {
                string.push_str("((,))");
            } else if tuple.len() == 1 {
                string.push_str("((");
                string.push_str(&tuple.get(0).unwrap().accept(self));
                string.push_str(",))");
            } else {
                string.push_str("((");
                self.increase_indentation();
                for value in tuple.into_iter() {
                    string.push_str(&self.indentation());
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push_str("))");
            }
        } else {
            if tuple.is_empty() {
                string.push_str("(,)");
            } else if tuple.len() == 1 {
                string.push_str("(");
                string.push_str(&tuple.get(0).unwrap().accept(self));
                string.push_str(",)");
            } else {
                string.push('(');
                self.increase_indentation();
                for value in tuple.into_iter() {
                    string.push_str(&self.indentation());
                    string.push_str(&value.accept(self));
                    string.push_str(",\n");
                }
                self.decrease_indentation();
                string.push(')');
            }
        }

        string
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&Cow<'a, str>>,
        value: &Cow<'a, str>,
    ) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_string(value);
            if value.len() + self.indent > 60 {
                let mut string = String::new();
                string.push_str(&identifier);
                string.push_str("(\n");
                self.increase_indentation();
                string.push_str(&self.indentation());
                string.push_str(&value);
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push_str(")");
                string
            } else {
                format!("{identifier}({value})")
            }
        } else {
            format_string(value).into_owned()
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&Cow<'a, str>>,
        bytes: &Cow<'a, [u8]>,
    ) -> Self::Value {
        if let Some(identifier) = identifier {
            let bytes = format_bytes(bytes);
            if bytes.len() + self.indent > 60 {
                let mut string = String::new();
                string.push_str(&identifier);
                string.push_str("(\n");
                self.increase_indentation();
                string.push_str(&self.indentation());
                string.push_str(&bytes);
                self.decrease_indentation();
                string.push_str(&self.indentation());
                string.push_str(")");
                string
            } else {
                format!("{identifier}({bytes})")
            }
        } else {
            format_bytes(bytes).into_owned()
        }
    }

    fn visit_integer(&mut self, identifier: Option<&Cow<'_, str>>, integer: i64) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_integer(integer, identifier.as_ref().try_into().ok());
            format!("{identifier}({value})")
        } else {
            format_integer(integer, None)
        }
    }

    fn visit_float(&mut self, identifier: Option<&Cow<'_, str>>, float: f64) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_float(float);
            format!("{identifier}({value})")
        } else {
            format_float(float)
        }
    }

    fn visit_boolean(&mut self, identifier: Option<&Cow<'_, str>>, boolean: bool) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_boolean(boolean);
            format!("{identifier}({value})")
        } else {
            format_boolean(boolean)
        }
    }

    fn visit_null(&mut self, identifier: Option<&Cow<'_, str>>) -> Self::Value {
        if let Some(identifier) = identifier {
            format!("{identifier}(null)")
        } else {
            "null".into()
        }
    }
}
