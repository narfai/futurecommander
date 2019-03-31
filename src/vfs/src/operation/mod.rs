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
#[cfg(test)]
mod test;

mod copy;
pub use self::copy::Copy;

mod remove;
pub use self::remove::Remove;

mod create;
pub use self::create::Create;

mod status;
pub use self::status::{ Status };

mod read_dir;
pub use self::read_dir::ReadDir;

mod entry;
pub use self::entry::{ NodeIterator, Entry, Node, EntryCollection };

pub struct Real<O>(pub O);
pub struct Virtual<O>(pub O);

use crate::errors::VfsError;

pub trait WriteOperation <F> {
    fn execute(&self, fs: F) -> Result<(), VfsError>;
}

pub trait ReadQuery<F, T> {
    fn retrieve(&self, fs: F) -> Result<T, VfsError>;
}
