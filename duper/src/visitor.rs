use std::borrow::Cow;

use crate::ast::DuperValue;

pub trait DuperVisitor {
    type Value;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<Cow<'a, str>>,
        object: Vec<(Cow<'a, str>, DuperValue<'a>)>,
    ) -> Self::Value;

    fn visit_array<'a>(
        &mut self,
        identifier: Option<Cow<'a, str>>,
        array: Vec<DuperValue<'a>>,
    ) -> Self::Value;

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<Cow<'a, str>>,
        tuple: Vec<DuperValue<'a>>,
    ) -> Self::Value;

    fn visit_string<'a>(
        &mut self,
        identifier: Option<Cow<'a, str>>,
        string: Cow<'a, str>,
    ) -> Self::Value;

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<Cow<'a, str>>,
        bytes: Cow<'a, [u8]>,
    ) -> Self::Value;

    fn visit_integer<'a>(&mut self, identifier: Option<Cow<'a, str>>, integer: i64) -> Self::Value;

    fn visit_float<'a>(&mut self, identifier: Option<Cow<'a, str>>, float: f64) -> Self::Value;

    fn visit_boolean<'a>(&mut self, identifier: Option<Cow<'a, str>>, boolean: bool)
    -> Self::Value;

    fn visit_null<'a>(&mut self, identifier: Option<Cow<'a, str>>) -> Self::Value;
}
