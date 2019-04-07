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

use crate::{ VfsError, VirtualFileSystem, Kind };
use crate::operation::{CopyOperation, Operation};
use crate::representation::{ VirtualPath };
use crate::query::{Query, ReadDirQuery, StatusQuery, IdentityStatus, Entry };

impl CopyOperation {
    fn copy_virtual_children(fs: &mut VirtualFileSystem, source: &VirtualPath, identity: &VirtualPath) -> Result<(), VfsError> {
        let read_dir = ReadDirQuery::new(source.as_identity());
        for child in read_dir.retrieve(&fs)?.into_iter() {
            match child.as_inner() {
                IdentityStatus::ExistsVirtually(_) =>
                    CopyOperation::new(
                        child.path(),
                        identity.as_identity()
                            .join(
                                child.path()
                                    .file_name()
                                    .unwrap()
                            ).as_path()
                    ).execute(fs)?
                ,
                _ => {}
            };
        }
        Ok(())
    }
}

//TODO should use Transaction to do nothing over recursive error => Maybe it's impossible => maybe with subdelta and "vfs preview"
impl Operation<VirtualFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let source = StatusQuery::new(self.source()).retrieve(fs)?;
        let parent_path = VirtualPath::get_parent_or_root(self.destination());
        let parent = StatusQuery::new(parent_path.as_path()).retrieve(fs)?;

        if ! parent.exists() {
            return Err(VfsError::DoesNotExists(parent_path.to_path_buf()));
        } else if !parent.is_dir() {
            return Err(VfsError::IsNotADirectory(parent_path.to_path_buf()));
        }

        if ! source.exists() {
            return Err(VfsError::DoesNotExists(self.source().to_path_buf()));
        }

        let source_identity = source.as_inner().as_virtual();

        let new_identity = VirtualPath::from(
            self.destination().to_path_buf(),
            source_identity.to_source(),
            source_identity.to_kind()
        )?;

        if new_identity.is_contained_by(source_identity) {
            return Err(VfsError::CopyIntoItSelf(source_identity.to_identity(), self.destination().to_path_buf()));
        }

        let stat_new = StatusQuery::new(new_identity.as_identity());

        match stat_new.retrieve(&fs)?.into_inner() {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity) => {
                match virtual_identity.to_kind() {
                    Kind::Directory =>
                        match source_identity.to_kind() {
                            Kind::Directory =>
                                match self.merge() {
                                    true => Self::copy_virtual_children(fs, &source_identity, &virtual_identity)?,
                                    false => return Err(VfsError::Custom("Merge is not allowed".to_string()))
                                }, //merge
                            Kind::File => return Err(VfsError::Custom("Cannot overwrite an existing directory with a file".to_string())),
                            _ => {}
                        },
                    Kind::File =>
                        match source_identity.to_kind() {
                            Kind::Directory => return Err(VfsError::Custom("Cannot copy directory into file".to_string())),
                            Kind::File =>
                                match self.overwrite() {
                                    true => {
                                        fs.mut_add_state().detach(virtual_identity.as_identity())?;
                                        fs.mut_add_state().attach_virtual(&new_identity)?;
                                    },
                                    false => return Err(VfsError::Custom("Overwrite is not allowed".to_string())), //overwrite
                                }
                            _ => {}
                        },
                    _ => {}
                }
            },
            IdentityStatus::NotExists(_) => {
                fs.mut_add_state().attach_virtual(&new_identity)?;
                match new_identity.to_kind() {
                    Kind::Directory => Self::copy_virtual_children(fs, &source_identity, &new_identity)?,
                    _ => {}
                }
            },
            IdentityStatus::Removed(_) | IdentityStatus::RemovedVirtually(_) => {
                fs.mut_sub_state().detach(new_identity.as_identity())?;
                fs.mut_add_state().attach_virtual(&new_identity)?;
                match new_identity.to_kind() {
                    Kind::Directory => Self::copy_virtual_children(fs, &source_identity, &new_identity)?,
                    _ => {}
                }
            },
        }
        Ok(())
    }
}
