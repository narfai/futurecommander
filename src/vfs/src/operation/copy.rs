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
    overwrite: bool
}

impl CopyOperation {
    pub fn new(source: &Path, destination: &Path) -> CopyOperation {
        CopyOperation {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            merge: true,
            overwrite: false
        }
    }

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

    pub fn copy_real_children(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        for result in self.source.read_dir()? {
            let child = result?.path();
            let new_destination = self.destination.join(child.strip_prefix(self.source.as_path()).unwrap());

            CopyOperation::new(
                child.as_path(),
                new_destination.as_path()
            ).execute(fs)?;
        }
        Ok(())
    }

    pub fn copy_file(&self, fs: &mut RealFileSystem) -> Result<(), VfsError>{
        match fs.copy_file_to_file(
            self.source.as_path(),
            self.destination.as_path(),
            &|_s| { /* println!("{} {}", self.destination.file_name().unwrap().to_string_lossy(), s)*/ },
            self.overwrite
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(VfsError::from(error))
        }
    }
}

//TODO should use Transaction to do nothing over recursive error => Maybe it's impossible => maybe with subdelta and "vfs preview"
impl WriteOperation<VirtualFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let source = StatusQuery::new(self.source.as_path()).retrieve(fs)?;
        let parent_path = VirtualPath::get_parent_or_root(self.destination.as_path());
        let parent = StatusQuery::new(parent_path.as_path()).retrieve(fs)?;

        if ! parent.exists() {
            return Err(VfsError::DoesNotExists(parent_path.to_path_buf()));
        } else if !parent.is_dir() {
            return Err(VfsError::IsNotADirectory(parent_path.to_path_buf()));
        }

        if ! source.exists() {
            return Err(VfsError::DoesNotExists(self.source.to_path_buf()));
        }

        let source_identity = source.as_inner().as_virtual();

        let new_identity = VirtualPath::from(
            self.destination.to_path_buf(),
            source_identity.to_source(),
            source_identity.to_kind()
        )?;

        if new_identity.is_contained_by(source_identity) {
            return Err(VfsError::CopyIntoItSelf(source_identity.to_identity(), self.destination.to_path_buf()));
        }

        let stat_new = StatusQuery::new(new_identity.as_identity());

        match stat_new.retrieve(&fs)?.into_inner() {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity) => {
                match virtual_identity.to_kind() {
                    VirtualKind::Directory =>
                        match source_identity.to_kind() {
                            VirtualKind::Directory =>
                                match self.merge {
                                    true => Self::copy_virtual_children(fs, &source_identity, &virtual_identity)?,
                                    false => return Err(VfsError::Custom("Merge is not allowed".to_string()))
                                }, //merge
                            VirtualKind::File => return Err(VfsError::Custom("Cannot overwrite an existing directory with a file".to_string())),
                            _ => {}
                        },
                    VirtualKind::File =>
                        match source_identity.to_kind() {
                            VirtualKind::Directory => return Err(VfsError::Custom("Cannot copy directory into file".to_string())),
                            VirtualKind::File =>
                                match self.overwrite {
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
                    VirtualKind::Directory => Self::copy_virtual_children(fs, &source_identity, &new_identity)?,
                    _ => {}
                }
            },
            IdentityStatus::Removed(_) | IdentityStatus::RemovedVirtually(_) => {
                fs.mut_sub_state().detach(new_identity.as_identity())?;
                fs.mut_add_state().attach_virtual(&new_identity)?;
                match new_identity.to_kind() {
                    VirtualKind::Directory => Self::copy_virtual_children(fs, &source_identity, &new_identity)?,
                    _ => {}
                }
            },
        }
        Ok(())
    }
}

impl WriteOperation<RealFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        if ! self.source.exists() {
            return Err(VfsError::DoesNotExists(self.source.to_path_buf()));
        }

        if self.source.is_dir() && self.destination.is_file() {
            return Err(VfsError::Custom("Cannot copy directory to the path of existing file".to_string())); //Error dir to existing file
        }

        if self.source.is_file() && self.destination.is_dir() {
            return Err(VfsError::Custom("Cannot copy file to the path existing directory".to_string()));
        }

        match self.source.is_dir() {
            true =>
                match self.destination.exists() {
                    true => {
                            if ! self.merge {
                                return Err(VfsError::Custom("Merge is not allowed".to_string()))
                            }
                            self.copy_real_children(fs)
                        },
                    false => {
                        fs.create_directory(self.destination.as_path(), false)?;
                        self.copy_real_children(fs)
                    } //dir to dir
                },
            false =>
                match self.destination.exists() {
                    true => {
                            if !self.overwrite {
                                return Err(VfsError::Custom("Overwrite is not allowed".to_string()));
                            }
                            self.copy_file(fs)
                        },
                    false => self.copy_file(fs)
                },
        }
    }
}
