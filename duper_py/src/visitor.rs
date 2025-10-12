use duper::visitor::DuperVisitor;
use pyo3::{
    Bound, PyAny, PyResult, Python,
    types::{IntoPyDict, PyBool, PyBytes, PyFloat, PyInt, PyList, PyString, PyTuple},
};

#[derive(Clone)]
pub(crate) struct Visitor<'py> {
    pub(crate) py: Python<'py>,
}

impl<'py> DuperVisitor for Visitor<'py> {
    type Value = PyResult<Bound<'py, PyAny>>;

    fn visit_object<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        object: &duper::DuperObject<'a>,
    ) -> Self::Value {
        let seq: PyResult<Vec<_>> = object
            .iter()
            .map(|(key, value)| value.accept(self).map(|value| (key.as_ref(), value)))
            .collect();
        Ok(seq?.into_py_dict(self.py).unwrap().into_any())
    }

    fn visit_array<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        array: &duper::DuperArray<'a>,
    ) -> Self::Value {
        let vec: PyResult<Vec<_>> = array.iter().map(|value| value.accept(self)).collect();
        PyList::new(self.py, vec?).map(|value| value.into_any())
    }

    fn visit_tuple<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        tuple: &duper::DuperTuple<'a>,
    ) -> Self::Value {
        let vec: PyResult<Vec<_>> = tuple.iter().map(|value| value.accept(self)).collect();
        PyTuple::new(self.py, vec?).map(|value| value.into_any())
    }

    fn visit_string<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        string: &duper::DuperString<'a>,
    ) -> Self::Value {
        Ok(PyString::new(self.py, &string.clone().into_inner()).into_any())
    }

    fn visit_bytes<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        bytes: &duper::DuperBytes<'a>,
    ) -> Self::Value {
        Ok(PyBytes::new(self.py, &bytes.clone().into_inner()).into_any())
    }

    fn visit_integer<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        Ok(PyInt::new(self.py, integer).into_any())
    }

    fn visit_float<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value {
        Ok(PyFloat::new(self.py, float).into_any())
    }

    fn visit_boolean<'a>(
        &mut self,
        _identifier: Option<&duper::DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        Ok(PyBool::new(self.py, boolean).to_owned().into_any())
    }

    fn visit_null<'a>(&mut self, _identifier: Option<&duper::DuperIdentifier<'a>>) -> Self::Value {
        Ok(self.py.None().into_bound(self.py).into_any())
    }
}
