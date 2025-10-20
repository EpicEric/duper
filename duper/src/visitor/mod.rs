//! Utilities for using and implementing your own [`DuperVisitor`].

#[cfg(feature = "ansi")]
pub mod ansi;
pub mod pretty_printer;
pub mod serializer;

use crate::ast::{DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTuple};

/// A trait for implementing a Duper visitor. You can visit a `DuperValue`
/// with `value.accept(&mut visitor)`.
///
/// # Example
///
/// ```
/// use duper::{
///     DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString,
///     DuperTuple, visitor::DuperVisitor,
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
///         array: &DuperArray<'a>,
///     ) -> Self::Value {
///         println!("array with identifier: {:?}", identifier);
///         for value in array.iter() {
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
///     #     tuple: &DuperTuple<'a>,
///     # ) -> Self::Value {}
///     #
///     # fn visit_string<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     string: &DuperString<'a>,
///     # ) -> Self::Value {}
///     #
///     # fn visit_bytes<'a>(
///     #     &mut self,
///     #     identifier: Option<&DuperIdentifier<'a>>,
///     #     bytes: &DuperBytes<'a>,
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

    /// Visits an array. You can access an iterator of values by calling
    /// `array.iter()`.
    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value;

    /// Visits a tuple. You can access an iterator of values by calling
    /// `tuple.iter()`.
    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &DuperTuple<'a>,
    ) -> Self::Value;

    /// Visits a string. You can access a `Cow` of a str slice by calling
    /// `string.as_ref()`.
    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &DuperString<'a>,
    ) -> Self::Value;

    /// Visits bytes. You can access a `Cow` of a byte slice by calling
    /// `bytes.as_ref()`.
    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value;

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
