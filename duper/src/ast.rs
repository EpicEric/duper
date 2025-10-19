use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::{DuperParser, DuperRule, visitor::DuperVisitor};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperIdentifier<'a>(pub(crate) Cow<'a, str>);

#[derive(Debug, Clone)]
pub struct DuperValue<'a> {
    pub identifier: Option<DuperIdentifier<'a>>,
    pub inner: DuperInner<'a>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperKey<'a>(pub(crate) Cow<'a, str>);

#[derive(Debug, Clone)]
pub struct DuperObject<'a>(pub(crate) Vec<(DuperKey<'a>, DuperValue<'a>)>);

#[derive(Debug, Clone, PartialEq)]
pub struct DuperArray<'a>(pub(crate) Vec<DuperValue<'a>>);

#[derive(Debug, Clone, PartialEq)]
pub struct DuperTuple<'a>(pub(crate) Vec<DuperValue<'a>>);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperString<'a>(pub(crate) Cow<'a, str>);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DuperBytes<'a>(pub(crate) Cow<'a, [u8]>);

#[derive(Debug, Clone)]
pub enum DuperIdentifierTryFromError<'a> {
    EmptyIdentifier,
    InvalidChar(Cow<'a, str>, usize),
}

#[derive(Debug, Clone)]
pub enum DuperObjectTryFromError<'a> {
    DuplicateKey(Cow<'a, str>),
}

impl<'a> DuperIdentifier<'a> {
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    pub fn try_from_lossy(value: Cow<'a, str>) -> Result<Self, DuperIdentifierTryFromError<'a>> {
        #[allow(unused_assignments)]
        let mut new_value = None;
        let mut chars = value.char_indices();
        match chars.next() {
            None => return Err(DuperIdentifierTryFromError::EmptyIdentifier),
            Some((_, c)) if c.is_alphabetic() && !c.is_uppercase() => {
                new_value = Some(c.to_uppercase().to_string());
            }
            Some((_, c)) if c.is_alphabetic() && c.is_uppercase() => (),
            _ => return Err(DuperIdentifierTryFromError::InvalidChar(value, 0)),
        }
        let mut last_char_was_separator = false;
        for (pos, char) in chars {
            match char {
                '-' | '_' => {
                    match new_value.as_mut() {
                        Some(new_value) if !last_char_was_separator => {
                            new_value.push(char);
                        }
                        _ => (),
                    }
                    last_char_was_separator = true;
                }
                char if char.is_alphanumeric() => {
                    match new_value.as_mut() {
                        Some(new_value) => {
                            new_value.push(char);
                        }
                        None => (),
                    }
                    last_char_was_separator = false;
                }
                _ => match new_value.as_mut() {
                    Some(_) => (),
                    None => new_value = Some(value.split_at(pos).0.to_owned()),
                },
            }
        }
        Ok(Self(match new_value {
            Some(new_value) if new_value.is_empty() => {
                return Err(DuperIdentifierTryFromError::EmptyIdentifier);
            }
            Some(new_value) => Cow::Owned(new_value),
            None => value,
        }))
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

impl<'a> TryFrom<Cow<'a, str>> for DuperIdentifier<'a> {
    type Error = DuperIdentifierTryFromError<'a>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let mut chars = value.char_indices();
        match chars.next() {
            None => return Err(DuperIdentifierTryFromError::EmptyIdentifier),
            Some((_, c)) if !c.is_uppercase() => {
                return Err(DuperIdentifierTryFromError::InvalidChar(value, 0));
            }
            _ => (),
        }
        let mut last_char_was_separator = false;
        for (pos, char) in chars {
            match char {
                '-' | '_' => {
                    if last_char_was_separator {
                        return Err(DuperIdentifierTryFromError::InvalidChar(value, pos));
                    } else {
                        last_char_was_separator = true;
                    }
                }
                char if char.is_alphanumeric() => {
                    last_char_was_separator = false;
                }
                _ => return Err(DuperIdentifierTryFromError::InvalidChar(value, pos)),
            }
        }
        Ok(Self(value))
    }
}

impl Display for DuperIdentifierTryFromError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuperIdentifierTryFromError::EmptyIdentifier => f.write_str("empty identifier"),
            DuperIdentifierTryFromError::InvalidChar(identifier, pos) => f.write_fmt(format_args!(
                "invalid character in position {pos} of identifier {identifier}"
            )),
        }
    }
}

impl std::error::Error for DuperIdentifierTryFromError<'_> {}

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

impl<'a> PartialEq for DuperValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

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

impl<'a> TryFrom<Vec<(DuperKey<'a>, DuperValue<'a>)>> for DuperObject<'a> {
    type Error = DuperObjectTryFromError<'a>;

    fn try_from(value: Vec<(DuperKey<'a>, DuperValue<'a>)>) -> Result<Self, Self::Error> {
        let mut keys = std::collections::HashSet::with_capacity(value.len());
        for (key, _) in value.iter() {
            if keys.contains(key) {
                return Err(DuperObjectTryFromError::DuplicateKey(key.0.clone()));
            }
            keys.insert(key);
        }
        Ok(Self(value))
    }
}

impl<'a> PartialEq for DuperObject<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        let other_map: HashMap<_, _> = other.0.iter().map(|(k, v)| (k.clone(), v)).collect();
        for (k, v) in self.0.iter() {
            match other_map.get(k) {
                Some(v2) => {
                    if v != *v2 {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}

impl Display for DuperObjectTryFromError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuperObjectTryFromError::DuplicateKey(key) => {
                f.write_fmt(format_args!("duplicate key {key} in object"))
            }
        }
    }
}

impl std::error::Error for DuperObjectTryFromError<'_> {}

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
