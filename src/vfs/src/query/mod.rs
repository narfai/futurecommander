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

mod status;
pub use self::status::StatusQuery;

mod read_dir;
pub use self::read_dir::ReadDirQuery;

mod entry;
pub use self::entry::{ NodeIterator, Entry, Node, EntryCollection };

mod identity_status;
pub use self::identity_status::{ IdentityStatus };

pub trait ReadQuery<F, T> {
    fn retrieve(&self, fs: F) -> Result<T, crate::errors::VfsError>;
}