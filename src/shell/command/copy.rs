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

use vfs::{ VirtualFileSystem, VirtualPath, VfsError, VirtualKind, IdentityStatus };
use std::path::{ Path, PathBuf, MAIN_SEPARATOR };
use clap::ArgMatches;
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;
use crate::path::normalize;
use std::ffi::{ OsStr, OsString };

pub struct CopyCommand {}

impl Command for CopyCommand {
    const NAME : &'static str = "copy";

    fn new(cwd: &Path, args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        let source = Self::extract_path_from_args(cwd, args, "source")?;
        let (name, destination) = Self::extract_name_and_destination(cwd, args)?;

        Ok(
            Box::new(
                InitializedCopyCommand {
                    source,
                    destination,
                    name
                }
            )
        )
    }
}

pub struct InitializedCopyCommand {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub name: Option<OsString>
}

impl InitializedCommand for InitializedCopyCommand {
    fn execute(&self, vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match vfs.copy(
            self.source.as_path(),
            self.destination.as_path(),
            self.name.clone()
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(CommandError::from(error))
        }
    }
}
