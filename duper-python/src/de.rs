use duper::{DuperTemporal, visitor::DuperVisitor};
use pyo3::{prelude::*, types::*};

use crate::Duper;

#[derive(Clone)]
pub(crate) struct Visitor<'py> {
    pub(crate) py: Python<'py>,
}

pub(crate) struct VisitorValue<'py> {
    pub(crate) value: Bound<'py, PyAny>,
    pub(crate) duper: Option<Bound<'py, Duper>>,
}

impl<'py> DuperVisitor for Visitor<'py> {
    type Value = PyResult<VisitorValue<'py>>;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        object: &duper::DuperObject<'a>,
    ) -> Self::Value {
        let seq = object
            .iter()
            .map(|(key, value)| {
                let value = value.accept(self)?;
                Ok((key.as_ref(), value))
            })
            .collect::<PyResult<Vec<_>>>()?;
        let model_fields = PyDict::new(self.py);
        let instance_values = PyDict::new(self.py);
        for (key, value) in seq.into_iter() {
            let ty = match &value.duper {
                Some(duper) => self
                    .py
                    .import("typing")?
                    .getattr("Annotated")?
                    .get_item((value.value.get_type(), duper))?,
                None => value.value.get_type().into_any(),
            };
            model_fields.set_item(key, ty)?;
            instance_values.set_item(key, value.value)?;
        }
        let pydantic: Bound<'py, PyModule> = self.py.import("duper.pydantic")?;
        let model = pydantic.getattr("create_model")?.call(
            (identifier
                .map(|identifier| identifier.as_ref())
                .unwrap_or("Object"),),
            Some(&model_fields),
        )?;
        Ok(VisitorValue {
            value: model.call((), Some(&instance_values))?,
            duper: Some(
                match identifier {
                    Some(identifier) => Duper::from_identifier(identifier)?,
                    None => Duper { identifier: None },
                }
                .into_pyobject(self.py)?,
            ),
        })
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        array: &duper::DuperArray<'a>,
    ) -> Self::Value {
        let vec: PyResult<Vec<_>> = array
            .iter()
            .map(|value| Ok(value.accept(self)?.value))
            .collect();
        Ok(VisitorValue {
            value: PyList::new(self.py, vec?).map(|value| value.into_any())?,
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        tuple: &duper::DuperTuple<'a>,
    ) -> Self::Value {
        let vec: PyResult<Vec<_>> = tuple
            .iter()
            .map(|value| Ok(value.accept(self)?.value))
            .collect();
        Ok(VisitorValue {
            value: PyTuple::new(self.py, vec?).map(|value| value.into_any())?,
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        string: &duper::DuperString<'a>,
    ) -> Self::Value {
        Ok(VisitorValue {
            value: PyString::new(self.py, &string.clone().into_inner()).into_any(),
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        bytes: &duper::DuperBytes<'a>,
    ) -> Self::Value {
        Ok(VisitorValue {
            value: PyBytes::new(self.py, &bytes.clone().into_inner()).into_any(),
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_temporal<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        temporal: &DuperTemporal<'a>,
    ) -> Self::Value {
        Ok(VisitorValue {
            value: match temporal {
                DuperTemporal::Instant(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("Instant")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::ZonedDateTime(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("ZonedDateTime")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::PlainDate(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("PlainDate")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::PlainTime(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("PlainTime")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::PlainDateTime(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("PlainDateTime")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::PlainYearMonth(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("PlainYearMonth")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::PlainMonthDay(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("PlainMonthDay")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::Duration(inner) => self
                    .py
                    .import("duper.temporal")?
                    .getattr("Duration")?
                    .getattr("from_string")?
                    .call1((PyString::new(self.py, inner.as_ref()),))?,
                DuperTemporal::Unspecified(inner) => {
                    PyString::new(self.py, inner.as_ref()).into_any()
                }
            },
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        Ok(VisitorValue {
            value: PyInt::new(self.py, integer).into_any(),
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value {
        Ok(VisitorValue {
            value: PyFloat::new(self.py, float).into_any(),
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        Ok(VisitorValue {
            value: PyBool::new(self.py, boolean).to_owned().into_any(),
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }

    fn visit_null<'a>(&mut self, identifier: Option<&duper::DuperIdentifier<'a>>) -> Self::Value {
        Ok(VisitorValue {
            value: self.py.None().into_bound(self.py).into_any(),
            duper: identifier
                .map(|identifier| Duper::from_identifier(identifier)?.into_pyobject(self.py))
                .transpose()?,
        })
    }
}
