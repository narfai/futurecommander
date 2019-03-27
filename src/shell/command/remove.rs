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

use vfs::{ VirtualFileSystem, VirtualPath };
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;

pub struct RemoveCommand {}

impl Command for RemoveCommand {
    const NAME : &'static str = "rm";

    fn new(cwd: &Path, args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        let path = Self::extract_path_from_args(cwd, args, "path")?;
        for ancestor in cwd.ancestors() {
            if path == ancestor {
                return Err(CommandError::CwdIsInside(path.to_path_buf()))
            }
        }

        Ok(
            Box::new(
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

impl InitializedCommand for InitializedRemoveCommand {
    fn execute(&self, vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match vfs.remove(self.path.as_path()) {
            Ok(_)       => Ok(()),
            Err(error)  => Err(CommandError::from(error))
        }
    }
}
