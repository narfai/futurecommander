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

#[allow(unused_imports)]
use crate::query::Entry;

use crate::{ VfsError, Kind };
use crate::representation::{ VirtualPath, VirtualState };
use crate::virtual_file_system::{ VirtualFileSystem };
use crate::query::{ Query, VirtualStatus, EntryAdapter };


pub struct StatusQuery {
    path: PathBuf
}

impl StatusQuery {
    pub fn new(path: &Path) -> StatusQuery {
        StatusQuery {
            path: path.to_path_buf()
        }
    }

    fn virtual_unknown(&self) -> Result<VirtualPath, VfsError>{
        VirtualPath::from(
            self.path.to_path_buf(),
            Some(self.path.to_path_buf()),
            Kind::Unknown
        )
    }

    fn status_virtual(&self, fs: &VirtualFileSystem) -> Result<VirtualStatus, VfsError> {
        match fs.sub_state().is_virtual(self.path.as_path())? {
            true =>
                match fs.add_state().get(self.path.as_path())? {
                    Some(_virtual_state) => Err(VfsError::AddSubDanglingVirtualPath(self.path.to_path_buf())),
                    None => Ok(VirtualStatus::new(VirtualState::RemovedVirtually, self.virtual_unknown()?))
                }
            false =>
                match fs.add_state().get(self.path.as_path())? {//IN ADD AND NOT IN SUB
                    Some(virtual_identity) =>
                        match self.path.exists() {
                            true => Ok(VirtualStatus::new(VirtualState::Replaced, virtual_identity.clone())),
                            false => Ok(VirtualStatus::new(VirtualState::ExistsVirtually, virtual_identity.clone()))
                        }
                    None =>
                        match fs.virtual_state()?.resolve(self.path.as_path())? {
                            Some(real_path) => {
                                match real_path.exists() {
                                    true =>
                                        Ok(
                                            VirtualStatus::new(VirtualState::ExistsThroughVirtualParent,
                                                VirtualPath::from(
                                                    self.path.to_path_buf(),
                                                    Some(real_path.clone()),
                                                    Kind::from_path_buf(real_path)
                                                )?
                                            )
                                        ),
                                    false => Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown()?))
                                }
                            },
                            None => Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown()?))//Got a virtual parent but does not exists
                        }
                }
        }
    }

    fn status_real(&self, fs: &VirtualFileSystem) -> Result<VirtualStatus, VfsError> {
        match fs.sub_state().is_virtual(self.path.as_path())? {
            true => Ok(VirtualStatus::new(VirtualState::Removed, self.virtual_unknown()?)),
            false =>
                match self.path.exists() {
                    true =>
                        Ok(
                            VirtualStatus::new(VirtualState::Exists,
                                VirtualPath::from(
                                    self.path.to_path_buf(),
                                    Some(self.path.to_path_buf()),
                                    match self.path.is_dir() {
                                        true => Kind::Directory,
                                        false => Kind::File
                                    }
                                )?
                            )
                        ),
                    false => Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown()?))
                },
        }
    }
}

impl Query<&VirtualFileSystem> for StatusQuery{
    type Result = EntryAdapter<VirtualStatus>;

    fn retrieve(&self, fs: &VirtualFileSystem) -> Result<Self::Result, VfsError> {
        match fs.add_state().is_virtual(self.path.as_path())? {
            true =>
                match self.status_virtual(&fs) {
                    Ok(status) => Ok(EntryAdapter(status)),
                    Err(error) => Err(error)
                }
            false =>
                match self.status_real(&fs) {
                    Ok(status) => Ok(EntryAdapter(status)),
                    Err(error) => Err(error)
                }
        }
    }
}
