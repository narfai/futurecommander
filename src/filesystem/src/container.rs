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
    path::{ Path, PathBuf },
    collections::vec_deque::VecDeque
};

use crate::{
    errors:: {DomainError, QueryError },
    port::{
        Listener,
        ReadableFileSystem,
        WriteableFileSystem,
        FileSystemAdapter,
        Entry,
        EntryAdapter,
        EntryCollection,
        Event,
        Delayer
    },
    infrastructure::{
        VirtualFileSystem,
        VirtualStatus,
        RealFileSystem
    }
};

type RealEvent      = Event<EntryAdapter<PathBuf>,          FileSystemAdapter<RealFileSystem>>;
type VirtualEvent   = Event<EntryAdapter<VirtualStatus>,    FileSystemAdapter<VirtualFileSystem>> ;

pub struct Container {
    virtual_fs: FileSystemAdapter<VirtualFileSystem>,
    real_fs:    FileSystemAdapter<RealFileSystem>,
    transaction: VecDeque<Box<RealEvent>>
}

impl Container {
    pub fn new() -> Container {
        Container {
            virtual_fs: FileSystemAdapter(VirtualFileSystem::default()),
            real_fs:    FileSystemAdapter(RealFileSystem::default()),
            transaction: VecDeque::new()
        }
    }

    pub fn apply(&mut self) -> Result<(), DomainError> {
        while let Some(event) = self.transaction.pop_front() {
            &event.atomize(&self.real_fs)?
                .apply(&mut self.real_fs)?;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.virtual_fs.as_inner_mut().reset();
        self.transaction.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.virtual_fs.as_inner().is_empty()
    }
}

impl ReadableFileSystem for Container {
    type Item = EntryAdapter<VirtualStatus>;

    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Item>,QueryError> {
        self.virtual_fs.read_dir(path)
    }

    fn status(&self, path: &Path) -> Result<Self::Item, QueryError> {
        self.virtual_fs.status(path)
    }
}

impl Delayer<Box<RealEvent>> for Container {
    fn delay(&mut self, event: Box<RealEvent>) {
        self.transaction.push_back(event);
    }
}

impl Listener<&VirtualEvent> for Container {
    fn emit(&mut self, event: &VirtualEvent) -> Result<(), DomainError> {
        event.atomize(&self.virtual_fs)?
             .apply(&mut self.virtual_fs)?;
        Ok(())
    }
}


#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        event::CopyEvent,
        sample::Samples
    };

    #[test]
    fn copy_directory_recursively() {
        let chroot = Samples::init_simple_chroot("container_copy_directory_recursively");
        let mut container = Container::new();
        let event = CopyEvent::new(
            chroot.join("RDIR").as_path(),
            chroot.join("COPIED").as_path(),
            false,
            false
        );
        container.emit(&event).unwrap();
        container.delay(Box::new(event));

        assert!(container.status(chroot.join("COPIED").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEA").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEB").as_path()).unwrap().exists());
    }
}
