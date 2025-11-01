---
version: "0.3.1"
---

<p align="center">
    <img src="/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">Duper: The format that's super!</h1>

<p align="center">
    <img :alt="`Specification version ${$frontmatter.version}`" :src="`https://img.shields.io/badge/spec_version-${$frontmatter.version}-3868c7?style=for-the-badge`">
</p>

Duper aims to be a human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.

## Preliminaries

- Duper is case-sensitive and must be a valid UTF-8 encoded Unicode document.
- Whitespace means tab (U+0009), space (U+0020), line feed (U+000A), or carriage return (U+000D).
- Newline means line feed (U+000A) or carriage return (U+000D).
- Files must have only one root value. Parsers must always accept objects, arrays, and tuples as the root value, but implementations may allow other values as the root value.
- JSON values are valid Duper values.

## Comments

Two forward slashes `//` mark the rest of the line, including the newline, as a comment, except when inside of a string.

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

Objects are composed of zero or more key-value pairs.

Keys are on the left of the colon `:`, and values are on the right. Whitespace is ignored around key names, colon, and values. The key, colon, and value may be on the same line or different ones.

```duper
{
  key: "value",
  anotherKey
    :
      42
}
```

There must be a comma `,` between key-value pairs.

```duper
{
  key: "value"  // INVALID: Missing comma
  foo: "bar"
}
```

Additionally, a trailing comma after the last key-value pair is allowed.

```duper
{
  key: "value",
  foo: "bar",  // Comma here is valid but not required
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

**Plain keys** may only contain ASCII letters, ASCII digits, underscores `_`, and hyphens `-`. They must start with an ASCII letter, or an underscore followed by a letter or digit. Sequences of underscores and hyphens are not allowed, and plain keys must not end with them.

```duper
{
  // Allowed
  key: "value",
  plain_key: "value",
  pla1n-k3y: "value",
  _1234: "value",

  // Allowed but discouraged
  Capitalized: "value",

  // Not allowed
  _: "value",               // INVALID
  útf8: "value",            // INVALID
  : "value",                // INVALID
  kebabest--case: "value",  // INVALID
}
```

**Quoted keys** follow the exact same rules as quoted strings.

```duper
{
  "127.0.0.1": "value",
  "character encoding": "value",
  "maçã": "value",
  "_": "value",
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

Indentation around keys is treated as whitespace and ignored.

Defining a key multiple times is invalid. Note that plain keys, quoted keys, and raw keys are equivalent.

```duper
{
  name: "Eric",
  "n\x61me": "Erik",  // INVALID
  r"name": "Erick",   // INVALID
}
```

## Strings

A string may be either quoted or raw.

**Quoted strings** are surrounded by quotation marks `"`. Any Unicode character may be used, except those that must be escaped: quotation mark `"`, backslash `\`, and the control characters including tab (U+0000 to U+0009, U+000A to U+001F, U+007F).

```duper
{
  str1: "I'm a string.",
  str2: "\"You can quote me\"",
  str3: "Name\tJos\xE9\nLocation\tBR.",
  str4: "  padded  ",
  str5: "ඞ",  // U+0D9E  SINHALA LETTER KANTAJA NAASIKYAYA
}
```

For convenience, some characters have a compact escape sequence:

```duper
[
  r#"| Sequence | Character       | Value  |"#,
  r#"| -------- | --------------- | ------ |"#,
  r#"| \0       | null            | U+0000 |"#,
  r#"| \b       | backspace       | U+0008 |"#,
  r#"| \t       | tab             | U+0009 |"#,
  r#"| \n       | line feed       | U+000A |"#,
  r#"| \f       | form feed       | U+000C |"#,
  r#"| \r       | carriage return | U+000D |"#,
  r#"| \"       | quote           | U+0022 |"#,
  r#"| \\       | backslash       | U+005C |"#,
  r#"| \xHH     | arbitrary byte  | U+00HH |"#,
  r#"| \uHHHH   | unicode         | U+HHHH |"#,
]
```

Any Unicode character may be escaped with `\uHHHH` or a sequence of one or more `\xHH`, where `H` is a hexadecimal digit. The escape codes must be valid Unicode [scalar values](https://unicode.org/glossary/#unicode_scalar_value).

Keep in mind that Duper strings are sequences of Unicode characters, _not_ byte sequences. Parsers should raise an error if a string decodes into invalid Unicode. For binary data, use [byte strings](#byte-strings).

**Raw strings** start with the lowercase letter R, immediately followed by zero or more hash symbols `#`, immediately followed by a quotation mark `"`. They end with a quotation mark, followed by the same number of starting hash symbols. (for example: `r"..."`, `r#"..."#`, `r##"..."##`, and so on.). They allow newlines and have no escaping whatsoever.

```duper
{
  winpath: r"C:\Users\nodejs\templates",
  regex: r"<\i\c*\s*>",
  quoted: r#"Hello, "world"!"#,
  excessive_hashtags: r####"Just to be safe..."####,
  lines: r"
The first newline is not trimmed.
  All whitespace is
    preserved in here.   ",
}
```

The hashtags are required to disambiguate quotes (`"`, or `"#`, or `"##`, etc.) which are part of the value from the raw string terminator.

```duper
{
  inner_quotes: r"Well, "that" just happened.",      // INVALID
  too_few_ending_hashes: r#"",                       // INVALID
  too_many_ending_hashes: r#""##,                    // INVALID
  not_enough_hashes: r#"will "# close the string"#,  // INVALID
}
```

Control characters, including tab, are not permitted in a raw string.

## Byte strings

Byte strings are similar to strings, but represent binary data. Like strings, they come in quoted or raw variants.

**Quoted byte strings** start with the lowercase letter B immediately followed by a quotation mark `b"`, and end with a quotation mark `"`. The escape sequences are the same as in quoted strings, although they are not required to form valid UTF-8 codepoints.

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
  shrug: br#" "Whatever." ¯\_(ツ)_/¯ "#,
  rust_expression: br##"{ let str = r#"meta string"#; }"##,
}
```

## Integers

Integers are whole numbers. Positive numbers may be prefixed with a plus sign `+`. Negative numbers are prefixed with a minus sign `-`.

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
  // Allowed
  int5: 1_000,
  int6: 5_349_221,
  int7: 53_49_221,
  int8: 1_2_3_4_5,

  // Not allowed
  wrong1: 1__2,  // INVALID
  wrong2: _12,   // INVALID
  wrong3: 12_,   // INVALID
}
```

