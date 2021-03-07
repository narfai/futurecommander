// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::Path;
use crate::{
    DomainError,
    Capability
};
use super::{ Guard };

#[derive(Debug)]
pub struct ZealousGuard;

impl Guard for ZealousGuard {
    fn authorize(&mut self, capability: Capability, target: &Path) -> Result<bool, DomainError> {
        match capability {
            Capability::Merge => Err(
                DomainError::MergeNotAllowed(target.to_path_buf())
            ),
            Capability::Overwrite => Err(
                DomainError::OverwriteNotAllowed(target.to_path_buf())
            ),
            Capability::Recursive => Err(
                DomainError::RecursiveNotAllowed(target.to_path_buf())
            )
        }
    }
}
