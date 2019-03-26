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

mod path;
mod delta;
mod file_system;
mod children;
mod errors;

pub use self::errors::VfsError;
pub use self::path::VirtualPath;
pub use self::path::VirtualKind;
pub use self::delta::VirtualDelta;
pub use self::children::VirtualChildrenIterator;
pub use self::children::VirtualChildren;
pub use self::file_system::VirtualFileSystem;
pub use self::file_system::IdentityStatus;



