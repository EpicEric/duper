//! Utilities for generating ANSI sequences from Duper values.

use std::io::{Error, Write};

use crate::{
    ast::{
        DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTuple, DuperValue,
    },
    format::{
        format_boolean, format_duper_bytes, format_duper_string, format_float, format_integer,
        format_key, format_null,
    },
    visitor::DuperVisitor,
};
use owo_colors::{AnsiColors, DynColors, OwoColorize};

/// A Duper visitor which generates colored ANSI escaping.
pub struct Ansi<'ansi> {
    buf: Vec<u8>,
    strip_identifiers: bool,
    theme: &'ansi AnsiTheme<'ansi>,
    bracket_depth: usize,
}

/// A struct representing a theme from whose colors the [`Ansi`] visitor will
/// use.
#[derive(Debug, Clone)]
pub struct AnsiTheme<'theme> {
    /// Duper identifiers: `Identifier(...)`
    pub identifier: DynColors,
    /// Duper keys: `{foo: ..., "bar": ...}`
    pub key: DynColors,
    /// Duper strings: `("Hello", r"#world")`
    pub string: DynColors,
    /// Duper bytes: `(b"Hello", br"#world")`
    pub bytes: DynColors,
    /// Duper integers: `(42, 0xdeadbeef)`
    pub integer: DynColors,
    /// Duper floats: `(2.17, 3.5e50)`
    pub float: DynColors,
    /// Duper booleans: `(true, false)`
    pub boolean: DynColors,
    /// Duper null: `null`
    pub null: DynColors,
    /// Brackets (for identifiers, arrays, tuples, and objects): `Id([({...})])`
    ///
    /// By default, [`Ansi`] will iterate over brackets, matching their colors
    /// and looping around when the slice is exhausted.
    ///
    /// An empty slice will disable coloring of brackets.
    pub brackets: &'theme [DynColors],
}

/// A theme using standard ANSI colors. This is the theme used in
/// [`Default::default()`].
pub static ANSI_THEME: &AnsiTheme = &AnsiTheme {
    identifier: DynColors::Ansi(AnsiColors::BrightBlue),
    key: DynColors::Ansi(AnsiColors::BrightCyan),
    string: DynColors::Ansi(AnsiColors::BrightRed),
    bytes: DynColors::Ansi(AnsiColors::BrightRed),
    integer: DynColors::Ansi(AnsiColors::BrightGreen),
    float: DynColors::Ansi(AnsiColors::BrightGreen),
    boolean: DynColors::Ansi(AnsiColors::Blue),
    null: DynColors::Ansi(AnsiColors::Blue),
    brackets: &[
        DynColors::Ansi(AnsiColors::Yellow),
        DynColors::Ansi(AnsiColors::Magenta),
        DynColors::Ansi(AnsiColors::Blue),
    ],
};

/// A theme using the colors for VSCode's Dark+ theme.
pub static VSCODE_DARK_PLUS_THEME: &AnsiTheme = &AnsiTheme {
    identifier: DynColors::Rgb(0x4E, 0xC9, 0xB0),
    key: DynColors::Rgb(0x9C, 0xDC, 0xFE),
    string: DynColors::Rgb(0xCE, 0x91, 0x78),
    bytes: DynColors::Rgb(0xCE, 0x91, 0x78),
    integer: DynColors::Rgb(0xB5, 0xCE, 0xA8),
    float: DynColors::Rgb(0xB5, 0xCE, 0xA8),
    boolean: DynColors::Rgb(0x56, 0x9C, 0xD6),
    null: DynColors::Rgb(0x56, 0x9C, 0xD6),
    brackets: &[
        DynColors::Rgb(0xFF, 0xD7, 0x00),
        DynColors::Rgb(0xDA, 0x70, 0xD6),
        DynColors::Rgb(0x17, 0x9F, 0xFF),
    ],
};

// TO-DO: More ANSI themes

impl Default for Ansi<'static> {
    fn default() -> Self {
        Self {
            buf: Vec::new(),
            strip_identifiers: false,
            theme: ANSI_THEME,
            bracket_depth: 0,
        }
    }
}

impl<'ansi> Ansi<'ansi> {
    /// Create a new [`Ansi`] visitor with the provided option and desired
    /// theme.
    pub fn new(strip_identifiers: bool, theme: &'ansi AnsiTheme) -> Self {
        Self {
            buf: Vec::new(),
            strip_identifiers,
            theme,
            bracket_depth: 0,
        }
    }

    /// Convert the [`DuperValue`] into a [`Vec`] of bytes.
    pub fn to_ansi<'a>(&mut self, value: DuperValue<'a>) -> Result<Vec<u8>, Error> {
        self.buf.clear();
        value.accept(self)?;
        Ok(std::mem::take(&mut self.buf))
    }

    fn increase_bracket_depth(&mut self) {
        self.bracket_depth += 1;
    }

    fn decrease_bracket_depth(&mut self) {
        self.bracket_depth -= 1;
    }

    fn colorize_bracket(&mut self, bracket: &str) -> Result<(), Error> {
        if self.theme.brackets.is_empty() {
            self.buf.write_all(bracket.as_bytes())?;
            Ok(())
        } else {
            self.buf.write_fmt(format_args!(
                "{}",
                bracket.color(self.theme.brackets[self.bracket_depth % self.theme.brackets.len()])
            ))
        }
    }
}

