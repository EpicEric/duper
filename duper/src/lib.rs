#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//! # Duper
//!
//! The format that's super!
//!
//! Duper aims to be a human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.
//!
//! ```duper
//! DatabaseConfig({
//!   host: IPV4("127.0.0.1"),
//!   port: Port(5432),
//!   username: "admin",
//!   password: SecureString("encrypted_data"),
//!   pool_size: 10,
//!   timeout: Duration("30s"),
//!   ssl: SSLConfig({
//!     enabled: true,
//!     cert: Path("/etc/ssl/cert.pem"),
//!     verify: true,
//!   }),
//!   created_at: Timestamp(1704067200),
//! })
//! ```
//!
//! ## Feature flags
//!
//! - `ansi`: Enables the [`Ansi`] module for printing Duper values to a console.
//! - `serde`: Enables `serde` serialization/deserialization for [`DuperValue`].
//!
//! ## Other crates
//!
//! - [`serde_duper`](https://docs.rs/serde_duper): Provides full
//!   serialization/deserialization support between Duper and native data types.
//! - [`axum_duper`](https://docs.rs/axum_duper): Provides an extractor/response
//!   for use with [`axum`](https://docs.rs/axum).
//!

pub mod ast;
mod builder;
mod escape;
mod format;
mod parser;
pub mod visitor;

pub use ast::{
    DuperArray, DuperBytes, DuperIdentifier, DuperIdentifierTryFromError, DuperInner, DuperKey,
    DuperObject, DuperObjectTryFromError, DuperString, DuperTuple, DuperValue,
};
pub use parser::{DuperParser, Rule as DuperRule};
#[cfg(feature = "ansi")]
pub use visitor::ansi::Ansi;
pub use visitor::{pretty_printer::PrettyPrinter, serializer::Serializer};
