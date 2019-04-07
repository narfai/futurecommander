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

use std::path::{ PathBuf, Path, Ancestors };
use crate::{ VfsError };
//use crate::{ VirtualFileSystem, RealFileSystem, VfsError, VirtualKind, VirtualPath };
use crate::representation::{ VirtualKind, VirtualPath };
use crate::file_system::{ VirtualFileSystem, RealFileSystem };
use crate::operation::{ WriteOperation };
use crate::query::{ReadQuery, StatusQuery};

#[derive(Debug, Clone)]
pub struct CreateOperation {
    path: PathBuf,
    kind: VirtualKind,
    recursive: bool
}

impl CreateOperation {
    pub fn new(path: &Path, kind: VirtualKind) -> CreateOperation {
        CreateOperation {
            path: path.to_path_buf(),
            kind,
            recursive: false
        }
    }
}

impl WriteOperation<VirtualFileSystem> for CreateOperation{
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match StatusQuery::new(self.path.as_path()).retrieve(&fs)?.as_virtual_identity() {
            Some(_) => return Err(VfsError::AlreadyExists(self.path.to_path_buf())),
            None => {
                if ! self.recursive || self.kind == VirtualKind::File {
                    let parent = VirtualPath::get_parent_or_root(self.path.as_path());
                    let parent_status = StatusQuery::new(parent.as_path()).retrieve(&fs)?;
                    if ! parent_status.exists() {
                        return Err(VfsError::DoesNotExists(self.path.to_path_buf()));
                    } else if !parent_status.is_dir() {
                        return Err(VfsError::IsNotADirectory(self.path.to_path_buf()));
                    }
                    fs.mut_add_state().attach(self.path.as_path(), None, self.kind)
                } else {
                    fn recursive_dir_creation(fs: &mut VirtualFileSystem, mut ancestors: &mut Ancestors<'_>) -> Result<(), VfsError> {
                        if let Some(path) = ancestors.next() {
                            if ! StatusQuery::new(path).retrieve(&fs)?.exists() {
                                recursive_dir_creation(fs, &mut ancestors)?;
                                fs.mut_add_state().attach(path, None, VirtualKind::Directory)?;
                            }
                        }
                        Ok(())
                    }
                    recursive_dir_creation(fs, &mut self.path.ancestors())
                }
            }
        }
    }
}

impl WriteOperation<RealFileSystem> for CreateOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        if self.path.exists() {
            return Err(VfsError::AlreadyExists(self.path.to_path_buf()));
        }

        if ! self.recursive {
            let parent = VirtualPath::get_parent_or_root(self.path.as_path());
            if !parent.exists() {
                return Err(VfsError::DoesNotExists(self.path.to_path_buf()));
            } else if !parent.is_dir() {
                return Err(VfsError::IsNotADirectory(self.path.to_path_buf()));
            }
        }

        let result = match self.kind {
            VirtualKind::File => fs.create_file(self.path.as_path()),
            VirtualKind::Directory => fs.create_directory(self.path.as_path(), self.recursive),
            _ => Ok(())
        };

        match result {
            Err(error) => Err(VfsError::from(error)),
            Ok(_) => Ok(())
        }
    }
}
