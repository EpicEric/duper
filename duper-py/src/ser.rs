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
            inner: DuperInner::Object(
                DuperObject::try_from(serialize_pydict(obj.downcast()?)?)
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
    // Handle well-known types
    else if let Some(well_known_type) = WellKnownType::identify(&obj)? {
        Ok(well_known_type.serialize()?)
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
    } else if obj.hasattr("__str__")?
        && let Ok(string) = obj.str().and_then(|string| string.extract())
    {
        let identifier = serialize_pyclass_identifier(&obj)?;
        Ok(DuperValue {
            identifier,
            inner: DuperInner::String(DuperString::from(Cow::Owned(string))),
        })
    } else if obj.hasattr("__dict__")?
        && let Ok(object) = serialize_pydict(obj.getattr("__dict__")?.downcast()?)
    {
        let identifier = serialize_pyclass_identifier(&obj)?;
        Ok(DuperValue {
            identifier,
            inner: DuperInner::Object(
                DuperObject::try_from(object).expect("no duplicate keys in dict"),
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

#[derive(Debug)]
enum WellKnownType<'py> {
    // collections
    Deque(Bound<'py, PyAny>),
    // dataclasses
    Dataclass(Bound<'py, PyAny>),
    // datetime
    DateTime(Bound<'py, PyAny>),
    TimeDelta(Bound<'py, PyAny>),
    Date(Bound<'py, PyAny>),
    Time(Bound<'py, PyAny>),
    // decimal
    Decimal(Bound<'py, PyAny>),
    // duper
    DuperBaseModel(Bound<'py, PyAny>),
    // enum
    Enum(Bound<'py, PyAny>),
    // ipaddress
    IPv4Address(Bound<'py, PyAny>),
    IPv4Interface(Bound<'py, PyAny>),
    IPv4Network(Bound<'py, PyAny>),
    IPv6Address(Bound<'py, PyAny>),
    IPv6Interface(Bound<'py, PyAny>),
    IPv6Network(Bound<'py, PyAny>),
    // pathlib
    Path(Bound<'py, PyAny>),
    PosixPath(Bound<'py, PyAny>),
    WindowsPath(Bound<'py, PyAny>),
    PurePath(Bound<'py, PyAny>),
    PurePosixPath(Bound<'py, PyAny>),
    PureWindowsPath(Bound<'py, PyAny>),
    // pydantic
    BaseModel(Bound<'py, PyAny>),
    ByteSize(Bound<'py, PyAny>),
    // re
    Pattern(Bound<'py, PyAny>),
    // uuid
    Uuid(Bound<'py, PyAny>),
}

impl<'py> WellKnownType<'py> {
    fn identify(value: &Bound<'py, PyAny>) -> PyResult<Option<Self>> {
        if !value.hasattr("__class__")? {
            return Ok(None);
        }
        let inspect: Bound<'py, PyModule> = value.py().import("inspect")?;
        let mro = inspect
            .getattr("getmro")?
            .call1((value.getattr("__class__")?,))?;
        for class in mro.try_iter()? {
            let Ok(class) = class else {
                continue;
            };
            let module_attr = class.getattr("__module__")?;
            let module: &str = module_attr.extract()?;
            let classname_attr = class.getattr("__name__")?;
            let classname: &str = classname_attr.extract()?;
            match (module, classname) {
                // collections
                ("collections", "deque") => return Ok(Some(WellKnownType::Deque(value.clone()))),
                // datetime
                ("datetime", "datetime") => {
                    return Ok(Some(WellKnownType::DateTime(value.clone())));
                }
                ("datetime", "timedelta") => {
                    return Ok(Some(WellKnownType::TimeDelta(value.clone())));
                }
                ("datetime", "date") => return Ok(Some(WellKnownType::Date(value.clone()))),
                ("datetime", "time") => return Ok(Some(WellKnownType::Time(value.clone()))),
                // decimal
                ("decimal", "Decimal") => return Ok(Some(WellKnownType::Decimal(value.clone()))),
                // duper
                ("duper.pydantic", "BaseModel") => {
                    return Ok(Some(WellKnownType::DuperBaseModel(value.clone())));
                }
                // enum
                ("enum", "Enum") => {
                    return Ok(Some(WellKnownType::Enum(value.clone())));
                }
                // ipaddress
                ("ipaddress", "IPv4Address") => {
                    return Ok(Some(WellKnownType::IPv4Address(value.clone())));
                }
                ("ipaddress", "IPv4Interface") => {
                    return Ok(Some(WellKnownType::IPv4Interface(value.clone())));
                }
                ("ipaddress", "IPv4Network") => {
                    return Ok(Some(WellKnownType::IPv4Network(value.clone())));
                }
                ("ipaddress", "IPv6Address") => {
                    return Ok(Some(WellKnownType::IPv6Address(value.clone())));
                }
                ("ipaddress", "IPv6Interface") => {
                    return Ok(Some(WellKnownType::IPv6Interface(value.clone())));
                }
                ("ipaddress", "IPv6Network") => {
                    return Ok(Some(WellKnownType::IPv6Network(value.clone())));
                }
                // pathlib
                ("pathlib", "Path") => return Ok(Some(WellKnownType::Path(value.clone()))),
                ("pathlib", "PosixPath") => {
                    return Ok(Some(WellKnownType::PosixPath(value.clone())));
                }
                ("pathlib", "WindowsPath") => {
                    return Ok(Some(WellKnownType::WindowsPath(value.clone())));
                }
                ("pathlib", "PurePath") => return Ok(Some(WellKnownType::PurePath(value.clone()))),
                ("pathlib", "PurePosixPath") => {
                    return Ok(Some(WellKnownType::PurePosixPath(value.clone())));
                }
                ("pathlib", "PureWindowsPath") => {
                    return Ok(Some(WellKnownType::PureWindowsPath(value.clone())));
                }
                // pydantic
                ("pydantic.main", "BaseModel") => {
                    return Ok(Some(WellKnownType::BaseModel(value.clone())));
                }
                ("pydantic", "ByteSize") => {
                    return Ok(Some(WellKnownType::ByteSize(value.clone())));
                }
                // re
                ("re", "Pattern") => return Ok(Some(WellKnownType::Pattern(value.clone()))),
                // uuid
                ("uuid", "UUID") => return Ok(Some(WellKnownType::Uuid(value.clone()))),
                _ => (),
            }
        }
        let dataclasses: Bound<'py, PyModule> = value.py().import("dataclasses")?;
        if dataclasses
            .getattr("is_dataclass")?
            .call1((value,))?
            .extract()?
        {
            return Ok(Some(WellKnownType::Dataclass(value.clone())));
        }
        Ok(None)
    }

    fn serialize(self) -> PyResult<DuperValue<'py>> {
        match self {
            // collections
            WellKnownType::Deque(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Deque")).expect("valid identifier"),
                ),
                inner: DuperInner::Array(DuperArray::from(
                    value
                        .try_iter()?
                        .map(|elem| -> PyResult<DuperValue<'_>> { serialize_pyany(elem?) })
                        .collect::<PyResult<Vec<_>>>()?,
                )),
            }),
            // dataclasses
            WellKnownType::Dataclass(value) => Ok(DuperValue {
                identifier: serialize_pyclass_identifier(&value)?,
                inner: serialize_pyany(value.getattr("__dict__")?)?.inner,
            }),
            // datetime
            WellKnownType::DateTime(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("DateTime")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(
                    value.call_method0("isoformat")?.extract()?,
                ))),
            }),
            WellKnownType::TimeDelta(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("TimeDelta"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::Date(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Date")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(
                    value.call_method0("isoformat")?.extract()?,
                ))),
            }),
            WellKnownType::Time(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Time")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(
                    value.call_method0("isoformat")?.extract()?,
                ))),
            }),
            // decimal
            WellKnownType::Decimal(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Decimal")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // duper
            WellKnownType::DuperBaseModel(value) => serialize_pydantic_model(value),
            // enum
            WellKnownType::Enum(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Address"))
                        .expect("valid identifier"),
                ),
                inner: serialize_pyany(value.getattr("value")?)?.inner,
            }),
            // ipaddress
            WellKnownType::IPv4Address(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Address"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv4Interface(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Interface"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv4Network(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Network"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv6Address(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv6Address"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv6Interface(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv6Interface"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv6Network(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv6Network"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // pathlib
            WellKnownType::Path(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Path")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PosixPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PosixPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::WindowsPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("WindowsPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PurePath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PurePath")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PurePosixPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PurePosixPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PureWindowsPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PureWindowsPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // pydantic
            WellKnownType::BaseModel(value) => serialize_pydantic_model(value),
            WellKnownType::ByteSize(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ByteSize")).expect("valid identifier"),
                ),
                inner: serialize_pyany(value.call_method0("__int__")?)?.inner,
            }),
            // re
            WellKnownType::Pattern(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Pattern")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(
                    value.getattr("pattern")?.extract()?,
                ))),
            }),
            // uuid
            WellKnownType::Uuid(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Uuid")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
        }
    }
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

fn serialize_pyclass_identifier<'py>(
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
                    "Invalid identifier: {}\n{}",
                    identifier, error
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
                    "Invalid identifier: {}\n{}",
                    identifier, error
                ))
            })?,
        ))
    } else {
        Ok(None)
    }
}

fn serialize_pydantic_model<'py>(obj: Bound<'py, PyAny>) -> PyResult<DuperValue<'py>> {
    if let Ok(class) = obj.getattr("__class__") {
        let model_fields = class.getattr("model_fields")?;
        if model_fields.is_instance_of::<PyDict>() {
            let field_dict = model_fields.downcast::<PyDict>()?;
            let fields: PyResult<Vec<_>> = field_dict
                .iter()
                .map(|(field_name, _field_info)| {
                    let field_name: &Bound<'py, PyString> = field_name.downcast()?;
                    let value = obj.getattr(field_name)?;
                    Ok((
                        DuperKey::from(Cow::Owned(field_name.to_string())),
                        serialize_pyany(value)?,
                    ))
                })
                .collect();
            return Ok(DuperValue {
                identifier: serialize_pyclass_identifier(&obj)?,
                inner: DuperInner::Object(
                    DuperObject::try_from(fields?).expect("no duplicate keys in pydantic model"),
                ),
            });
        }
    }
    Err(PyErr::new::<PyValueError, String>(format!(
        "Unsupported type: {}",
        obj.get_type()
    )))
}
