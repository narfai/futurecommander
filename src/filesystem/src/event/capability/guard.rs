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
    fmt::{ Debug, Display, Formatter, Result as FmtResult }
};


use serde::{ Serialize, Deserialize };

use crate::{
    DomainError,
    capability::{
        Capability
    }
};

#[typetag::serde(tag = "type")]
pub trait Guard : Debug {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError>;
}

#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct BlindGuard;

#[typetag::serde]
impl Guard for BlindGuard {
    fn authorize(&mut self, capability: Capability, _default: bool, _target: &Path) -> Result<bool, DomainError> {
        match capability {
            Capability::Merge => Ok(true),
            Capability::Overwrite => Ok(true),
            Capability::Recursive => Ok(true)
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuietGuard;

#[typetag::serde]
impl Guard for QuietGuard {
    fn authorize(&mut self, capability: Capability, default: bool, _target: &Path) -> Result<bool, DomainError> {
        match capability {
            Capability::Merge => Ok(default),
            Capability::Overwrite => Ok(default),
            Capability::Recursive => Ok(default)
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZealedGuard;

#[typetag::serde]
impl Guard for ZealedGuard {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError> {
        match capability {
            Capability::Merge => {
                if default {
                    Ok(true)
                } else {
                    Err(
                        DomainError::MergeNotAllowed(
                            target.to_path_buf()
                        )
                    )
                }
            },
            Capability::Overwrite => {
                if default {
                    Ok(true)
                } else {
                    Err(
                        DomainError::OverwriteNotAllowed(
                            target.to_path_buf()
                        )
                    )
                }
            },
            Capability::Recursive => {
                if default {
                    Ok(true)
                } else {
                    Err(
                        DomainError::RecursiveNotAllowed(
                            target.to_path_buf()
                        )
                    )
                }
            }
        }
    }
}
