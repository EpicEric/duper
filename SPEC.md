<p align="center">
    <img src="https://raw.githubusercontent.com/EpicEric/duper/refs/heads/main/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">Duper: The format that's super!</h1>

<p align="center">
    <a href="https://github.com/EpicEric/duper/blob/main/SPEC.md"><img alt="Specification version" src="https://img.shields.io/badge/spec_version-0.2.0-blue"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

Duper aims to be a human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.

## Preliminaries

- Duper is case-sensitive and must be a valid UTF-8 encoded Unicode document.
- Whitespace means tab (U+0009), space (U+0020), line feed (U+000A), or carriage return (U+000D).
- Newline means line feed (U+000A) or carriage return (U+000D).
- Files must have only one root value. Objects and arrays as the root value must always be accepted by a Duper parser, but implementations may allow other values as the root value.

## Comments

Two forward slashes `//` mark the rest of the line of a comment, except when inside of a string.

```duper
// This is a full-line comment
{
  key: "value",  // This is a comment at the end of a line
  another: "// This is not a comment"
}
```

The area delimited by a forward slash immediately followed by an asterisk `/*`, and an asterisk immediately followed by a forward slash `*/`, is a comment that may span multiple lines, except when inside of a string.

```duper
/* This is a fancier comment */
{
  /* Notice here that we can also span
     multiple lines in a single comment. Neat!
     */
  key: "value",
  another: "/* This is not a comment,
despite spanning multiple lines */"
}
```

Comments should be used to communicate between the human readers of a file. Parsers must not modify keys or values, based on the presence (or contents) of a comment.

## Objects

Objects are composed of zero or more key/value pairs.

Keys are on the left of the colon `:` and values are on the right. Whitespace is ignored around key names, colon, and values. The key, colon, and value may be on the same line or different ones.

```duper
{
  key: "value",
  anotherKey
    :
      42
}
```

There must be a comma `,` between key/value pairs.

```duper
{
  key: "value"  // INVALID: Missing comma
  foo: "bar"
}
```

Additionally, there may be a comma after the last key-value pair.

```duper
{
  key: "value",
  foo: "bar",  // Comma here is valid
}
```

Values must have one of the following types:

