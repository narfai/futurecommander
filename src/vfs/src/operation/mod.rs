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
pub use self::copy::CopyOperation;

mod remove;
pub use self::remove::RemoveOperation;

mod create;
pub use self::create::CreateOperation;

mod transaction;
pub use self::transaction::Transaction;

pub trait WriteOperation <F: ?Sized>: std::fmt::Debug {
    fn execute(&self, fs: &mut F) -> Result<(), crate::errors::VfsError>;
}

//@TODO trait TranslationOperation & AtomicOperation ?
