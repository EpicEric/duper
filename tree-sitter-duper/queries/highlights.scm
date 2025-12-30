(line_comment) @comment
(block_comment) @comment
[
  "["
  "]"
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket
(object_key) @property
(boolean) @keyword
(null) @keyword
(identifier) @type
(float) @number
(integer) @number
(quoted_escape) @string.escape
(string) @string
(bytes) @string
(temporal) @string.special
