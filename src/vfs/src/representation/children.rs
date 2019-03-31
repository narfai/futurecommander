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

use std::path::{ Path };
use std::ops::{ Add, Sub };

use std::fs::ReadDir;

use std::collections::hash_set::Iter as HashSetIter;
use std::collections::hash_set::IntoIter as HashSetIntoIter;
use std::collections::{ HashSet };

use crate::{ VfsError };
use crate::representation::{ VirtualPath, VirtualKind, VirtualDelta };

#[derive(Debug, Clone)]
pub struct VirtualChildren {
    set: HashSet<VirtualPath>
}

//TODO refactor with generics
impl VirtualChildren {
    pub fn new() -> VirtualChildren {
        VirtualChildren {
            set: HashSet::new()
        }
    }

    //TODO move it to real_fs mod
    pub fn from_file_system(path: &Path, source: Option<&Path>, parent: Option<&Path>) -> Result<VirtualChildren, VfsError> {
        let mut virtual_children = VirtualChildren::new();
        if !path.exists() {
            return Ok(VirtualChildren::new());
        }

        match path.read_dir() {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(result) => {
                            let result_path = result.path();
                            let mut path = VirtualPath::from_path(result.path().as_path())?
                                .with_source(Some(result_path.as_path()))
                                .with_kind(VirtualKind::from_path(result_path.as_path()));

                            if let Some(source) = source {
                                path = path.with_new_source_parent(source);
                            }

                            if let Some(parent) = parent {
                                path = path.with_new_identity_parent(parent);
                            }

                            virtual_children.insert(path);
                        },
                        Err(error) => return Err(VfsError::from(error))
                    };
                }
                Ok(virtual_children)
            },
            Err(error) => Err(VfsError::from(error))
        }
    }

    pub fn insert(&mut self, virtual_identity: VirtualPath) -> bool {
        self.set.insert(virtual_identity)
    }

    pub fn remove(&mut self, virtual_identity: &VirtualPath) -> bool {
        self.set.remove(virtual_identity)
    }

    pub fn get(&self, virtual_identity: &VirtualPath) -> Option<&VirtualPath> {
        self.set.get(&virtual_identity)
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn contains(&self, virtual_identity: &VirtualPath) -> bool {
        self.set.contains(virtual_identity)
    }

    pub fn iter <'a>(&self) -> VirtualChildrenIterator {
        VirtualChildrenIterator::new(self.set.iter())
    }

    //TODO unused yet but seems useful at least for debugging
    pub fn into_delta(self) -> Result<VirtualDelta, VfsError> {
        let mut delta = VirtualDelta::new();
        for virtual_identity in self.iter() {
            delta.attach_virtual(&virtual_identity)?;
        }
        Ok(delta)
    }
}

impl <'a, 'b> Add<&'b VirtualChildren> for &'a VirtualChildren {
    type Output = VirtualChildren;

    fn add(self, right_collection: &'b VirtualChildren) -> VirtualChildren {
        let mut result = self.clone();
        for virtual_identity in right_collection.iter() {
            result.insert(virtual_identity.clone());
        }
        result
    }
}

impl <'a, 'b> Sub<&'b VirtualChildren> for &'a VirtualChildren {
    type Output = VirtualChildren;

    fn sub(self, right_collection: &'b VirtualChildren) -> VirtualChildren {
        let mut result = self.clone();
        for virtual_identity in right_collection.iter() {
            if result.contains(virtual_identity) {
                result.remove(virtual_identity);
            }
        }
        result
    }
}


#[derive(Debug, Clone)]
pub struct VirtualChildrenIterator<'a> {
    iter: HashSetIter<'a, VirtualPath>
}

impl <'a>VirtualChildrenIterator<'a> {
    pub fn new(iter: HashSetIter<'a, VirtualPath>) -> VirtualChildrenIterator {
        VirtualChildrenIterator {
            iter
        }
    }
}

impl <'a>Iterator for VirtualChildrenIterator<'a> {
    type Item = &'a VirtualPath;

    fn next(&mut self) -> Option<&'a VirtualPath> {
        self.iter.next()
    }
}

impl IntoIterator for VirtualChildren {
    type Item = VirtualPath;
    type IntoIter = HashSetIntoIter<VirtualPath>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}
