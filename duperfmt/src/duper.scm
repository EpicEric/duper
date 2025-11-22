; Sometimes we want to indicate that certain parts of our source text should
; not be formatted, but taken as is. We use the leaf capture name to inform the
; tool of this.
[
  (block_comment)
  (line_comment)
  (quoted_content)
  (raw_content)
] @leaf

; Allow lines before comments
[
  (block_comment)
  (line_comment)
] @allow_blank_line_before

; Input softlines before and after all comments. This means that the input
; decides if a comment should have line breaks before or after. A line comment
; always ends with a line break.
[
  (block_comment)
  (line_comment)
] @prepend_input_softline @append_input_softline
(line_comment) @append_hardline
(block_comment) @multi_line_indent_all

; Add space after colons
(object_entry
  ":" @prepend_antispace @append_space
)

; Indent objects
(object
  "{" @append_empty_softline @append_indent_start
  "}" @prepend_indent_end @prepend_empty_softline
)

; Remove last comma if single line object
(object
  "," @delete
  .
  "}"
)

; Add last comma if multi-line object
(object
  (object_entry) @append_delimiter
  .
  "}"
  (#delimiter! ",")
  (#multi_line_only!)
)

; Add a newline between object entries
(object
  ("," @prepend_antispace
  .
  [
    (block_comment)
    (line_comment)
  ]? @do_nothing
  ) @append_spaced_softline
)

; Indent arrays
(array
  "[" @append_empty_softline @append_indent_start
  "]" @prepend_indent_end @prepend_empty_softline
)

; Remove last comma if single line array
(array
  "," @delete
  .
  "]"
)

; Add last comma if multi-line array
(array
  (duper_value) @append_delimiter
  .
  "]"
  (#delimiter! ",")
  (#multi_line_only!)
)

; Add a newline between array values
(array
  "," @prepend_antispace @append_spaced_softline
)

; Indent tuples
(tuple
  "(" @append_empty_softline @append_indent_start
  ")" @prepend_indent_end @prepend_empty_softline
)

; Remove last comma if single line tuple
(tuple
  "," @delete
  .
  ")"
)

; Add last comma if multi-line tuple
(tuple
  (duper_value) @append_delimiter
  .
  ")"
  (#delimiter! ",")
  (#multi_line_only!)
)

; Add a newline between tuple values
(tuple
  "," @prepend_antispace @append_spaced_softline
)
