// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use crate::{ tools::normalize };

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

pub mod import;
pub use self::import::{ InitializedImportCommand, ImportCommand };

use clap::ArgMatches;
use std::path::{ Path, PathBuf, MAIN_SEPARATOR };

pub struct Command<C>(pub C);

mod guard;
pub use self::guard::AvailableGuard;

impl <C>Command<C> {
    pub fn extract_path_from_args(cwd: &Path, args: &ArgMatches<'_>, key: &str) -> Result<PathBuf, CommandError> {
        match args.value_of(key) {
            Some(str_path) => {
                Ok(normalize(&cwd.join(Path::new(str_path.trim()))))
            },
            None => Err(CommandError::ArgumentMissing("generic".to_string(), key.to_string(), args.usage().to_string()))
        }
    }

    pub fn extract_path_and_trail_from_args(cwd: &Path, args: &ArgMatches<'_>, key: &str) -> Result<(PathBuf, bool), CommandError> {
        match args.value_of(key) {
            Some(str_path) => Ok((
                normalize(&cwd.join(Path::new(str_path.trim()))),
                str_path.chars().last().unwrap() == MAIN_SEPARATOR
            )),
            None => Err(CommandError::ArgumentMissing("generic".to_string(), key.to_string(), args.usage().to_string()))
        }
    }

    pub fn extract_available_guard(args: &ArgMatches<'_>, key: &str) -> Result<AvailableGuard, CommandError> {
        match args.value_of(key) {
            Some(str_guard) => {
                if ! AvailableGuard::available(str_guard) {
                    return Err(CommandError::InvalidGuard(str_guard.to_string()));
                }
                Ok(AvailableGuard::from(str_guard))
            },
            None => Ok(AvailableGuard::default())
        }
    }
}

pub fn add_red_color(text: &str) -> String {
    format!("\x1b[1;91m{}\x1b[0m",
            text
    )
}
