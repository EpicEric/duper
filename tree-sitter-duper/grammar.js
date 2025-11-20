/**
 * @file The format that's super!
 * @author Eric Rodrigues Pires <eric@eric.dev.br>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "duper",

  extras: ($) => [
    new RustRegex(`[ \\t\\r\\n]`),
    $.line_comment,
    $.block_comment,
  ],

  externals: ($) => [$.raw_start, $.raw_content, $.raw_end],

  rules: {
    duper_value: ($) => choice($.identified_value, $._value),

    identified_value: ($) => seq($.identifier, "(", $._value, ")"),

    _value: ($) =>
      choice(
        $.object,
        $.array,
        $.tuple,
        $.string,
        $.bytes,
        $.temporal,
        $.float,
        $.integer,
        $.boolean,
        $.null,
      ),

    object: ($) =>
      seq(
        "{",
        optional(
          seq($.object_entry, repeat(seq(",", $.object_entry)), optional(",")),
        ),
        "}",
      ),
    array: ($) =>
      seq(
        "[",
        optional(seq($.duper_value, repeat(seq(",", $.duper_value)))),
        optional(","),
        "]",
      ),
    tuple: ($) =>
      seq(
        "(",
        optional(seq($.duper_value, repeat(seq(",", $.duper_value)))),
        optional(","),
        ")",
      ),
    string: ($) => choice($.quoted_string, $.raw_string),
    bytes: ($) => choice($.quoted_bytes, $.raw_bytes, $.base64_bytes),
    temporal: ($) => seq(`'`, $.temporal_content, `'`),
    integer: ($) =>
      choice(
        $.decimal_integer,
        $.hex_integer,
        $.octal_integer,
        $.binary_integer,
      ),
    float: (_) =>
      new RustRegex(
        `[+-]?([0-9]|[1-9](_?[0-9])+)(\\.[0-9](_?[0-9])*([eE][+-]?([0-9]|[1-9](_?[0-9])+))?|[eE][+-]?([0-9]|[1-9](_?[0-9])+))`,
      ),
    boolean: (_) => choice("true", "false"),
    null: (_) => "null",

    identifier: (_) => new RustRegex(`[A-Z]([_-]?[a-zA-Z0-9])*`),

    object_entry: ($) => seq($.object_key, ":", $.duper_value),

    object_key: ($) => choice($.plain_key, $.quoted_string, $.raw_string),
    plain_key: (_) =>
      new RustRegex(`(_[a-zA-Z0-9]|[a-zA-Z])([_-]?[a-zA-Z0-9])*`),

    quoted_string: ($) => seq(`"`, $.quoted_content, `"`),
    raw_string: ($) => seq("r", $.raw_start, $.raw_content, $.raw_end),

    quoted_bytes: ($) => seq(`b"`, $.quoted_content, `"`),
    raw_bytes: ($) => seq("br", $.raw_start, $.raw_content, $.raw_end),
    base64_bytes: ($) => seq(`b64"`, $.base64_content, `"`),

    quoted_content: (_) => new RustRegex(`(\\\\"|[^"])*`),
    base64_content: (_) =>
      new RustRegex(`[a-zA-Z0-9+/ \\t\\r\\n]*[= \\t\\r\\n]*`),
    temporal_content: (_) =>
      new RustRegex(
        `[ \\t\\r\\n]*[^' \\t\\r\\n][^']+[^' \\t\\r\\n][ \\t\\r\\n]*`,
      ),

    decimal_integer: (_) => new RustRegex(`[+-]?([0-9]|[1-9](_?[0-9])+)`),
    hex_integer: (_) => new RustRegex(`0x[0-9a-fA-F](_?[0-9a-fA-F])*`),
    octal_integer: (_) => new RustRegex(`0o[0-7](_?[0-7])*`),
    binary_integer: (_) => new RustRegex(`0b[01](_?[01])*`),

    line_comment: (_) => token(seq("//", new RustRegex(`.*`))),
    block_comment: (_) =>
      token(seq("/*", new RustRegex(`[^*]*\\*+([^/*][^*]*\\*+)*`), "/")),
  },
});
