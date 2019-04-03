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
//use crate::{ VirtualFileSystem, RealFileSystem, VfsError, VirtualKind, VirtualPath };
use crate::representation::{ VirtualKind };
use crate::file_system::{ VirtualFileSystem, RealFileSystem };
use crate::operation::{ WriteOperation };
use crate::query::{ReadQuery, StatusQuery};

#[derive(Debug, Clone)]
pub struct CreateOperation {
    path: PathBuf,
    kind: VirtualKind
}

impl CreateOperation {
    pub fn new(path: &Path, kind: VirtualKind) -> CreateOperation {
        CreateOperation {
            path: path.to_path_buf(),
            kind
        }
    }
}

impl WriteOperation<VirtualFileSystem> for CreateOperation{
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match StatusQuery::new(self.path.as_path()).retrieve(&fs)?.virtual_identity() {
            Some(_) => return Err(VfsError::AlreadyExists(self.path.to_path_buf())),
            None => fs.mut_add_state()
                .attach(self.path.as_path(), None, self.kind)
        }
    }
}

impl WriteOperation<RealFileSystem> for CreateOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        match fs.create(self.path.as_path(), false) {
            Ok(_) => Ok(()),
            Err(error) => Err(VfsError::from(error))
        }
    }
}
