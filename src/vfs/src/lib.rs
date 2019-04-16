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

mod shared;

mod virtual_file_system;
mod file_system;

//Internal
use self::shared::transaction::Transaction;

//Internal but open for debug / extend
pub mod representation;

//Read API
pub mod query;
pub use self::shared::kind::Kind;
pub use self::shared::path_helper;

//Write API
pub use self::shared::operation;

//Containers
pub use self::file_system::RealFileSystem;
pub use self::virtual_file_system::{ VirtualFileSystem,  HybridFileSystem };

//Errors
pub use self::shared::errors::VfsError;

#[cfg_attr(tarpaulin, skip)]
pub use self::shared::sample::Samples;
