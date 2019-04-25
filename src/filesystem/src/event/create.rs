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
    path:: { Path, PathBuf, Ancestors }
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
pub struct CreateEvent {
    path: PathBuf,
    kind: Kind,
    recursive: bool,
    overwrite: bool
}

impl CreateEvent {
    pub fn new(path: &Path, kind: Kind, recursive: bool, overwrite: bool) -> CreateEvent {
        CreateEvent {
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

impl <E, F> Event <E, F> for CreateEvent where F: ReadableFileSystem<Item=E>, E: Entry {
    fn atomize(&self, fs: &F) -> Result<AtomicTransaction, DomainError> {
        let entry = fs.status(self.path())?;
        let mut transaction = AtomicTransaction::default();

        fn recursive_dir_creation<E, F> (fs: &F, mut ancestors: &mut Ancestors<'_>) -> Result<AtomicTransaction, DomainError>
            where F: ReadableFileSystem<Item=E>,
                  E: Entry {
            let mut transaction = AtomicTransaction::default();
            if let Some(path) = ancestors.next() {
                let entry = fs.status(path)?;
                if !entry.exists() {
                    transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
                    transaction.add(Atomic::CreateEmptyDirectory(entry.to_path()));
                }
            }
            Ok(transaction)
        }

        self.path().ancestors().next();
        let mut ancestors = self.path().ancestors();

        match self.kind() {
            Kind::Directory => {
                if entry.exists() {
                    return Err(DomainError::Custom("Directory overwrite not allowed".to_string()))
                } else {
                    if self.recursive() {
                        transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
                    }
                    transaction.add(Atomic::CreateEmptyDirectory(entry.to_path()));
                }
            },
            Kind::File => {
                if entry.exists() {
                    if self.overwrite() {
                        if self.recursive() {
                            transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
                        }
                        transaction.add(Atomic::RemoveFile(entry.to_path()));
                        transaction.add(Atomic::CreateEmptyFile(entry.to_path()));
                    } else {
                        return Err(DomainError::Custom("Overwrite not allowed".to_string()))
                    }
                } else {
                    transaction.add(Atomic::CreateEmptyFile(entry.to_path()));
                }
            },
            Kind::Unknown => {
                return Err(DomainError::Custom("Cannot create with unknown kind".to_string()))
            }
        }
        Ok(transaction)
    }
}
