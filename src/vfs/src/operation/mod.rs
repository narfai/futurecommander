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

mod apply;
pub use self::apply::ApplyOperation;

//use crate::file_system::{ VirtualFileSystem, RealFileSystem };
//use crate::{ Virtual, Real };

pub trait WriteOperation <F: ?Sized> {
    fn execute(&mut self, fs: &mut F) -> Result<(), crate::errors::VfsError>;
    fn virtual_version(&self) -> Option<usize>;
    fn real_version(&self) -> Option<usize>;
}

//impl <F> std::fmt::Debug for dyn WriteOperation <F> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{:?}", &self)
//    }
//}


