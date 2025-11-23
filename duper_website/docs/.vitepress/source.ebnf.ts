// https://github.com/mfederczuk/w3c-ebnf-vscode
// Licensed under MPL-2.0 AND Apache-2.0

const ebnfGrammar = {
  $schema:
    "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
  name: "W3C EBNF",
  scopeName: "source.w3c-ebnf",
  aliases: ["ebnf"],
  patterns: [
    { include: "#comments" },
    { include: "#rule" },
    { include: "#non-whitespace-illegal" },
  ],
  repository: {
    comments: {
      patterns: [
        {
          name: "comment.line.double-slash.w3c-ebnf",
          begin: "//",
          end: "$",
        },
        {
          name: "comment.block.w3c-ebnf",
          begin: "/\\*",
          end: "\\*/",
        },
      ],
    },
    grammar: {
      name: "variable.language.w3c-ebnf",
      match: "\\b(grammar|document)\\b",
    },
    rule: {
      name: "meta.rule.w3c-ebnf",
      begin: "^\\s*([a-zA-Z_][a-zA-Z0-9_]*)\\s*(::=)\\s*",
      end: "(;)|(?=^\\s*[a-zA-Z_][a-zA-Z0-9_]*\\s*::=)",
      beginCaptures: {
        "1": {
          patterns: [
            {
              name: "invalid.illegal.w3c-ebnf",
              match: "\\bEOF\\b",
            },
            {
              include: "#grammar",
            },
            {
              name: "variable.other.w3c-ebnf",
              match: ".*",
            },
          ],
        },
        "2": { name: "keyword.operator.new.w3c-ebnf" },
      },
      endCaptures: {
        "1": { name: "punctuation.rule-terminator.w3c-ebnf" },
      },
      patterns: [{ include: "#expression" }],
    },
    expression: {
      patterns: [
        {
          include: "#comments",
        },
        {
          name: "string.quoted.double.w3c-ebnf",
          begin: "[\"']",
          end: "\\0",
        },
        {
          name: "constant.character.escape.w3c-ebnf",
          match: "#x[0-9a-fA-F]+\\b",
        },
        {
          name: "invalid.illegal.w3c-ebnf",
          match: "\\[\\^?\\]",
        },
        {
          name: "keyword.other.w3c-ebnf",
          match: "\\[\\s*([vV][cC]|[wW][fF][cC])\\s*:[^\\]]*\\]",
        },
        {
          name: "meta.character-class.w3c-ebnf",
          match: "(\\[\\^?)(-{2,})(\\])",
          captures: {
            "1": { name: "keyword.other.w3c-ebnf" },
            "2": { name: "invalid.illegal.w3c-ebnf" },
            "3": { name: "keyword.other.w3c-ebnf" },
          },
        },
        {
          name: "meta.character-class.w3c-ebnf",
          match: "(\\[\\^?)(#x[0-9a-fA-F]+[^\\]]*)(\\])",
          captures: {
            "1": {
              name: "keyword.other.w3c-ebnf",
            },
            "2": {
              patterns: [
                {
                  name: "meta.character-class.range.w3c-ebnf",
                  match: "(#x[0-9a-fA-F]+)(-)(#x[0-9a-fA-F]+)",
                  captures: {
                    "1": { name: "constant.character.escape.w3c-ebnf" },
                    "2": { name: "keyword.other.w3c-ebnf" },
                    "3": { name: "constant.character.escape.w3c-ebnf" },
                  },
                },
                {
                  name: "constant.character.escape.w3c-ebnf",
                  match: "#x[0-9a-fA-F]+",
                },
                {
                  name: "invalid.illegal.w3c-ebnf",
                  match: ".",
                },
              ],
            },
            "3": {
              name: "keyword.other.w3c-ebnf",
            },
          },
        },
        {
          name: "meta.character-class.w3c-ebnf",
          match: "(\\[\\^?)([^\\]]+)(\\])",
          captures: {
            "1": {
              name: "keyword.other.w3c-ebnf",
            },
            "2": {
              patterns: [
                {
                  name: "meta.character-class.range.w3c-ebnf",
                  match: "(.)(-)(.)",
                  captures: {
                    "1": { name: "string.unquoted.w3c-ebnf" },
                    "2": { name: "keyword.other.w3c-ebnf" },
                    "3": { name: "string.unquoted.w3c-ebnf" },
                  },
                },
                {
                  name: "invalid.illegal.w3c-ebnf",
                  match: "#x[0-9a-fA-F]+",
                },
                {
                  name: "string.unquoted.w3c-ebnf",
                  match: ".",
                },
              ],
            },
            "3": {
              name: "keyword.other.w3c-ebnf",
            },
          },
        },
        {
          name: "meta.parens.w3c-ebnf",
          begin: "\\(",
          end: "\\)",
          beginCaptures: {
            "0": { name: "punctuation.parens.open.w3c-ebnf" },
          },
          endCaptures: {
            "0": { name: "punctuation.parens.closed.w3c-ebnf" },
          },
          patterns: [{ include: "#expression" }],
        },
        {
          name: "constant.language.w3e-ebnf",
          match: "\\bEOF\\b",
        },
        {
          include: "#grammar",
        },
        {
          name: "variable.other.w3c-ebnf",
          match: "\\b[a-zA-Z_][a-zA-Z0-9_]*\\b",
        },
        {
          name: "keyword.operator.w3c-ebnf",
          match: "[?|+*-]",
        },
        {
          include: "#non-whitespace-illegal",
        },
      ],
    },
    "non-whitespace-illegal": {
      patterns: [
        {
          name: "invalid.illegal.w3c-ebnf",
          match: "\\S",
        },
      ],
    },
  },
};

export default ebnfGrammar;
