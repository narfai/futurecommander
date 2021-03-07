// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN
use std::{
    path::{ Path },
    collections::HashMap
};

use crate::{
    OperationRegistry,
    DomainError,
    capability::{
        Capability,
        Guard,
        Capabilities
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
