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

/* I EXPECT THE PATH destination TO EXISTS WITH SOURCE source */

use std::path::{ PathBuf, Path };
use std::ffi::OsString;

use crate::{ VfsError };

use crate::file_system::{ VirtualFileSystem, RealFileSystem };
use crate::representation::{ VirtualPath, VirtualKind };
use crate::operation::{ WriteOperation };
use crate::query::{ ReadQuery, ReadDirQuery, StatusQuery, IdentityStatus, Entry };

#[derive(Debug, Clone)]
pub struct CopyOperation {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool //To honour overwrite or merge error, we should crawl recursively the entire vfs children of dst ...
}

impl CopyOperation {
    pub fn new(source: &Path, destination: &Path) -> CopyOperation {
        CopyOperation {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            merge: false,
            overwrite: false
        }
    }

    fn copy_virtual_children(fs: &mut VirtualFileSystem, source: &VirtualPath, identity: &VirtualPath) -> Result<(), VfsError> {
        match identity.to_kind() {
            VirtualKind::Directory => {
                let read_dir = ReadDirQuery::new(source.as_identity());
                for child in read_dir.retrieve(&fs)? {
                    match StatusQuery::new(child.path()).retrieve(&fs)? {
                        IdentityStatus::ExistsVirtually(_) =>
                            CopyOperation::new(
                                child.path(),
                                identity.as_identity()
                            ).execute(fs)?
                        ,
                        _ => {}
                    };
                }
                Ok(())
            },
            _ => Ok(())
        }
    }
}


impl WriteOperation<VirtualFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let source = match StatusQuery::new(self.source.as_path()).retrieve(fs)?.into_virtual_identity() {
            Some(virtual_identity) => virtual_identity,
            None => return Err(VfsError::DoesNotExists(self.source.to_path_buf()))
        };

        let destination = StatusQuery::new(self.destination.as_path()).retrieve(fs)?;

        //TODO check parent exists
        let new_identity = VirtualPath::from(
            self.destination.to_path_buf(),
            source.to_source(),
            source.to_kind()
        )?;

        //TODO check for merge errors in dst recursively if merge = false
        if new_identity.is_contained_by(&source) {
            return Err(VfsError::CopyIntoItSelf(source.to_identity(), self.destination.to_path_buf()));
        }

        let stat_new = StatusQuery::new(new_identity.as_identity());

        match stat_new.retrieve(&fs)? {
            IdentityStatus::Exists(_)
            | IdentityStatus::ExistsVirtually(_)
            | IdentityStatus::Replaced(_)
            | IdentityStatus::ExistsThroughVirtualParent(_) => return Err(VfsError::AlreadyExists(new_identity.to_identity())),
            IdentityStatus::NotExists => {
                fs.mut_add_state().attach_virtual(&new_identity)?;
            },
            IdentityStatus::Removed | IdentityStatus::RemovedVirtually => {
                fs.mut_sub_state().detach(new_identity.as_identity())?;
                fs.mut_add_state().attach_virtual(&new_identity)?;
            },
        }

        Self::copy_virtual_children(fs, &source, &new_identity)
    }
}

impl WriteOperation<RealFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
//        let new_destination = match &self.name {
//            Some(name) => self.destination.join(name),
//            None => self.destination.to_path_buf()
//        };
//
//        match fs.copy(self.source.as_path(), new_destination.as_path(), &|_read| {}, true, false) {
//            Ok(_) => Ok(()),
//            Err(error) => Err(VfsError::from(error))
//        }
        unimplemented!()
    }
}
