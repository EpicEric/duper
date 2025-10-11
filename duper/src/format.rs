use json_escape::explicit::escape_str;
use std::{ascii, borrow::Cow};

use crate::{
    ast::{DuperBytes, DuperKey, DuperString},
    types::DuperTypes,
};

pub(crate) fn format_key<'a>(key: &'a DuperKey<'a>) -> Cow<'a, str> {
    if key.0.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Cow::Borrowed(key.0.as_ref())
    } else if key.0.is_empty() {
        Cow::Borrowed(r#""""#)
    } else {
        // TO-DO raw string heuristics
        let escaped_key = Cow::from(escape_str(&key.0)).into_owned();
        Cow::Owned(format!(r#""{escaped_key}""#))
    }
}

pub(crate) fn format_string<'a>(string: &'a DuperString<'a>) -> Cow<'a, str> {
    if string.0.is_empty() {
        Cow::Borrowed(r#""""#)
    } else {
        // TO-DO raw string heuristics
        let escaped_key = Cow::from(escape_str(&string.0)).into_owned();
        Cow::Owned(format!(r#""{escaped_key}""#))
    }
}

pub(crate) fn format_bytes<'a>(bytes: &'a DuperBytes<'a>) -> Cow<'a, str> {
    if bytes.0.is_empty() {
        return Cow::Borrowed(r#"b"""#);
    }
    // TO-DO raw bytes heuristics
    let escaped_bytes: String = bytes
        .0
        .into_iter()
        .copied()
        .flat_map(ascii::escape_default)
        .map(|b| b as char)
        .collect();
    Cow::Owned(format!(r#"b"{escaped_bytes}""#))
}

pub(crate) fn format_integer(integer: i64, typ: Option<DuperTypes>) -> String {
    match typ {
        Some(DuperTypes::HexInteger) => format!("0x{integer:x}"),
        Some(DuperTypes::OctInteger) => format!("0o{integer:o}"),
        Some(DuperTypes::BinInteger) => format!("0b{integer:b}"),
        _ => integer.to_string(),
    }
}

pub(crate) fn format_float(float: f64) -> String {
    float.to_string()
}

pub(crate) fn format_boolean(bool: bool) -> String {
    bool.to_string()
}
