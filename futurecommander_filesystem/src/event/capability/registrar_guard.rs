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
    OperationRegistry,
    DomainError,
    capability::{
        Capability,
        Guard,
        Capabilities,
        ZealousGuard
    }
};

pub struct RegistrarGuard {
    inner: Box<dyn Guard>,
    registry: OperationRegistry
}

/* impl Default for RegistrarGuard {
    fn default() -> Self {
        Self::from(&mut ZealousGuard)
    }
} */

impl RegistrarGuard {
    pub fn new(guard: Box<dyn Guard>, registry: OperationRegistry) -> Self {
        RegistrarGuard {
            inner: guard,
            registry
        }
    }

    pub fn from(guard: Box<dyn Guard>) -> Self {
        RegistrarGuard {
            inner: guard,
            registry: HashMap::new()
        }
    }
}

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

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        event::{
            capability::{
                BlindGuard,
                QuietGuard
            }
        }
    };

    #[test]
    fn registrar_persist_choice_for_target(){
        let mut registrar = RegistrarGuard::from(Box::new(BlindGuard));
        let target = Path::new("/virtual/directory");
        let default = false;

        assert!(registrar.authorize(Capability::Overwrite, default, target).unwrap());
        assert!(registrar.registry[&target.to_path_buf()].overwrite());

        assert!(registrar.authorize(Capability::Merge, default, target).unwrap());
        assert!(registrar.registry[&target.to_path_buf()].merge());

        assert!(registrar.authorize(Capability::Recursive, default, target).unwrap());
        assert!(registrar.registry[&target.to_path_buf()].recursive());
    }

    #[test]
    fn registrar_register_only_authorized(){
        let mut registrar = RegistrarGuard::from(Box::new(QuietGuard));
        let target = Path::new("/virtual/directory");
        let default = false;

        assert!(!registrar.authorize(Capability::Overwrite, default, target).unwrap());
        assert!(registrar.registry.get(&target.to_path_buf()).is_none());

        assert!(!registrar.authorize(Capability::Merge, default, target).unwrap());
        assert!(registrar.registry.get(&target.to_path_buf()).is_none());

        assert!(!registrar.authorize(Capability::Recursive, default, target).unwrap());
        assert!(registrar.registry.get(&target.to_path_buf()).is_none());
    }
}
