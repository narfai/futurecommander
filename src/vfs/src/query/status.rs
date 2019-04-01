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

use crate::{ Virtual, VfsError };
use crate::representation::{ VirtualPath, VirtualKind, VirtualChildren };
use crate::file_system::{ VirtualFileSystem };
use crate::query::{ ReadQuery, IdentityStatus };


pub struct StatusQuery {
    path: PathBuf
}

impl StatusQuery {
    pub fn new(path: &Path) -> StatusQuery {
        StatusQuery {
            path: path.to_path_buf()
        }
    }
}

impl Virtual<StatusQuery> {
    fn status_virtual(&self, fs: &VirtualFileSystem) -> Result<IdentityStatus, VfsError> {
        match fs.sub_state().is_virtual(self.0.path.as_path())? {
            true => //Ok(IdentityStatus::RemovedVirtually),
                match fs.add_state().get(self.0.path.as_path())? {
                    Some(virtual_state) => Err(VfsError::AddSubDanglingVirtualPath(self.0.path.to_path_buf())),
                    None => Ok(IdentityStatus::RemovedVirtually),
//                        match self.0.path.exists() {
//                            true => Ok(IdentityStatus::RemovedVirtually),
//                            false => Err(VfsError::SubDanglingVirtualPath(self.0.path.to_path_buf())),
//                        }
                }
            false =>
                match fs.add_state().get(self.0.path.as_path())? {//IN ADD AND NOT IN SUB
                    Some(virtual_identity) =>
                        match self.0.path.exists() {
                            true => Ok(IdentityStatus::Replaced(virtual_identity.clone())),
                            false => Ok(IdentityStatus::ExistsVirtually(virtual_identity.clone()))
                        }
                    None =>
                        match fs.virtual_state()?.resolve(self.0.path.as_path())? {
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
            true => Ok(IdentityStatus::Removed),
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

impl ReadQuery<&VirtualFileSystem, IdentityStatus> for Virtual<StatusQuery>{
    fn retrieve(&self, fs: &VirtualFileSystem) -> Result<IdentityStatus, VfsError> {
        match fs.add_state().is_virtual(self.0.path.as_path())? {
            true => self.status_virtual(&fs),
            false => self.status_real(&fs),
        }
    }
}
