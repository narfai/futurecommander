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
mod mock;

#[cfg(test)]
pub use self::mock::VfsMock;

mod representation;
mod file_system;
mod operation;
mod query;
mod errors;

pub use self::errors::VfsError;

pub use self::representation::*;
pub use self::file_system::*;
pub use self::operation::*;
pub use self::query::*;

pub struct Real<O>(pub O);

pub struct Virtual<O>(pub O);
