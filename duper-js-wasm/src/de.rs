use duper::visitor::DuperVisitor;
use js_sys::{Array, Object, Reflect, Symbol};
use wasm_bindgen::prelude::*;

use crate::{SYMBOL_DUPER_IDENTIFIER, SYMBOL_DUPER_TYPE, SYMBOL_DUPER_VALUE};

#[derive(Clone)]
pub(crate) struct Visitor {
    pub(crate) json_safe: bool,
}

impl DuperVisitor for Visitor {
    type Value = Result<JsValue, JsError>;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        object: &duper::DuperObject<'a>,
    ) -> Self::Value {
        let js_object = Object::new();

        for (key, value) in object.iter() {
            let value_result = value.accept(self)?;
            Reflect::set(&js_object, &JsValue::from(key.as_ref()), &value_result).map_err(|e| {
                JsError::new(&format!("Failed to set property {}: {:?}", key.as_ref(), e))
            })?;
        }

        // Mark as object
        let type_symbol = Symbol::for_(SYMBOL_DUPER_TYPE);
        Reflect::set(&js_object, &type_symbol, &JsValue::from_str("object"))
            .map_err(|e| JsError::new(&format!("Failed to set object symbol: {:?}", e)))?;

        if let Some(identifier) = identifier {
            let duper_symbol = Symbol::for_(SYMBOL_DUPER_IDENTIFIER);
            Reflect::set(
                &js_object,
                &duper_symbol,
                &JsValue::from_str(identifier.as_ref()),
            )
            .map_err(|e| JsError::new(&format!("Failed to set Duper identifier: {:?}", e)))?;
        }

        Ok(js_object.into())
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        array: &duper::DuperArray<'a>,
    ) -> Self::Value {
        let js_array = Array::new_with_length(array.len() as u32);

        for (index, value) in array.iter().enumerate() {
            let value_result = value.accept(self)?;
            js_array.set(index as u32, value_result);
        }

        // Mark as array
        let type_symbol = Symbol::for_(SYMBOL_DUPER_TYPE);
        Reflect::set(&js_array, &type_symbol, &JsValue::from_str("array"))
            .map_err(|e| JsError::new(&format!("Failed to set array symbol: {:?}", e)))?;

        if let Some(identifier) = identifier {
            let duper_symbol = Symbol::for_(SYMBOL_DUPER_IDENTIFIER);
            Reflect::set(
                &js_array,
                &duper_symbol,
                &JsValue::from_str(identifier.as_ref()),
            )
            .map_err(|e| JsError::new(&format!("Failed to set Duper identifier: {:?}", e)))?;
        }

        Ok(js_array.into())
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        tuple: &duper::DuperTuple<'a>,
    ) -> Self::Value {
        let js_array = Array::new_with_length(tuple.len() as u32);

        for (index, value) in tuple.iter().enumerate() {
            let value_result = value.accept(self)?;
            js_array.set(index as u32, value_result);
        }

        // Mark as tuple
        let type_symbol = Symbol::for_(SYMBOL_DUPER_TYPE);
        Reflect::set(&js_array, &type_symbol, &JsValue::from_str("tuple"))
            .map_err(|e| JsError::new(&format!("Failed to set tuple symbol: {:?}", e)))?;

        if let Some(identifier) = identifier {
            let duper_symbol = Symbol::for_(SYMBOL_DUPER_IDENTIFIER);
            Reflect::set(
                &js_array,
                &duper_symbol,
                &JsValue::from_str(identifier.as_ref()),
            )
            .map_err(|e| JsError::new(&format!("Failed to set Duper identifier: {:?}", e)))?;
        }

        Ok(js_array.into())
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        string: &duper::DuperString<'a>,
    ) -> Self::Value {
        let value = JsValue::from_str(string.as_ref());
        if self.json_safe {
            Ok(value)
        } else {
            attach_identifier(value, identifier, "string")
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        bytes: &duper::DuperBytes<'a>,
    ) -> Self::Value {
        if self.json_safe {
            Ok(bytes.as_ref().to_vec().into())
        } else {
            let uint8_array = js_sys::Uint8Array::from(bytes.as_ref());
            let value: JsValue = uint8_array.into();
            attach_identifier(value, identifier, "bytes")
        }
    }

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        // TO-DO: Handle big integers
        let value = JsValue::from_f64(integer as f64);
        if self.json_safe {
            Ok(value)
        } else {
            attach_identifier(value, identifier, "integer")
        }
    }

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value {
        let value = JsValue::from_f64(float);
        if self.json_safe {
            Ok(value)
        } else {
            attach_identifier(value, identifier, "float")
        }
    }

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        let value = JsValue::from_bool(boolean);
        if self.json_safe {
            Ok(value)
        } else {
            attach_identifier(value, identifier, "boolean")
        }
    }

    fn visit_null<'a>(&mut self, identifier: Option<&duper::DuperIdentifier<'a>>) -> Self::Value {
        let value = JsValue::NULL;
        if self.json_safe {
            Ok(value)
        } else {
            attach_identifier(value, identifier, "null")
        }
    }
}

fn attach_identifier(
    value: JsValue,
    identifier: Option<&duper::DuperIdentifier>,
    typ: &'static str,
) -> Result<JsValue, JsError> {
    if let Some(identifier) = identifier {
        let boxed = Object::new();
        let value_key = Symbol::for_(SYMBOL_DUPER_VALUE);
        let identifier_symbol = Symbol::for_(SYMBOL_DUPER_IDENTIFIER);
        let type_symbol = Symbol::for_(SYMBOL_DUPER_TYPE);

        Reflect::set(&boxed, &value_key, &value)
            .map_err(|e| JsError::new(&format!("Failed to set Duper value: {:?}", e)))?;
        Reflect::set(
            &boxed,
            &identifier_symbol,
            &JsValue::from_str(identifier.as_ref()),
        )
        .map_err(|e| JsError::new(&format!("Failed to set Duper identifier symbol: {:?}", e)))?;
        Reflect::set(&boxed, &type_symbol, &JsValue::from_str(typ))
            .map_err(|e| JsError::new(&format!("Failed to set Duper type: {:?}", e)))?;

        let to_json = js_sys::Function::new_with_args(
            "",
            &format!(r#"return this[Symbol.for("{SYMBOL_DUPER_VALUE}")];"#),
        );
        Reflect::set(&boxed, &JsValue::from("toJSON"), &to_json)
            .map_err(|e| JsError::new(&format!("Failed to set toJSON function: {:?}", e)))?;

        Ok(boxed.into())
    } else {
        Ok(value)
    }
}
