use std::borrow::Cow;

use crate::{
    ast::{DuperBytes, DuperKey, DuperString},
    escape::{escape_bytes, escape_str, is_invisible_unicode},
};

pub(crate) fn format_key<'a>(key: &'a DuperKey<'a>) -> Cow<'a, str> {
    if key.0.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Cow::Borrowed(key.0.as_ref())
    } else {
        format_cow_str(&key.0)
    }
}

pub(crate) fn format_duper_string<'a>(string: &'a DuperString<'a>) -> Cow<'a, str> {
    format_cow_str(&string.0)
}

fn format_cow_str<'a>(string: &Cow<'a, str>) -> Cow<'a, str> {
    if string.is_empty() {
        // Empty string
        Cow::Borrowed(r#""""#)
    } else {
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
                }
                '#' if was_hashtag => {
                    curr_hashtags += 1;
                    max_hashtags = max_hashtags.max(curr_hashtags);
                }
                '#' if was_quotes => {
                    was_hashtag = true;
                    was_quotes = false;
                    curr_hashtags = 1;
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
            Cow::Owned(format!(r#"r{}"{}"{}"#, hashtags, string, hashtags))
        } else {
            // Regular string with escaping
            let escaped_key = escape_str(string);
            Cow::Owned(format!(r#""{escaped_key}""#))
        }
    }
}

pub(crate) fn format_duper_bytes<'a>(bytes: &'a DuperBytes<'a>) -> Cow<'a, str> {
    if bytes.0.is_empty() {
        // Empty bytes
        Cow::Borrowed(r#"b"""#)
    } else {
        // Check if it's benefic to turn into raw bytes
        let mut bytes_to_escape = 0usize;
        let mut was_quotes = false;
        let mut was_hashtag = false;
        let mut curr_hashtags = 0usize;
        let mut max_hashtags = 0usize;
        let mut has_char_that_should_be_escaped = false;
        for byte in bytes.0.iter() {
            match byte {
                b'"' => {
                    was_hashtag = false;
                    was_quotes = true;
                    bytes_to_escape += 1;
                }
                b'#' if was_hashtag => {
                    curr_hashtags += 1;
                    max_hashtags = max_hashtags.max(curr_hashtags);
                }
                b'#' if was_quotes => {
                    was_hashtag = true;
                    was_quotes = false;
                    curr_hashtags = 1;
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
                }
                byte if byte.is_ascii_control() || byte.is_ascii_whitespace() => {
                    has_char_that_should_be_escaped = true;
                    break;
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
            let unesecaped_bytes: String = bytes.0.iter().copied().map(|b| b as char).collect();
            Cow::Owned(format!(
                r#"br{}"{}"{}"#,
                hashtags, unesecaped_bytes, hashtags
            ))
        } else {
            // Regular bytes with escaping
            let escaped_bytes = escape_bytes(&bytes.0);
            Cow::Owned(format!(r#"b"{escaped_bytes}""#))
        }
    }
}

pub(crate) fn format_integer(integer: i64) -> String {
    integer.to_string()
}

pub(crate) fn format_float(float: f64) -> String {
    ryu::Buffer::new().format(float).into()
}

pub(crate) fn format_boolean(bool: bool) -> &'static str {
    if bool { "true" } else { "false" }
}

pub(crate) fn format_null() -> &'static str {
    "null"
}