- [Object](#objects)
- [Array](#arrays)
- [Tuple](#tuples)
- [String](#strings)
- [Byte string](#byte-strings)
- [Integer](#integers)
- [Float](#floats)
- [Boolean](#booleans)
- [Null](#null)

## Keys

A key may be either plain, quoted, or raw.

**Plain keys** may only contain ASCII letters, ASCII digits, underscores `_`, and hyphens `-`. They must start with an ASCII letter, or an underscore followed by a letter or digit. Sequences of underscores and hyphens are not allowed, and bare keys must not end with them.

```duper
{
  key: "value",
  bare_key: "value",
  bare-key: "value",
  _1234: "value",
}
```

**Quoted keys** follow the exact same rules as quoted strings.

```duper
{
  "127.0.0.1": "value",
  "character encoding": "value",
  "ʎǝʞ": "value",
  "": "value",
}
```

**Raw keys** follow the exact same rules as raw strings.

```duper
{
  r"key2": "value",
  r#"quoted "value""#: "value",
}
```

Indentation is treated as whitespace and ignored.

Defining a key multiple times is invalid. Note that plain keys, quoted keys, and raw keys are equivalent.

```duper
{
  name: "Eric Rodrigues Pires",
  name: "Epic Eric",  // INVALID
  "n\x61me": "Eric",  // INVALID
  r"name": "Eric",  // INVALID
}
```

## Strings

A string may be either quoted or raw.

**Quoted strings** are surrounded by quotation marks `"`. Any Unicode character may be used, except those that must be escaped: quotation mark `"`, backslash `\`, and the control characters other than tab (U+0000 to U+0008, U+000A to U+001F, U+007F).

```duper
{
  str: "I'm a string. \"You can quote me\". Name\tJos\xE9\nLocation\tBR.",
}
```

For convenience, some popular characters have a compact escape sequence:

```duper
[
  r#" \b     - backspace       (U+0008) "#,
  r#" \f     - form feed       (U+000C) "#,
  r#" \n     - line feed       (U+000A) "#,
  r#" \r     - carriage return (U+000D) "#,
  r#" \t     - tab             (U+0009) "#,
  r#" \0     - null            (U+0000) "#,
  r#" \"     - quote           (U+0022) "#,
  r#" \\     - backslash       (U+005C) "#,
  r#" \xHH   - arbitrary byte  (U+00HH) "#,
  r#" \uHHHH - unicode         (U+HHHH) "#,
]
```

Any Unicode character may be escaped with `\uHHHH` or a sequence of one or more `\xHH`. The escape codes must be Unicode [scalar values](https://unicode.org/glossary/#unicode_scalar_value).

Keep in mind that Duper strings are sequences of Unicode characters, _not_ byte sequences. Parsers should raise an error if a string contains invalid Unicode. For binary data, use [byte strings](#byte-strings).

**Raw strings** start with the lowercase letter R, immediately followed by zero or more hash symbols `#`, immediately followed by a quotation mark `r"`. They end with a quotation mark `"`, followed by the same number of starting hash symbols `#`. They allow newlines and have no escaping whatsoever.

```duper
{
  winpath: r"C:\Users\nodejs\templates",
  regex: r"<\i\c*\s*>",
  quoted: r#"Hello, "world"!"#,
  lines: r"
The first newline is not trimmed.
  All whitespace is
    preserved.",
}
```

Control characters, including tabs, are not permitted in a literal string.

## Byte strings

Byte strings are similar to strings, but represent binary data. Like strings, they come in quoted or raw varaints.

**Quoted byte strings** start with the lowercase letter B immediately followed by a quotation mark `b"`, and end with a quotation mark `"`. The escape sequences are the same as quoted strings, although they are not required to form valid UTF-8 codepoints.

```duper
{
  png_signature: b"\x89PNG\r\n\x1a\n",
  ascii: b"Hello, World!",
  ansi_reset: b"\x1b[0m",
}
```

**Raw byte strings** are similar to raw strings, using the `br` (all lowercase) prefix instead.

```duper
{
  path: br"C:\Windows\System32",
  special_characters: br#"/\*"#,
  binary_data: br##"Raw bytes with "# symbol"##,
}
```

## Integers

Integers are whole numbers. Positive numbers may be prefixed with a plus sign. Negative numbers are prefixed with a minus sign.

```duper
{
  int1: +99,
  int2: 42,
  int3: 0,
  int4: -17,
}
```

For large numbers, you may use underscores between digits to enhance readability. Each underscore must be surrounded by at least one digit on each side.

```duper
{
  int5: 1_000,
  int6: 5_349_221,
  int7: 53_49_221,
  int8: 1_2_3_4_5,
  // INVALID: 1__2,
  // INVALID: _12,
  // INVALID: 12_,
}
```

Leading zeros are not allowed. Integer values `-0` and `+0` are valid and identical to an unprefixed zero.

Non-negative integer values may also be expressed in hexadecimal, octal, or binary. In these formats, leading `+` is not allowed and leading zeros are allowed (after the prefix). Hex values are case-insensitive. Underscores are allowed between digits (but not between the prefix and the value).

```duper
{
  // Hexadecimal with prefix `0x`
  hex1: 0xDEADBEEF,
  hex2: 0xdeadbeef,
  hex3: 0xdead_beef,

  // Octal with prefix `0o`
  oct1: 0o01234567,
  oct2: 0o755,

  // Binary with prefix `0b`
  bin1: 0b11010110,
}
```

Implementations are free to support any integer size. It's recommended that at least 64-bit signed integers (i.e. long integers, from −2^63 to 2^63−1) are accepted and handled losslessly. If an integer cannot be represented in the chosen integer size, it's recommended that implementations convert it losslessly into a float or a string, using an appropriate [identifier](#identifiers) for its original type in both cases.

## Floats

A float consists of an integer part (which follows the same rules as decimal integer values) followed by a fractional part and/or an exponent part. If both a fractional part and exponent part are present, the fractional part must precede the exponent part.

```duper
{
  // Fractional
  flt1: +1.0,
  flt2: 3.1415,
  flt3: -0.01,

  // Exponent
  flt4: 5e+22,
  flt5: 1e06,
  flt6: -2E-2,

  // Both
  flt7: 6.626e-34,
}
```

A fractional part is a decimal point followed by one or more digits.

An exponent part is an E (upper or lower case) followed by an integer part (which follows the same rules as decimal integer values but may include leading zeros).

The decimal point, if used, must be surrounded by at least one digit on each side.

```duper
{
  invalid_float_1: .7,  // INVALID
  invalid_float_2: 7.,  // INVALID
  invalid_float_3: 3.e+20,  // INVALID

}
```

Similar to integers, you may use underscores to enhance readability. Each underscore must be surrounded by digits.

```duper
{
  flt8: 224_617.445_991_228,
}
```

Float values `-0.0` and `+0.0` are valid and should map according to IEEE 754.

Implementations are free to support any precision level. It's recommended that at least IEEE 754 64-bit floating point values (i.e. doubles) are supported.

## Booleans

Booleans are one of `true` or `false`.

```duper
{
  bool1: true,
  bool2: false,
}
```

## Null

Null is always `null`.

```duper
{
  nullValue: null,
}
```

## Arrays

Arrays are ordered values surrounded by square brackets `[` and `]`. Whitespace is ignored. Elements are separated by commas. Arrays can contain values of the same data types as allowed in key/value pairs. Values of different types may be mixed.

```duper
{
  empty: [],
  integers: [1, 2, 3],
  colors: ["red", "yellow", "green"],
  nested_arrays_of_ints: [[1, 2], [3, 4, 5]],
  nested_mixed_array: [[1, 2], ["a", "b", "c"]],
  string_array: ["all string", r"are the", r#"same type"#],

  // Mixed-type arrays are allowed
  numbers: [0.1, 0.2, 0.5, 1, 2, 5],
  contributors: [
    "Foo Bar <foo@example.com>",
    {name: "Baz Qux", email: "bazqux@example.com", url: "https://example.com/bazqux"}
  ],
}
```

Arrays can span multiple lines. A terminating comma (also called a trailing comma) is permitted after the last value of the array. Any number of newlines and comments may precede values, commas, and the closing bracket. Indentation between array values and commas is treated as whitespace and ignored.

```duper
[
  1,
  2,
]
```

## Tuples

Tuples are similar to arrays, although parsers may choose to handle them differently. They are surrounded by parenthesis `(` and `)`.

```duper
{
  empty: (),
  another_empty: (,),
  single: (1),
  another_single: (1,),
  tuple_of_arrays: ([1, 2], [3, 4, 5]),
  array_of_tuples: [(1, 2), ("a", "b", "c")],
}
```

Any parenthesized expression must be interpreted as a tuple by parsers.

## Identifiers

Identifiers are optional type-like annotations that wrap any kind of value, providing semantic meaning or hinting at special handling during parsing/validation. Identified values are composed of the identifier name, followed by the value wrapped in parenthesis `(` and `)`.

The first character must be an ASCII uppercase letter, followed by ASCII letters, ASCII digits, underscores `_`, and hyphens `-`. Sequences of underscores and hyphens are not allowed in the identifier, and identifiers may not start or end with either of them.

```duper
{
  user_id: Uuid("550e8400-e29b-41d4-a716-446655440000"),
  created: DateTime("2024-01-15T10:30:00Z"),
  birthday: ISO-8601("1990-05-20"),
  price: Decimal("19.99"),
  weight: Kilograms(2.5),
  color: RGB((255, 0, 128)),  // Two sets of parenthesis for tuples
  address: IPV4("192.168.1.1"),
  nested: Metadata({
    version: Version("1.2.3"),
    hash: SHA256(b"\xde\xad\xbe\xef"),
  }),
}
```

The root value may also contain an identifier.

```duper
Items([
  "item1",
  "item2",
])
```

Values may not contain more than one identifier.

```duper
{
  many: IpAddress(Ipv4Address("192.168.0.1"))  // INVALID
}
```

Parsers should preserve identifier information. Deserializers may ignore identifiers, or use them for validation. Serializers may choose to output or omit identifiers by the user's request.

Implementations are free to define their own identifiers with specific semantics.

## Filename Extension

Duper files should use the extension `.duper`.

## MIME Type

When transferring Duper files over the internet, the appropriate MIME type is `application/duper`.
