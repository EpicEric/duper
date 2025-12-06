use std::borrow::Cow;

use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{
    ast::DuperKey,
    escape::{escape_bytes, escape_str, is_invisible_unicode},
};

pub fn format_key<'a>(key: &'a DuperKey<'a>) -> Cow<'a, str> {
    if key.0.is_empty() {
        return Cow::Borrowed(r#""""#);
    }
    let mut was_underscore_or_hyphen = false;
    for (i, c) in key.0.char_indices() {
        if i == 0 {
            if c == '_' {
                was_underscore_or_hyphen = true;
            } else if !c.is_ascii_alphabetic() {
                return Cow::Owned(format_duper_string(key.0.as_ref()));
            }
        } else if c == '_' || c == '-' {
            if was_underscore_or_hyphen {
                return Cow::Owned(format_duper_string(key.0.as_ref()));
            }
            was_underscore_or_hyphen = true;
        } else if c.is_ascii_alphanumeric() {
            was_underscore_or_hyphen = false;
        } else {
            return Cow::Owned(format_duper_string(key.0.as_ref()));
        }
    }
    if was_underscore_or_hyphen {
        Cow::Owned(format_duper_string(key.0.as_ref()))
    } else {
        Cow::Borrowed(key.0.as_ref())
    }
}

pub fn format_duper_string(string: &str) -> String {
    if string.is_empty() {
        // Empty string
        return r#""""#.into();
    }
    // Check if it's benefic to turn into a raw string
    let mut chars_to_escape = 0usize;
    let mut was_quotes = false;
    let mut was_hashtag = false;
    let mut curr_hashtags = 0usize;
    let mut max_hashtags = 0usize;
    let mut has_char_that_should_be_escaped = false;
    for char in string.chars() {
        match char {
            '"' => {
                was_hashtag = false;
                was_quotes = true;
                chars_to_escape += 1;
                max_hashtags = max_hashtags.max(1);
            }
            '#' if was_hashtag => {
                curr_hashtags += 1;
                max_hashtags = max_hashtags.max(curr_hashtags);
            }
            '#' if was_quotes => {
                was_hashtag = true;
                was_quotes = false;
                curr_hashtags = 2;
                max_hashtags = max_hashtags.max(curr_hashtags);
            }
            ' ' => {
                was_hashtag = false;
                was_quotes = false;
            }
            '\\' => {
                was_hashtag = false;
                was_quotes = false;
                chars_to_escape += 1;
            }
            char if char.is_control() || char.is_whitespace() || is_invisible_unicode(char) => {
                has_char_that_should_be_escaped = true;
                break;
            }
            _ => {
                was_hashtag = false;
                was_quotes = false;
            }
        }
    }
    if chars_to_escape > max_hashtags && !has_char_that_should_be_escaped {
        // Raw string
        let hashtags: String = (0..max_hashtags).map(|_| '#').collect();
        format!(r#"r{hashtags}"{string}"{hashtags}"#)
    } else {
        // Regular string with escaping
        let cow = Cow::Borrowed(string);
        let escaped_key = escape_str(&cow);
        format!(r#""{escaped_key}""#)
    }
}

pub fn format_duper_bytes(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        // Empty bytes
        return r#"b"""#.into();
    }
    // Check if it's benefic to turn into raw bytes
    let mut bytes_to_escape = 0usize;
    let mut escaped_bytes_length = 0usize;
    let mut was_quotes = false;
    let mut was_hashtag = false;
    let mut curr_hashtags = 0usize;
    let mut max_hashtags = 0usize;
    let mut has_char_that_should_be_escaped = false;
    for byte in bytes.iter() {
        match byte {
            b'"' => {
                was_hashtag = false;
                was_quotes = true;
                bytes_to_escape += 1;
                escaped_bytes_length += 1;
            }
            b'#' if was_hashtag => {
                curr_hashtags += 1;
                max_hashtags = max_hashtags.max(curr_hashtags);
            }
            b'#' if was_quotes => {
                was_hashtag = true;
                was_quotes = false;
                curr_hashtags = 2;
                max_hashtags = max_hashtags.max(curr_hashtags);
            }
            b' ' => {
                was_hashtag = false;
                was_quotes = false;
            }
            b'\\' => {
                was_hashtag = false;
                was_quotes = false;
                bytes_to_escape += 1;
                escaped_bytes_length += 1;
            }
            b'\0' | b'\n' | b'\r' | b'\t' => {
                has_char_that_should_be_escaped = true;
                bytes_to_escape += 1;
                escaped_bytes_length += 1;
                was_hashtag = false;
                was_quotes = false;
            }
            byte if byte.is_ascii_control() || byte.is_ascii_whitespace() => {
                has_char_that_should_be_escaped = true;
                bytes_to_escape += 1;
                escaped_bytes_length += 3;
                was_hashtag = false;
                was_quotes = false;
            }
            _ => {
                was_hashtag = false;
                was_quotes = false;
            }
        }
    }
    if bytes_to_escape > max_hashtags && !has_char_that_should_be_escaped {
        // Raw bytes
        let hashtags: String = (0..max_hashtags).map(|_| '#').collect();
        let unesecaped_bytes: String = bytes.iter().copied().map(|b| b as char).collect();
        format!(r#"br{hashtags}"{unesecaped_bytes}"{hashtags}"#)
    } else if 3 * (escaped_bytes_length + bytes.len()) > (bytes.len() << 2) + 5 {
        // Base64 bytes
        let base64_bytes = BASE64_STANDARD.encode(bytes.as_ref());
        format!(r#"b64"{base64_bytes}""#)
    } else {
        // Regular bytes with escaping
        let cow = Cow::Borrowed(bytes);
        let escaped_bytes = escape_bytes(&cow);
        format!(r#"b"{escaped_bytes}""#)
    }
}

pub fn format_temporal(temporal: impl AsRef<str>) -> String {
    format!("'{}'", temporal.as_ref())
}

pub fn format_integer(integer: i64) -> String {
    integer.to_string()
}

pub fn format_float(float: f64) -> String {
    ryu::Buffer::new().format(float).into()
}

pub fn format_boolean(bool: bool) -> &'static str {
    if bool { "true" } else { "false" }
}

pub fn format_null() -> &'static str {
    "null"
}
