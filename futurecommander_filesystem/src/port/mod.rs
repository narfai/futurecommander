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

mod entry;
mod entry_collection;
mod filesystem;
mod atomic;
mod serializable;

pub use self::{
    entry_collection::{ EntryCollection },
    entry::{ EntryAdapter, Entry },
    filesystem::{ FileSystemAdapter, WriteableFileSystem, ReadableFileSystem, FileSystemTransaction },
    atomic::{ AtomicTransaction, Atomic },
    serializable::SerializableEntry
};
