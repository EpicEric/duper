//! # Duper
//!
//! The format that's super.
//!
//! Duper aims to be a human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.
//!
//! ## Feature flags
//!
//! - `ansi`: Enables the [`Ansi`] module for printing Duper values to a console.
//! - `serde`: Enables [`serde`] serialization/deserialization for [`DuperValue`].
//!

mod ast;
mod builder;
mod escape;
mod format;
mod parser;
#[cfg(feature = "serde")]
mod serde;
pub mod types;
pub mod visitor;

pub use ast::{
    DuperArray, DuperBytes, DuperIdentifier, DuperIdentifierTryFromError, DuperInner, DuperKey,
    DuperObject, DuperObjectTryFromError, DuperString, DuperTuple, DuperValue,
};
pub use parser::{DuperParser, Rule as DuperRule};
#[cfg(feature = "ansi")]
pub use visitor::ansi::Ansi;
pub use visitor::{pretty_printer::PrettyPrinter, serializer::Serializer};