Leading zeros are not allowed. Integer values `-0` and `+0` are valid and identical to an unprefixed zero.

Non-negative integer values may also be expressed in hexadecimal (`0x...`), octal (`0o...`), or binary (`0b...`). In these formats, plus or minus signs `+` or `-` are not allowed, but leading zeros (after the prefix) are allowed. Hexadecimal values are case-insensitive. Underscores are allowed between digits (but not between the prefix and the value).

```duper
{
  // Hexadecimal with prefix `0x`
  hex1: 0xDEADBEEF,
  hex2: 0x2001_0db1,

  // Octal with prefix `0o`
  oct1: 0o755,
  oct2: 0o01_234_567,

  // Binary with prefix `0b`
  bin1: 0b1101,
  bin2: 0b0101_0101,
}
```

Implementations are free to support any integer size. It's recommended that at least 64-bit signed integers (i.e. long integers, from −2^63 to 2^63−1) are accepted and handled losslessly. If an integer cannot be represented in the chosen integer size, implementations may raise an error or convert it losslessly into a float or a string, using an appropriate [identifier](#identifiers) for its original type in both cases.

## Floats

A float consists of an integer part (which follows the same rules as decimal integer values) followed by a fractional part and/or an exponent part. If both a fractional part and exponent part are present, the fractional part must precede the exponent part.

```duper
{
  // Fractional
  float1: +1.0,
  float2: 3.1415,
  float3: -0.01,

  // Exponent
  float4: 5e+22,
  float5: 1e06,
  float6: -2E-2,

  // Both
  float7: 6.626e-34,
}
```

