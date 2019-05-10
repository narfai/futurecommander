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

mod copy;
mod create;
mod mov;
mod remove;
mod serializable;

pub mod capability;

pub use self::{
    copy::CopyEvent,
    create::CreateEvent,
    mov::MoveEvent,
    remove::RemoveEvent,
    serializable::SerializableEvent
};

use std::{
    fmt::Debug,
    path::{ PathBuf }
};

use crate::{
    errors::DomainError,
    capability::{
        RegistrarGuard
    },
    port::{
        Entry,
        EntryAdapter,
        FileSystemAdapter,
        ReadableFileSystem,
        AtomicTransaction
    },
    infrastructure::{
        VirtualStatus,
        VirtualFileSystem,
        RealFileSystem
    }
};

pub trait Event<E, F> : SerializableEvent + Debug
    where F: ReadableFileSystem<Item=E>,
          E: Entry {

    fn atomize(&self, fs: &F, guard: &mut capability::Guard) -> Result<AtomicTransaction, DomainError>;
}

pub type RawVirtualEvent = Event<EntryAdapter<VirtualStatus>, FileSystemAdapter<VirtualFileSystem>>;
pub struct VirtualEvent(pub Box<RawVirtualEvent>);
impl VirtualEvent {
    pub fn as_inner(&self) -> &RawVirtualEvent {
        &*self.0
    }
    pub fn into_inner(self) -> Box<RawVirtualEvent>{
        self.0
    }
}

pub type RawRealEvent = Event<EntryAdapter<PathBuf>, FileSystemAdapter<RealFileSystem>>;

#[derive(Debug)]
pub struct RealEvent(pub Box<RawRealEvent>);
impl RealEvent {
    pub fn as_inner(&self) -> &RawRealEvent {
        &*self.0
    }
    pub fn into_inner(self) -> Box<RawRealEvent> {
        self.0
    }
}


pub trait Listener<E> {
    fn emit(&mut self, event: &RawVirtualEvent, guard: RegistrarGuard) -> Result<RegistrarGuard, DomainError>;
}

pub trait Delayer {
    type Event;
    fn delay(&mut self, event: Self::Event, guard: RegistrarGuard);
}
