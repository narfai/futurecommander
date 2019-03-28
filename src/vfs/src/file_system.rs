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
use crate::operation::{ Virtual, Copy, Remove, Create, WriteOperation };

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
                None => match self.virtual_state()?.resolve(path)? {
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

    //TODO @deprecated
    pub fn create(&mut self, identity: &Path, kind: VirtualKind) -> Result<(), VfsError>{
        Virtual(Create::new(identity, kind)).execute(self)
    }

    //TODO @deprecated
    pub fn remove(&mut self, identity: &Path) -> Result<(), VfsError>{
        Virtual(Remove::new(identity)).execute(self)
    }

    //TODO @deprecated
    pub fn copy(&mut self, source: &Path, destination: &Path, with_name: Option<OsString>) -> Result<(), VfsError>{
        Virtual(Copy::new(source, destination, with_name)).execute(self)
    }

    pub fn reset(&mut self) {
        self.add = VirtualDelta::new();
        self.sub = VirtualDelta::new();
    }

    pub fn is_empty(&self) -> bool {
        self.add.is_empty() && self.sub.is_empty()
    }

    pub fn mut_add_state(&mut self) -> &mut VirtualDelta {
        &mut self.add
    }

    pub fn mut_sub_state(&mut self) -> &mut VirtualDelta {
        &mut self.sub
    }

    pub fn add_state(&self) -> VirtualDelta {
        self.add.clone()
    }

    pub fn sub_state(&self) -> VirtualDelta {
        self.sub.clone()
    }

    pub fn virtual_state(&self) -> Result<VirtualDelta, VfsError> { &self.add - &self.sub }
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