impl<'ansi> DuperVisitor for Ansi<'ansi> {
    type Value = Result<(), Error>;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let len = object.len();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.colorize_bracket("{")?;
            self.increase_bracket_depth();
            for (i, (key, value)) in object.iter().enumerate() {
                self.buf
                    .write_fmt(format_args!("{}", format_key(key).color(self.theme.key)))?;
                self.buf.write_all(b": ")?;
                value.accept(self)?;
                if i < len - 1 {
                    self.buf.write_all(b", ")?;
                }
            }
            self.decrease_bracket_depth();
            self.colorize_bracket("}")?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.colorize_bracket("{")?;
            self.increase_bracket_depth();
            for (i, (key, value)) in object.iter().enumerate() {
                self.buf
                    .write_fmt(format_args!("{}", format_key(key).color(self.theme.key)))?;
                self.buf.write_all(b": ")?;
                value.accept(self)?;
                if i < len - 1 {
                    self.buf.write_all(b", ")?;
                }
            }
            self.decrease_bracket_depth();
            self.colorize_bracket("}")?;
        }

        Ok(())
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value {
        let len = array.len();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.colorize_bracket("[")?;
            self.increase_bracket_depth();
            for (i, value) in array.iter().enumerate() {
                value.accept(self)?;
                if i < len - 1 {
                    self.buf.write_all(b", ")?;
                }
            }
            self.decrease_bracket_depth();
            self.colorize_bracket("]")?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.colorize_bracket("[")?;
            self.increase_bracket_depth();
            for (i, value) in array.iter().enumerate() {
                value.accept(self)?;
                if i < len - 1 {
                    self.buf.write_all(b", ")?;
                }
            }
            self.decrease_bracket_depth();
            self.colorize_bracket("]")?;
        }

        Ok(())
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &DuperTuple<'a>,
    ) -> Self::Value {
        let len = tuple.len();

        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            for (i, value) in tuple.iter().enumerate() {
                value.accept(self)?;
                if i < len - 1 {
                    self.buf.write_all(b", ")?;
                }
            }
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            for (i, value) in tuple.iter().enumerate() {
                value.accept(self)?;
                if i < len - 1 {
                    self.buf.write_all(b", ")?;
                }
            }
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        }

        Ok(())
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        value: &DuperString<'a>,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.buf.write_fmt(format_args!(
                "{}",
                format_duper_string(value).color(self.theme.string)
            ))?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.buf.write_fmt(format_args!(
                "{}",
                format_duper_string(value).color(self.theme.string)
            ))?;
        }

        Ok(())
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.buf.write_fmt(format_args!(
                "{}",
                format_duper_bytes(bytes).color(self.theme.bytes)
            ))?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.buf.write_fmt(format_args!(
                "{}",
                format_duper_bytes(bytes).color(self.theme.bytes)
            ))?;
        }

        Ok(())
    }

    fn visit_integer(
        &mut self,
        identifier: Option<&DuperIdentifier<'_>>,
        integer: i64,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.buf.write_fmt(format_args!(
                "{}",
                format_integer(integer).color(self.theme.integer)
            ))?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.buf.write_fmt(format_args!(
                "{}",
                format_integer(integer).color(self.theme.integer)
            ))?;
        }

        Ok(())
    }

    fn visit_float(&mut self, identifier: Option<&DuperIdentifier<'_>>, float: f64) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.buf.write_fmt(format_args!(
                "{}",
                format_float(float).color(self.theme.float)
            ))?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.buf.write_fmt(format_args!(
                "{}",
                format_float(float).color(self.theme.float)
            ))?;
        }

        Ok(())
    }

    fn visit_boolean(
        &mut self,
        identifier: Option<&DuperIdentifier<'_>>,
        boolean: bool,
    ) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.buf.write_fmt(format_args!(
                "{}",
                format_boolean(boolean).color(self.theme.boolean)
            ))?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.buf.write_fmt(format_args!(
                "{}",
                format_boolean(boolean).color(self.theme.boolean)
            ))?;
        }

        Ok(())
    }

    fn visit_null(&mut self, identifier: Option<&DuperIdentifier<'_>>) -> Self::Value {
        if !self.strip_identifiers
            && let Some(identifier) = identifier
        {
            self.buf.write_fmt(format_args!(
                "{}",
                identifier.as_ref().color(self.theme.identifier)
            ))?;
            self.colorize_bracket("(")?;
            self.increase_bracket_depth();
            self.buf
                .write_fmt(format_args!("{}", format_null().color(self.theme.null)))?;
            self.decrease_bracket_depth();
            self.colorize_bracket(")")?;
        } else {
            self.buf
                .write_fmt(format_args!("{}", format_null().color(self.theme.null)))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod ansi_tests {
    use std::borrow::Cow;

    use insta::assert_debug_snapshot;

    use crate::{
        Ansi, DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject,
        DuperString, DuperTuple, DuperValue,
        visitor::ansi::{ANSI_THEME, VSCODE_DARK_PLUS_THEME},
    };

    fn example_value() -> DuperValue<'static> {
        DuperValue {
            identifier: Some(DuperIdentifier(Cow::Borrowed("Product"))),
            inner: DuperInner::Object(DuperObject(vec![
                (
                    DuperKey(Cow::Borrowed("product_id")),
                    DuperValue {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Uuid"))),
                        inner: DuperInner::String(DuperString(Cow::Borrowed(
                            "1dd7b7aa-515e-405f-85a9-8ac812242609",
                        ))),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("name")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::String(DuperString(Cow::Borrowed(
                            "Wireless Bluetooth Headphones",
                        ))),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("brand")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::String(DuperString(Cow::Borrowed("AudioTech"))),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("price")),
                    DuperValue {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Decimal"))),
                        inner: DuperInner::String(DuperString(Cow::Borrowed("129.99"))),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("dimensions")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Tuple(DuperTuple(vec![
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::Float(18.5),
                            },
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::Float(15.2),
                            },
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::Float(7.8),
                            },
                        ])),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("weight")),
                    DuperValue {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Weight"))),
                        inner: DuperInner::Float(0.285),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("in_stock")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Boolean(true),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("specifications")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Object(DuperObject(vec![
                            (
                                DuperKey(Cow::Borrowed("battery_life")),
                                DuperValue {
                                    identifier: Some(DuperIdentifier(Cow::Borrowed("Duration"))),
                                    inner: DuperInner::String(DuperString(Cow::Borrowed("30h"))),
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("noise_cancellation")),
                                DuperValue {
                                    identifier: None,
                                    inner: DuperInner::Boolean(true),
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("connectivity")),
                                DuperValue {
                                    identifier: None,
                                    inner: DuperInner::Array(DuperArray(vec![
                                        DuperValue {
                                            identifier: None,
                                            inner: DuperInner::String(DuperString(Cow::Borrowed(
                                                "Bluetooth 5.0",
                                            ))),
                                        },
                                        DuperValue {
                                            identifier: None,
                                            inner: DuperInner::String(DuperString(Cow::Borrowed(
                                                "3.5mm Jack",
                                            ))),
                                        },
                                    ])),
                                },
                            ),
                        ])),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("image_thumbnail")),
                    DuperValue {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Png"))),
                        inner: DuperInner::Bytes(DuperBytes(Cow::Borrowed(
                            b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x64",
                        ))),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("tags")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Array(DuperArray(vec![
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::String(DuperString(Cow::Borrowed(
                                    "electronics",
                                ))),
                            },
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::String(DuperString(Cow::Borrowed("audio"))),
                            },
                            DuperValue {
                                identifier: None,
                                inner: DuperInner::String(DuperString(Cow::Borrowed("wireless"))),
                            },
                        ])),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("release_date")),
                    DuperValue {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Date"))),
                        inner: DuperInner::String(DuperString(Cow::Borrowed("2023-11-15"))),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("warranty_period")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Null,
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("customer_ratings")),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Object(DuperObject(vec![
                            (
                                DuperKey(Cow::Borrowed("latest_review")),
                                DuperValue {
                                    identifier: None,
                                    inner: DuperInner::String(DuperString(Cow::Borrowed(
                                        r#"Absolutely ""astounding""!! 😎"#,
                                    ))),
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("average")),
                                DuperValue {
                                    identifier: None,
                                    inner: DuperInner::Float(4.5),
                                },
                            ),
                            (
                                DuperKey(Cow::Borrowed("count")),
                                DuperValue {
                                    identifier: None,
                                    inner: DuperInner::Integer(127),
                                },
                            ),
                        ])),
                    },
                ),
                (
                    DuperKey(Cow::Borrowed("created_at")),
                    DuperValue {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("DateTime"))),
                        inner: DuperInner::String(DuperString(Cow::Borrowed(
                            "2023-11-17T21:50:43+00:00",
                        ))),
                    },
                ),
            ])),
        }
    }

    #[test]
    fn ansi() {
        let mut ansi = Ansi::new(false, ANSI_THEME);
        let printed = ansi.to_ansi(example_value()).unwrap();
        println!("{}", str::from_utf8(&printed).unwrap());
        assert_debug_snapshot!(printed);
    }

    #[test]
    fn vscode_dark_plus() {
        let mut ansi = Ansi::new(false, VSCODE_DARK_PLUS_THEME);
        let printed = ansi.to_ansi(example_value()).unwrap();
        println!("{}", str::from_utf8(&printed).unwrap());
        assert_debug_snapshot!(printed);
    }
}
