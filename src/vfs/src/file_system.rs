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

use std::path::{ Path, PathBuf };
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

    pub fn exists(&self, path: &Path) -> bool {
        match self.add.resolve(path) {
            Some(_real_path) =>
                match self.sub.resolve(path).is_some() {
                    true =>
                        match path.exists() {
                            true => panic!("DANGLING : {:?} EXISTS IN ADD, SUB & FS", path),
                            false => false
                        },
                    false =>
                        match path.exists() {
                            true => true,
                            false => true
                        },
                },
            None =>
                match self.sub.resolve(path).is_some() {
                    true => match path.exists() {
                        true => false,
                        false => false
                    },
                    false =>
                        match path.exists() {
                            true => true,
                            false => false
                        },
                },
        }
    }

    pub fn stat(&self, path: &Path) -> Option<VirtualPath> {
        let state = self.get_virtual_state();

        match state.resolve(path) {
            Some(resolved) =>
                match state.get(path) {
                    Some(virtual_identity) => Some(virtual_identity.clone()),
                    None =>
                        match resolved.exists() {
                            true => Some( //Is virtual but exists in fs
                                VirtualPath::from_path(path)
                                    .with_source(Some(resolved.as_path()))
                                    .with_kind(
                                        match resolved.is_dir() {
                                            true => VirtualKind::Directory,
                                            false => VirtualKind::File
                                        }
                                    )
                            ),
                            false => None
                        }
                }
            None => match path.exists() {
                true => Some(
                    VirtualPath::from_path(path)
                        .with_kind(
                            match path.is_dir() {
                                true => VirtualKind::Directory,
                                false => VirtualKind::File
                            }
                        )
                ),
                false => None
            }
        }
    }

    pub fn read_dir(&self, path: &Path) -> Result<VirtualChildren, VfsError> {
        let directory = match self.stat(path) {
            Some(virtual_identity) =>
                match virtual_identity.as_kind() {
                    VirtualKind::Directory => virtual_identity,
                    _ => return Err(VfsError::IsNotADirectory(path.to_path_buf()))
                },
            None => return Err(VfsError::DoesNotExists(path.to_path_buf()))
        };

        let mut real_children = match VirtualChildren::from_file_system(
            directory.as_referent_source(),
            directory.as_source(),
            Some(&path)
        ) {
            Ok(virtual_children) => virtual_children,
            Err(error) => return Err(VfsError::from(error))
        };

        if let Some(to_add_children) = self.add.children(directory.as_identity()) {
            real_children = &real_children + &to_add_children;
        }

        if let Some(to_del_children) = self.sub.children(directory.as_identity()) {
            real_children = &real_children - &to_del_children;
        }

        Ok(real_children)
    }

    pub fn create(&mut self, identity: &Path, kind: VirtualKind) -> Result<(), VfsError>{
        match self.stat(identity) {
            Some(_) => return Err(VfsError::AlreadyExists(identity.to_path_buf())),
            None => {
                self.add.attach(identity, None, kind);
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, identity: &Path) -> Result<(), VfsError>{
        match self.stat(identity) {
            Some(virtual_identity) => {
                self.sub.attach_virtual(&virtual_identity);
                if self.add.get(virtual_identity.as_identity()).is_some() {
                    self.add.detach(virtual_identity.as_identity());
                }
                Ok(())
            },
            None => return Err(VfsError::DoesNotExists(identity.to_path_buf()))
        }
    }

    pub fn copy(&mut self, source: &Path, destination: &Path) -> Result<(), VfsError>{
        let source = match self.stat(source) {
            Some(virtual_identity) => virtual_identity,
            None => return Err(VfsError::DoesNotExists(source.to_path_buf()))
        };

        let destination = match self.stat(destination) {
            Some(virtual_identity) => virtual_identity,
            None => return Err(VfsError::DoesNotExists(destination.to_path_buf()))
        };

        let new_identity = &VirtualPath::from_path(source.as_identity())
            .with_new_parent(destination.as_identity())
            .with_source(Some(source.as_referent_source()))
            .with_kind(source.to_kind());

        if self.stat(new_identity.as_identity()).is_some() {
            return Err(VfsError::AlreadyExists(new_identity.to_identity()));
        }

        self.add.attach_virtual(new_identity);

        if self.sub.get(new_identity.as_identity()).is_some() {
            self.sub.detach(new_identity.as_identity())
        }

        Ok(())
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
