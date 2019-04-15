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
        if fs.sub_state().is_virtual(self.path.as_path())? {
            match fs.add_state().get(self.path.as_path())? {
                Some(_virtual_state) => Err(VfsError::AddSubDanglingVirtualPath(self.path.to_path_buf())),
                None => Ok(VirtualStatus::new(VirtualState::RemovedVirtually, self.virtual_unknown()?))
            }
        } else {
            match fs.add_state().get(self.path.as_path())? {//IN ADD AND NOT IN SUB
                Some(virtual_identity) =>
                    if self.path.exists() {
                        Ok(VirtualStatus::new(VirtualState::Replaced, virtual_identity.clone()))
                    } else {
                        Ok(VirtualStatus::new(VirtualState::ExistsVirtually, virtual_identity.clone()))
                    }
                None =>
                    match fs.virtual_state()?.resolve(self.path.as_path())? {
                        Some(real_path) => {
                            if real_path.exists() {
                                Ok(
                                    VirtualStatus::new(
                                        VirtualState::ExistsThroughVirtualParent,
                                        VirtualPath::from(
                                            self.path.to_path_buf(),
                                            Some(real_path.clone()),
                                            Kind::from_path_buf(real_path)
                                        )?
                                    )
                                )
                            } else {
                                Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown()?))
                            }
                        },
                        None => Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown()?))//Got a virtual parent but does not exists
                    }
            }
        }
    }

    fn status_real(&self, fs: &VirtualFileSystem) -> Result<VirtualStatus, VfsError> {
        if fs.sub_state().is_virtual(self.path.as_path())? {
            Ok(VirtualStatus::new(VirtualState::Removed, self.virtual_unknown()?))
        } else if self.path.exists() {
            Ok(
                VirtualStatus::new(
                    VirtualState::Exists,
                          VirtualPath::from(
                               self.path.to_path_buf(),
                               Some(self.path.to_path_buf()),
                               if self.path.is_dir()
                                   { Kind::Directory }
                                   else { Kind::File }
                           )?
                )
            )
        } else {
            Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown()?))
        }

    }
}

impl Query<&VirtualFileSystem> for StatusQuery{
    type Result = EntryAdapter<VirtualStatus>;

    fn retrieve(&self, fs: &VirtualFileSystem) -> Result<Self::Result, VfsError> {
        if fs.add_state().is_virtual(self.path.as_path())? {
            match self.status_virtual(&fs) {
                Ok(status) => Ok(EntryAdapter(status)),
                Err(error) => Err(error)
            }
        } else {
            match self.status_real(&fs) {
                Ok(status) => Ok(EntryAdapter(status)),
                Err(error) => Err(error)
            }
        }
    }
}


#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        Samples
    };

    #[test]
    fn status_query_relay_real_fs(){
        let static_samples = Samples::static_samples_path();
        let vfs = VirtualFileSystem::default();

        let entry = StatusQuery::new(static_samples.as_path()).retrieve(&vfs).unwrap();
        assert_eq!(entry.as_inner().state(), VirtualState::Exists);
        assert!(entry.is_dir());
    }
}
