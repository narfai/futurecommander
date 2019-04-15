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

use std::path::{ Path, PathBuf };

use crate::Kind;
use crate::VfsError;

pub trait Operation<F: ?Sized>: std::fmt::Debug {
    fn execute(&self, fs: &mut F) -> Result<(), VfsError>;
}

#[derive(Debug, Clone)]
pub struct CopyOperation {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool
}

impl CopyOperation {
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> CopyOperation {
        CopyOperation {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            merge,
            overwrite
        }
    }

    pub fn source(&self) -> &Path { self.source.as_path() }
    pub fn destination(&self) -> &Path { self.destination.as_path() }
    pub fn merge(&self) -> bool { self.merge }
    pub fn overwrite(&self) -> bool { self.overwrite }
}

#[derive(Debug, Clone)]
pub struct CreateOperation {
    path: PathBuf,
    kind: Kind,
    recursive: bool,
    overwrite: bool
}

impl CreateOperation {
    pub fn new(path: &Path, kind: Kind, recursive: bool, overwrite: bool) -> CreateOperation {
        CreateOperation {
            path: path.to_path_buf(),
            kind,
            recursive,
            overwrite
        }
    }

    pub fn path(&self) -> &Path { self.path.as_path() }
    pub fn kind(&self) -> Kind { self.kind }
    pub fn recursive(&self) -> bool { self.recursive }
    pub fn overwrite(&self) -> bool { self.overwrite }
}

#[derive(Debug, Clone)]
pub struct MoveOperation {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool //To honour overwrite or merge error, we should crawl recursively the entire vfs children of dst ...
}

impl MoveOperation {
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> MoveOperation {
        MoveOperation {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            merge,
            overwrite
        }
    }

    pub fn source(&self) -> &Path { self.source.as_path() }
    pub fn destination(&self) -> &Path { self.destination.as_path() }
    pub fn merge(&self) -> bool { self.merge }
    pub fn overwrite(&self) -> bool { self.overwrite }
}

#[derive(Debug, Clone)]
pub struct RemoveOperation {
    path: PathBuf,
    recursive: bool
}

impl RemoveOperation {
    pub fn new(path: &Path, recursive: bool) -> RemoveOperation {
        RemoveOperation {
            path: path.to_path_buf(),
            recursive
        }
    }

    pub fn path(&self) -> &Path { self.path.as_path() }
    pub fn recursive(&self) -> bool { self.recursive }
}
