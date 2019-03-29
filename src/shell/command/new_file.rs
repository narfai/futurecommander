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

use vfs::{ VirtualKind, VirtualFileSystem, WriteOperation, Virtual, Create };
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;

pub struct NewFileCommand {}

impl Command for NewFileCommand {
    const NAME : &'static str = "touch";

    fn new(cwd: &Path, args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        Ok(
            Box::new(
                InitializedNewFileCommand {
                    path: Self::extract_path_from_args(cwd, args, "path")?
                }
            )
        )
    }
}

pub struct InitializedNewFileCommand {
    pub path: PathBuf
}

impl InitializedCommand for InitializedNewFileCommand {
    fn execute(&self, mut vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match Virtual(Create::new(
            self.path.as_path(),
            VirtualKind::File
        )).execute(&mut vfs) {
            Ok(_)       => Ok(()),
            Err(error)  => Err(CommandError::from(error))
        }
    }
}
