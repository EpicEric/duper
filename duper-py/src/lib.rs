use duper::{DuperParser, DuperValue, PrettyPrinter, Serializer};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{de::Visitor, ser::serialize_pyany};

mod de;
mod ser;

/// Utilities for converting to and from Python types into the Duper format.
#[pymodule(name = "_duper")]
fn duper_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    /// Serialize obj as a Duper value formatted str.
    #[pyfn(m)]
    #[pyo3(signature = (obj, *, indent=None))]
    fn dumps<'py>(obj: Bound<'py, PyAny>, indent: Option<usize>) -> PyResult<String> {
        let value: DuperValue = serialize_pyany(obj)?;
        if let Some(indent) = indent {
            Ok(PrettyPrinter::new(indent).pretty_print(value))
        } else {
            Ok(Serializer::new().serialize(value))
        }
    }

    /// Serialize obj as a Duper value formatted stream to fp (a file-like object).
    #[pyfn(m)]
    #[pyo3(signature = (obj, fp, *, indent=None))]
    fn dump<'py>(
        obj: Bound<'py, PyAny>,
        fp: Bound<'py, PyAny>,
        indent: Option<usize>,
    ) -> PyResult<()> {
        let value: DuperValue = serialize_pyany(obj)?;
        if let Some(indent) = indent {
            fp.call_method1("write", (PrettyPrinter::new(indent).pretty_print(value),))?;
        } else {
            fp.call_method1("write", (Serializer::new().serialize(value),))?;
        }
        Ok(())
    }

    /// Deserialize s (a str instance containing a Duper object or array) to a Python object.
    #[pyfn(m)]
    #[pyo3(signature = (s, *, parse_any=false))]
    fn loads<'py>(py: Python<'py>, s: &str, parse_any: bool) -> PyResult<Bound<'py, PyAny>> {
        let value = match parse_any {
            true => DuperParser::parse_duper_value(&s),
            false => DuperParser::parse_duper(&s),
        }
        .map_err(|err| PyErr::new::<PyValueError, String>(err.to_string()))?;
        value.accept(&mut Visitor { py })
    }

    /// Deserialize fp (a file-like object containing a Duper object or array) to a Python object.
    #[pyfn(m)]
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
            false => DuperParser::parse_duper(s),
        }
        .map_err(|err| PyErr::new::<PyValueError, String>(err.to_string()))?;
        value.accept(&mut Visitor { py })
    }

    Ok(())
}
