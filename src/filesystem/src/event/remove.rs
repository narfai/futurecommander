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
    path:: { Path, PathBuf }
};

use crate::{
    Kind,
    errors::{ DomainError },
    port::{
        Entry,
        ReadableFileSystem,
        Event,
        Atomic,
        AtomicTransaction
    }
};

#[derive(Debug, Clone)]
pub struct RemoveEvent {
    path: PathBuf,
    recursive: bool
}

impl RemoveEvent {
    pub fn new(path: &Path, recursive: bool) -> RemoveEvent {
        RemoveEvent {
            path: path.to_path_buf(),
            recursive
        }
    }

    pub fn path(&self) -> &Path { self.path.as_path() }
    pub fn recursive(&self) -> bool { self.recursive }
}

impl <E, F> Event <E, F> for RemoveEvent where F: ReadableFileSystem<Item=E>, E: Entry {
    fn atomize(&self, fs: &F) -> Result<AtomicTransaction, DomainError> {
        let entry = fs.status(self.path())?;
        let mut transaction = AtomicTransaction::default();

        if !entry.exists() {
            return Err(DomainError::DoesNotExists(self.path().to_path_buf()))
        }

        if entry.is_file() {
            transaction.add(Atomic::RemoveFile(entry.path().to_path_buf()));
        } else if entry.is_dir() {
            let children = fs.read_dir(entry.path())?;

            if children.is_empty() {
                transaction.add(Atomic::RemoveEmptyDirectory(entry.path().to_path_buf()))
            } else if self.recursive(){
                for child in children {
                    transaction.merge(
                        RemoveEvent::new(
                            child.path(),
                            true
                        ).atomize(fs)?
                    )
                }
                transaction.add(Atomic::RemoveEmptyDirectory(entry.path().to_path_buf()))
            } else {
                return Err(DomainError::DeleteRecursiveNotAllowed(entry.path().to_path_buf()))
            }
        }

        Ok(transaction)
    }
}
