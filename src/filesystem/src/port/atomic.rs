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
    path    ::{ Path, PathBuf },
    vec     ::{ IntoIter }
};

use crate::{
    errors  ::{ BusinessError },
    port    ::{
        WriteableFileSystem,
        FileSystemTransaction
    },
    infrastructure::{ errors:: InfrastructureError }

};

pub enum Atomic {
    CreateEmptyDirectory(PathBuf),
    CreateEmptyFile(PathBuf),
    CopyFileIntoDirectory(PathBuf, PathBuf),
    MoveFileIntoDirectory(PathBuf, PathBuf),
    RemoveFile(PathBuf),
    RemoveEmptyDirectory(PathBuf)
}

impl Atomic {
    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), InfrastructureError> {
        use self::Atomic::*;
        match self {
            CreateEmptyDirectory(path) => fs.create_empty_directory(path.as_path()),
            CreateEmptyFile(path) => fs.create_empty_file(path.as_path()),
            CopyFileIntoDirectory(source, destination) => fs.copy_file_into_directory(source.as_path(), destination.as_path()),
            MoveFileIntoDirectory(source, destination) => fs.move_file_into_directory(source.as_path(), destination.as_path()),
            RemoveFile(path) => fs.remove_file(path.as_path()),
            RemoveEmptyDirectory(path) => fs.remove_empty_directory(path.as_path())
        }
    }

    //TODO apply_at(op position) => makes the index bubble through errors
    //TODO rollback & reverse there
}

pub struct AtomicTransaction(pub Vec<Atomic>);

impl AtomicTransaction {
    pub fn add(&mut self, atomic: Atomic) {
        self.0.push(atomic)
    }

    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic in self.0 {
            atomic.apply(fs)?
        }
        Ok(())
    }
}

impl IntoIterator for AtomicTransaction {
    type Item = Atomic;
    type IntoIter = IntoIter<Atomic>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

