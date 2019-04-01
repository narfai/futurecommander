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

use vfs::{ VirtualFileSystem, Virtual, Copy, WriteOperation };
use std::path::{ Path, PathBuf };
use clap::ArgMatches;
use crate::command::{ Command, InitializedCommand };
use crate::command::errors::CommandError;

pub struct ApplyCommand;

impl Command for ApplyCommand {
    const NAME : &'static str = "apply";

    fn new(_cwd: &Path, _args: &ArgMatches) -> Result<Box<InitializedCommand>, CommandError> {
        Ok(
            Box::new(
                InitializedApplyCommand
            )
        )
    }
}

pub struct InitializedApplyCommand;

impl InitializedCommand for InitializedApplyCommand {
    fn execute(&self, mut vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        unimplemented!()
    }
}
