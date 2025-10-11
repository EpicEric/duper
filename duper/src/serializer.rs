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

pub struct Serializer;

impl Serializer {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize<'a>(&mut self, value: DuperValue<'a>) -> String {
        value.accept(self)
    }
}

impl DuperVisitor for Serializer {
    type Value = String;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let mut string = String::new();
        let len = object.len();

        if let Some(identifier) = identifier {
            string.push_str(identifier.as_ref());
            string.push_str("({");
            for (i, (key, value)) in object.iter().enumerate() {
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
            for (i, (key, value)) in object.iter().enumerate() {
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
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value {
        let mut string = String::new();
        let len = array.len();

        if let Some(identifier) = identifier {
            string.push_str(identifier.as_ref());
            string.push_str("([");
            for (i, value) in array.iter().enumerate() {
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
            string.push_str("])");
        } else {
            string.push('[');
            for (i, value) in array.iter().enumerate() {
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
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
        let len = tuple.len();

        if let Some(identifier) = identifier {
            string.push_str(identifier.as_ref());
            string.push_str("((");
            for (i, value) in tuple.iter().enumerate() {
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
            if len <= 1 {
                string.push(',');
            }
            string.push_str("))");
        } else {
            string.push('(');
            for (i, value) in tuple.iter().enumerate() {
                string.push_str(&value.accept(self));
                if i < len - 1 {
                    string.push_str(", ");
                }
            }
            if len <= 1 {
                string.push(',');
            }
            string.push(')');
        }

        string
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        value: &DuperString<'a>,
    ) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_duper_string(value);
            format!("{identifier}({value})")
        } else {
            format_duper_string(value).into_owned()
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value {
        if let Some(identifier) = identifier {
            let bytes = format_duper_bytes(bytes);
            format!("{identifier}({bytes})")
        } else {
            format_duper_bytes(bytes).into_owned()
        }
    }

    fn visit_integer(
        &mut self,
        identifier: Option<&DuperIdentifier<'_>>,
        integer: i64,
    ) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_integer(integer, identifier.try_into().ok());
            format!("{identifier}({value})")
        } else {
            format_integer(integer, None)
        }
    }

    fn visit_float(&mut self, identifier: Option<&DuperIdentifier<'_>>, float: f64) -> Self::Value {
        if let Some(identifier) = identifier {
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
        if let Some(identifier) = identifier {
            let value = format_boolean(boolean);
            format!("{identifier}({value})")
        } else {
            format_boolean(boolean)
        }
    }

    fn visit_null(&mut self, identifier: Option<&DuperIdentifier<'_>>) -> Self::Value {
        if let Some(identifier) = identifier {
            format!("{identifier}(null)")
        } else {
            "null".into()
        }
    }
}
