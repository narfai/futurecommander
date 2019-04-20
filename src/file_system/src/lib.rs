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

mod errors;

mod shared;

mod virt;
mod real;
mod hybrid;

//Internal but open for debug / extend
pub mod representation;

//Read API
pub use self::virt::query;
pub use self::shared::kind::Kind;
pub use self::shared::path_helper;

//Write API
pub mod operation;

//Containers
pub use self::real::RealFileSystem;
pub use self::virt::VirtualFileSystem;
pub use self::hybrid::HybridFileSystem;

//Errors
pub use self::errors::OperationError;

#[cfg_attr(tarpaulin, skip)]
pub use self::shared::sample::Samples;
