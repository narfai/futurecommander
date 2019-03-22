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

#[macro_use]
extern crate clap;

mod path;
mod operation;
mod shell;

pub use self::shell::Shell;
pub use operation::copy::CopyOperation;
pub use operation::list::ListOperation;
pub use operation::new_directory::NewDirectoryOperation;
pub use operation::mov::MoveOperation;
pub use operation::new_file::NewFileOperation;
pub use operation::remove::RemoveOperation;
