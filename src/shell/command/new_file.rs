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

use vfs::{VirtualKind, CreateOperation, Transaction, HybridFileSystem};
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command };
use crate::command::errors::CommandError;

pub struct NewFileCommand {}

impl Command<NewFileCommand> {
    pub const NAME : &'static str = "touch";

    pub fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedNewFileCommand>, CommandError> {
        Ok(
            Command(
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

impl Command<InitializedNewFileCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let operation = CreateOperation::new(
            self.0.path.as_path(),
            VirtualKind::File
        );

        fs.mut_transaction().add_operation(Box::new(operation.clone()));

        match operation.execute(fs.mut_vfs()) {
            Ok(_)       => Ok(()),
            Err(error)  => Err(CommandError::from(error))
        }
    }
}
