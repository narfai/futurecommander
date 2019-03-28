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
use std::ffi::{ OsStr, OsString };
use crate::{ VirtualDelta, VirtualChildren, VirtualPath, VirtualKind, VfsError };

#[derive(Debug)]
pub enum IdentityStatus {
    Exists(VirtualPath), //Exists only in real FS
    ExistsVirtually(VirtualPath), //Directly added
    ExistsThroughVirtualParent(VirtualPath), //Indirectly added
    NotExists, //Does not exists in virtual fs or real, indirectly or not
    Deleted, //Does exists in real fs and should be deleted
    RemovedVirtually, //Does exists in virtual but is also virtually deleted
}

#[derive(Debug)]
pub struct VirtualFileSystem {
    add: VirtualDelta,
    sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            add: VirtualDelta::new(),
            sub: VirtualDelta::new()
        }
    }

    fn status_virtual(&self, path: &Path) -> Result<IdentityStatus, VfsError> {
        match self.sub.is_virtual(path)? {
            true =>
                match path.exists() {
                    true =>  Err(VfsError::DanglingVirtualPath(path.to_path_buf())), //TODO May happen over system-level concurrency
                    false => Ok(IdentityStatus::RemovedVirtually)
                },
            false => match self.add.get(path)? {//IN ADD AND NOT IN SUB
                Some(virtual_identity) => Ok(IdentityStatus::ExistsVirtually(virtual_identity.clone())),
                None => match self.get_virtual_state()?.resolve(path)? {
                    Some(real_path) => {
                        match real_path.exists() {
                            true =>
                                Ok(
                                    IdentityStatus::ExistsThroughVirtualParent(
                                        VirtualPath::from(
                                            path.to_path_buf(),
                                            Some(real_path.clone()),
                                            VirtualKind::from_path_buf(real_path)
                                        )?
                                    )
                                ),
                            false => Ok(IdentityStatus::NotExists)
                        }
                    },
                    None => Ok(IdentityStatus::NotExists)//Got a virtual parent but does not exists
                }
            }
        }
    }

    fn status_real(&self, path: &Path) -> Result<IdentityStatus, VfsError> {
        match self.sub.is_virtual(path)? {
            true => Ok(IdentityStatus::Deleted),
            false =>
                match path.exists() {
                    true =>
                        Ok(
                            IdentityStatus::Exists(
                                VirtualPath::from(
                                    path.to_path_buf(),
                                    Some(path.to_path_buf()),
                                    match path.is_dir() {
                                        true => VirtualKind::Directory,
                                        false => VirtualKind::File
                                    }
                                )?
                            )
                        ),
                    false => Ok(IdentityStatus::NotExists)
                },
        }
    }

    pub fn status(&self, path: &Path) -> Result<IdentityStatus, VfsError> {
        match self.add.is_virtual(path)? {
            true => self.status_virtual(path),
            false =>self.status_real(path),
        }
    }

    pub fn stat(&self, path: &Path) -> Result<Option<VirtualPath>, VfsError> {
        match self.status(path)? {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity) => Ok(Some(virtual_identity)),

            IdentityStatus::NotExists
            | IdentityStatus::Deleted
            | IdentityStatus::RemovedVirtually => Ok(None)
        }
    }

    pub fn exists(&self, path: &Path) -> Result<bool, VfsError> {
        match self.stat(path)? {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }

    pub fn read_dir(&self, path: &Path) -> Result<VirtualChildren, VfsError> {
        let directory = match self.stat(path)? {
            Some(virtual_identity) =>
                match virtual_identity.as_kind() {
                    VirtualKind::Directory => virtual_identity,
                    _ => return Err(VfsError::IsNotADirectory(path.to_path_buf()))
                },
            None => return Err(VfsError::DoesNotExists(path.to_path_buf()))
        };

        let mut real_children = VirtualChildren::from_file_system(
                directory.as_source().unwrap_or(directory.as_identity()),
                directory.as_source(),
                Some(&path)
        )?;

        if let Some(to_add_children) = self.add.children(directory.as_identity()) {
            real_children = &real_children + &to_add_children;
        }
        if let Some(to_del_children) = self.sub.children(directory.as_identity()) {
            real_children = &real_children - &to_del_children;
        }

        Ok(real_children)
    }

    pub fn create(&mut self, identity: &Path, kind: VirtualKind) -> Result<(), VfsError>{
        match self.stat(identity)? {
            Some(_) => return Err(VfsError::AlreadyExists(identity.to_path_buf())),
            None => {
                self.add.attach(identity, None, kind)?;
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, identity: &Path) -> Result<(), VfsError>{
        match self.stat(identity)? {
            Some(virtual_identity) => {
                self.sub.attach_virtual(&virtual_identity)?;
                if self.add.get(virtual_identity.as_identity())?.is_some() {
                    self.add.detach(virtual_identity.as_identity())?;
                }
                Ok(())
            },
            None => return Err(VfsError::DoesNotExists(identity.to_path_buf()))
        }
    }

    fn create_new_virtual_identity(&self, source: &VirtualPath, destination: &VirtualPath, with_name: Option<OsString>) -> Result<VirtualPath, VfsError>{
        if destination.is_contained_by(&source) {
            return Err(VfsError::CopyIntoItSelft(source.to_identity(), destination.to_identity()));
        }

        let mut new_identity = VirtualPath::from(
            source.to_identity(),
            source.to_source(),
            source.to_kind()
        )?.with_new_identity_parent(destination.as_identity());

        if let Some(name) = &with_name {
            new_identity = new_identity.with_file_name(name.as_os_str());
        }

        Ok(new_identity)
    }

    pub fn copy_virtual_children(&mut self, source: &VirtualPath, identity: &VirtualPath) -> Result<(), VfsError>{
        match identity.to_kind() {
            VirtualKind::Directory => {
                for child in self.read_dir(source.as_identity())? {
                    match self.status(child.as_identity())? {
                        IdentityStatus::ExistsVirtually(_) =>
                            self.copy(
                                child.as_identity(),
                                identity.as_identity(),
                                None
                            )?,
                        _ => {}
                    };
                }
                Ok(())
            },
            _ => Ok(())
        }
    }

    pub fn copy(&mut self, source: &Path, destination: &Path, with_name: Option<OsString>) -> Result<(), VfsError>{
        let source = match self.stat(source)? {
            Some(virtual_identity) => virtual_identity,
            None => return Err(VfsError::DoesNotExists(source.to_path_buf()))
        };

        let destination = match self.stat(destination)? {
            Some(virtual_identity) => match virtual_identity.to_kind() {
                VirtualKind::Directory => virtual_identity,
                _ => return Err(VfsError::IsNotADirectory(destination.to_path_buf()))
            },
            None => return Err(VfsError::DoesNotExists(destination.to_path_buf()))
        };

        let new_identity = self.create_new_virtual_identity(
            &source,
            &destination,
            with_name
        )?;

        if self.exists(new_identity.as_identity())? {
            return Err(VfsError::AlreadyExists(new_identity.to_identity()));
        }

        self.add.attach_virtual(&new_identity)?;

        if self.sub.get(new_identity.as_identity())?.is_some() {
            self.sub.detach(new_identity.as_identity())?
        }

        self.copy_virtual_children(&source, &new_identity)
    }

    pub fn reset(&mut self) {
        self.add = VirtualDelta::new();
        self.sub = VirtualDelta::new();
    }

    pub fn is_empty(&self) -> bool {
        self.add.is_empty() && self.sub.is_empty()
    }

    pub fn get_add_state(&self) -> VirtualDelta {
        self.add.clone()
    }

    pub fn get_sub_state(&self) -> VirtualDelta {
        self.sub.clone()
    }

    pub fn get_virtual_state(&self) -> Result<VirtualDelta, VfsError> { &self.add - &self.sub }
}

/*
TODO : https://trello.com/c/ocihsIuv/29-as-human-i-can-apply-virtual-file-system-to-real-file-system-in-order-to-get-them-both-into-the-closer-possible-state
walk over vfs virtual path which have a source. for each of them, sorted by path depth asc :
ExistsVirtually(VirtualPath), => copy recursively source path to identity path ( with handling of name change ) then remove childs from add
Exists(VirtualPath), => Do nothing
ExistsThroughVirtualParent(VirtualPath), => Do nothing
NotExists, => Do nothing
Deleted, => Do nothing
RemovedVirtually, => Do nothing ?

walk over subs virtual path which have a source. for each of them, sorted by path depth asc :
ExistsVirtually(VirtualPath), => copy recursively source path to identity path ( with handling of name change ) then remove vpath with are childs by source from add
Exists(VirtualPath), => Delete recursively the source path, then remove vpath with are childs by source from sub
ExistsThroughVirtualParent(VirtualPath), => Do nothing
NotExists, => Do nothing
Deleted, => Do nothing
RemovedVirtually, => Do nothing ?

*/
