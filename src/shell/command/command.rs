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


use vfs::{ VirtualPath, VfsError };
use clap::ArgMatches;
use std::ffi::OsString;
use std::path::{ Path, PathBuf, MAIN_SEPARATOR };
use crate::path::{ normalize };
use crate::command::errors::CommandError;

pub struct Command<C>(pub C);

impl <C>Command<C> {
    pub fn extract_path_from_args(cwd: &Path, args: &ArgMatches<'_>, key: &str) -> Result<PathBuf, CommandError> {
        match args.value_of(key) {
            Some(str_path) => {
                Ok(normalize(&cwd.join(Path::new(str_path.trim()))))
            },
            None => return Err(CommandError::ArgumentMissing("generic".to_string(), key.to_string(), args.usage().to_string()))
        }
    }

    pub fn extract_path_and_trail_from_args(cwd: &Path, args: &ArgMatches<'_>, key: &str) -> Result<(PathBuf, bool), CommandError> {
        match args.value_of(key) {
            Some(str_path) => Ok((
                normalize(&cwd.join(Path::new(str_path.trim()))),
                str_path.chars().last().unwrap() == MAIN_SEPARATOR
            )),
            None => return Err(CommandError::ArgumentMissing("generic".to_string(), key.to_string(), args.usage().to_string()))
        }
    }
}
