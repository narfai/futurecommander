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
use crate::{ VirtualFileSystem, VfsError };
use crate::operation::{ WriteOperation, Virtual };

pub struct Remove {
    path: PathBuf
}

impl Remove {
    pub fn new(path: &Path) -> Remove {
        Remove {
            path: path.to_path_buf()
        }
    }
}

impl WriteOperation<&mut VirtualFileSystem> for Virtual<Remove>{
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match fs.stat(self.0.path.as_path())? {
            Some(virtual_identity) => {
                fs.mut_sub_state().attach_virtual(&virtual_identity)?;
                if fs.add_state().get(virtual_identity.as_identity())?.is_some() {
                    fs.mut_add_state().detach(virtual_identity.as_identity())?;
                }
                Ok(())
            },
            None => return Err(VfsError::DoesNotExists(self.0.path.to_path_buf()))
        }
    }
}
