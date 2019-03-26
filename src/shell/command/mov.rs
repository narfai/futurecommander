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

use vfs::VirtualFileSystem;
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;


pub struct MoveCommand {}

impl Command for MoveCommand {
    const NAME : &'static str = "mv";

    fn new(cwd: &Path, args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        let source = Self::extract_path_from_args(cwd, args, "source")?;
        let destination = Self::extract_path_from_args(cwd, args, "destination")?;

        Ok(
            Box::new(
                InitializedMoveCommand {
                    source,
                    destination,
                }
            )
        )
    }
}

pub struct InitializedMoveCommand {
    source: PathBuf,
    destination: PathBuf
}

impl InitializedCommand for InitializedMoveCommand {
    fn execute(&self, vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match vfs.copy(
            self.source.as_path(),
            self.destination.as_path()
        ) {
            Ok(_) =>
                match vfs.remove(self.source.as_path()) {
                    Ok(_) => { println!("MOVE {:?} to {:?}", self.source, self.destination); Ok(()) },
                    Err(error) => Err(CommandError::from(error))
                },
            Err(error) => Err(CommandError::from(error))
        }
    }
}
