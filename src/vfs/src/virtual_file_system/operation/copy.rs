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
use crate::operation::{CopyOperation, Operation };
use crate::representation::{ VirtualPath, VirtualState };
use crate::query::{Query, ReadDirQuery, StatusQuery, VirtualStatus, Entry };

impl CopyOperation {
    fn copy_virtual_children(&self, fs: &mut VirtualFileSystem, source: &VirtualPath, identity: &VirtualPath) -> Result<(), VfsError> {
        let read_dir = ReadDirQuery::new(source.as_identity());
        for child in read_dir.retrieve(&fs)?.into_iter() {
            if let VirtualState::ExistsVirtually = child.as_inner().state() {
                CopyOperation::new(
                    child.path(),
                    identity.as_identity()
                        .join(
                            child.path()
                                .file_name()
                                .unwrap()
                        ).as_path(),
                    self.merge(),
                    self.overwrite()
                ).execute(fs)?
            }
        }
        Ok(())
    }

    fn copy_exiting_children(&self, fs: &mut VirtualFileSystem, source: &VirtualPath, identity: &VirtualPath) -> Result<(), VfsError> {
        let read_dir = ReadDirQuery::new(source.as_identity());
        for child in read_dir.retrieve(&fs)?.into_iter() {
            if child.exists() {
                CopyOperation::new(
                    child.path(),
                    identity.as_identity()
                        .join(
                            child.path()
                                .file_name()
                                .unwrap()
                        ).as_path(),
                    self.merge(),
                    self.overwrite()
                ).execute(fs)?
            }
        }
        Ok(())
    }
}

//TODO should use Transaction to do nothing over recursive error => Maybe it's impossible => maybe with subdelta and "vfs preview"
impl Operation<VirtualFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let source = StatusQuery::new(self.source()).retrieve(fs)?;
        let parent_path = crate::path_helper::get_parent_or_root(self.destination());
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

        let entry = stat_new.retrieve(&fs)?;
        let state = entry.as_inner().state();
        match entry.into_inner() {
            VirtualStatus{ state: VirtualState::Exists, identity }
            | VirtualStatus{ state: VirtualState::ExistsVirtually, identity }
            | VirtualStatus{ state: VirtualState::ExistsThroughVirtualParent, identity}
            | VirtualStatus{ state: VirtualState::Replaced, identity } => {
                match identity.to_kind() {
                    Kind::Directory =>
                        match source_identity.to_kind() {
                            Kind::Directory =>
                                if self.merge() {
                                    self.copy_exiting_children(fs, &source_identity, &identity)?
                                } else {
                                    return Err(VfsError::Custom("Merge is not allowed".to_string()))
                                },
                            Kind::File => return Err(VfsError::Custom("Cannot overwrite an existing directory with a file".to_string())),
                            _ => {}
                        },
                    Kind::File =>
                        match source_identity.to_kind() {
                            Kind::Directory => return Err(VfsError::Custom("Cannot copy directory into file".to_string())),
                            Kind::File =>
                                if self.overwrite() {
                                    match state {
                                        VirtualState::Replaced
                                        | VirtualState::ExistsVirtually => {
                                            fs.mut_add_state().detach(identity.as_identity())?;
                                        },
                                        _ => {}
                                    }

                                    fs.mut_add_state().attach_virtual( & new_identity)?;
                                } else {
                                    return Err(VfsError::Custom("Overwrite is not allowed".to_string()))
                                }
                            _ => {}
                        },
                    _ => {}
                }
            },
            VirtualStatus{ state: VirtualState::NotExists, .. } => {
                fs.mut_add_state().attach_virtual(&new_identity)?;
                if let Kind::Directory = new_identity.to_kind() {
                    self.copy_virtual_children(fs, &source_identity, &new_identity)?
                }
            },
            VirtualStatus{ state: VirtualState::Removed, .. }
            | VirtualStatus{ state: VirtualState::RemovedVirtually, .. } => {
                fs.mut_sub_state().detach(new_identity.as_identity())?;
                fs.mut_add_state().attach_virtual(&new_identity)?;
                if let Kind::Directory = new_identity.to_kind() {
                    self.copy_virtual_children(fs, &source_identity, &new_identity)?
                }
            },
        }
        Ok(())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod virtual_file_system {
    use super::*;

    use crate::Samples;

    //Error testing
    #[test]
    fn copy_or_move_directory_into_itself_must_not_be_allowed() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();

        let source = sample_path.join("B");
        let destination = sample_path.join("B/D/B");

        match CopyOperation::new(
            source.as_path(),
            destination.as_path(),
            true,
            false
        ).execute(&mut vfs) {
            Err(VfsError::CopyIntoItSelf(err_source, err_destination)) => {
                assert_eq!(source.as_path(), err_source.as_path());
                assert_eq!(destination.as_path(), err_destination.as_path());
            }
            Err(error) => panic!("{}", error),
            Ok(_) => panic!("Should not be able to copy into itself")
        };
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
    fn virtual_copy_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        let operation = CopyOperation::new(
            samples_path.join("A").as_path(),
            samples_path.join("Z").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_directory(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_directory_merge(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        //Avoid need of override because of .gitkeep file present in both directory
        let gitkeep = samples_path.join("B/.gitkeep");
        fs.mut_sub_state().attach(gitkeep.as_path(),Some(gitkeep.as_path()), Kind::File).unwrap();

        let operation = CopyOperation::new(
            samples_path.join("B").as_path(),
            samples_path.join("A").as_path(),
            true,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_file(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        let operation = CopyOperation::new(
            samples_path.join("F").as_path(),
            samples_path.join("Z").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_file(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_file_overwrite(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        let operation = CopyOperation::new(
            samples_path.join("F").as_path(),
            samples_path.join("A/C").as_path(),
            false,
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("A/C").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_file(samples_path.join("A/C").as_path()).unwrap());
    }
}
