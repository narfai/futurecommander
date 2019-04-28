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

pub mod errors;
pub use self::errors::CommandError;

pub mod copy;
pub use self::copy::{ InitializedCopyCommand, CopyCommand };

pub mod list;
pub use self::list::{ InitializedListCommand, ListCommand };

pub mod mov;
pub use self::mov::{ InitializedMoveCommand, MoveCommand };

pub mod new_directory;
pub use self::new_directory::{ InitializedNewDirectoryCommand, NewDirectoryCommand };

pub mod new_file;
pub use self::new_file::{ InitializedNewFileCommand, NewFileCommand };

pub mod remove;
pub use self::remove::{ InitializedRemoveCommand, RemoveCommand };

pub mod tree;
pub use self::tree::{ InitializedTreeCommand, TreeCommand };

pub mod save;
pub use self::save::{ InitializedSaveCommand, SaveCommand };

use clap::ArgMatches;
use std::path::{ Path, PathBuf, MAIN_SEPARATOR };

pub struct Command<C>(pub C);

impl <C>Command<C> {
    pub fn extract_path_from_args(cwd: &Path, args: &ArgMatches<'_>, key: &str) -> Result<PathBuf, CommandError> {
        match args.value_of(key) {
            Some(str_path) => {
                Ok(file_system::tools::normalize(&cwd.join(Path::new(str_path.trim()))))
            },
            None => Err(CommandError::ArgumentMissing("generic".to_string(), key.to_string(), args.usage().to_string()))
        }
    }

    pub fn extract_path_and_trail_from_args(cwd: &Path, args: &ArgMatches<'_>, key: &str) -> Result<(PathBuf, bool), CommandError> {
        match args.value_of(key) {
            Some(str_path) => Ok((
                file_system::tools::normalize(&cwd.join(Path::new(str_path.trim()))),
                str_path.chars().last().unwrap() == MAIN_SEPARATOR
            )),
            None => Err(CommandError::ArgumentMissing("generic".to_string(), key.to_string(), args.usage().to_string()))
        }
    }
}

