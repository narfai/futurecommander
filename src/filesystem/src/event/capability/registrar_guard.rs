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
    path::{ Path, PathBuf },
    collections::HashMap
};

use serde::{ Serialize, Deserialize };

use crate::{
    DomainError,
    capability::{
        Capability,
        Guard,
        Capabilities,
        ZealedGuard
    }
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrarGuard {
    inner: Box<Guard>,
    registry: HashMap<PathBuf, Capabilities>
}

impl Default for RegistrarGuard {
    fn default() -> Self {
        Self::from(Box::new(ZealedGuard))
    }
}

impl RegistrarGuard {
    pub fn from(guard: Box<Guard>) -> Self {
        RegistrarGuard {
            inner: guard,
            registry: HashMap::new()
        }
    }
}

#[typetag::serde]
impl Guard for RegistrarGuard {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError> {
        let capabilities = match self.registry.get(&target.to_path_buf()) {
            Some(capabilities) => *capabilities,
            None => (Capabilities::default())
        };

        if capabilities.authorize(capability) || self.inner.authorize(capability, default, target)? {
            self.registry.insert(target.to_path_buf(), capabilities + capability);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
