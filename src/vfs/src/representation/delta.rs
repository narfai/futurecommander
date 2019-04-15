/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommanderVfs.
 *
 * FutureCommanderVfs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommanderVfs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommanderVfs.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{
    collections::{
        BTreeMap
    },
    path::{ PathBuf, Path },
    ops::{ Add, Sub }
};

use crate::{
    VfsError,
    Kind,
    representation::{
        VirtualChildren,
        VirtualPath
    }
};

#[derive(Debug, Clone, Default)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<PathBuf, VirtualChildren>,
    pub detached: Vec<VirtualPath>
}

impl VirtualDelta {
    pub fn attach_virtual(&mut self, virtual_path: &VirtualPath) -> Result<(), VfsError>{
        self.attach(
            virtual_path.as_identity(),
            virtual_path.as_source(),
            virtual_path.to_kind()
        )
    }

    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, kind: Kind) -> Result<(), VfsError> {
       if self.get(identity)?.is_some() { Err(VfsError::AlreadyExists(identity.to_path_buf())) }
       else {
            let parent = VirtualPath::get_parent_or_root(identity);

            if !self.hierarchy.contains_key(parent.as_path()) {
                self.hierarchy.insert(parent.to_path_buf(), VirtualChildren::default());
            }

            if self.is_file(parent.as_path())? {
                return Err(VfsError::VirtualParentIsAFile(identity.to_path_buf()));
            }

            if identity != VirtualPath::root_identity().as_path() {
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

    pub fn detach(&mut self, identity: &Path) -> Result<(), VfsError> {
        if self.get(identity)?.is_some() {
            let parent = VirtualPath::get_parent_or_root(identity);

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
        } else { Err(VfsError::DoesNotExists(identity.to_path_buf())) }
    }

    pub fn is_directory(&self, identity: &Path) -> Result<bool, VfsError> {
        if identity == VirtualPath::root_identity().as_path() {
            return Ok(true);
        }

        match self.get(identity)? {
            Some(virtual_identity) => Ok(virtual_identity.kind == Kind::Directory),
            None => Ok(false) //Do not exists
        }
    }

    pub fn is_file(&self, identity: &Path) -> Result<bool, VfsError> {
        if identity == VirtualPath::root_identity().as_path() {
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

    //TODO unused but could be usefull
    pub fn walk(&self, collection: &mut VirtualChildren, identity: &Path) -> Result<(), VfsError>{
        match self.get(identity)? {
            Some(virtual_identity) => match self.is_directory(identity) {
                Ok(true)   => { self._walk(collection, virtual_identity); Ok(()) },
                Ok(false)  => Err(VfsError::IsNotADirectory(identity.to_path_buf())),
                Err(error) => Err(error)
            },
            None => Err(VfsError::DoesNotExists(identity.to_path_buf()))
        }
    }

    //TODO unused but could be usefull
    fn _walk(&self, collection: &mut VirtualChildren, virtual_identity: &VirtualPath){
        collection.insert(virtual_identity.clone());
        if let Some(children) = self.children(virtual_identity.as_identity()) {
            for child in children.iter() {
                self._walk(collection, &child);
            }
        };
    }

    pub fn get(&self, identity: &Path) -> Result<Option<&VirtualPath>, VfsError> {
        match self.hierarchy.get(VirtualPath::get_parent_or_root(identity).as_path()) {
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

    //TODO unused yet but seems useful at least for debugging
    pub fn sub_delta(&self, identity: &Path) -> Result<Option<VirtualDelta>, VfsError> {
        if self.get(identity)?.is_some() {
            let mut collection = VirtualChildren::default();
            self.walk(&mut collection, identity)?;
            Ok(Some(collection.into_delta()?))
        } else {
            Ok(None)
        }
    }

    pub fn resolve(&self, path: &Path) -> Result<Option<PathBuf>, VfsError> {
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

    pub fn first_virtual_ancestor(&self, path: &Path) -> Result<Option<(usize, VirtualPath)>, VfsError>{
        for (index, ancestor) in path.ancestors().enumerate() {
            if let Some(virtual_identity) = self.get(ancestor)? {
                return Ok(Some((index, virtual_identity.clone())))
            }
        }
        Ok(None)
    }

    pub fn is_virtual(&self, path: &Path) -> Result<bool, VfsError> {
        match self.first_virtual_ancestor(path)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = Result<VirtualDelta, VfsError>;

    fn add(self, right_delta: &'b VirtualDelta) -> Result<VirtualDelta, VfsError> {
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
    type Output = Result<VirtualDelta, VfsError>;

    fn sub(self, right_delta: &'b VirtualDelta) -> Result<VirtualDelta, VfsError> {
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
