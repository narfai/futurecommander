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
    path::{ Path },
    collections::vec_deque::VecDeque
};

use crate::{
    errors:: { DomainError, QueryError },
    capability::{
        RegistrarGuard
    },
    event::{
        Listener,
        Delayer,
        RealEvent,
        RawRealEvent,
        RawVirtualEvent,
        SerializableEvent,
    },
    port::{
        ReadableFileSystem,
        FileSystemAdapter,
        EntryAdapter,
        EntryCollection

    },
    infrastructure::{
        VirtualFileSystem,
        VirtualStatus,
        RealFileSystem
    }
};

type Commitment = (RealEvent, RegistrarGuard);

#[derive(Debug)]
pub struct EventQueue(VecDeque<Commitment>);

impl Default for EventQueue {
    fn default() -> EventQueue {
        EventQueue(VecDeque::new())
    }
}

impl EventQueue {
    pub fn pop_front(&mut self) -> Option<Commitment>{
        self.0.pop_front()
    }

    pub fn push_back(&mut self, commitment: Commitment){
        self.0.push_back(commitment)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        let mut serializable : Vec<(Box<SerializableEvent>, &RegistrarGuard)> = Vec::new();
        for (event, guard) in self.0.iter() {
            serializable.push((event.as_inner().serializable(), &guard));
        }
        serde_json::to_string(&serializable)
    }
}

#[derive(Debug)]
pub struct Container {
    virtual_fs  : FileSystemAdapter<VirtualFileSystem>,
    real_fs     : FileSystemAdapter<RealFileSystem>,
    event_queue : EventQueue
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
        while let Some((RealEvent(event), mut guard)) = self.event_queue.pop_front() {
            event.atomize(&self.real_fs, &mut guard)?
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

    pub fn emit_json(&mut self, json: String) -> Result<(), DomainError> {
        let events : Vec<(Box<SerializableEvent>, RegistrarGuard)> = serde_json::from_str(json.as_str()).unwrap();
        for (event, guard) in events {
            let guard = self.emit(&*event.virt().into_inner(), guard)?;
            self.delay(event.real().into_inner(), guard );
        }
        Ok(())
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

impl Delayer for Container {
    type Event = Box<RawRealEvent>;
    fn delay(&mut self, event: Self::Event, guard: RegistrarGuard) {
        self.event_queue.push_back((RealEvent(event), guard));
    }
}


impl Listener<&RawVirtualEvent> for Container {
    fn emit(&mut self, event: &RawVirtualEvent, mut guard: RegistrarGuard) -> Result<RegistrarGuard, DomainError> {
        event.atomize(&self.virtual_fs, &mut guard)?
             .apply(&mut self.virtual_fs)?;
        Ok(guard)
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        event::CopyEvent,
        sample::Samples,
        Entry,
        capability::{
            ZealedGuard
        }
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

        let guard = container.emit(&event, RegistrarGuard::from(Box::new(ZealedGuard))).unwrap();
        container.delay(Box::new(event), guard);

        assert!(container.status(chroot.join("COPIED").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEA").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEB").as_path()).unwrap().exists());
    }

    #[test]
    fn can_export_virtual_state_into_json_string() {
        let chroot = Samples::init_simple_chroot("can_export_virtual_state_into_json_string");
        let mut container = Container::new();
        let event = CopyEvent::new(
            chroot.join("RDIR").as_path(),
            chroot.join("COPIED").as_path(),
            false,
            false
        );
        container.delay(Box::new(event), RegistrarGuard::from(Box::new(ZealedGuard)));
        let expected : String = format!(
            "[[{{\"type\":\"CopyEvent\",\"source\":\"{}\",\"destination\":\"{}\",\"merge\":false,\"overwrite\":false}},{{\"inner\":{{\"type\":\"ZealedGuard\"}},\"registry\":{{}}}}]]",
            chroot.join("RDIR").to_string_lossy(),
            chroot.join("COPIED").to_string_lossy(),
        );

        assert_eq!(container.to_json().unwrap(), expected);
    }

    #[test]
    fn can_import_virtual_state_from_json_string() {
        let chroot = Samples::init_simple_chroot("can_import_virtual_state_from_json_string");
        let mut container_a = Container::new();
        let event = CopyEvent::new(
            chroot.join("RDIR").as_path(),
            chroot.join("COPIED").as_path(),
            false,
            false
        );

        let guard = container_a.emit(&event, RegistrarGuard::from(Box::new(ZealedGuard))).unwrap();
        assert!(!container_a.is_empty());
        let a_stat = container_a.status(chroot.join("COPIED").as_path()).unwrap();
        assert!(a_stat.exists());
        assert!(a_stat.is_dir());

        container_a.delay(Box::new(event), guard);

        let mut container_b = Container::new();

        container_b.emit_json(container_a.to_json().unwrap()).unwrap();

        assert!(!container_b.is_empty());
        let b_stat = container_b.status(chroot.join("COPIED").as_path()).unwrap();
        assert!(b_stat.exists());
        assert!(b_stat.is_dir());
    }
}
