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

use vfs::{ VirtualFileSystem, WriteOperation, Virtual, Copy, Remove };
use clap::ArgMatches;
use std::path::{ Path, PathBuf };
use std::ffi::{ OsString };
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;


pub struct MoveCommand {}

impl Command for MoveCommand {
    const NAME : &'static str = "mv";

    fn new(cwd: &Path, args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        let source = Self::extract_path_from_args(cwd, args, "source")?;
        for ancestor in cwd.ancestors() {
            if source.as_path() == ancestor {
                return Err(CommandError::CwdIsInside(source.to_path_buf()))
            }
        }
        let (name, destination) = Self::extract_name_and_destination(cwd, args)?;

        Ok(
            Box::new(
                InitializedMoveCommand {
                    source,
                    destination,
                    name
                }
            )
        )
    }
}

pub struct InitializedMoveCommand {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub name: Option<OsString>
}

impl InitializedCommand for InitializedMoveCommand {
    fn execute(&self, mut vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match Virtual(Copy::new(
            self.source.as_path(),
            self.destination.as_path(),
            self.name.clone()
        )).execute(&mut vfs) {
            Ok(_) =>
                match Virtual(Remove::new(self.source.as_path())).execute(&mut vfs) {
                    Ok(_) => { println!("MOVE {:?} to {:?}", self.source, self.destination); Ok(()) },
                    Err(error) => Err(CommandError::from(error))
                },
            Err(error) => Err(CommandError::from(error))
        }
    }
}
