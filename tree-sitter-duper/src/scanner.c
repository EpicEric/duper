#include "tree_sitter/parser.h"
#include "tree_sitter/alloc.h"

enum TokenType {
  RAW_START,
  RAW_CONTENT,
  RAW_END
};

typedef struct {
    uint8_t opening_hash_count;
} Scanner;

void *tree_sitter_duper_external_scanner_create() { return ts_calloc(1, sizeof(Scanner)); }

void tree_sitter_duper_external_scanner_destroy(void *payload) { ts_free((Scanner *)payload); }

unsigned tree_sitter_duper_external_scanner_serialize(void *payload, char *buffer) {
    Scanner *scanner = (Scanner *)payload;
    buffer[0] = (char)scanner->opening_hash_count;
    return 1;
}

void tree_sitter_duper_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
    Scanner *scanner = (Scanner *)payload;
    scanner->opening_hash_count = 0;
    if (length == 1) {
        Scanner *scanner = (Scanner *)payload;
        scanner->opening_hash_count = buffer[0];
    }
}

bool tree_sitter_duper_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
  Scanner *scanner = (Scanner *)payload;

  if (valid_symbols[RAW_START]) {
    uint8_t opening_hash_count = 0;
    while (lexer->lookahead == '#') {
        lexer->advance(lexer, false);
        opening_hash_count++;
    }
    if (lexer->lookahead != '"') {
        return false;
    }
    lexer->advance(lexer, false);
    scanner->opening_hash_count = opening_hash_count;
    lexer->result_symbol = RAW_START;
    return true;
  }

  if (valid_symbols[RAW_CONTENT]) {
    for (;;) {
      if (lexer->eof(lexer)) {
        return false;
      }
      if (lexer->lookahead == '"') {
        lexer->mark_end(lexer);
        lexer->advance(lexer, false);
        unsigned hash_count = 0;
        while (lexer->lookahead == '#' && hash_count < scanner->opening_hash_count) {
            lexer->advance(lexer, false);
          hash_count++;
        }
        if (hash_count == scanner->opening_hash_count) {
          lexer->result_symbol = RAW_CONTENT;
          return true;
        }
      } else {
          lexer->advance(lexer, false);
      }
    }
  }

  if (valid_symbols[RAW_END] && lexer->lookahead == '"') {
      lexer->advance(lexer, false);
    for (unsigned i = 0; i < scanner->opening_hash_count; i++) {
      if (lexer->lookahead != '#') {
        return false;
      }
      lexer->advance(lexer, false);
    }
    lexer->result_symbol = RAW_END;
    return true;
  }

  return false;
}
