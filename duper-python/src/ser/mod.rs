use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTuple, DuperValue,
};
use pyo3::{BoundObject, exceptions::PyValueError, prelude::*, types::*};

use well_known_type::WellKnownType;

mod well_known_type;

pub(crate) fn serialize_pyany<'py>(obj: Bound<'py, PyAny>) -> PyResult<DuperValue<'py>> {
    // Handle well-known types
    if let Some(well_known_type) = WellKnownType::identify(&obj)? {
        Ok(well_known_type.serialize()?)
    }
    // Handle basic types
    else if obj.is_instance_of::<PyDict>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(
                DuperObject::try_from(serialize_pydict(obj.cast()?)?)
                    .expect("no duplicate keys in dict"),
            ),
        })
    } else if obj.is_instance_of::<PyList>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray::from(serialize_pyiter(obj.try_iter()?)?)),
        })
    } else if obj.is_instance_of::<PySet>() {
        Ok(DuperValue {
            identifier: Some(
                DuperIdentifier::try_from(Cow::Borrowed("Set")).expect("valid identifier"),
            ),
            inner: DuperInner::Array(DuperArray::from(serialize_pyiter(obj.try_iter()?)?)),
        })
    } else if obj.is_instance_of::<PyTuple>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Tuple(DuperTuple::from(serialize_pyiter(obj.try_iter()?)?)),
        })
    } else if obj.is_instance_of::<PyBytes>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Bytes(DuperBytes::from(Cow::Owned(obj.extract()?))),
        })
    } else if obj.is_instance_of::<PyString>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::String(DuperString::from(Cow::Owned(obj.extract()?))),
        })
    } else if obj.is_instance_of::<PyBool>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Boolean(obj.extract()?),
        })
    } else if obj.is_instance_of::<PyInt>() {
        let identifier = {
            let identifier = serialize_pyclass_identifier(&obj)?;
            if identifier
                .as_ref()
                .is_some_and(|identifier| identifier.as_ref() != "Int")
            {
                identifier
            } else {
                None
            }
        };
        if let Ok(integer) = obj.extract() {
            Ok(DuperValue {
                identifier,
                inner: DuperInner::Integer(integer),
            })
        } else {
            Ok(DuperValue {
                identifier: identifier.or(Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Int")).expect("valid identifier"),
                )),
                inner: DuperInner::String(DuperString::from(Cow::Owned(obj.str()?.extract()?))),
            })
        }
    } else if obj.is_instance_of::<PyFloat>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Float(obj.extract()?),
        })
    } else if obj.is_none() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Null,
        })
    }
    // Handle sequences
    else if let Ok(pyiter) = obj.try_iter() {
        let identifier = serialize_pyclass_identifier(&obj)?;
        Ok(DuperValue {
            identifier,
            inner: DuperInner::Array(DuperArray::from(serialize_pyiter(pyiter.into_bound())?)),
        })
    }
    // Handle unknown types
    else if obj.hasattr("__bytes__")?
        && let Ok(bytes) = obj
            .call_method0("__bytes__")
            .and_then(|bytes| bytes.extract())
    {
        let identifier = serialize_pyclass_identifier(&obj)?;
        Ok(DuperValue {
            identifier,
            inner: DuperInner::Bytes(DuperBytes::from(Cow::Owned(bytes))),
        })
    } else if obj.hasattr("__slots__")?
        && let Ok(object) = serialize_pyslots(&obj)
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(
                DuperObject::try_from(object).expect("no duplicate keys in slots"),
            ),
        })
    } else {
        Err(PyErr::new::<PyValueError, String>(format!(
            "Unsupported type: {}",
            obj.get_type()
        )))
    }
}

fn serialize_pydict<'py>(
    dict: &Bound<'py, PyDict>,
) -> PyResult<Vec<(DuperKey<'py>, DuperValue<'py>)>> {
    dict.iter()
        .map(|(key, value)| {
            let key: &Bound<'py, PyString> = key.cast()?;
            Ok((
                DuperKey::from(Cow::Owned(key.to_string())),
                serialize_pyany(value)?,
            ))
        })
        .collect()
}

fn serialize_pyiter<'py>(iterator: Bound<'py, PyIterator>) -> PyResult<Vec<DuperValue<'py>>> {
    iterator.map(|value| serialize_pyany(value?)).collect()
}

fn serialize_pyslots<'py>(
    obj: &Bound<'py, PyAny>,
) -> PyResult<Vec<(DuperKey<'py>, DuperValue<'py>)>> {
    obj.getattr("__slots__")?
        .try_iter()?
        .map(|key: PyResult<Bound<'py, PyAny>>| {
            let key = key?;
            let key: &Bound<'py, PyString> = key.cast()?;
            let value = obj.getattr(key)?;
            Ok((
                DuperKey::from(Cow::Owned(key.to_string())),
                serialize_pyany(value)?,
            ))
        })
        .collect()
}

fn standardize_pyclass_identifier(mut identifier: String) -> PyResult<String> {
    let first_char = identifier.chars().next().ok_or_else(|| {
        PyErr::new::<PyValueError, &'static str>("Class identifier is empty string")
    })?;
    identifier.replace_range(
        0..first_char.len_utf8(),
        &first_char.to_uppercase().to_string(),
    );
    Ok(identifier)
}

pub(crate) fn serialize_pyclass_identifier<'py>(
    obj: &Bound<'py, PyAny>,
) -> PyResult<Option<DuperIdentifier<'py>>> {
    if obj.hasattr("__class__")?
        && let class = obj.getattr("__class__")?
        && class.hasattr("__name__")?
        && let Ok(name) = class.getattr("__name__")
        && let Ok(identifier) = name.extract::<&str>()
    {
        Ok(Some(
            DuperIdentifier::try_from_lossy(Cow::Owned(standardize_pyclass_identifier(
                identifier.to_string(),
            )?))
            .map_err(|error| {
                PyErr::new::<PyValueError, String>(format!(
                    "Invalid identifier: {identifier} ({error})"
                ))
            })?,
        ))
    } else if let typ = obj.get_type()
        && typ.hasattr("__name__")?
        && let Ok(name) = typ.getattr("__name__")
        && let Ok(identifier) = name.extract::<&str>()
    {
        Ok(Some(
            DuperIdentifier::try_from_lossy(Cow::Owned(standardize_pyclass_identifier(
                identifier.to_string(),
            )?))
            .map_err(|error| {
                PyErr::new::<PyValueError, String>(format!(
                    "Invalid identifier: {identifier} ({error})"
                ))
            })?,
        ))
    } else {
        Ok(None)
    }
}
