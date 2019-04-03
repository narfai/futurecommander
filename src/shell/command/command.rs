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


use vfs::{ VirtualFileSystem, VirtualPath, VfsError };
use clap::ArgMatches;
use std::ffi::OsString;
use std::path::{ Path, PathBuf, MAIN_SEPARATOR };
use crate::path::{ normalize };
use crate::command::errors::CommandError;


pub trait Command {
    const NAME : &'static str;

    fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Box<dyn InitializedCommand>, CommandError>;

    fn extract_path_from_args(cwd: &Path, args: &ArgMatches<'_>, key: &str) -> Result<PathBuf, CommandError> {
        match args.value_of(key) {
            Some(str_path) => {
                Ok(normalize(&cwd.join(Path::new(str_path.trim()))))
            },
            None => return Err(CommandError::ArgumentMissing((&Self::NAME).to_string(), key.to_string(), args.usage().to_string()))
        }
    }

    fn extract_name_and_destination(cwd: &Path, args: &ArgMatches<'_>) -> Result<(Option<OsString>, PathBuf), CommandError>{
        match args.value_of("destination") {
            Some(str_path) =>
                match str_path.chars().last().unwrap() == MAIN_SEPARATOR {
                    false => {
                        let destination = normalize(&cwd.join(Path::new(str_path.trim())));
                        let name = match destination.file_name() {
                            None => return Err(CommandError::from(VfsError::IsDotName(destination.to_path_buf()))),
                            Some(name) => Some(name.to_os_string())
                        };
                        Ok((name, VirtualPath::get_parent_or_root(destination.as_path())))
                    },
                    true => Ok((None, normalize(&cwd.join(Path::new(str_path.trim())))))
                },
            None => return Err(CommandError::ArgumentMissing((&Self::NAME).to_string(), "destination".to_string(), args.usage().to_string()))
        }
    }
}

pub trait InitializedCommand {
    fn execute(&self, vfs: &mut VirtualFileSystem) -> Result<()/*ReversableCommand*/, CommandError>;
}

//TODO https://trello.com/c/53teSBkz/38-as-human-im-able-to-roll-back-any-operations-performed-over-the-virtual-fs
//pub trait ReversableCommand {
//    fn reverse(&self, vfs: &mut VirtualFileSystem) -> Result<InitializedCommand, CommandError>; //Shoud ble idempotent
//}
