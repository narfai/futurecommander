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

use std::path::{ Path };
use crate::{ VirtualDelta, VirtualChildren, VirtualPath, VirtualKind, VfsError };

#[derive(Debug)]
pub struct VirtualFileSystem {
    pub add: VirtualDelta,
    pub sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            add: VirtualDelta::new(),
            sub: VirtualDelta::new()
        }
    }

    pub fn read_dir(&self, path: &Path) -> Result<VirtualChildren, VfsError> {
        if !self.exists(path) {
            return Err(VfsError::VirtuallyDoesNotExists(path.to_path_buf()));
        }

        let resolved_path = self.get_virtual_state().resolve(path);

        let mut real_children = match VirtualChildren::from_file_system(resolved_path.as_path(), None) {
            Ok(virtual_children) => virtual_children,
            Err(error) => return Err(VfsError::from(error))
        };

        if let Some(to_del_children) = self.sub.children(resolved_path.as_path()) {
            real_children = &real_children - &to_del_children;
        }

        if let Some(to_add_children) = self.add.children(resolved_path.as_path()) {
            real_children = &real_children + &to_add_children;
        }

        Ok(real_children)
    }

    pub fn copy(&mut self, source: &Path, destination: &Path) -> Result<VirtualPath, VfsError>{
        let virtual_source = match self.get(source) {
            Ok(virtual_source) => virtual_source,
            Err(error) => return Err(error)
        };

        match self.get(destination) {
            Ok(virtual_destination) => match virtual_destination.to_kind() {
                VirtualKind::Directory => {},
                _ => return Err(VfsError::IsNotADirectory(virtual_destination.to_identity()))
            },
            Err(error) => return Err(error)
        }

        let new_identity = &VirtualPath::from_path(source)
            .with_new_parent(destination)
            .with_source(Some(virtual_source.as_referent_source()))
            .with_kind(virtual_source.to_kind());

        if self.exists(new_identity.as_identity()) {
            return Err(VfsError::AlreadyExists(new_identity.to_identity()))
        }

        self.add.attach_virtual(new_identity);

        if self.sub.exists(new_identity.as_identity()) {
            self.sub.detach(new_identity.as_identity())
        }

        Ok(new_identity.clone())
     }

    pub fn remove(&mut self, path: &Path) -> Result<VirtualPath, VfsError> {
        let identity = match self.add.get(path) {
            Some(identity) => {
                let cloned = identity.clone();
                self.add.detach(cloned.as_identity());
                cloned
            },
            None => match path.exists() {
                true => VirtualPath::from_path(path).with_kind(match path.is_dir() {
                    true => VirtualKind::Directory,
                    false => VirtualKind::File
                }),
                false => return Err(VfsError::DoesNotExists(path.to_path_buf()))
            }
        };

        return match self.sub.get(path) {
            Some(_) => Err(VfsError::DoesNotExists(path.to_path_buf())),
            None => {
                self.sub.attach_virtual(&identity);
                Ok(identity.clone())
            }
        }
    }

    pub fn mkdir(&mut self, path: &Path) -> Result<(), VfsError>{
        match self.exists(path) {
            true => Err(VfsError::AlreadyExists(path.to_path_buf())),
            false => {
                self.add.attach(path, None, true);
                Ok(())
            }
        }
    }

    pub fn touch(&mut self, path: &Path) -> Result<(), VfsError>{
        match self.exists(path) {
            true => Err(VfsError::AlreadyExists(path.to_path_buf())),
            false => {
                self.add.attach(path, None, false);
                Ok(())
            }
        }
    }

    pub fn mv(&mut self, source: &Path, destination: &Path) -> Result<VirtualPath, VfsError>{
        let result = self.copy(source, destination);
        match self.remove(source) {
            Ok(_) => result,
            Err(error) => Err(error)
        }
    }

    pub fn get(&self, path: &Path) -> Result<VirtualPath, VfsError> {
        let state = self.get_virtual_state();
        match state.first_virtual_ancestor(path) {
            Some(_ancestor) => match state.get(path) {
                Some(virtual_identity) => Ok(virtual_identity.clone()),
                None => {
                    let resolved = state.resolve(path);
                    Ok(VirtualPath::from_path(path)
                        .with_kind(match resolved.is_dir() {
                            true => VirtualKind::Directory,
                            false => VirtualKind::File
                        })
                        .with_source(Some(resolved.as_path()))
                    )
                }
            },
            None => match path.exists() {
                true => Ok(VirtualPath::from_path(path)
                    .with_kind(
                        match path.is_dir() {
                            true => VirtualKind::Directory,
                            false => VirtualKind::File
                        }
                    )),
                false => Err(VfsError::DoesNotExists(path.to_path_buf()))
            }
        }
    }

    pub fn exists(&self, path: &Path) -> bool {
        !self.sub.exists(path) && (self.add.exists(path) || path.exists())
    }

    pub fn exists_virtually(&self, path: &Path) -> bool {
        self.get_virtual_state().exists(path)
    }

    pub fn is_directory_virtually(&self, path: &Path) -> Option<bool> {
        self.get_virtual_state().is_directory(path)
    }

    pub fn get_add_state(&self) -> VirtualDelta {
        self.add.clone()
    }

    pub fn get_sub_state(&self) -> VirtualDelta {
        self.sub.clone()
    }

    pub fn get_virtual_state(&self) -> VirtualDelta {
        &self.add - &self.sub
    }
}
