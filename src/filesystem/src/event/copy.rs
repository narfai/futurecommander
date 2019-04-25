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
    path::{ PathBuf, Path },
};

use crate::{
    Kind,
    errors::{DomainError},
    port::{
        Entry,
        ReadableFileSystem,
        Event,
        AtomicTransaction,
        Atomic
    }
};

#[derive(Debug, Clone)]
pub struct CopyEvent {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool
}

impl CopyEvent {
    //App
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> CopyEvent {
        CopyEvent {
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

impl <E, F> Event <E, F> for CopyEvent where F: ReadableFileSystem<Item=E>, E: Entry {
    fn atomize(&self, fs: &F) -> Result<AtomicTransaction, DomainError> {
        let source = fs.status(self.source())?;

        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(self.source().to_path_buf()))
        }

        let mut transaction = AtomicTransaction::default();
        let destination = fs.status(self.destination())?;

        if destination.exists() {
            if source.is_dir() {
                if destination.is_dir() {
                    if self.merge() {
                        for child in fs.read_dir(source.path())? {
                            transaction.merge(
                                CopyEvent::new( //TODO here vfs
                                    child.path(),
                                    destination.path()
                                        .join(child.name().unwrap())
                                        .as_path(),
                                    self.merge(),
                                    self.overwrite()
                                ).atomize(fs)?
                            );
                        }
                    } else {
                        return Err(DomainError::Custom("Merge not allowed".to_string()))
                    }
                } else {
                    return Err(DomainError::Custom("Cannot merge file with directory".to_string()));
                }
            } else if source.is_file() {
                if destination.is_file() {
                    if self.overwrite() {
                        transaction.add(Atomic::RemoveFile(destination.to_path()));
                        transaction.add(Atomic::CopyFileToFile(source.to_path(), destination.to_path()));
                    } else {
                        return Err(DomainError::Custom("Overwrite not allowed".to_string()))
                    }
                } else {
                    return Err(DomainError::Custom("Cannot overwrite directory with file".to_string()))
                }
            }
        } else {
            if source.is_dir() {
                transaction.add(Atomic::CreateEmptyDirectory(destination.to_path()));
                for child in fs.read_dir(source.path())? {
                    transaction.merge(
                        CopyEvent::new(
                            child.path(),
                            destination.path()
                                .join(child.name().unwrap())
                                .as_path(),
                            self.merge(),
                            self.overwrite()
                        ).atomize(fs)?
                    );
                }
            } else if source.is_file() {
                transaction.add(Atomic::CopyFileToFile(source.to_path(), destination.to_path()));
            }
        }
        Ok(transaction)
    }
}
