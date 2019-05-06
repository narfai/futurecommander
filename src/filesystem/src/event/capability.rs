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
    fmt::Debug,
    collections::HashMap,
    ops::{ Add, Sub }
};

use serde::{ Serialize, Deserialize };

use crate::{
    errors::{ DomainError }
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Capability {
    Merge,
    Overwrite,
    Recursive
}

impl Eq for Capability {}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Capabilities {
    merge: bool,
    overwrite: bool,
    recursive: bool
}

impl Capabilities {
    fn merge(&self) -> bool {
        self.merge
    }

    fn overwrite(&self) -> bool {
        self.overwrite
    }

    fn recursive(&self) -> bool {
        self.recursive
    }

    fn authorize(&self, capability: Capability) -> bool {
        match capability {
            Capability::Merge => self.merge(),
            Capability::Recursive => self.recursive(),
            Capability::Overwrite => self.overwrite()
        }
    }
}

impl Add<Capability> for Capabilities {
    type Output = Capabilities;

    fn add(self, right_cap: Capability) -> Capabilities {
        Capabilities {
            merge: self.merge() || right_cap == Capability::Merge,
            overwrite: self.overwrite() || right_cap == Capability::Overwrite,
            recursive: self.recursive() || right_cap == Capability::Recursive,
        }
    }
}

impl Sub<Capability> for Capabilities {
    type Output = Capabilities;

    fn sub(self, right_cap: Capability) -> Capabilities {
        Capabilities {
            merge: self.merge() && right_cap != Capability::Merge,
            overwrite: self.overwrite() && right_cap != Capability::Overwrite,
            recursive: self.recursive() && right_cap != Capability::Recursive,
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait Guard : Debug {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError>;
}

/**
A guard should be able to :
* throw an error
* unblocking skip behavior
* allow a behavior
* persist its own data
*/

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
