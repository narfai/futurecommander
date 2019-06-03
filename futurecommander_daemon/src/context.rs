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

use std::{
    collections::{ HashMap }
};

use crate::{
    DaemonError
};

pub trait ContextType {
    fn to_bool(&self) -> Result<bool, DaemonError>;

    fn to_string(&self) -> Result<String, DaemonError>;

    fn box_clone(&self) -> Box<dyn ContextType>;
}

#[derive(Clone)]
pub struct ContextString {
    inner: String
}

impl From<String> for ContextString {
    fn from(s : String) -> ContextString {
        ContextString {
            inner: s
        }
    }
}

impl ContextType for ContextString {
    fn to_bool(&self) -> Result<bool, DaemonError> {
        if self.inner == "1" {
            Ok(true)
        } else if self.inner == "0" {
            Ok(false)
        } else {
            Err(DaemonError::ContextCannotCast("String".to_string(), "bool".to_string()))
        }
    }

    fn to_string(&self) -> Result<String, DaemonError> {
        Ok(self.inner.clone())
    }

    fn box_clone(&self) -> Box<dyn ContextType> {
        Box::new(self.clone())
    }
}

#[derive(Default)]
pub struct Context {
    values: HashMap<String, Box<dyn ContextType>>,
}

impl Context {
    pub fn get(&self, key: &str) -> Result<Box<dyn ContextType>, DaemonError> {
        if let Some(context) = self.values.get(key) {
            Ok(context.box_clone())
        } else {
            Err(DaemonError::ContextKeyDoesNotExists(key.to_string()))
        }
    }

    pub fn set(&mut self, key: &str, value: Box<dyn ContextType>) {
        self.values.insert(key.to_string(), value);
    }

    pub fn debug_keys(&self) -> Vec<String> {
        let mut debug = Vec::new();
        for v in self.values.keys() {
            debug.push(v.clone());
        }
        debug
    }
}
