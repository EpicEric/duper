use pyo3::{exceptions::PyValueError, prelude::*};

mod de;
mod ser;

#[pyclass(frozen)]
pub(crate) struct DuperType;

#[pyclass(frozen)]
pub(crate) struct Duper {
    pub(crate) identifier: Option<duper::DuperIdentifier<'static>>,
}

impl Duper {
    pub(crate) fn from_identifier<'a, 'py>(
        identifier: &duper::DuperIdentifier<'a>,
    ) -> PyResult<Self> {
        Ok(Self {
            identifier: Some(identifier.static_clone()),
        })
    }
}

#[pymethods]
impl Duper {
    #[new]
    fn new<'py, 'a>(identifier: Option<String>) -> PyResult<Self> {
        match identifier {
            Some(identifier) => match duper::DuperIdentifier::try_from(identifier) {
                Ok(identifier) => Self::from_identifier(&identifier),
                Err(error) => Err(PyErr::new::<PyValueError, String>(error.to_string())),
            },
            None => Ok(Self { identifier: None }),
        }
    }

    #[getter]
    fn identifier(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .map(|identifier| identifier.as_ref())
    }

    fn __repr__(&self) -> String {
        match self.identifier.as_ref() {
            Some(identifier) => format!("Duper('{}')", identifier.as_ref()),
            None => "Duper(None)".into(),
        }
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

#[pyo3::pymodule(name = "_duper")]
mod duper_py {
    use duper::{DuperParser, DuperValue, PrettyPrinter, Serializer};
    use pyo3::{
        exceptions::PyValueError,
        prelude::*,
        types::{PyInt, PyString},
    };

    #[pymodule_export]
    use crate::{Duper, DuperType};
    use crate::{de::Visitor, ser::serialize_pyany};

    #[pyfunction]
    #[pyo3(signature = (obj, *, indent=None, strip_identifiers=false))]
    fn dumps<'py>(
        obj: Bound<'py, PyAny>,
        indent: Option<Bound<'py, PyAny>>,
        strip_identifiers: bool,
    ) -> PyResult<String> {
        let value: DuperValue = serialize_pyany(obj)?;
        if let Some(indent) = indent {
            if indent.is_instance_of::<PyInt>() {
                let indent: usize = indent.extract()?;
                Ok(PrettyPrinter::new(
                    strip_identifiers,
                    &(0..indent).map(|_| ' ').collect::<String>(),
                )
                .map_err(|error| PyErr::new::<PyValueError, String>(error.into()))?
                .pretty_print(value))
            } else if indent.is_instance_of::<PyString>() {
                let indent: &str = indent.extract()?;
                Ok(PrettyPrinter::new(strip_identifiers, indent)
                    .map_err(|error| PyErr::new::<PyValueError, String>(error.into()))?
                    .pretty_print(value))
            } else {
                Err(PyErr::new::<PyValueError, String>(format!(
                    "expect indent to be string or int, found {indent:?}"
                )))
            }
        } else {
            Ok(Serializer::new(strip_identifiers).serialize(value))
        }
    }

    #[pyfunction]
    #[pyo3(signature = (obj, fp, *, indent=None, strip_identifiers=false))]
    fn dump<'py>(
        obj: Bound<'py, PyAny>,
        fp: Bound<'py, PyAny>,
        indent: Option<Bound<'py, PyAny>>,
        strip_identifiers: bool,
    ) -> PyResult<()> {
        let value: DuperValue = serialize_pyany(obj)?;
        fp.call_method1(
            "write",
            (if let Some(indent) = indent {
                if indent.is_instance_of::<PyInt>() {
                    let indent: usize = indent.extract()?;
                    PrettyPrinter::new(
                        strip_identifiers,
                        &(0..indent).map(|_| ' ').collect::<String>(),
                    )
                    .map_err(|error| PyErr::new::<PyValueError, String>(error.into()))?
                    .pretty_print(value)
                } else if indent.is_instance_of::<PyString>() {
                    let indent: &str = indent.extract()?;
                    PrettyPrinter::new(strip_identifiers, indent)
                        .map_err(|error| PyErr::new::<PyValueError, String>(error.into()))?
                        .pretty_print(value)
                } else {
                    Err(PyErr::new::<PyValueError, String>(format!(
                        "expect indent to be string or int, found {indent:?}"
                    )))?
                }
            } else {
                Serializer::new(strip_identifiers).serialize(value)
            },),
        )?;
        Ok(())
    }

    #[pyfunction]
    #[pyo3(signature = (s, *, parse_any=false))]
    fn loads<'py>(py: Python<'py>, s: &str, parse_any: bool) -> PyResult<Bound<'py, PyAny>> {
        let value = match parse_any {
            true => DuperParser::parse_duper_value(s),
            false => DuperParser::parse_duper_trunk(s),
        }
        .map_err(|err| {
            PyErr::new::<PyValueError, String>(format!(
                "{:?}",
                miette::Error::new(err.into_miette())
            ))
        })?;
        value
            .accept(&mut Visitor { py })
            .map(|visitor_value| visitor_value.value)
    }

    #[pyfunction]
    #[pyo3(signature = (fp, *, parse_any=false))]
    fn load<'py>(
        py: Python<'py>,
        fp: Bound<'py, PyAny>,
        parse_any: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let read = fp.call_method0("read")?;
        let s: &str = read.extract()?;
        let value = match parse_any {
            true => DuperParser::parse_duper_value(s),
            false => DuperParser::parse_duper_trunk(s),
        }
        .map_err(|err| {
            PyErr::new::<PyValueError, String>(format!(
                "{:?}",
                miette::Error::new(err.into_miette())
            ))
        })?;
        value
            .accept(&mut Visitor { py })
            .map(|visitor_value| visitor_value.value)
    }
}
