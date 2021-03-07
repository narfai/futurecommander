// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::{ Path };
use crate::{
    DomainError,
    Capability,
    Capabilities
};
use super::{ Guard };

pub struct PresetGuard<G: Guard> {
    inner: G,
    capabilities: Capabilities
}

impl <G: Guard>PresetGuard<G> {
    pub fn new(inner: G, capabilities: Capabilities) -> Self {
        PresetGuard {
            inner,
            capabilities
        }
    }
}

impl <G: Guard>Guard for PresetGuard<G> {
    fn authorize(&mut self, capability: Capability, target: &Path) -> Result<bool, DomainError> {
        Ok(self.capabilities.authorize(capability) || self.inner.authorize(capability, target)?)
    }
}
