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
//use crate::{ VirtualFileSystem, RealFileSystem, VfsError, VirtualKind, VirtualPath };
use crate::representation::{ VirtualKind };
use crate::file_system::{ VirtualFileSystem, RealFileSystem, VirtualVersion, RealVersion };
use crate::operation::{ WriteOperation };
use crate::query::{ReadQuery, StatusQuery};

pub struct CreateOperation {
    path: PathBuf,
    kind: VirtualKind,
    virtual_version: Option<usize>,
    real_version: Option<usize>
}

impl Virtual<CreateOperation> {
    pub fn new(path: &Path, kind: VirtualKind) -> Virtual<CreateOperation> {
        Virtual(
            CreateOperation {
                path: path.to_path_buf(),
                kind,
                virtual_version: None,
                real_version: None
            }
        )
    }
}

impl WriteOperation<VirtualFileSystem> for Virtual<CreateOperation>{
    fn execute(&mut self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match Virtual(StatusQuery::new(self.0.path.as_path())).retrieve(&fs)?.virtual_identity() {
            Some(_) => return Err(VfsError::AlreadyExists(self.0.path.to_path_buf())),
            None => {
                fs.mut_add_state().attach(self.0.path.as_path(), None, self.0.kind)?;
                self.0.virtual_version = Some(VirtualVersion::increment());
                Ok(())
            }
        }
    }

    fn virtual_version(&self) -> Option<usize> {
        self.0.virtual_version
    }
    fn real_version(&self) -> Option<usize> { None }
}


impl Real<CreateOperation> {
    pub fn new(path: &Path, kind: VirtualKind, virtual_version: Option<usize>) -> Real<CreateOperation> {
        Real(
            CreateOperation {
                path: path.to_path_buf(),
                kind,
                virtual_version,
                real_version: None
            }
        )
    }
}

impl WriteOperation<RealFileSystem> for Real<CreateOperation>{
    fn execute(&mut self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        match fs.create(self.0.path.as_path(), false) {
            Ok(_) => { self.0.real_version = Some(RealVersion::increment()); Ok(()) },
            Err(error) => Err(VfsError::from(error))
        }
    }

    fn virtual_version(&self) -> Option<usize> {
        self.0.virtual_version
    }
    fn real_version(&self) -> Option<usize> { self.0.real_version }
}
