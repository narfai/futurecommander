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
    errors:: { DomainError, QueryError },
    port::{
        Listener,
        ReadableFileSystem,
        FileSystemAdapter,
        EntryAdapter,
        EntryCollection,
        Event,
        SerializableEvent,
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

pub struct EventQueue(VecDeque<Box<RealEvent>>);

impl Default for EventQueue {
    fn default() -> EventQueue {
        EventQueue(VecDeque::new())
    }
}

impl EventQueue {
    pub fn pop_front(&mut self) -> Option<Box<RealEvent>>{
        self.0.pop_front()
    }

    pub fn push_back(&mut self, event: Box<RealEvent>){
        self.0.push_back(event)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        let mut serializable : Vec<Box<SerializableEvent>> = Vec::new();
        for event in self.0.iter() {
            serializable.push(event.serializable());
        }
        serde_json::to_string(&serializable)
    }
}


pub struct Container {
    virtual_fs: FileSystemAdapter<VirtualFileSystem>,
    real_fs:    FileSystemAdapter<RealFileSystem>,
    event_queue: EventQueue
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Container {
        Container {
            virtual_fs: FileSystemAdapter(VirtualFileSystem::default()),
            real_fs:    FileSystemAdapter(RealFileSystem::default()),
            event_queue: EventQueue::default()
        }
    }

    pub fn apply(&mut self) -> Result<(), DomainError> {
        while let Some(event) = self.event_queue.pop_front() {
            event.atomize(&self.real_fs)?
                .apply(&mut self.real_fs)?;
        }
        self.reset();
        Ok(())
    }

    pub fn reset(&mut self) {
        self.virtual_fs.as_inner_mut().reset();
        self.event_queue.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.virtual_fs.as_inner().is_empty()
    }

    pub fn vfs(&self) -> &FileSystemAdapter<VirtualFileSystem> {
        &self.virtual_fs
    }

    pub fn rfs(&self) -> &FileSystemAdapter<RealFileSystem> {
        &self.real_fs
    }

    pub fn to_json(&self) -> Result<String, DomainError> {
        Ok(self.event_queue.serialize()?)
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
        self.event_queue.push_back(event);
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
        sample::Samples,
        Entry
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