A fractional part is a decimal point followed by one or more digits.

An exponent part is an `e` (upper or lower case) followed by an integer part (which follows the same rules as decimal integer values, but may include leading zeros).

The decimal point, if used, must be surrounded by at least one digit on each side.

```duper
{
  invalid_float_1: .7,      // INVALID
  invalid_float_2: 7.,      // INVALID
  invalid_float_3: 3.e+20,  // INVALID
}
```

Similar to integers, you may use underscores to enhance readability. Each underscore must be surrounded by digits.

```duper
{
  float8: 224_617.445_991_228,
  float9: 1e2_00,
}
```

Float values `-0.0` and `+0.0` are valid and should map according to IEEE 754. Infinity and NaN are not allowed.

Implementations are free to support any precision level. It's recommended that at least IEEE 754 64-bit floating point values (i.e. doubles) are supported.

## Booleans

Booleans are one of `true` or `false`.

```duper
{
  tis: true,
  nah: false,
}
```

## Null

Null is always `null`.

```duper
{
  nuclear_launch_code: null,
}
```

## Arrays

Arrays are ordered values surrounded by square brackets `[` and `]`. Whitespace is ignored. Elements are separated by commas. Arrays can contain values of the same data types as allowed in key-value pairs. Values of different types may be mixed.

```duper
{
  empty_array: [],
  another_empty_array: [,],
  integers: [1, 2, 3],
  colors: ["red", "yellow", r"green"],
  unflattened_ints: [[1, 2], [3, 4, 5]],

  // Mixed-type arrays are allowed
  numbers: [0.1, 0.2, 0.5, 1, 2, 5],
  nested_mixed_array: [[1, "a"], [2, "b", {}]],
  contributors: [
    "Foo Bar <foo@example.com>",
    {
      name: "Baz Qux",
      email: "bazqux@example.com",
      url: "https://example.com/bazqux",
    },
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
  empty_tuple: (),
  another_empty_tuple: (,),
  single_element: (1),
  another_single_element: (1,),
  tuple_of_arrays: ([true, 1.0], ["x", "y", "z"]),
  array_of_tuples: [(1, null), (3, 4.0, 5)],
  multiline_tuple: (
    "Vec",
    "Cow",
    "Arc",
  ),
}
```

Any parenthesized expression must be interpreted as a tuple by parsers.

## Identifiers

Identifiers are optional type-like annotations that wrap any kind of value, providing semantic meaning or hinting at special handling during parsing/validation. Identified values are composed of the identifier name, followed by the value wrapped in parenthesis `(` and `)`.

The first character must be an ASCII uppercase letter, followed by zero or more ASCII letters, ASCII digits, underscores `_`, and hyphens `-`. Sequences of underscores and hyphens are not allowed in the identifier, and identifiers may not start or end with either of them.

```duper
{
  user_id: Uuid("550e8400-e29b-41d4-a716-446655440000"),
  created: DateTime("2024-01-15T10:30:00Z"),
  birthday: ISO-8601("2025-10-20"),
  price: Decimal("19.99"),
  weight: Kilograms(2.5),
  color: RGB((255, 0, 128)),  // Two sets of parenthesis for tuples
  address: IPV4("192.168.1.1"),
  nested: Metadata({
    version: Version("1.2.3"),
    hash: SHA_256(b"\xde\xad\xbe\xef"),
  }),
  minimal: A(null),
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
  too_many: IpAddress(Ipv4Address("192.168.0.1"))  // INVALID
}
```

Parsers should preserve identifier information on a best-effort basis. Deserializers may ignore identifiers, or use them for validation. Serializers may choose to output or omit identifiers by the user's request.

Implementations are free to define their own identifiers with specific semantics. For example, in strongly-typed or OOP languages, serializers may use them as annotations for the underlying types.

## Filename extension

Duper files should use the extension `.duper`.

## MIME type

When transferring Duper files over the internet, the appropriate MIME type is `application/duper`.

Webservers should also accept the `application/x-duper` MIME type.
