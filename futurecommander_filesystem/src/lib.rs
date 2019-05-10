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

mod event;

mod port;
mod infrastructure;
mod errors;
mod container;

pub use futurecommander_representation::Kind;

pub use self::{
    errors::{ DomainError, QueryError },
    event::{
        Listener,
        Delayer,
        Event,
        capability
    },
    port::{
        Entry,
        ReadableFileSystem,
        WriteableFileSystem,
        EntryAdapter
    },
    event::*,
    container::Container
};

//Mainly for testing
pub use self::infrastructure::VirtualState;

pub mod tools;

#[cfg_attr(tarpaulin, skip)]
pub mod sample;
