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

use vfs::{ VirtualFileSystem, RemoveOperation, RealFileSystem, Transaction };
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command };
use crate::command::errors::CommandError;

pub struct RemoveCommand {}

impl Command<RemoveCommand> {
    pub const NAME : &'static str = "rm";

    pub fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedRemoveCommand>, CommandError> {
        let path = Self::extract_path_from_args(cwd, args, "path")?;
        for ancestor in cwd.ancestors() {
            if path == ancestor {
                return Err(CommandError::CwdIsInside(path.to_path_buf()))
            }
        }

        Ok(
            Command(
                InitializedRemoveCommand {
                    path
                }
            )
        )
    }
}

pub struct InitializedRemoveCommand {
    pub path: PathBuf
}

impl Command<InitializedRemoveCommand> {
    pub fn execute(&self, transaction: &mut Transaction<RealFileSystem>, vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        let operation = RemoveOperation::new(self.0.path.as_path());

        transaction.add_operation(Box::new(operation.clone()));

        match operation.execute(vfs) {
            Ok(_)       => Ok(()),
            Err(error)  => Err(CommandError::from(error))
        }
    }
}
