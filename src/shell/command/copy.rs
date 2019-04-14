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

use std::path::{ Path, PathBuf };

use clap::ArgMatches;

use vfs::{
    HybridFileSystem,
    operation::{
        Operation,
        CopyOperation
    },
    query::{
        Entry,
        Query,
        StatusQuery
    }
};

use crate::command::{ Command };
use crate::command::errors::CommandError;


pub struct CopyCommand {}

impl Command<CopyCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedCopyCommand>, CommandError> {
        let (source, _source_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "source")?;
        let (destination, _destination_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "destination")?;

        Ok(
            Command(InitializedCopyCommand {
                source,
                destination
            })
        )
    }
}

pub struct InitializedCopyCommand {
    pub source: PathBuf,
    pub destination: PathBuf
}

impl Command<InitializedCopyCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let source = StatusQuery::new(self.0.source.as_path()).retrieve(fs.vfs())?;
        let destination = StatusQuery::new(self.0.destination.as_path()).retrieve(fs.vfs())?;

        if ! source.exists() {
            return Err(CommandError::DoesNotExists(self.0.source.to_path_buf()));
        }

        let operation = if destination.exists() {
            if destination.is_dir() {
                CopyOperation::new(
                    self.0.source.as_path(),
                    self.0.destination
                        .join(self.0.source.file_name().unwrap())
                        .as_path()
                )
            } else if source.is_dir() {
                return Err(CommandError::CustomError(format!("Directory into a file {:?} {:?}", source.is_dir(), destination.is_dir())))
            } else {
                return Err(CommandError::CustomError(format!("Overwrite {:?} {:?}", source.is_dir(), destination.is_dir()))) //OVERWRITE
            }
        } else {
            CopyOperation::new(self.0.source.as_path(), self.0.destination.as_path())
        };


        match operation.execute(fs.mut_vfs()) {
            Ok(_) => {
                fs.mut_transaction().add_operation(Box::new(operation));
                Ok(())
            },
            Err(error) => Err(CommandError::from(error))
        }
    }
}
