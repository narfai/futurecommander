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

use std::{ path::{ Path } };

use crate::{
    errors::{
      QueryError
    },
    port::{
        ReadableFileSystem,
        FileSystemAdapter,
        EntryAdapter,
        EntryCollection,
    },
    infrastructure::virt::{
        VirtualFileSystem,
        entry_status::{ VirtualStatus }
    }
};

impl ReadableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    type Result = EntryAdapter<VirtualStatus>;
    //Read virtual specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Result>, QueryError> { unimplemented!() }
    fn status(&self, path: &Path) -> Result<Self::Result, QueryError> { unimplemented!() }
}
