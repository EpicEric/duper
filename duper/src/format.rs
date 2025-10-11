use json_escape::explicit::escape_str;
use std::{ascii, borrow::Cow};

use crate::types::DuperTypes;

pub(crate) fn format_key<'a>(key: Cow<'a, str>) -> Cow<'a, str> {
    if key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        key
    } else {
        format_string(key)
    }
}

pub(crate) fn format_string<'a>(string: Cow<'a, str>) -> Cow<'a, str> {
    if string.is_empty() {
        return Cow::Borrowed(r#""""#);
    }
    // TO-DO raw string heuristics
    let escaped_key = Cow::from(escape_str(&string)).into_owned();
    Cow::Owned(format!(r#""{escaped_key}""#))
}

pub(crate) fn format_bytes<'a>(bytes: Cow<'a, [u8]>) -> Cow<'a, str> {
    if bytes.is_empty() {
        return Cow::Borrowed(r#"b"""#);
    }
    // TO-DO raw bytes heuristics
    let escaped_bytes: String = bytes
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

pub(crate) fn format_float(float: f64, _typ: Option<DuperTypes>) -> String {
    float.to_string()
}

pub(crate) fn format_boolean(bool: bool) -> String {
    bool.to_string()
}
