#[cfg(feature = "ansi")]
pub mod ansi;
pub mod pretty_printer;
pub mod serializer;

use crate::ast::{DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTuple};

pub trait DuperVisitor {
    type Value;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value;

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value;

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &DuperTuple<'a>,
    ) -> Self::Value;

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &DuperString<'a>,
    ) -> Self::Value;

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value;

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value;

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value;

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value;

    fn visit_null<'a>(&mut self, identifier: Option<&DuperIdentifier<'a>>) -> Self::Value;
}
