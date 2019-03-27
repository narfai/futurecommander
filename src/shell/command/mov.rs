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
use std::path::{ Path, PathBuf, MAIN_SEPARATOR };
use std::ffi::{ OsStr, OsString };
use crate::path::normalize;
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;


pub struct MoveCommand {}

impl Command for MoveCommand {
    const NAME : &'static str = "mv";

    fn new(cwd: &Path, args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        let source = Self::extract_path_from_args(cwd, args, "source")?;
        for ancestor in cwd.ancestors() {
            if source.as_path() == ancestor {
                return Err(CommandError::CwdIsInside(source.to_path_buf()))
            }
        }

        let mut name = None;
        let destination =  match args.value_of("destination") {
            Some(str_path) =>
                match str_path.chars().last().unwrap() == MAIN_SEPARATOR {
                    false => {
                        let destination = normalize(&cwd.join(Path::new(str_path.trim())));
                        name = match destination.file_name() {
                            None => return Err(CommandError::from(VfsError::IsDotName(destination.to_path_buf()))),
                            Some(name) => Some(name.to_os_string())
                        };
                        VirtualPath::get_parent_or_root(destination.as_path())
                    },
                    true => normalize(&cwd.join(Path::new(str_path.trim())))
                },
            None => return Err(CommandError::ArgumentMissing((&Self::NAME).to_string(), "destination".to_string(), args.usage().to_string()))
        };

        Ok(
            Box::new(
                InitializedMoveCommand {
                    source,
                    destination,
                    name
                }
            )
        )
    }
}

pub struct InitializedMoveCommand {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub name: Option<OsString>
}

impl InitializedCommand for InitializedMoveCommand {
    fn execute(&self, vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match vfs.copy(
            self.source.as_path(),
            self.destination.as_path(),
            self.name.clone()
        ) {
            Ok(_) =>
                match vfs.remove(self.source.as_path()) {
                    Ok(_) => { println!("MOVE {:?} to {:?}", self.source, self.destination); Ok(()) },
                    Err(error) => Err(CommandError::from(error))
                },
            Err(error) => Err(CommandError::from(error))
        }
    }
}

/*
TODO https://trello.com/c/GVGBLNK6/10-as-human-i-can-virtually-rename-a-virtual-file-or-directory
2 problems :
 1/ detect wether we should consider destination as new parent or new node
 2/ renaming ( destination as new node ) is virtually nothing less than :
      for directorie => create destination -> copy all the children of source in destination -> remove source
      for file => create destination -> remove source
*/
