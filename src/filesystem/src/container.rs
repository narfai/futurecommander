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
    path::Path,
    collections::vec_deque::VecDeque
};

use crate::{
    errors:: { BusinessError, QueryError },
    port::{
        Listener,
        ReadableFileSystem,
//        WriteableFileSystem,
        FileSystemAdapter,
        Entry,
        EntryAdapter,
        EntryCollection,
        Event
    },
    infrastructure::{
        VirtualFileSystem,
        VirtualStatus,
        RealFileSystem
    }
};

pub struct Container {
    virtual_fs: FileSystemAdapter<VirtualFileSystem>,
    real_fs: FileSystemAdapter<RealFileSystem>,
    transaction: VecDeque<Box<Event<Entry>>>
}

impl Container {
    pub fn new() -> Container {
        Container {
            virtual_fs: FileSystemAdapter(VirtualFileSystem::default()),
            real_fs: FileSystemAdapter(RealFileSystem::default()),
            transaction: VecDeque::new()
        }
    }

    fn apply(&mut self) -> Result<(), BusinessError> {
        while let Some(event) = self.transaction.pop_front() {
            match self.emit(event) {
                Err(error) => {
                    self.transaction.push_front(event);
                    return Err(error);
                },
                Ok(_) => Ok(())
            };
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.transaction.clear()
    }
}

impl ReadableFileSystem for Container {
    type Result = EntryAdapter<VirtualStatus>;

    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Result>,QueryError> {
        self.virtual_fs.read_dir(path)
    }

    fn status(&self, path: &Path) -> Result<Self::Result, QueryError> {
        self.virtual_fs.status(path)
    }
}

impl Listener for Container {
    fn emit(&mut self, event: Box<Event<Entry>>) -> Result<(), BusinessError> {
        event.atomize(&self.virtual_fs)?
             .apply(&mut self.virtual_fs)?;

        self.transaction.push_back(event);
        Ok(())
    }
}


