use std::{ascii, borrow::Cow, fmt::Display};
use unicode_general_category::{GeneralCategory, get_general_category};

#[derive(Debug)]
pub(crate) enum UnescapeError {
    InvalidByte(String),
    InvalidUnicode(String),
}

impl Display for UnescapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnescapeError::InvalidByte(hex_str) => f.write_fmt(format_args!(
                "invalid escape sequence for bytes: \\x{hex_str}"
            )),
            UnescapeError::InvalidUnicode(hex_str) => f.write_fmt(format_args!(
                "invalid escape sequence for unicode: \\u{hex_str}"
            )),
        }
    }
}

impl std::error::Error for UnescapeError {}

pub(crate) fn unescape_str<'a>(input: &'a str) -> Result<Cow<'a, str>, UnescapeError> {
    if !input.contains('\\') {
        return Ok(Cow::Borrowed(input));
    }

    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('/') => result.push('/'),
                Some('b') => result.push('\x08'),
                Some('f') => result.push('\x0C'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('x') => {
                    let hex_str: String = chars.by_ref().take(2).collect();
                    if hex_str.len() == 2
                        && let Ok(byte_val) = u8::from_str_radix(&hex_str, 16)
                    {
                        result.push(byte_val as char);
                    } else {
                        return Err(UnescapeError::InvalidByte(hex_str));
                    }
                }
                Some('u') => {
                    let hex_str: String = chars.by_ref().take(4).collect();
                    if hex_str.len() == 4
                        && let Ok(code_point) = u32::from_str_radix(&hex_str, 16)
                        && let Some(unicode_char) = char::from_u32(code_point)
                    {
                        result.push(unicode_char);
                    } else {
                        return Err(UnescapeError::InvalidUnicode(hex_str));
                    }
                }
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }

    Ok(Cow::Owned(result))
}

pub(crate) fn escape_str<'a>(input: &'a Cow<'a, str>) -> Cow<'a, str> {
    let mut result = None;

    for (i, char) in input.char_indices() {
        match char {
            '"' => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str("\\\"");
                    result
                });
            }
            '\\' => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str("\\\\");
                    result
                });
            }
            '\x08' => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str("\\b");
                    result
                });
            }
            '\x0C' => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str("\\f");
                    result
                });
            }
            '\n' => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str("\\n");
                    result
                });
            }
            '\r' => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str("\\r");
                    result
                })
            }
            '\t' => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str("\\t");
                    result
                });
            }
            c if c.is_ascii_control() => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str(&format!("\\x{:02x}", c as u8));
                    result
                });
            }
            c if is_invisible_unicode(c) => {
                result = Some({
                    let mut result = result.unwrap_or_else(|| input.split_at(i).0.to_string());
                    result.push_str(&format!("\\u{:04x}", c as u32));
                    result
                });
            }
            _ => {
                result.as_mut().map(|result| result.push(char));
            }
        }
    }

    match result {
        Some(result) => Cow::Owned(result),
        None => Cow::Borrowed(&input),
    }
}

pub(crate) fn unescape_bytes<'a>(input: &'a str) -> Result<Cow<'a, [u8]>, UnescapeError> {
    if !input.contains('\\') {
        return Ok(Cow::Borrowed(input.as_bytes()));
    }

    let mut result = Vec::with_capacity(input.len());
    let mut chars = input.chars();
    let mut buf = [0u8; 4];

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push(b'"'),
                Some('\\') => result.push(b'\\'),
                Some('/') => result.push(b'/'),
                Some('b') => result.push(b'\x08'),
                Some('f') => result.push(b'\x0C'),
                Some('n') => result.push(b'\n'),
                Some('r') => result.push(b'\r'),
                Some('t') => result.push(b'\t'),
                Some('x') => {
                    let hex_str: String = chars.by_ref().take(2).collect();
                    if hex_str.len() == 2
                        && let Ok(byte_val) = u8::from_str_radix(&hex_str, 16)
                    {
                        result.push(byte_val);
                    } else {
                        return Err(UnescapeError::InvalidByte(hex_str));
                    }
                }
                Some('u') => {
                    let hex_str: String = chars.by_ref().take(4).collect();
                    if hex_str.len() == 4
                        && let Ok(code_point) = u32::from_str_radix(&hex_str, 16)
                        && let Some(unicode_char) = char::from_u32(code_point)
                    {
                        result.extend_from_slice(unicode_char.encode_utf8(&mut buf).as_bytes());
                    } else {
                        return Err(UnescapeError::InvalidUnicode(hex_str));
                    }
                }
                Some(other) => {
                    result.push(b'\\');
                    result.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
                }
                None => result.push(b'\\'),
            }
        } else {
            result.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        }
    }

    Ok(Cow::Owned(result))
}

pub(crate) fn escape_bytes<'a>(input: &'a Cow<'a, [u8]>) -> String {
    input
        .iter()
        .copied()
        .flat_map(ascii::escape_default)
        .map(|b| b as char)
        .collect()
}

pub(crate) fn is_invisible_unicode(c: char) -> bool {
    let category = get_general_category(c);

    match category {
        // Control characters
        GeneralCategory::Control
        | GeneralCategory::Format
        | GeneralCategory::Surrogate
        // Characters that are typically invisible
        | GeneralCategory::NonspacingMark
        | GeneralCategory::EnclosingMark
        | GeneralCategory::SpacingMark
        | GeneralCategory::LineSeparator
        | GeneralCategory::ParagraphSeparator
        // Private use and unassigned (might be invisible)
        | GeneralCategory::PrivateUse
        | GeneralCategory::Unassigned => true,

        _ => {
            matches!(
                c,
                // Zero-width characters
                '\u{200B}'
                | '\u{200C}'
                | '\u{200D}'
                // Word joiners, BOM
                | '\u{2060}'
                | '\u{FEFF}' 
                 // Interlinear annotation chars
                | '\u{FFF9}'..='\u{FFFB}'
            ) || c.is_whitespace() && c != ' '
        }
    }
}
