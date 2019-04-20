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

use file_system::{
    HybridFileSystem,
    query::{
        Entry,
        Query,
        ReadDirQuery
    }
};

use crate::command::{
    Command,
    errors::CommandError
};

pub struct ListCommand {}

impl Command<ListCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedListCommand>, CommandError> {
        Ok(
            Command(
                InitializedListCommand {
                    path:
                        Self::extract_path_from_args(cwd, args, "path").unwrap_or_else(|_| cwd.to_path_buf())
                }
            )
        )
    }
}

pub struct InitializedListCommand {
    pub path: PathBuf
}

impl Command<InitializedListCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        match ReadDirQuery::new(self.0.path.as_path()).retrieve(&fs.vfs()) {
            Ok(collection) => {
                if ! collection.is_empty() {
                    for child in collection.into_iter() {
                        println!(
                            "{}    {}",
                           if child.is_dir() {
                               "Directory"
                           } else if child.is_file() {
                               "File     "
                           } else {
                               "Unknown  "
                           },
                            child.name().unwrap().to_string_lossy()
                        );
                    }
                } else {
                    println!("Directory is empty");
                }
                Ok(())
            },
            Err(error) => Err(CommandError::from(error))
        }
    }
}

