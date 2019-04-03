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
use std::ffi::OsString;

use crate::{ VfsError };

use crate::file_system::{ VirtualFileSystem, RealFileSystem };
use crate::representation::{ VirtualPath, VirtualKind };
use crate::operation::{ WriteOperation };
use crate::query::{ReadQuery, ReadDirQuery, StatusQuery, IdentityStatus, Entry };

#[derive(Debug, Clone)]
pub struct CopyOperation {
    source: PathBuf,
    destination: PathBuf,
    name: Option<OsString>
}

impl CopyOperation {
    pub fn new(source: &Path, destination: &Path, name: Option<OsString>) -> CopyOperation {
        CopyOperation {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            name
        }
    }

    fn create_new_virtual_identity(source: &VirtualPath, destination: &VirtualPath, with_name: &Option<OsString>) -> Result<VirtualPath, VfsError> {
        if destination.is_contained_by(&source) {
            return Err(VfsError::CopyIntoItSelf(source.to_identity(), destination.to_identity()));
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

    fn copy_virtual_children(fs: &mut VirtualFileSystem, source: &VirtualPath, identity: &VirtualPath) -> Result<(), VfsError> {
        match identity.to_kind() {
            VirtualKind::Directory => {
                let read_dir = ReadDirQuery::new(source.as_identity());
                for child in read_dir.retrieve(&fs)? {
                    match StatusQuery::new(child.path()).retrieve(&fs)? {
                        IdentityStatus::ExistsVirtually(_) =>
                            CopyOperation::new(
                                child.path(),
                                identity.as_identity(),
                                None
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

    fn retrieve_virtual_identity(fs: &VirtualFileSystem, path: &Path) -> Result<VirtualPath, VfsError> {
        let stat = StatusQuery::new(path);
        match stat.retrieve(&fs)?.virtual_identity() {
            Some(virtual_identity) => Ok(virtual_identity),
            None => return Err(VfsError::DoesNotExists(path.to_path_buf()))
        }
    }
}


impl WriteOperation<VirtualFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let source = Self::retrieve_virtual_identity(&fs, self.source.as_path())?;
        let destination = Self::retrieve_virtual_identity(&fs, self.destination.as_path())?;

        match destination.to_kind() {
            VirtualKind::Directory => {},
            _ => return Err(VfsError::IsNotADirectory(self.destination.to_path_buf()))
        }

        let new_identity = Self::create_new_virtual_identity(
            &source,
            &destination,
            &self.name
        )?;

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
        let new_destination = match &self.name {
            Some(name) => self.destination.join(name),
            None => self.destination.to_path_buf()
        };

        match fs.copy(self.source.as_path(), new_destination.as_path(), &|_read| {}, true, false) {
            Ok(_) => Ok(()),
            Err(error) => Err(VfsError::from(error))
        }
    }
}
