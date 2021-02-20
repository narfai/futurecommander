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
    fmt::Debug
};

use serde::{Serialize, Deserialize};

mod copy;
mod create;
mod mov;
mod remove;

pub mod capability;

pub use self::{
    copy::CopyEvent,
    create::CreateEvent,
    mov::MoveEvent,
    remove::RemoveEvent
};

use crate::{
    errors::DomainError,
    capability::{
        Guard,
        RegistrarGuard
    },
    port::{
        Entry,
        ReadableFileSystem,
        AtomicTransaction
    }
};

pub trait Listener {
    fn emit(&mut self, event: &FileSystemEvent, guard: RegistrarGuard) -> Result<RegistrarGuard, DomainError>;
}

pub trait Delayer {
    fn delay(&mut self, event: FileSystemEvent, guard: RegistrarGuard);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileSystemEvent {
    Create(create::CreateEvent),
    Copy(copy::CopyEvent),
    Move(mov::MoveEvent),
    Remove(remove::RemoveEvent)
}

impl FileSystemEvent {
    pub fn atomize<E: Entry, F: ReadableFileSystem<Item=E>>(&self, fs: &F, guard: &mut dyn Guard) -> Result<AtomicTransaction, DomainError> {
        match self {
            FileSystemEvent::Create(event) => create::atomize(event, fs, guard),
            FileSystemEvent::Copy(event) => copy::atomize(event, fs, guard),
            FileSystemEvent::Move(event) => mov::atomize(event, fs, guard),
            FileSystemEvent::Remove(event) => remove::atomize(event, fs, guard),
        }
    }
}