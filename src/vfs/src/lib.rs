/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommanderVfs.
 *
 * FutureCommanderVfs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommanderVfs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommanderVfs.  If not, see <https://www.gnu.org/licenses/>.
 */

#[cfg(test)]
mod test;

mod representation;
mod operation;

mod real_fs;
mod virtual_fs;
mod errors;

pub use self::errors::VfsError;
pub use self::representation::{ VirtualPath, VirtualKind, VirtualDelta, VirtualChildren, VirtualChildrenIterator };
pub use self::virtual_fs::VirtualFileSystem;
pub use self::real_fs::RealFileSystem;
pub use self::operation::{ Copy, Create, Remove, ReadDir, Status, IdentityStatus };

