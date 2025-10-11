use std::borrow::Cow;

use crate::{
    ast::DuperValue,
    format::{
        format_boolean, format_bytes, format_float, format_integer, format_key, format_string,
    },
    visitor::DuperVisitor,
};

pub struct DuperSerializer;

impl DuperSerializer {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize<'a>(&mut self, value: DuperValue<'a>) -> String {
        value.accept(self)
    }
}

impl DuperVisitor for DuperSerializer {
    type Value = String;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&'a str>,
        object: Vec<(Cow<'a, str>, DuperValue<'a>)>,
    ) -> Self::Value {
        let mut string = String::new();
        let len = object.len();

        if let Some(identifier) = identifier {
            string.push_str(identifier);
            string.push_str("({");
            for (i, (key, value)) in object.into_iter().enumerate() {
                string.push_str(&format_key(key));
                string.push_str(": ");
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
            string.push_str("})");
        } else {
            string.push('{');
            for (i, (key, value)) in object.into_iter().enumerate() {
                string.push_str(&format_key(key));
                string.push_str(": ");
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
            string.push('}');
        }

        string
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&str>,
        array: Vec<DuperValue<'a>>,
    ) -> Self::Value {
        let mut string = String::new();
        let len = array.len();

        if let Some(identifier) = identifier {
            string.push_str(identifier);
            string.push_str("([");
            for (i, value) in array.into_iter().enumerate() {
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
            string.push_str("])");
        } else {
            string.push('[');
            for (i, value) in array.into_iter().enumerate() {
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
            string.push(']');
        }

        string
    }

    fn visit_string<'a>(&mut self, identifier: Option<&str>, value: Cow<'a, str>) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_string(value);
            format!("{identifier}({value})")
        } else {
            format_string(value).into_owned()
        }
    }

    fn visit_bytes<'a>(&mut self, identifier: Option<&str>, bytes: Cow<'a, [u8]>) -> Self::Value {
        if let Some(identifier) = identifier {
            let bytes = format_bytes(bytes);
            format!("{identifier}({bytes})")
        } else {
            format_bytes(bytes).into_owned()
        }
    }

    fn visit_integer(&mut self, identifier: Option<&str>, integer: i64) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_integer(integer, identifier.try_into().ok());
            format!("{identifier}({value})")
        } else {
            format_integer(integer, None)
        }
    }

    fn visit_float(&mut self, identifier: Option<&str>, float: f64) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_float(float, identifier.try_into().ok());
            format!("{identifier}({value})")
        } else {
            format_float(float, None)
        }
    }

    fn visit_boolean(&mut self, identifier: Option<&str>, boolean: bool) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_boolean(boolean);
            format!("{identifier}({value})")
        } else {
            format_boolean(boolean)
        }
    }

    fn visit_null(&mut self, identifier: Option<&str>) -> Self::Value {
        if let Some(identifier) = identifier {
            format!("{identifier}(null)")
        } else {
            "null".into()
        }
    }
}
