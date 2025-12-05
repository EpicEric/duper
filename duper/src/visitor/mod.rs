//! Utilities for using and implementing your own [`DuperVisitor`].

#[cfg(feature = "ansi")]
pub mod ansi;
pub mod pretty_printer;
pub mod serializer;

use crate::{DuperIdentifier, DuperTemporal, DuperValue, ast::DuperObject};

/// A trait for implementing a Duper visitor. You can visit a `DuperValue`
/// with `value.accept(&mut visitor)`.
///
/// # Example
///
/// ```
/// use duper::{
///     DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString,
///     DuperTemporal, DuperTuple, visitor::DuperVisitor,
/// };
///
/// struct MyVisitor;
///
/// impl DuperVisitor for MyVisitor {
///     type Value = ();
///
///     fn visit_object<'a>(
///         &mut self,
///         identifier: Option<&DuperIdentifier<'a>>,
///         object: &DuperObject<'a>,
///     ) -> Self::Value {
///         println!("object with identifier: {:?}", identifier);
///         for (key, value) in object.iter() {
///             print!("-> {:?}: ", key);
///             value.accept(self);
///         }
///     }
///
///     fn visit_array<'a>(
///         &mut self,
///         identifier: Option<&DuperIdentifier<'a>>,
///         array: &[DuperValue<'a>],
///     ) -> Self::Value {
///         println!("array with identifier: {:?}", identifier);
///         for value in array {
///             print!("-> ");
///             value.accept(self);
///         }
///     }
///
///     // ... Same for the remaining methods ...
///     #
///     # fn visit_tuple<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     tuple: &[DuperValue<'a>],
///     # ) -> Self::Value {}
///     #
///     # fn visit_string<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     string: &'a str,
///     # ) -> Self::Value {}
///     #
///     # fn visit_bytes<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     bytes: &'a [u8],
///     # ) -> Self::Value {}
///     #
///     # fn visit_temporal<'a>(
///     #     &mut self,
///     #     temporal: &DuperTemporal<'a>,
///     # ) -> Self::Value {}
///     #
///     # fn visit_integer<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     integer: i64,
///     # ) -> Self::Value {}
///     #
///     # fn visit_float<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     float: f64,
///     # ) -> Self::Value {}
///     #
///     # fn visit_boolean<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     boolean: bool,
///     # ) -> Self::Value {}
///     #
///     # fn visit_null<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>
///     # ) -> Self::Value {}
/// }
/// ```
pub trait DuperVisitor {
    type Value;

    /// Visits an object. You can access an iterator of `(key, value)` pairs by
    /// calling `object.iter()`.
    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value;

    /// Visits an array.
    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &[DuperValue<'a>],
    ) -> Self::Value;

    /// Visits a tuple.
    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[DuperValue<'a>],
    ) -> Self::Value;

    /// Visits a string.
    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &'a str,
    ) -> Self::Value;

    /// Visits bytes.
    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &'a [u8],
    ) -> Self::Value;

    /// Visits a Temporal value. You can access a `&str` by calling `temporal.as_ref()`.
    fn visit_temporal<'a>(&mut self, temporal: &DuperTemporal<'a>) -> Self::Value;

    /// Visits an integer.
    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value;

    /// Visits a floating point number.
    fn visit_float<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value;

    /// Visits a boolean.
    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value;

    /// Visits null.
    fn visit_null<'a>(&mut self, identifier: Option<&DuperIdentifier<'a>>) -> Self::Value;
}
