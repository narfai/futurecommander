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
    errors::{ BusinessError },
    port::{
        Entry,
        ReadableFileSystem,
        Event,
        AtomicTransaction
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

impl <E: Entry> Event<E> for CopyEvent {
    fn atomize(&self, fs: &ReadableFileSystem<Result=E>) -> Result<AtomicTransaction, BusinessError> {
        //Business
        unimplemented!()
    }
}
