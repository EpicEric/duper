#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//! # Serde Duper
//!
//! Duper is a format which aims to be a human-friendly extension of JSON, with
//! quality-of-life improvements, extra types, and semantic identifiers.
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
//!   image_thumbnail: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ="),
//!   tags: ["electronics", "audio", "wireless"],
//!   release_date: Date("2023-11-15"),
//!   /* Warranty is optional */
//!   warranty_period: null,
//!   customer_ratings: {
//!     latest_review: r#"Absolutely ""astounding""!! 😎"#,
//!     average: 4.5,
//!     count: 127,
//!   },
//!   created_at: Instant('2023-11-17T21:50:43+00:00'),
//! })
//! ```
//!
//! This crate allows you to convert between Duper's text representation and
//! Rust's native data types, thanks to the `serde` framework.
//!
//! Serde provides a powerful way of mapping Duper data to and from Rust data
//! structures largely automatically.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_duper::Result;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u8,
//!     phones: Vec<String>,
//! }
//!
//! fn deserialize() -> Result<Person> {
//!     // Some Duper input data as a &str. Maybe this comes from the user.
//!     let data = r#"
//!         Person({
//!             name: "John Doe",
//!             age: 43,
//!             phones: [
//!                 ResidentialPhone("+44 1234567"),
//!                 CellPhone("+44 2345678"),
//!             ],
//!         })"#;
//!
//!     // Parse the string of data into a Person object.
//!     let p: Person = serde_duper::from_string(data)?;
//!
//!     // Do things just like with any other Rust data structure.
//!     println!("Please call {} at the number {}", p.name, p.phones[0]);
//!
//!     Ok(p)
//! }
//!
//! fn serialize(p: Person) -> Result<()> {
//!     // Serialize the person back into a Duper string.
//!     let d = serde_duper::to_string(&p)?;
//!
//!     // Print, write to a file, or send to an HTTP server.
//!     println!("{}", d);
//!
//!     Ok(())
//! }
//!
//! fn main() {
//!     let p = deserialize().unwrap();
//!     serialize(p).unwrap();
//! }
//! ```
//!
//! Any type that implements Serde's `Deserialize` trait can be deserialized
//! into a struct like this. This includes built-in Rust standard library type
//! like `Vec<T>` and `HashMap<K, V>`, as well as any structs or enums annotated
//! with `#[derive(Deserialize)]` in the Rust ecosystem.
//!
//! Conversely, any type that implements Serde's `Serialize` trait can be
//! serialized into a string like this. This includes built-in Rust standard
//! library types like `Vec<T>` and `HashMap<K, V>`, as well as any structs or
//! enums annotated with `#[derive(Serialize)]` in the Rust ecosystem.
//!
//! # Handling bytes
//!
//! This crate re-exports [`serde_bytes`] wrapper types via the [`bytes`]
//! module. Prefer using those over the following types if you'd like better
//! byte support:
//!
//! - [`serde_bytes::ByteBuf`] over [`Vec<u8>`]
//! - [`serde_bytes::ByteArray`] over `[u8; N]`
//! - [`serde_bytes::Bytes`] over `[u8]`
//!
//! # Support for Temporal values
//!
//! For generic Temporal value support, use [`TemporalString`] as a field in
//! your structs/enums. This will bring support for both serialization with the
//! appropriate identifier, and deserialization with value parsing.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_duper::TemporalString;
//!
//! #[derive(Serialize, Deserialize)]
//! struct SomeData<'a> {
//!     temporal: TemporalString<'a>,
//! }
//! ```
//!
//! For [`chrono`] support, use the types provided by [`types::chrono`] with the
//! `#[serde(with = "...")]` attribute. For example:
//!
//! ```
//! use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};
//! use serde::{Deserialize, Serialize};
//! use serde_duper::types::chrono::{
//!     DuperDateTime, DuperNaiveDateTime, DuperOptionNaiveDate, DuperOptionNaiveTime,
//! };
//!
//! #[derive(Serialize, Deserialize)]
//! struct ChronoStuff {
//!     #[serde(with = "DuperDateTime")]
//!     utc: DateTime<Utc>,
//!     #[serde(with = "DuperDateTime")]
//!     fo: DateTime<FixedOffset>,
//!     #[serde(with = "DuperNaiveDateTime")]
//!     ndt: NaiveDateTime,
//!     #[serde(with = "DuperOptionNaiveDate")]
//!     nd: Option<NaiveDate>,
//!     #[serde(with = "DuperOptionNaiveTime")]
//!     nt: Option<NaiveTime>,
//! }
//! ```
//!
//! # Support for identifiers
//!
//! By default, serialization will attempt to include identifiers for structs
//! and enums, while deserialization will discard them. It's possible to
//! customize the emitted identifiers with the `#[serde(rename = "...")]`
//! attribute.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use uuid::Uuid;
//!
//! #[derive(Serialize, Deserialize)]
//! #[serde(rename = "Status")]
//! enum UserStatus {
//!     Disabled,
//!     PendingApproval,
//!     Enabled,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct User {
//!     id: Uuid,
//!     status: UserStatus,
//!     last_known_ips: Vec<String>,
//! }
//!
//! let u = User {
//!     id: "314dfe6f-7a76-4c43-80b9-3b0ceb0960c0".parse().unwrap(),
//!     status: UserStatus::Enabled,
//!     last_known_ips: vec!["2a02:ec80:700:ed1a::1".to_string()],
//! };
//! let d = serde_duper::to_string(&u).unwrap();
//! println!("{}", d);
//! // This should print:
//! //     User({
//! //       id: "314dfe6f-7a76-4c43-80b9-3b0ceb0960c0",
//! //       status: Status("Enabled"),
//! //       last_known_ips: ["2a02:ec80:700:ed1a::1"],
//! //     })
//! ```
//!
//! It's also possible to remove an identifier with `#[serde(rename = "")]`.
//!
//! In order to generate identifiers for fields, there are currently three
//! possibilities:
//!
//! ## 1. Wrapping your field in a newtype
//!
//! ```
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! #[serde(rename = "Status")]
//! enum UserStatus {
//!     Disabled,
//!     PendingApproval,
//!     Enabled,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct Uuid(uuid::Uuid);
//!
//! #[derive(Serialize, Deserialize)]
//! struct User {
//!     id: Uuid,
//!     status: UserStatus,
//!     last_known_ips: Vec<String>,
//! }
//!
//! let u = User {
//!     id: Uuid("314dfe6f-7a76-4c43-80b9-3b0ceb0960c0".parse().unwrap()),
//!     status: UserStatus::Enabled,
//!     last_known_ips: vec!["2a02:ec80:700:ed1a::1".to_string()],
//! };
//! let d = serde_duper::to_string(&u).unwrap();
//! println!("{}", d);
//! // This should print:
//! //     User({
//! //       id: Uuid("314dfe6f-7a76-4c43-80b9-3b0ceb0960c0"),
//! //       status: Status("Enabled"),
//! //       last_known_ips: ["2a02:ec80:700:ed1a::1"],
//! //     })
//! ```
//!
//! This offers maximum customizability, but requires at best an extra layer of
//! indirection in your code, and at worst the implementation of custom
//! (de)serializers.
//!
//! ## 2. Using a remote (de)serializer
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_duper::types::DuperUuid;
//! use uuid::Uuid;
//!
//! #[derive(Serialize, Deserialize)]
//! #[serde(rename = "Status")]
//! enum UserStatus {
//!     Disabled,
//!     PendingApproval,
//!     Enabled,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct User {
//!     #[serde(with = "DuperUuid")]
//!     id: Uuid,
//!     status: UserStatus,
//!     last_known_ips: Vec<String>,
//! }
//!
//! let u = User {
//!     id: "314dfe6f-7a76-4c43-80b9-3b0ceb0960c0".parse().unwrap(),
//!     status: UserStatus::Enabled,
//!     last_known_ips: vec!["2a02:ec80:700:ed1a::1".to_string()],
//! };
//! let d = serde_duper::to_string(&u).unwrap();
//! println!("{}", d);
//! // This should print:
//! //     User({
//! //       id: Uuid("314dfe6f-7a76-4c43-80b9-3b0ceb0960c0"),
//! //       status: Status("Enabled"),
//! //       last_known_ips: ["2a02:ec80:700:ed1a::1"],
//! //     })
//! ```
//!
//! The [`types`] module provides a simple and quick plug-and-play
//! way of annotating types from [`std`] (as well as a few popular third-party
//! crates behind feature flags) with Duper identifiers. It works by providing
//! remote modules that will handle (de)serialization. This is less flexible,
//! but allows you to use the original types directly.
//!
//! Currently, modules are provided for `T` and `Option<T>`.
//!
//! ## 3. Using the proc-macro
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_duper::duper;
//! use uuid::Uuid;
//!
//! #[derive(Serialize, Deserialize)]
//! #[serde(rename = "Status")]
//! enum UserStatus {
//!     Disabled,
//!     PendingApproval,
//!     Enabled,
//! }
//!
//! duper! {
//!     #[derive(Serialize, Deserialize)]
//!     struct User {
//!         #[duper(MyUuid)]
//!         id: Uuid,
//!         status: UserStatus,
//!         #[duper(IpList)]
//!         last_known_ips: Vec<String>,
//!     }
//! }
//!
//! # fn main() {
//! let u = User {
//!     id: "314dfe6f-7a76-4c43-80b9-3b0ceb0960c0".parse().unwrap(),
//!     status: UserStatus::Enabled,
//!     last_known_ips: vec!["2a02:ec80:700:ed1a::1".to_string()],
//! };
//! let d = serde_duper::to_string(&u).unwrap();
//! println!("{}", d);
//! // This should print:
//! //     User({
//! //       id: MyUuid("314dfe6f-7a76-4c43-80b9-3b0ceb0960c0"),
//! //       status: Status("Enabled"),
//! //       last_known_ips: IpList(["2a02:ec80:700:ed1a::1"]),
//! //     })
//! # }
//! ```
//!
//! This will automatically generate the modules for any type that implements
//! [`serde_core::Serialize`] and/or [`serde_core::Deserialize`], not being
//! restricted only to those with a remote (de)serializer module.
//!
//! This requires the `macros` feature flag.
//!

