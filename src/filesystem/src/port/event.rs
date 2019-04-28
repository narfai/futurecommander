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

use crate::{
    errors::DomainError,
    port::{
        Entry,
        ReadableFileSystem,
        AtomicTransaction,
    }
};

#[typetag::serde(tag = "type")]
pub trait SerializableEvent {
    fn serializable(&self) -> Box<SerializableEvent>;
}

pub trait Event<E, F> : SerializableEvent
    where F: ReadableFileSystem<Item=E>,
          E: Entry {

    fn atomize(&self, fs: &F) -> Result<AtomicTransaction, DomainError>;
}


pub trait Listener<E> {
    fn emit(&mut self, event: E) -> Result<(), DomainError>;
}

pub trait Delayer<E> {
    fn delay(&mut self, event: E);
}
