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
use crate::{ Virtual, Real, VfsError };

use crate::file_system::{ VirtualFileSystem, RealFileSystem, VirtualVersion, RealVersion };
use crate::operation::{ WriteOperation };
use crate::query::{ReadQuery, StatusQuery, IdentityStatus };

#[derive(Debug)]
pub struct RemoveOperation {
    path: PathBuf,
    virtual_version: Option<usize>,
    real_version: Option<usize>
}

impl Virtual<RemoveOperation> {
    pub fn new(path: &Path) -> Virtual<RemoveOperation> {
        Virtual(
            RemoveOperation {
                path: path.to_path_buf(),
                virtual_version: None,
                real_version: None
            }
        )
    }
}


impl WriteOperation<VirtualFileSystem> for Virtual<RemoveOperation>{
    fn execute(&mut self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match Virtual(StatusQuery::new(self.0.path.as_path())).retrieve(&fs)? {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity) => {
                fs.mut_sub_state().attach_virtual(&virtual_identity)?;
                self.0.virtual_version = Some(VirtualVersion::increment());
            },
            IdentityStatus::ExistsVirtually(virtual_identity) => {
                fs.mut_add_state().detach(virtual_identity.as_identity())?;
                if let Some(source) = virtual_identity.as_source() {
                    match Virtual(StatusQuery::new(source)).retrieve(&fs)? {
                        IdentityStatus::Replaced(virtual_path) => {
                            if fs.add_state().is_directory_empty(virtual_path.as_identity()) {
                                fs.mut_add_state().detach(virtual_path.as_identity())?;
                            }
                        }
                        _ => {}
                    }
                }
            }
            IdentityStatus::NotExists | IdentityStatus::Removed | IdentityStatus::RemovedVirtually =>
                return Err(VfsError::DoesNotExists(self.0.path.to_path_buf()))
            ,
        }
        Ok(())
    }

    fn virtual_version(&self) -> Option<usize> {
        self.0.virtual_version
    }
    fn real_version(&self) -> Option<usize> { None }
}

impl Real<RemoveOperation> {
    pub fn new(path: &Path, virtual_version: Option<usize>) -> Real<RemoveOperation> {
        Real(
            RemoveOperation {
                path: path.to_path_buf(),
                virtual_version,
                real_version: None
            }
        )
    }
}

impl WriteOperation<RealFileSystem> for Real<RemoveOperation>{
    fn execute(&mut self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        match fs.remove(self.0.path.as_path()) {
            Ok(_) => { self.0.real_version = Some(RealVersion::increment()); Ok(()) },
            Err(error) => Err(VfsError::from(error))
        }
    }

    fn virtual_version(&self) -> Option<usize> {
        self.0.virtual_version
    }
    fn real_version(&self) -> Option<usize> { self.0.real_version }
}
