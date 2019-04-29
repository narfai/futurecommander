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
    fs::File,
    io::prelude::*,
    path::{ Path, PathBuf }
};

use clap::ArgMatches;

use file_system::{ Container };

use crate::command::{
    Command,
    errors::CommandError
};

pub struct SaveCommand {}

impl Command<SaveCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedSaveCommand>, CommandError> {
        Ok(
            Command(
                InitializedSaveCommand {
                    path:
                    Self::extract_path_from_args(cwd, args, "path").unwrap_or_else(|_| cwd.to_path_buf().join(".fc.json")),
                    overwrite: args.is_present("o")
                }
            )
        )
    }
}

pub struct InitializedSaveCommand {
    pub path: PathBuf,
    pub overwrite: bool
}

impl Command<InitializedSaveCommand> {
    pub fn execute(&self, fs: &mut Container) -> Result<(), CommandError> { //TODO consume command with "execute(self,"
        if ! self.0.overwrite && self.0.path.exists() {
            return Err(CommandError::AlreadyExists(self.0.path.clone()));
        }
        let mut file = File::create(self.0.path.as_path())?;
        file.write_all(fs.to_json()?.as_bytes())?;
        Ok(())
    }
}

