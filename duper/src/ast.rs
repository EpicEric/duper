use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use crate::{DuperParser, DuperRule, visitor::DuperVisitor};

#[derive(Debug, Clone)]
pub struct DuperIdentifier<'a>(pub(crate) Cow<'a, str>);

impl<'a> DuperIdentifier<'a> {
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> Display for DuperIdentifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'a> AsRef<str> for DuperIdentifier<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> From<Cow<'a, str>> for DuperIdentifier<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperKey<'a>(pub(crate) Cow<'a, str>);

impl<'a> DuperKey<'a> {
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> AsRef<str> for DuperKey<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> From<Cow<'a, str>> for DuperKey<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct DuperValue<'a> {
    pub identifier: Option<DuperIdentifier<'a>>,
    pub inner: DuperInner<'a>,
}

#[derive(Debug, Clone)]
pub struct DuperObject<'a>(pub(crate) Vec<(DuperKey<'a>, DuperValue<'a>)>);

impl<'a> DuperObject<'a> {
    pub fn into_inner(self) -> Vec<(DuperKey<'a>, DuperValue<'a>)> {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(DuperKey<'a>, DuperValue<'a>)> {
        self.0.iter()
    }
}

impl<'a> From<Vec<(DuperKey<'a>, DuperValue<'a>)>> for DuperObject<'a> {
    fn from(value: Vec<(DuperKey<'a>, DuperValue<'a>)>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct DuperArray<'a>(pub(crate) Vec<DuperValue<'a>>);

impl<'a> DuperArray<'a> {
    pub fn into_inner(self) -> Vec<DuperValue<'a>> {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &DuperValue<'a>> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&DuperValue<'a>> {
        self.0.get(index)
    }
}

impl<'a> From<Vec<DuperValue<'a>>> for DuperArray<'a> {
    fn from(value: Vec<DuperValue<'a>>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct DuperTuple<'a>(pub(crate) Vec<DuperValue<'a>>);

impl<'a> DuperTuple<'a> {
    pub fn into_inner(self) -> Vec<DuperValue<'a>> {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &DuperValue<'a>> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&DuperValue<'a>> {
        self.0.get(index)
    }
}

impl<'a> From<Vec<DuperValue<'a>>> for DuperTuple<'a> {
    fn from(value: Vec<DuperValue<'a>>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct DuperString<'a>(pub(crate) Cow<'a, str>);

impl<'a> DuperString<'a> {
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> From<Cow<'a, str>> for DuperString<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(value)
    }
}

impl<'a> AsRef<str> for DuperString<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct DuperBytes<'a>(pub(crate) Cow<'a, [u8]>);

impl<'a> DuperBytes<'a> {
    pub fn into_inner(self) -> Cow<'a, [u8]> {
        self.0
    }
}

impl<'a> From<Cow<'a, [u8]>> for DuperBytes<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        Self(value)
    }
}

impl<'a> AsRef<[u8]> for DuperBytes<'a> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum DuperInner<'a> {
    Object(DuperObject<'a>),
    Array(DuperArray<'a>),
    Tuple(DuperTuple<'a>),
    String(DuperString<'a>),
    Bytes(DuperBytes<'a>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl<'a> DuperValue<'a> {
    pub fn accept<V: DuperVisitor>(&self, visitor: &mut V) -> V::Value {
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

impl<'a> TryFrom<&'a str> for DuperValue<'a> {
    type Error = Box<pest::error::Error<DuperRule>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        DuperParser::parse_duper_value(value)
    }
}
