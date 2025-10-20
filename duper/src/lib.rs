//! # Duper
//!
//! The format that's super.
//!
//! Duper aims to be a human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.
//!
//! ```duper
//! Product({
//!   product_id: Uuid("1dd7b7aa-515e-405f-85a9-8ac812242609"),
//!   name: "Wireless Bluetooth Headphones",
//!   brand: "AudioTech",
//!   price: Decimal("129.99"),
//!   dimensions: (18.5, 15.2, 7.8),  // In centimeters
//!   weight: Kilograms(0.285),
//!   in_stock: true,
//!   specifications: {
//!     battery_life: Duration("30h"),
//!     noise_cancellation: true,
//!     connectivity: ["Bluetooth 5.0", "3.5mm Jack"],
//!   },
//!   image_thumbnail: Png(b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x64"),
//!   tags: ["electronics", "audio", "wireless"],
//!   release_date: Date("2023-11-15"),
//!   /* Warranty is optional */
//!   warranty_period: null,
//!   customer_ratings: {
//!     latest_review: r#"Absolutely ""astounding""!! 😎"#,
//!     average: 4.5,
//!     count: 127,
//!   },
//!   created_at: DateTime("2023-11-17T21:50:43+00:00"),
//! })
//! ```
//!
//! ## Feature flags
//!
//! - `ansi`: Enables the [`Ansi`] module for printing Duper values to a console.
//! - `serde`: Enables `serde` serialization/deserialization for [`DuperValue`].
//!

pub mod ast;
mod builder;
mod escape;
mod format;
mod parser;
#[cfg(feature = "serde")]
mod serde;
pub mod visitor;

pub use ast::{
    DuperArray, DuperBytes, DuperIdentifier, DuperIdentifierTryFromError, DuperInner, DuperKey,
    DuperObject, DuperObjectTryFromError, DuperString, DuperTuple, DuperValue,
};
pub use parser::{DuperParser, Rule as DuperRule};
#[cfg(feature = "ansi")]
pub use visitor::ansi::Ansi;
pub use visitor::{pretty_printer::PrettyPrinter, serializer::Serializer};
