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
use crate::{ VirtualFileSystem, VfsError, VirtualKind, VirtualPath };
use crate::operation::{ReadQuery, WriteOperation, Virtual, Status };

pub struct Create {
    path: PathBuf,
    kind: VirtualKind
}

impl Create {
    pub fn new(path: &Path, kind: VirtualKind) -> Create {
        Create {
            path: path.to_path_buf(),
            kind
        }
    }
}

impl WriteOperation<&mut VirtualFileSystem> for Virtual<Create>{
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match Virtual(Status::new(self.0.path.as_path())).retrieve(&fs)?.virtual_identity() {
            Some(_) => return Err(VfsError::AlreadyExists(self.0.path.to_path_buf())),
            None => {
                fs.mut_add_state().attach(self.0.path.as_path(), None, self.0.kind)?;
                Ok(())
            }
        }
    }
}
