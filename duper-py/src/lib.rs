use duper::{DuperParser, DuperValue, PrettyPrinter, Serializer};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{de::Visitor, ser::serialize_pyany};

mod de;
mod ser;

#[pymodule(name = "_duper")]
fn duper_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(signature = (obj, *, indent=None, strip_identifiers=false))]
    fn dumps<'py>(
        obj: Bound<'py, PyAny>,
        indent: Option<usize>,
        strip_identifiers: bool,
    ) -> PyResult<String> {
        let value: DuperValue = serialize_pyany(obj)?;
        if let Some(indent) = indent {
            Ok(PrettyPrinter::new(strip_identifiers, indent).pretty_print(value))
        } else {
            Ok(Serializer::new(strip_identifiers).serialize(value))
        }
    }

    #[pyfn(m)]
    #[pyo3(signature = (obj, fp, *, indent=None, strip_identifiers=false))]
    fn dump<'py>(
        obj: Bound<'py, PyAny>,
        fp: Bound<'py, PyAny>,
        indent: Option<usize>,
        strip_identifiers: bool,
    ) -> PyResult<()> {
        let value: DuperValue = serialize_pyany(obj)?;
        fp.call_method1(
            "write",
            if let Some(indent) = indent {
                (PrettyPrinter::new(strip_identifiers, indent).pretty_print(value),)
            } else {
                (Serializer::new(strip_identifiers).serialize(value),)
            },
        )?;
        Ok(())
    }

    #[pyfn(m)]
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
        value.accept(&mut Visitor { py })
    }

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
            false => DuperParser::parse_duper_trunk(s),
        }
        .map_err(|err| {
            PyErr::new::<PyValueError, String>(format!(
                "{:?}",
                miette::Error::new(err.into_miette())
            ))
        })?;
        value.accept(&mut Visitor { py })
    }

    Ok(())
}
