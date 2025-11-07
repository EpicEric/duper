use std::borrow::Cow;

use pyo3::{
    IntoPyObjectExt,
    exceptions::PyValueError,
    prelude::*,
    types::{PyCFunction, PyDict, PyTuple, PyType},
};

use crate::ser::serialize_pyany;

#[pyclass(frozen, module = "duper", eq)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TemporalString {
    pub(crate) temporal: duper::DuperTemporal<'static>,
}

impl TemporalString {
    pub(crate) fn from_temporal<'a>(temporal: &duper::DuperTemporal<'a>) -> PyResult<Self> {
        Ok(Self {
            temporal: temporal.static_clone(),
        })
    }
}

#[pymethods]
impl TemporalString {
    #[new]
    fn new(value: String, r#type: Option<String>) -> PyResult<Self> {
        Ok(TemporalString {
            temporal: match r#type.as_ref().map(|typ| typ.as_str()) {
                Some("Instant") => duper::DuperTemporal::try_instant_from(Cow::Owned(value))
                    .map_err(|err| {
                        PyValueError::new_err(format!(
                            "Failed to parse Instant Temporal value: {err}"
                        ))
                    })?,
                Some("ZonedDateTime") => duper::DuperTemporal::try_zoned_date_time_from(
                    Cow::Owned(value),
                )
                .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Failed to parse ZonedDateTime Temporal value: {err}"
                    ))
                })?,
                Some("PlainDate") => duper::DuperTemporal::try_plain_date_from(Cow::Owned(value))
                    .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Failed to parse PlainDate Temporal value: {err}"
                    ))
                })?,
                Some("PlainTime") => duper::DuperTemporal::try_plain_time_from(Cow::Owned(value))
                    .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Failed to parse PlainTime Temporal value: {err}"
                    ))
                })?,
                Some("PlainDateTime") => duper::DuperTemporal::try_plain_date_time_from(
                    Cow::Owned(value),
                )
                .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Failed to parse PlainDateTime Temporal value: {err}"
                    ))
                })?,
                Some("PlainYearMonth") => duper::DuperTemporal::try_plain_year_month_from(
                    Cow::Owned(value),
                )
                .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Failed to parse PlainYearMonth Temporal value: {err}"
                    ))
                })?,
                Some("PlainMonthDay") => duper::DuperTemporal::try_plain_month_day_from(
                    Cow::Owned(value),
                )
                .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Failed to parse PlainMonthDay Temporal value: {err}"
                    ))
                })?,
                Some("Duration") => duper::DuperTemporal::try_duration_from(Cow::Owned(value))
                    .map_err(|err| {
                        PyValueError::new_err(format!(
                            "Failed to parse Duration Temporal value: {err}"
                        ))
                    })?,
                Some(typ) => {
                    return Err(PyValueError::new_err(format!(
                        "Unknown TemporalString type {typ}"
                    )));
                }
                None => duper::DuperTemporal::try_unspecified_from(Cow::Owned(value)).map_err(
                    |err| {
                        PyValueError::new_err(format!(
                            "Failed to parse Unspecified Temporal value: {err}"
                        ))
                    },
                )?,
            },
        })
    }

    #[getter]
    fn r#type(&self) -> Option<&str> {
        match self.temporal {
            duper::DuperTemporal::Instant(_) => Some("Instant"),
            duper::DuperTemporal::ZonedDateTime(_) => Some("ZonedDateTime"),
            duper::DuperTemporal::PlainDate(_) => Some("PlainDate"),
            duper::DuperTemporal::PlainTime(_) => Some("PlainTime"),
            duper::DuperTemporal::PlainDateTime(_) => Some("PlainDateTime"),
            duper::DuperTemporal::PlainYearMonth(_) => Some("PlainYearMonth"),
            duper::DuperTemporal::PlainMonthDay(_) => Some("PlainMonthDay"),
            duper::DuperTemporal::Duration(_) => Some("Duration"),
            duper::DuperTemporal::Unspecified(_) => None,
        }
    }

    fn __repr__(&self) -> String {
        let typ = match self.r#type() {
            Some(typ) => typ,
            None => "None",
        };
        let value = self.temporal.as_ref();
        format!("TemporalString(type={typ} value='{value}')")
    }

    fn __str__(&self) -> String {
        self.temporal.as_ref().to_string()
    }

    #[staticmethod]
    fn _validate<'py>(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(temporal) = value.cast::<Self>() {
            Ok(temporal.get().clone())
        } else if let Ok(value) = value.extract() {
            Ok(Self {
                temporal: duper::DuperTemporal::try_unspecified_from(Cow::Owned(value)).map_err(
                    |err| {
                        PyValueError::new_err(format!(
                            "Failed to parse Unspecified Temporal value: {err}"
                        ))
                    },
                )?,
            })
        } else if let Ok(duper::DuperValue {
            identifier: _,
            inner: duper::DuperInner::Temporal(temporal),
        }) = serialize_pyany(value.clone())
        {
            Ok(Self {
                temporal: temporal.static_clone(),
            })
        } else {
            let typ = value.get_type();
            Err(PyValueError::new_err(format!(
                "Cannot convert {typ} to TemporalString"
            )))
        }
    }

    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, PyType>,
        _source: &Bound<'py, PyType>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = cls.py();
        let core_schema = py.import("pydantic_core")?.getattr("core_schema")?;

        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item("info_arg", false)?;

        let serialize =
            |args: &Bound<'_, PyTuple>, _kwargs: Option<&Bound<'_, PyDict>>| -> PyResult<_> {
                let py = args.py();
                let value = args.extract::<(TemporalString,)>()?.0;
                match value.temporal {
                    duper::DuperTemporal::Instant(inner)
                    | duper::DuperTemporal::PlainDateTime(inner) => {
                        let datetime = py.import("datetime")?.getattr("datetime")?;
                        Ok(datetime
                            .getattr("fromisoformat")?
                            .call1((inner.as_ref(),))?
                            .unbind())
                    }
                    duper::DuperTemporal::PlainDate(inner) => {
                        let date = py.import("datetime")?.getattr("date")?;
                        Ok(date
                            .getattr("fromisoformat")?
                            .call1((inner.as_ref(),))?
                            .unbind())
                    }
                    duper::DuperTemporal::PlainTime(inner) => {
                        let time = py.import("datetime")?.getattr("time")?;
                        Ok(time
                            .getattr("fromisoformat")?
                            .call1((inner.as_ref(),))?
                            .unbind())
                    }
                    duper::DuperTemporal::Duration(inner) => {
                        let timedelta = py.import("datetime")?.getattr("timedelta")?;
                        let adapter = py
                            .import("pydantic")?
                            .getattr("TypeAdapter")?
                            .call1((timedelta,))?;
                        Ok(adapter
                            .getattr("validate_python")?
                            .call1((inner.as_ref(),))?
                            .unbind())
                    }
                    _ => value.into_py_any(py),
                }
            };
        let serialize_fn = PyCFunction::new_closure(py, None, None, serialize)?;

        let kwargs = PyDict::new(py);
        kwargs.set_item(
            "serialization",
            core_schema
                .getattr("plain_serializer_function_ser_schema")?
                .call((serialize_fn,), Some(&serialization_kwargs))?,
        )?;

        core_schema
            .getattr("no_info_plain_validator_function")?
            .call((cls.getattr("_validate")?,), Some(&kwargs))
    }
}
