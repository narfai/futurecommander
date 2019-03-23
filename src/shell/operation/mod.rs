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

pub mod copy;
pub use self::copy::CopyOperation;

pub mod list;
pub use self::list::ListOperation;

pub mod mov;
pub use self::mov::MoveOperation;

pub mod new_directory;
pub use self::new_directory::NewDirectoryOperation;

pub mod new_file;
pub use self::new_file::NewFileOperation;

pub mod remove;
pub use self::remove::RemoveOperation;

use futurecommandervfs::VirtualFileSystem;
use clap::ArgMatches;
use std::path::Path;

pub trait Operation {
    fn from_context(cwd: &Path, args: &ArgMatches) -> Self;
    fn execute(&self, vfs: &mut VirtualFileSystem);
}