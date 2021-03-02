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

use std::{
    path    ::{ Path }
};

use crate::{
    errors::{ QueryError },
    port::{
        Entry,
        EntryCollection
    },
    infrastructure::errors::InfrastructureError
};

#[derive(Debug)]
pub struct FileSystemAdapter<F>(pub F);
impl <F>FileSystemAdapter<F> {
    pub fn as_inner(&self) -> &F {
        &self.0
    }

    pub fn as_inner_mut(&mut self) -> &mut F {
        &mut self.0
    }
}

pub trait ReadableFileSystem {
    type Item : Entry;
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Item>,QueryError>;
    fn status(&self, path: &Path) -> Result<Self::Item, QueryError>;
    fn read_maintained(&self, path: &Path) -> Result<EntryCollection<Self::Item>,QueryError> {
        self.read_dir(path)
    }
    fn is_directory_empty(&self, path: &Path) -> Result<bool, QueryError>;
}

pub trait WriteableFileSystem: ReadableFileSystem {
    //Write API Interface
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn create_empty_file(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn copy_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>;
    fn move_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>;
    fn bind_directory_to_directory(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>;
    fn remove_file(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn remove_maintained_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        self.remove_empty_directory(path)
    }
}

pub trait FileSystemTransaction<F: WriteableFileSystem> {
    fn apply(self, fs: &mut F) -> Result<(), InfrastructureError>;
}

