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

use std::{
    fs::{ File, read_to_string },
    io::prelude::*,
    path::{ Path, PathBuf }
};

use clap::ArgMatches;

use file_system::{ Container };

use crate::command::{
    Command,
    errors::CommandError
};

pub struct ImportCommand {}

impl Command<ImportCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedImportCommand>, CommandError> {
        Ok(
            Command(
                InitializedImportCommand {
                    path:
                    Self::extract_path_from_args(cwd, args, "path").unwrap_or_else(|_| cwd.to_path_buf().join(".fc.json"))
                }
            )
        )
    }
}

pub struct InitializedImportCommand {
    pub path: PathBuf
}

impl Command<InitializedImportCommand> {
    pub fn execute(&self, fs: &mut Container) -> Result<(), CommandError> {
        if ! self.0.path.exists() {
            return Err(CommandError::DoesNotExists(self.0.path.clone()));
        }

        fs.emit_json(read_to_string(self.0.path.as_path())?)?;
        Ok(())
    }
}

