use duper::{DuperParser, DuperValue, PrettyPrinter, Serializer};
use pyo3::{exceptions::PyValueError, prelude::*};
use serde_pyobject::from_pyobject;

use crate::visitor::Visitor;

mod visitor;

/// Utilities for converting to and from Python types into Duper.
#[pymodule(name = "duper")]
fn duper_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    /// Serialize obj as a Duper value formatted str.
    #[pyfn(m)]
    #[pyo3(signature = (obj, *, indent=None))]
    fn dumps<'py>(obj: Bound<'py, PyAny>, indent: Option<usize>) -> PyResult<String> {
        let value: DuperValue = from_pyobject(obj)?;
        if let Some(indent) = indent {
            Ok(PrettyPrinter::new(indent).pretty_print(value))
        } else {
            Ok(Serializer::new().serialize(value))
        }
    }

    /// Deserialize s (a str instance containing a Duper object or array) to a Python object.
    #[pyfn(m)]
    fn loads<'py>(py: Python<'py>, s: &str) -> PyResult<Bound<'py, PyAny>> {
        let value = DuperParser::parse_duper(s)
            .map_err(|err| PyErr::new::<PyValueError, String>(err.to_string()))?;
        value.accept(&mut Visitor { py })
    }

    Ok(())
}
