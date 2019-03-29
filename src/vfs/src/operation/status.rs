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

use std::path::{ PathBuf, Path };
use crate::{ VirtualFileSystem, VfsError, VirtualKind, VirtualPath, IdentityStatus };
use crate::operation::{ ReadOperation, Virtual };



pub struct Status {
    path: PathBuf
}

impl Status {
    pub fn new(path: &Path) -> Status {
        Status {
            path: path.to_path_buf()
        }
    }
}

impl Virtual<Status> {
    fn status_virtual(&self, fs: &VirtualFileSystem) -> Result<IdentityStatus, VfsError> {
        match fs.sub_state().is_virtual(self.0.path.as_path())? {
            true =>
                match self.0.path.exists() {
                    true =>  Err(VfsError::DanglingVirtualPath(self.0.path.to_path_buf())), //TODO May happen over system-level concurrency
                    false => Ok(IdentityStatus::RemovedVirtually)
                },
            false => match fs.add_state().get(self.0.path.as_path())? {//IN ADD AND NOT IN SUB
                Some(virtual_identity) => Ok(IdentityStatus::ExistsVirtually(virtual_identity.clone())),
                None => match fs.virtual_state()?.resolve(self.0.path.as_path())? {
                    Some(real_path) => {
                        match real_path.exists() {
                            true =>
                                Ok(
                                    IdentityStatus::ExistsThroughVirtualParent(
                                        VirtualPath::from(
                                            self.0.path.to_path_buf(),
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

    fn status_real(&self, fs: &VirtualFileSystem) -> Result<IdentityStatus, VfsError> {
        match fs.sub_state().is_virtual(self.0.path.as_path())? {
            true => Ok(IdentityStatus::Deleted),
            false =>
                match self.0.path.exists() {
                    true =>
                        Ok(
                            IdentityStatus::Exists(
                                VirtualPath::from(
                                    self.0.path.to_path_buf(),
                                    Some(self.0.path.to_path_buf()),
                                    match self.0.path.is_dir() {
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
}

impl ReadOperation<&VirtualFileSystem, IdentityStatus> for Virtual<Status>{
    fn retrieve(&self, fs: &VirtualFileSystem) -> Result<IdentityStatus, VfsError> {
        match fs.add_state().is_virtual(self.0.path.as_path())? {
            true => self.status_virtual(&fs),
            false => self.status_real(&fs),
        }
    }
}
