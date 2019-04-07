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
use crate::{ VfsError };

use crate::file_system::{ VirtualFileSystem, RealFileSystem };
use crate::operation::{ WriteOperation };
use crate::query::{ReadQuery, StatusQuery, IdentityStatus };

#[derive(Debug, Clone)]
pub struct RemoveOperation {
    path: PathBuf
}

impl RemoveOperation {
    pub fn new(path: &Path) -> RemoveOperation {
        RemoveOperation {
            path: path.to_path_buf()
        }
    }
}

impl WriteOperation<VirtualFileSystem> for RemoveOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match StatusQuery::new(self.path.as_path()).retrieve(&fs)?.into_inner() {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity) => {
                fs.mut_sub_state().attach_virtual(&virtual_identity)?;
            },
            IdentityStatus::ExistsVirtually(virtual_identity) => {
                fs.mut_add_state().detach(virtual_identity.as_identity())?;
                if let Some(source) = virtual_identity.as_source() {
                    match StatusQuery::new(source).retrieve(&fs)?.into_inner() {
                        IdentityStatus::Replaced(virtual_path) => {
                            if fs.add_state().is_directory_empty(virtual_path.as_identity()) {
                                fs.mut_add_state().detach(virtual_path.as_identity())?;
                            }
                        }
                        _ => {}
                    }
                }
            }
            IdentityStatus::NotExists(_) | IdentityStatus::Removed(_) | IdentityStatus::RemovedVirtually(_) =>
                return Err(VfsError::DoesNotExists(self.path.to_path_buf()))
            ,
        }
        Ok(())
    }
}


impl WriteOperation<RealFileSystem> for RemoveOperation{
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        if ! self.path.exists() {
            return Err(VfsError::DoesNotExists(self.path.to_path_buf()));
        }

        let result = match self.path.is_dir() {
            true => fs.remove_directory(self.path.as_path()),
            false => fs.remove_file(self.path.as_path())
        };

        match result {
            Err(error) => Err(VfsError::from(error)),
            Ok(_) => Ok(())
        }
    }
}
