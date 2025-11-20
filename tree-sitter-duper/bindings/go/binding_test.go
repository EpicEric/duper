package tree_sitter_duper_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_duper "github.com/epiceric/duper/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_duper.Language())
	if language == nil {
		t.Errorf("Error loading Duper grammar")
	}
}
