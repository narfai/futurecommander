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

use vfs::{VirtualFileSystem, HybridFileSystem, CopyOperation, WriteOperation, Transaction, RealFileSystem };
use std::path::{ Path, PathBuf };
use clap::ArgMatches;
use crate::command::{ Command };
use crate::command::errors::CommandError;
use std::ffi::{ OsString };

pub struct CopyCommand {}

impl Command<CopyCommand> {
    pub const NAME : &'static str = "copy";

    pub fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedCopyCommand>, CommandError> {
        let source = Self::extract_path_from_args(cwd, args, "source")?;
        let (name, destination) = Self::extract_name_and_destination(cwd, args)?;

        Ok(
            Command(InitializedCopyCommand {
                source,
                destination,
                name
            })
        )
    }
}

pub struct InitializedCopyCommand {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub name: Option<OsString>
}

impl Command<InitializedCopyCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let operation = CopyOperation::new(
            self.0.source.as_path(),
            self.0.destination.as_path(),
            self.0.name.clone()
        );

        fs.mut_transaction().add_operation(Box::new(operation.clone()));

        match operation.execute(fs.mut_vfs()) {
            Ok(_) => Ok(()),
            Err(error) => Err(CommandError::from(error))
        }
    }
}
