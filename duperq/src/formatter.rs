// duperq 'span.tagged && span[0]name == sp0001 | "[${level}] ${span[0]time} - ${span[0]status} ${telemetry.duration:ms}"'

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTemporal, DuperTuple,
    DuperValue,
    format::{
        format_boolean, format_duper_bytes, format_duper_string, format_float, format_integer,
        format_key, format_null, format_temporal,
    },
    visitor::DuperVisitor,
};

use crate::{accessor::DuperAccessor, types::DuperType};

pub(crate) enum FormatterAtom {
    Fixed(String),
    Dynamic(Box<dyn DuperAccessor>, Option<DuperType>),
}

pub(crate) struct Formatter {
    atoms: Vec<FormatterAtom>,
    visitor: FormatterVisitor,
}

impl Formatter {
    pub(crate) fn new(atoms: Vec<FormatterAtom>) -> Self {
        Self {
            atoms: atoms
                .into_iter()
                .filter_map(|atom| match &atom {
                    FormatterAtom::Fixed(string) if string.is_empty() => None,
                    _ => Some(atom),
                })
                .collect(),
            visitor: FormatterVisitor { buf: String::new() },
        }
    }

    pub(crate) fn format(&mut self, value: DuperValue<'static>) -> String {
        let mut buf = String::new();
        for atom in &self.atoms {
            match atom {
                FormatterAtom::Fixed(fixed) => buf.push_str(&fixed),
                FormatterAtom::Dynamic(duper_accessor, typ) => {
                    match duper_accessor.access(&value).into_iter().next() {
                        Some(value) => {
                            if let Some(typ) = typ {
                                if let Some(value) = typ.cast(value) {
                                    buf.push_str(&self.visitor.visit(&value))
                                } else {
                                    buf.push_str("<INVALID CAST>")
                                }
                            } else {
                                buf.push_str(&self.visitor.visit(value))
                            }
                        }
                        None => buf.push_str("<MISSING>"),
                    }
                }
            }
        }
        buf
    }
}

struct FormatterVisitor {
    buf: String,
}

impl FormatterVisitor {
    pub fn visit<'a>(&mut self, value: &'a DuperValue<'a>) -> String {
        self.buf.clear();
        value.accept(self);
        std::mem::take(&mut self.buf)
    }
}

impl DuperVisitor for FormatterVisitor {
    type Value = ();

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let len = object.len();

        if let Some(identifier) = identifier {
            self.buf.push_str(identifier.as_ref());
            self.buf.push_str("({");
            for (i, (key, value)) in object.iter().enumerate() {
                self.buf.push_str(&format_key(key));
                self.buf.push_str(": ");
                value.accept(self);
                if i < len - 1 {
                    self.buf.push_str(", ");
                }
            }
            self.buf.push_str("})");
        } else {
            self.buf.push('{');
            for (i, (key, value)) in object.iter().enumerate() {
                self.buf.push_str(&format_key(key));
                self.buf.push_str(": ");
                value.accept(self);
                if i < len - 1 {
                    self.buf.push_str(", ");
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

        if let Some(identifier) = identifier {
            self.buf.push_str(identifier.as_ref());
            self.buf.push_str("([");
            for (i, value) in array.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    self.buf.push_str(", ");
                }
            }
            self.buf.push_str("])");
        } else {
            self.buf.push('[');
            for (i, value) in array.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    self.buf.push_str(", ");
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

        if let Some(identifier) = identifier {
            self.buf.push_str(identifier.as_ref());
            self.buf.push_str("((");
            for (i, value) in tuple.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    self.buf.push_str(", ");
                }
            }
            self.buf.push_str("))");
        } else {
            self.buf.push('(');
            for (i, value) in tuple.iter().enumerate() {
                value.accept(self);
                if i < len - 1 {
                    self.buf.push_str(", ");
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
        if let Some(identifier) = identifier {
            let value = format_duper_string(value);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(value.as_ref());
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value {
        if let Some(identifier) = identifier {
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
        if let Some(identifier) = identifier {
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
        if let Some(identifier) = identifier {
            let value = format_integer(integer);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(&format_integer(integer));
        }
    }

    fn visit_float(&mut self, identifier: Option<&DuperIdentifier<'_>>, float: f64) -> Self::Value {
        if let Some(identifier) = identifier {
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
        if let Some(identifier) = identifier {
            let value = format_boolean(boolean);
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(format_boolean(boolean));
        }
    }

    fn visit_null(&mut self, identifier: Option<&DuperIdentifier<'_>>) -> Self::Value {
        if let Some(identifier) = identifier {
            let value = format_null();
            self.buf.push_str(&format!("{identifier}({value})"));
        } else {
            self.buf.push_str(format_null());
        }
    }
}
