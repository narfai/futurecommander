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
    collections::{
        BTreeMap
    },
    path::{ PathBuf, Path, MAIN_SEPARATOR },
    ops::{ Add, Sub }
};

use crate::{
    Kind,
    errors::RepresentationError,
    VirtualChildren,
    VirtualPath
};

#[derive(Debug, Clone, Default)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<PathBuf, VirtualChildren>,
    pub detached: Vec<VirtualPath>
}

impl VirtualDelta {
    pub fn attach_virtual(&mut self, virtual_path: &VirtualPath) -> Result<(), RepresentationError>{
        self.attach(
            virtual_path.as_identity(),
            virtual_path.as_source(),
            virtual_path.to_kind()
        )
    }

    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, kind: Kind) -> Result<(), RepresentationError> {
       if self.get(identity)?.is_some() { Err(RepresentationError::AlreadyExists(identity.to_path_buf())) }
       else {
            let parent = Self::get_parent_or_root(identity);

            if !self.hierarchy.contains_key(parent.as_path()) {
                // https://github.com/rust-lang/rust-clippy/issues/5595
                #[allow(clippy::redundant_clone)]
                self.hierarchy.insert(parent.to_path_buf(), VirtualChildren::default());
            }

            if self.is_file(parent.as_path())? {
                return Err(RepresentationError::VirtualParentIsAFile(identity.to_path_buf()));
            }

            if identity != Self::root_identity().as_path() {
                self.hierarchy
                    .get_mut(parent.as_path())
                    .unwrap() //Assumed
                    .insert(
                        VirtualPath::from_path(identity)?
                            .with_source(source)
                            .with_kind(kind)
                    );
            }

            Ok(())
        }
    }

    pub fn detach(&mut self, identity: &Path) -> Result<(), RepresentationError> {
        if self.get(identity)?.is_some() {
            let parent = Self::get_parent_or_root(identity);

            self.hierarchy.get_mut(&parent)
                .unwrap()//TODO Assumed ? self.get has not the same behavior as hierarchy.get_mut
                .remove(&VirtualPath::from_path(identity)?);


            if self.is_directory_empty(parent.as_path()) {
                self.hierarchy.remove(&parent);
            }

            if self.hierarchy.contains_key(&identity.to_path_buf()) {
                self.hierarchy.remove(identity);
            }
            Ok(())
        } else { Err(RepresentationError::DoesNotExists(identity.to_path_buf())) }
    }

    pub fn is_directory(&self, identity: &Path) -> Result<bool, RepresentationError> {
        if identity == Self::root_identity().as_path() {
            return Ok(true);
        }

        match self.get(identity)? {
            Some(virtual_identity) => Ok(virtual_identity.kind == Kind::Directory),
            None => Ok(false) //Do not exists
        }
    }

    pub fn is_file(&self, identity: &Path) -> Result<bool, RepresentationError> {
        if identity == Self::root_identity().as_path() {
            return Ok(false);
        }

        match self.get(identity)? {
            Some(virtual_identity) => Ok(virtual_identity.kind == Kind::File),
            None => Ok(false) //Do not exists
        }
    }

    pub fn children(&self, parent: &Path) -> Option<&VirtualChildren> {
        match self.hierarchy.get(parent) {
            Some(children) => Some(&children),
            None => None //No key parent
        }
    }

    pub fn get(&self, identity: &Path) -> Result<Option<&VirtualPath>, RepresentationError> {
        match self.hierarchy.get(Self::get_parent_or_root(identity).as_path()) {
            Some(children) => {
                match children.get(&VirtualPath::from_path(identity)?) {
                    Some(child) => Ok(Some(&child)),
                    None => Ok(None) //No matching child
                }
            }
            None => Ok(None) //No key parent
        }
    }

    pub fn is_directory_empty(&self, identity: &Path) -> bool {
        match self.children(identity) {
            Some(children) => children.is_empty(),
            None => true
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hierarchy.len() == 0
    }

    pub fn resolve(&self, path: &Path) -> Result<Option<PathBuf>, RepresentationError> {
        match self.first_virtual_ancestor(path)? {
            Some((depth, ancestor)) =>
                match ancestor.to_source() {
                    Some(source) =>
                        Ok(
                            Some(
                                source.join(
                                    path.strip_prefix(
                                        Self::remove_nth_parents(path, depth)
                                    ).unwrap()//Assumed
                                )
                            )
                        ),
                    None => Ok(None) //Has no source
                }
            None => Ok(None) //Is not virtual
        }
    }

    pub fn remove_nth_parents(path: &Path, depth: usize) -> PathBuf {
        for (index, ancestor) in path.ancestors().enumerate() {
            if index == depth {
                return ancestor.to_path_buf();
            }
        }
        path.to_path_buf()
    }

    pub fn first_virtual_ancestor(&self, path: &Path) -> Result<Option<(usize, VirtualPath)>, RepresentationError>{
        for (index, ancestor) in path.ancestors().enumerate() {
            if let Some(virtual_identity) = self.get(ancestor)? {
                return Ok(Some((index, virtual_identity.clone())))
            }
        }
        Ok(None)
    }

    pub fn is_virtual(&self, path: &Path) -> Result<bool, RepresentationError> {
        Ok(self.first_virtual_ancestor(path)?.is_some())
    }

    pub fn root_identity() -> PathBuf {
        PathBuf::from(MAIN_SEPARATOR.to_string())
    }

    pub fn get_parent_or_root(identity: &Path) -> PathBuf {
        match identity.parent() {
            Some(parent) => parent.to_path_buf(),
            None => Self::root_identity()
        }
    }
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = Result<VirtualDelta, RepresentationError>;

    fn add(self, right_delta: &'b VirtualDelta) -> Result<VirtualDelta, RepresentationError> {
        let mut result = self.clone();
        for children in right_delta.hierarchy.values() {
            for child in children.iter() {
                if right_delta.get(child.as_identity())?.is_some() {
                    if result.get(child.as_identity())?.is_some() {
                        result.detach(child.as_identity())?;
                    }

                    result.attach(
                        child.as_identity(),
                        child.as_source(),
                        child.to_kind()
                    )?;
                }
            }
        }
        Ok(result)
    }
}

impl <'a, 'b> Sub<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = Result<VirtualDelta, RepresentationError>;

    fn sub(self, right_delta: &'b VirtualDelta) -> Result<VirtualDelta, RepresentationError> {
        let mut result = self.clone();
        for children in right_delta.hierarchy.values() {
            for child in children.iter() {
                if result.get(child.as_identity())?.is_some() {
                    result.detach(child.as_identity())?;
                }
            }
        }
        Ok(result)
    }
}
