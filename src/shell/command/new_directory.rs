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

use vfs::{VirtualKind, VirtualFileSystem, WriteOperation, Virtual, CreateOperation};
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;

pub struct NewDirectoryCommand {}

impl Command for NewDirectoryCommand {
    const NAME : &'static str = "mkdir";

    fn new(cwd: &Path, args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        Ok(
            Box::new(
                InitializedNewDirectoryCommand {
                    path: Self::extract_path_from_args(cwd, args, "path")?
                }
            )
        )
    }
}

pub struct InitializedNewDirectoryCommand {
    pub path: PathBuf
}

impl InitializedCommand for InitializedNewDirectoryCommand {
    fn execute(&self, mut vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match Virtual::<CreateOperation>::new(
            self.path.as_path(),
            VirtualKind::Directory
        ).execute(&mut vfs) {
            Ok(_)       => Ok(()),
            Err(error)  => Err(CommandError::from(error))
        }
    }
}