pub mod bytes;
pub mod types;

pub use duper::serde::de::Deserializer;
pub use duper::serde::error::{DuperSerdeError, DuperSerdeErrorKind, ErrorImpl, Result};
pub use duper::serde::ser::{
    Serializer, to_duper, to_string, to_string_minified, to_string_pretty,
};
pub use duper::serde::temporal::TemporalString;
pub use duper::{DuperIdentifier, DuperKey, DuperObject, DuperTemporal, DuperValue};

#[cfg(feature = "macros")]
pub use serde_duper_macros::duper;

/// Interpret a [`DuperValue`] as an instance of type `T`.
///
/// # Example
///
/// ```
/// use std::borrow::Cow;
/// use serde::Deserialize;
/// use serde_duper::{
///     DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject,
///     DuperString, DuperValue,
/// };
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: Vec<u8>,
///     location: String,
/// }
///
/// // The type of `d` is `serde_duper::DuperValue`
/// let d = DuperValue {
///     identifier: Some(DuperIdentifier::try_from(Cow::Borrowed("User")).unwrap()),
///     inner: DuperInner::Object(DuperObject::try_from(vec![
///         (
///             DuperKey::from(Cow::Borrowed("fingerprint")),
///             DuperValue {
///                 identifier: None,
///                 inner: DuperInner::Bytes(DuperBytes::from(Cow::Borrowed(
///                     &b"\xF9\xBA\x14\x3B\x95\xFF\x6D\x82"[..],
///                 ))),
///             }
///         ),
///         (
///             DuperKey::from(Cow::Borrowed("location")),
///             DuperValue {
///                 identifier: Some(
///                     DuperIdentifier::try_from(Cow::Borrowed("City")).unwrap(),
///                 ),
///                 inner: DuperInner::String(DuperString::from(
///                     Cow::Borrowed("Menlo Park, CA"),
///                 )),
///             }
///         ),
///     ]).unwrap()),
/// };
///
/// let u: User = serde_duper::from_value(d).unwrap();
/// println!("{:#?}", u);
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a Duper object. It can also fail if the
/// structure is correct but `T`'s implementation of [`serde_core::Deserialize`] decides that
/// something is wrong with the data, for example required struct fields are
/// missing from the Duper object or some number is too big to fit in the
/// expected primitive type.
#[inline]
pub fn from_value<'a, T>(value: DuperValue<'a>) -> Result<T>
where
    T: serde_core::Deserialize<'a>,
{
    duper::serde::de::from_value(value)
}

/// Deserialize an instance of type `T` from a str slice of Duper text.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: Vec<u8>,
///     location: String,
/// }
///
///
/// // The type of `j` is `&str`
/// let j = r#"
///     User({
///         fingerprint: b64"+boUO5X/bYI=",
///         location: City("Menlo Park, CA"),
///     })"#;
///
/// let u: User = serde_duper::from_string(j).unwrap();
/// println!("{:#?}", u);
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a Duper object. It can also fail if the
/// structure is correct but `T`'s implementation of [`serde_core::Deserialize`] decides that
/// something is wrong with the data, for example required struct fields are
/// missing from the Duper object or some number is too big to fit in the
/// expected primitive type.
#[inline]
pub fn from_string<'a, T>(input: &'a str) -> Result<T>
where
    T: serde_core::Deserialize<'a>,
{
    duper::serde::de::from_string(input)
}
