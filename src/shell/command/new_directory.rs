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

use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command };
use crate::command::errors::CommandError;

use vfs::{
    HybridFileSystem,
    operation::{
        WriteOperation,
        CreateOperation
    },
    representation::Kind
};

pub struct NewDirectoryCommand {}

impl Command<NewDirectoryCommand> {
    pub fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedNewDirectoryCommand>, CommandError> {
        Ok(
            Command(
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

impl Command<InitializedNewDirectoryCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let operation = CreateOperation::new(
            self.0.path.as_path(),
            Kind::Directory
        );

        match operation.execute(fs.mut_vfs()) {
            Ok(_)       => {
                fs.mut_transaction().add_operation(Box::new(operation.clone()));
                Ok(())
            }
            Err(error)  => Err(CommandError::from(error))
        }
    }
}
