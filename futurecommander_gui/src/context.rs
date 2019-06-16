/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

use wasm_bindgen::{ prelude::* };

use futurecommander_protocol::{
    context::{
        ContextType,
        ContextError,
        ContextContainer
    }
};

#[derive(Clone)]
struct ContextValueWrapper(pub JsValue, pub String);

impl ContextType for ContextValueWrapper {
    fn to_bool(&self) -> Result<bool, ContextError> {
        if self.0.is_null() {
            return Err(ContextError::KeyDoesNotExists(self.1.clone()));
        }

        if let Some(b) = self.0.as_bool() {
            Ok(b)
        } else {
            Err(ContextError::CannotCast("JsValue".to_string(), "bool".to_string()))
        }
    }

    fn to_string(&self) -> Result<String, ContextError> {
        if self.0.is_null() {
            return Err(ContextError::KeyDoesNotExists(self.1.clone()))
        }

        if let Some(s) = self.0.as_string() {
            Ok(s)
        } else {
            Err(ContextError::CannotCast("JsValue".to_string(), "string".to_string()))
        }
    }

    fn box_clone(&self) -> Box<dyn ContextType> {
        Box::new(self.clone())
    }
}

#[wasm_bindgen]
pub struct RustMessageContext {
    #[wasm_bindgen(skip)]
    pub header: String,

    #[wasm_bindgen(skip)]
    pub inner: ContextContainer
}

#[wasm_bindgen]
impl RustMessageContext {
    #[wasm_bindgen(constructor)]
    pub fn new(header: &str) -> RustMessageContext {
        RustMessageContext {
            header: header.to_string(),
            inner: ContextContainer::default()
        }
    }

    pub fn set(&mut self, key: &str, value: JsValue) {
        self.inner.set(key, Box::new(ContextValueWrapper(value, key.to_string())));
    }
}
