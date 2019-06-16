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
    collections::HashMap
};

use crate::{
    context::{
        ContextError,
        ContextType
    }
};


#[derive(Default)]
pub struct ContextContainer {
    values: HashMap<String, Box<dyn ContextType>>,
}

impl ContextContainer {
    pub fn get(&self, key: &str) -> Result<Box<dyn ContextType>, ContextError> {
        if let Some(context) = self.values.get(key) {
            Ok(context.box_clone())
        } else {
            Err(ContextError::KeyDoesNotExists(key.to_string()))
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
