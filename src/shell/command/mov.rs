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

#[allow(unused_imports)]
use vfs::WriteOperation;


use vfs::{RealFileSystem, HybridFileSystem, CopyOperation, RemoveOperation, Transaction};
use clap::ArgMatches;
use std::path::{ Path, PathBuf };
use std::ffi::{ OsString };
use crate::command::{ Command };
use crate::command::errors::CommandError;


pub struct MoveCommand {}

impl Command<MoveCommand> {
    pub const NAME : &'static str = "mv";

    pub fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedMoveCommand>, CommandError> {
        let source = Self::extract_path_from_args(cwd, args, "source")?;
        for ancestor in cwd.ancestors() {
            if source.as_path() == ancestor {
                return Err(CommandError::CwdIsInside(source.to_path_buf()))
            }
        }
        let (name, destination) = Self::extract_name_and_destination(cwd, args)?;

        Ok(
            Command(
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

impl Command<InitializedMoveCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let operation = CopyOperation::new(
            self.0.source.as_path(),
            self.0.destination.as_path(),
            self.0.name.clone()
        );

        fs.mut_transaction().add_operation(Box::new(operation.clone()));

        match operation.execute(fs.mut_vfs()) {
            Ok(_) =>
                match RemoveOperation::new(self.0.source.as_path()).execute(fs.mut_vfs()) {
                    Ok(_) => { println!("MOVE {:?} to {:?}", self.0.source, self.0.destination); Ok(()) },
                    Err(error) => Err(CommandError::from(error))
                },
            Err(error) => Err(CommandError::from(error))
        }
    }
}
