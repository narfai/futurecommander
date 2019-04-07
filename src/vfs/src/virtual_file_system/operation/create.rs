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
use std::path::Ancestors;

use crate::{ VirtualFileSystem, VfsError, Kind };
use crate::operation::{Operation, CreateOperation };
use crate::query::{Query, StatusQuery, Entry };
use crate::representation::{ VirtualPath };

impl Operation<VirtualFileSystem> for CreateOperation{
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let path = self.path();
        match StatusQuery::new(path).retrieve(&fs)?.into_inner().into_existing_virtual() {
            Some(_) => return Err(VfsError::AlreadyExists(path.to_path_buf())),
            None => {
                if ! self.recursive() || self.kind() == Kind::File {
                    let parent = VirtualPath::get_parent_or_root(path);
                    let parent_status = StatusQuery::new(parent.as_path()).retrieve(&fs)?;
                    if ! parent_status.exists() {
                        return Err(VfsError::DoesNotExists(path.to_path_buf()));
                    } else if !parent_status.is_dir() {
                        return Err(VfsError::IsNotADirectory(path.to_path_buf()));
                    }
                    fs.mut_add_state().attach(path, None, self.kind())
                } else {
                    fn recursive_dir_creation(fs: &mut VirtualFileSystem, mut ancestors: &mut Ancestors<'_>) -> Result<(), VfsError> {
                        if let Some(path) = ancestors.next() {
                            if ! StatusQuery::new(path).retrieve(&fs)?.exists() {
                                recursive_dir_creation(fs, &mut ancestors)?;
                                fs.mut_add_state().attach(path, None, Kind::Directory)?;
                            }
                        }
                        Ok(())
                    }
                    recursive_dir_creation(fs, &mut path.ancestors())
                }
            }
        }
    }
}
