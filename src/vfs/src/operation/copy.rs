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
        for child in read_dir.retrieve(&fs)? {
            match StatusQuery::new(child.path()).retrieve(&fs)? {
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
impl WriteOperation<VirtualFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let source = StatusQuery::new(self.source.as_path()).retrieve(fs)?;
        let destination = StatusQuery::new(self.destination.as_path()).retrieve(fs)?;
        let parent_path = VirtualPath::get_parent_or_root(self.destination.as_path());
        let parent = StatusQuery::new(parent_path.as_path()).retrieve(fs)?;

        println!("SOURCE {:?} DEST {:?} {:?}", source, destination, self.destination.as_path());

        if ! parent.exists() {
            return Err(VfsError::DoesNotExists(parent_path.to_path_buf()));
        } else if !parent.is_dir() {
            return Err(VfsError::IsNotADirectory(parent_path.to_path_buf()));
        }

        let source_identity = match source.as_virtual_identity() {
            Some(identity) => identity,
            None => return Err(VfsError::DoesNotExists(self.source.to_path_buf()))
        };

        let new_identity = VirtualPath::from(
            self.destination.to_path_buf(),
            source_identity.to_source(),
            source_identity.to_kind()
        )?;

        if new_identity.is_contained_by(source_identity) {
            return Err(VfsError::CopyIntoItSelf(source_identity.to_identity(), self.destination.to_path_buf()));
        }

        let stat_new = StatusQuery::new(new_identity.as_identity());

        match stat_new.retrieve(&fs)? {
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
            IdentityStatus::NotExists => {
                fs.mut_add_state().attach_virtual(&new_identity)?;
                match new_identity.to_kind() {
                    VirtualKind::Directory => Self::copy_virtual_children(fs, &source_identity, &new_identity)?,
                    _ => {}
                }
            },
            IdentityStatus::Removed | IdentityStatus::RemovedVirtually => {
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

        println!("SOURCE {:?} DEST {:?}", self.source.as_path(), self.destination.as_path());

        match self.source.is_dir() {
            true =>
                match self.destination.exists() {
                    true =>
                        match self.destination.is_dir() {
                            true => {
                                if ! self.merge {
                                    return Err(VfsError::Custom("Merge is not allowed".to_string()))
                                }
                                for result in self.source.read_dir()? {
                                    let child = result?.path();
                                    let new_destination = self.destination.join(child.strip_prefix(self.source.as_path()).unwrap());

                                    CopyOperation::new(
                                        child.as_path(),
                                        new_destination.as_path()
                                    ).execute(fs)?;
                                }
                                Ok(())
                            }, //merge
                            false => return Err(VfsError::Custom("Cannot copy directory to existing file".to_string())) //Error dir to existing file
                        },
                    false => {
                        fs.create_directory(self.destination.as_path(), false)?;

                        for result in self.source.read_dir()? {
                            let child = result?.path();
                            let new_destination = self.destination.join(child.strip_prefix(self.source.as_path()).unwrap());

                            CopyOperation::new(
                                child.as_path(),
                                new_destination.as_path()
                            ).execute(fs)?;
                        }
                        Ok(())
                    } //dir to dir
                },
            false =>
                match self.destination.exists() {
                    true =>
                        match self.destination.is_dir() {
                            true => return Err(VfsError::Custom("Cannot copy file to existing".to_string())),//Error file to existing dir
                            false => {
                                if !self.overwrite {
                                    return Err(VfsError::Custom("Overwrite is not allowrd".to_string()));
                                }
                                match fs.copy_file_to_file(
                                    self.source.as_path(),
                                    self.destination.as_path(),
                                    &|_| {},
                                    self.overwrite
                                ) {
                                    Ok(_) => Ok(()),
                                    Err(error) => Err(VfsError::from(error))
                                }
                            }
                        },
                    false =>
                        match fs.copy_file_to_file(
                            self.source.as_path(),
                            self.destination.as_path(),
                            &|_| {},
                            false
                        ) {
                            Ok(_) => Ok(()),
                            Err(error) => Err(VfsError::from(error))
                        }
                },
        }
    }
}
