use base64::{Engine, prelude::BASE64_STANDARD};
use duper::{
    DuperFloat, DuperIdentifier, DuperObject, DuperTemporal, DuperValue, visitor::DuperVisitor,
};
use saphyr::{ScalarOwned, ScalarStyle, Tag, YamlOwned};

// A visitor that serializes Duper into a Saphyr YAML value.
pub(crate) struct SaphyrVisitor {}

impl DuperVisitor for SaphyrVisitor {
    type Value = Result<YamlOwned, String>;

    fn visit_object<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let mapping: Result<_, _> = object
            .iter()
            .map(|(key, value)| {
                value.accept(self).map(|value| {
                    (
                        YamlOwned::Value(ScalarOwned::String(key.as_ref().to_string())),
                        value,
                    )
                })
            })
            .collect();
        Ok(YamlOwned::Mapping(mapping?))
    }

    fn visit_array<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        array: &[DuperValue<'a>],
    ) -> Self::Value {
        let sequence: Result<_, _> = array.iter().map(|value| value.accept(self)).collect();
        Ok(YamlOwned::Sequence(sequence?))
    }

    fn visit_tuple<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[DuperValue<'a>],
    ) -> Self::Value {
        let sequence: Result<_, _> = tuple.iter().map(|value| value.accept(self)).collect();
        Ok(YamlOwned::Sequence(sequence?))
    }

    fn visit_string<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        string: &'a str,
    ) -> Self::Value {
        Ok(YamlOwned::Value(ScalarOwned::String(string.to_string())))
    }

    fn visit_bytes<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        bytes: &'a [u8],
    ) -> Self::Value {
        Ok(YamlOwned::Representation(
            BASE64_STANDARD.encode(bytes),
            ScalarStyle::DoubleQuoted,
            Some(Tag {
                handle: String::new(),
                suffix: "!binary".into(),
            }),
        ))
    }

    fn visit_temporal<'a>(&mut self, temporal: &DuperTemporal<'a>) -> Self::Value {
        Ok(YamlOwned::Representation(
            temporal.as_ref().to_string(),
            ScalarStyle::Plain,
            None,
        ))
    }

    fn visit_integer<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        Ok(YamlOwned::Value(ScalarOwned::Integer(integer)))
    }

    fn visit_float<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        float: DuperFloat,
    ) -> Self::Value {
        Ok(YamlOwned::Value(ScalarOwned::FloatingPoint(
            float.into_inner().into(),
        )))
    }

    fn visit_boolean<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        Ok(YamlOwned::Value(ScalarOwned::Boolean(boolean)))
    }

    fn visit_null<'a>(&mut self, _identifier: Option<&DuperIdentifier<'a>>) -> Self::Value {
        Ok(YamlOwned::Value(ScalarOwned::Null))
    }
}
