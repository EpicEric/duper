#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//! This crate provides Duper language support for the [tree-sitter] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add this language to a
//! tree-sitter [`Parser`], and then use the parser to parse some code:
//!
//! ```
//! let code = r##"
//!   UserProfile({
//!     id: Uuid("f111c275-bfce-f394-8e5b-19067ce39b53"),
//!     username: "",
//!     email: EmailAddress("eric@duper.dev.br"),
//!     settings: {
//!       "dark mode": false,
//!       language: Locale("pt-BR"),
//!       email: null,
//!     },
//!     score: 120.25,
//!     // Support for bytes, woohoo!
//!     avatar: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ="),
//!     bio: r#"Hello! I'm a super "duper" user!"#,
//!     last_logins: [
//!       (IPv4Address("192.168.1.100"), Instant('2024-02-29T14:30:00+00:00')),
//!     ],
//!   })
//! "##;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_duper::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading Duper parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [`Parser`]: https://docs.rs/tree-sitter/0.25.10/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_duper() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_duper) };

/// The content of the [`node-types.json`] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

// NOTE: uncomment these to include any queries that this grammar contains:

/// The syntax highlighting query for this language.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

// pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
// pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
// pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Duper parser");
    }
}
