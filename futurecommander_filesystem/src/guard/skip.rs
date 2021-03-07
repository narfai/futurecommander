// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::{ Path };
use crate::{
    DomainError,
    Capability
};

use super::{ Guard };

#[derive(Debug)]
pub struct SkipGuard;

impl Guard for SkipGuard {
    fn authorize(&mut self, _capability: Capability, _target: &Path) -> Result<bool, DomainError> {
        Ok(false)
    }
}
