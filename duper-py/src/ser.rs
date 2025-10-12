use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTuple, DuperValue,
};
use pyo3::{exceptions::PyValueError, prelude::*, types::*};

pub(crate) fn serialize_pyany<'py>(obj: Bound<'py, PyAny>) -> PyResult<DuperValue<'py>> {
    if obj.is_instance_of::<PyDict>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject::from(serialize_dict(obj.downcast()?)?)),
        })
    } else if obj.is_instance_of::<PyList>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray::from(serialize_list(obj.downcast()?)?)),
        })
    } else if obj.is_instance_of::<PyTuple>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Tuple(DuperTuple::from(serialize_tuple(obj.downcast()?)?)),
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
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(obj.extract()?),
        })
    } else if obj.is_instance_of::<PyFloat>() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Float(obj.extract()?),
        })
    } else if is_pydantic_model(&obj)? {
        serialize_pydantic_model(&obj)
    } else if obj.hasattr("__dict__")? {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject::from(serialize_dict(
                obj.getattr("__dict__")?.downcast()?,
            )?)),
        })
    } else if obj.hasattr("__slots__")? {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject::from(serialize_slots(&obj)?)),
        })
    } else if obj.is_none() {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Null,
        })
    } else {
        Err(PyErr::new::<PyValueError, String>(format!(
            "Unsupported type: {}",
            obj.get_type()
        )))
    }
}

fn serialize_dict<'py>(
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

fn serialize_list<'py>(list: &Bound<'py, PyList>) -> PyResult<Vec<DuperValue<'py>>> {
    list.iter().map(|value| serialize_pyany(value)).collect()
}

fn serialize_tuple<'py>(tuple: &Bound<'py, PyTuple>) -> PyResult<Vec<DuperValue<'py>>> {
    tuple.iter().map(|value| serialize_pyany(value)).collect()
}

fn serialize_slots<'py>(
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

fn is_pydantic_model<'py>(obj: &Bound<'py, PyAny>) -> PyResult<bool> {
    if obj.getattr("__class__")?.hasattr("model_fields")? {
        Ok(true)
    } else if let Ok(mro) = obj.getattr("__class__")?.getattr("__mro__") {
        for base in mro.try_iter()? {
            let base = base?;
            if let Ok(name) = base.getattr("__name__")?.extract::<String>() {
                if name == "BaseModel" {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    } else {
        Ok(false)
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
                identifier: obj
                    .getattr("__class__")
                    .and_then(|attr| attr.getattr("__name__")?.extract::<String>())
                    .map(|identifier| DuperIdentifier::from(Cow::Owned(identifier)))
                    .ok(),
                inner: DuperInner::Object(DuperObject::from(fields?)),
            });
        }
    }
    Err(PyErr::new::<PyValueError, String>(format!(
        "Unsupported type: {}",
        obj.get_type()
    )))
}
