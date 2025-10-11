use std::{borrow::Cow, fmt::Debug};

use crate::visitor::DuperVisitor;

#[derive(Debug, Clone)]
pub struct DuperValue<'a> {
    pub identifier: Option<Cow<'a, str>>,
    pub inner: DuperInner<'a>,
}

#[derive(Debug, Clone)]
pub enum DuperInner<'a> {
    Object(Vec<(Cow<'a, str>, DuperValue<'a>)>),
    Array(Vec<DuperValue<'a>>),
    Tuple(Vec<DuperValue<'a>>),
    String(Cow<'a, str>),
    Bytes(Cow<'a, [u8]>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl<'a> DuperValue<'a> {
    pub fn accept<'v, V: DuperVisitor>(&self, visitor: &'v mut V) -> V::Value {
        match &self.inner {
            DuperInner::Object(object) => visitor.visit_object(self.identifier.as_ref(), object),
            DuperInner::Array(array) => visitor.visit_array(self.identifier.as_ref(), array),
            DuperInner::Tuple(tuple) => visitor.visit_tuple(self.identifier.as_ref(), tuple),
            DuperInner::String(string) => visitor.visit_string(self.identifier.as_ref(), string),
            DuperInner::Bytes(bytes) => visitor.visit_bytes(self.identifier.as_ref(), bytes),
            DuperInner::Integer(integer) => {
                visitor.visit_integer(self.identifier.as_ref(), *integer)
            }
            DuperInner::Float(float) => visitor.visit_float(self.identifier.as_ref(), *float),
            DuperInner::Boolean(boolean) => {
                visitor.visit_boolean(self.identifier.as_ref(), *boolean)
            }
            DuperInner::Null => visitor.visit_null(self.identifier.as_ref()),
        }
    }
}
