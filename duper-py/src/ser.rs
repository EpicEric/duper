use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTuple, DuperValue,
};
use pyo3::{BoundObject, exceptions::PyValueError, prelude::*, types::*};

pub(crate) fn serialize_pyany<'py>(obj: Bound<'py, PyAny>) -> PyResult<DuperValue<'py>> {
    // Handle basic types
    if obj.is_instance_of::<PyDict>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject::from(serialize_pydict(obj.downcast()?)?)),
        })
    } else if obj.is_instance_of::<PyList>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray::from(serialize_pyiter(obj.try_iter()?)?)),
        })
    } else if obj.is_instance_of::<PySet>() {
        Ok(DuperValue {
            identifier: Some(DuperIdentifier::from(Cow::Borrowed("Set"))),
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
                identifier: identifier.or(Some(DuperIdentifier::from(Cow::Borrowed("Integer")))),
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
    // Handle Pydantic models and specialized cases
    else if is_pydantic_model(&obj)? {
        serialize_pydantic_model(&obj)
    } else if let Some(value) = serialize_specialized_python_types(&obj)? {
        Ok(value)
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
    else if obj.hasattr("__dict__")?
        && let Ok(object) = serialize_pydict(obj.getattr("__dict__")?.downcast()?)
    {
        let identifier = serialize_pyclass_identifier(&obj)?;
        Ok(DuperValue {
            identifier,
            inner: DuperInner::Object(DuperObject::from(object)),
        })
    } else if obj.hasattr("__slots__")?
        && let Ok(object) = serialize_pyslots(&obj)
    {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject::from(object)),
        })
    } else if obj.hasattr("__str__")?
        && let Ok(string) = obj.str().and_then(|string| string.extract())
    {
        let identifier = serialize_pyclass_identifier(&obj)?;
        Ok(DuperValue {
            identifier,
            inner: DuperInner::String(DuperString::from(Cow::Owned(string))),
        })
    } else if obj.hasattr("__bytes__")?
        && let Ok(bytes) = obj
            .call_method0("__bytes__")
            .and_then(|bytes| bytes.extract())
    {
        let identifier = serialize_pyclass_identifier(&obj)?;
        Ok(DuperValue {
            identifier: identifier,
            inner: DuperInner::Bytes(DuperBytes::from(Cow::Owned(bytes))),
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
            let key: &Bound<'py, PyString> = key.downcast()?;
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
            let key: &Bound<'py, PyString> = key.downcast()?;
            let value = obj.getattr(key)?;
            Ok((
                DuperKey::from(Cow::Owned(key.to_string())),
                serialize_pyany(value)?,
            ))
        })
        .collect()
}

fn standardize_pyclass_identifier(identifier: &str) -> String {
    let (start, end) = identifier.split_at(1);
    format!("{}{}", start.to_uppercase(), end.to_lowercase())
}

fn serialize_pyclass_identifier<'py>(
    obj: &Bound<'py, PyAny>,
) -> PyResult<Option<DuperIdentifier<'py>>> {
    if obj.hasattr("__class__")?
        && let class = obj.getattr("__class__")?
        && class.hasattr("__name__")?
        && let Ok(name) = class.getattr("__name__")
        && let Ok(identifier) = name.extract::<String>()
    {
        Ok(Some(DuperIdentifier::from(Cow::Owned(
            standardize_pyclass_identifier(&identifier),
        ))))
    } else if let typ = obj.get_type()
        && typ.hasattr("__name__")?
        && let Ok(name) = typ.getattr("__name__")
        && let Ok(identifier) = name.extract::<String>()
    {
        Ok(Some(DuperIdentifier::from(Cow::Owned(
            standardize_pyclass_identifier(&identifier),
        ))))
    } else {
        Ok(None)
    }
}

fn is_pydantic_model<'py>(obj: &Bound<'py, PyAny>) -> PyResult<bool> {
    if obj.hasattr("__class__")? && obj.getattr("__class__")?.hasattr("model_fields")? {
        Ok(true)
    } else {
        is_py_mro(obj, "BaseModel")
    }
}

fn serialize_pydantic_model<'py>(obj: &Bound<'py, PyAny>) -> PyResult<DuperValue<'py>> {
    if let Ok(class) = obj.getattr("__class__") {
        let model_fields = class.getattr("model_fields")?;
        if model_fields.is_instance_of::<PyDict>() {
            let field_dict = model_fields.downcast::<PyDict>()?;
            let fields: PyResult<Vec<_>> = field_dict
                .iter()
                .map(|(field_name, _field_info)| {
                    let field_name: &Bound<'py, PyString> = field_name.downcast()?;
                    let value = obj.getattr(&field_name)?;
                    Ok((
                        DuperKey::from(Cow::Owned(field_name.to_string())),
                        serialize_pyany(value)?,
                    ))
                })
                .collect();
            return Ok(DuperValue {
                identifier: serialize_pyclass_identifier(obj)?,
                inner: DuperInner::Object(DuperObject::from(fields?)),
            });
        }
    }
    Err(PyErr::new::<PyValueError, String>(format!(
        "Unsupported type: {}",
        obj.get_type()
    )))
}

fn serialize_specialized_python_types<'py>(
    obj: &Bound<'py, PyAny>,
) -> PyResult<Option<DuperValue<'py>>> {
    if let Some(identifier) = serialize_pyclass_identifier(obj)? {
        match identifier.as_ref() {
            "Timedelta" | "Uuid" | "Ipv4address" | "Ipv4interface" | "Ipv4network"
            | "Ipv6address" | "Ipv6interface" | "Ipv6network" => {
                let string = obj.str()?.extract()?;
                Ok(Some(DuperValue {
                    identifier: Some(identifier),
                    inner: DuperInner::String(DuperString::from(Cow::Owned(string))),
                }))
            }
            "Datetime" | "Date" | "Time" if obj.hasattr("isoformat")? => {
                let string = obj.call_method0("isoformat")?.extract()?;
                Ok(Some(DuperValue {
                    identifier: Some(identifier),
                    inner: DuperInner::String(DuperString::from(Cow::Owned(string))),
                }))
            }
            "Pattern" if obj.hasattr("pattern")? => {
                let string = obj.getattr("pattern")?.extract()?;
                Ok(Some(DuperValue {
                    identifier: Some(identifier),
                    inner: DuperInner::String(DuperString::from(Cow::Owned(string))),
                }))
            }
            _ if is_pyenum(obj)? => {
                let identifier = serialize_pyclass_identifier(&obj)?;
                let value = serialize_pyany(obj.getattr("value")?)?;
                Ok(Some(DuperValue {
                    identifier,
                    inner: value.inner,
                }))
            }
            // Ignore unknown types
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn is_pyenum<'py>(obj: &Bound<'py, PyAny>) -> PyResult<bool> {
    is_py_mro(obj, "Enum")
}

fn is_py_mro<'py>(obj: &Bound<'py, PyAny>, mro_str: &str) -> PyResult<bool> {
    if obj.hasattr("name")?
        && obj.hasattr("value")?
        && obj.hasattr("__class__")?
        && let Ok(mro) = obj.getattr("__class__")?.getattr("__mro__")
    {
        for base in mro.try_iter()? {
            let base = base?;
            if let Ok(base_name) = base.getattr("__name__")?.extract::<String>()
                && base_name == mro_str
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
