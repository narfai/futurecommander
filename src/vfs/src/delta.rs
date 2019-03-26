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

use crate::{ VirtualPath, VirtualKind, VfsError };
use crate::VirtualChildren;
use std::collections::{ BTreeMap };

use std::path::{ PathBuf, Path };
use std::ops::{ Add, Sub };


//impl PartialEq for VirtualDelta {
//    fn eq(&self, other: &VirtualDelta) -> bool {}
//}

#[derive(Debug, Clone)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<PathBuf, VirtualChildren>
}

impl VirtualDelta {
    pub fn new() -> VirtualDelta {
        VirtualDelta {
            hierarchy: BTreeMap::new()
        }
    }

    pub fn attach_virtual(&mut self, virtual_path: &VirtualPath) -> Result<(), VfsError>{
        self.attach(
            virtual_path.as_identity(),
            virtual_path.as_source(),
            virtual_path.to_kind()
        )
    }

    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, kind: VirtualKind) -> Result<(), VfsError> {
       match self.get(identity)?.is_some() {
            true => Err(VfsError::AlreadyExists(identity.to_path_buf())),
            false => {
                let parent = VirtualPath::get_parent_or_root(identity);

                if !self.hierarchy.contains_key(parent.as_path()) {
                    self.hierarchy.insert(parent.to_path_buf(), VirtualChildren::new());
                }

                if self.is_file(parent.as_path())? {
                    return Err(VfsError::VirtualParentIsAFile(identity.to_path_buf()));
                }

                if identity != VirtualPath::root_identity().as_path() {
                    self.hierarchy
                        .get_mut(parent.as_path())
                        .unwrap()
                        .insert(
                            VirtualPath::from_path(identity)?
                                .with_source(source)
                                .with_kind(kind)
                        );
                }

                Ok(())

            }
        }
    }

    pub fn detach(&mut self, identity: &Path) -> Result<(), VfsError> {
        match self.get(identity)?.is_some() {
            true => {
                let parent = VirtualPath::get_parent_or_root(identity);

                self.hierarchy.get_mut(&parent)
                    .unwrap()
                    .remove(&VirtualPath::from_path(identity)?);

                match self.is_directory_empty(parent.as_path()) {
                    true => { self.hierarchy.remove(&parent); },
                    false => {}
                }

                if self.hierarchy.contains_key(&identity.to_path_buf()) {
                    self.hierarchy.remove(identity);
                }
                Ok(())
            },
            false => Err(VfsError::DoesNotExists(identity.to_path_buf()))
        }
    }

    pub fn is_directory(&self, identity: &Path) -> Result<bool, VfsError> {
        if identity == VirtualPath::root_identity().as_path() {
            return Ok(true);
        }

        match self.get(identity)? {
            Some(virtual_identity) => Ok(virtual_identity.kind == VirtualKind::Directory),
            None => Ok(false) //Do not exists
        }
    }

    pub fn is_file(&self, identity: &Path) -> Result<bool, VfsError> {
        if identity == VirtualPath::root_identity().as_path() {
            return Ok(false);
        }

        match self.get(identity)? {
            Some(virtual_identity) => Ok(virtual_identity.kind == VirtualKind::File),
            None => Ok(false) //Do not exists
        }
    }

    pub fn children(&self, parent: &Path) -> Option<&VirtualChildren> {
        match self.hierarchy.get(parent) {
            Some(children) => Some(&children),
            None => None //No key parent
        }
    }

    pub fn children_owned(&self, parent: &Path) -> Option<VirtualChildren> {
        match self.hierarchy.get(parent) {
            Some(children) => Some(children.clone()),
            None => None //Do not exists
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
        let dir = VirtualPath::get_parent_or_root(identity);
        match self.children(dir.as_path()) {
            Some(children) => children.len() == 0,
            None => false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hierarchy.len() == 0
    }

    //TODO unused yet but seems useful at least for debugging
    pub fn sub_delta(&self, identity: &Path) -> Result<Option<VirtualDelta>, VfsError> {
        match self.get(identity)?.is_some() {
            true => {
                let mut collection = VirtualChildren::new();
                self.walk(&mut collection, identity)?;
                Ok(Some(collection.into_delta()?))
            },
            false => Ok(None)
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
                                    ).unwrap()
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
        return path.to_path_buf();
    }

    pub fn first_virtual_ancestor(&self, path: &Path) -> Result<Option<(usize, VirtualPath)>, VfsError>{
        for (index, ancestor) in path.ancestors().enumerate() {
            match self.get(ancestor)? {
                Some(virtual_identity) => return Ok(Some((index, virtual_identity.clone()))),
                None => {}
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
    type Output = VirtualDelta;

    fn add(self, right_delta: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_delta.hierarchy {
            for child in children.iter() {
                if right_delta.get(child.as_identity()).unwrap().is_some() {
                    if result.get(child.as_identity()).unwrap().is_some() {
                        result.detach(child.as_identity()).unwrap();
                    }

                    result.attach(
                        child.as_identity(),
                        child.as_source(),
                        child.to_kind()
                    ).unwrap();
                }
            }
        }
        result
    }
}

impl <'a, 'b> Sub<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn sub(self, right_delta: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_delta.hierarchy {
            for child in children.iter() {
                if result.get(child.as_identity()).unwrap().is_some() {
                    result.detach(child.as_identity()).unwrap();
                }
            }
        }
        result
    }
}
