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

use vfs::{HybridFileSystem, VirtualKind, Node, ReadQuery, ReadDirQuery};
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{Command};
use crate::command::errors::CommandError;


pub struct ListCommand {}

impl Command<ListCommand> {
    pub const NAME : &'static str = "ls";

    pub fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedListCommand>, CommandError> {
        Ok(
            Command(
                InitializedListCommand {
                    path:
                        Self::extract_path_from_args(cwd, args, "path").unwrap_or(cwd.to_path_buf())
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
            Ok(virtual_children) => {
                let collection = virtual_children.collection();
                let len = collection.len();
                if len > 0 {
                    for Node(child) in collection.into_iter() {
                        println!(
                            "{}    {}",
                            match child.to_kind() {
                                VirtualKind::Directory => "Directory",
                                VirtualKind::File      => "File     ",
                                VirtualKind::Unknown   => "Unknown  "
                            },
                            child.file_name()?.to_string_lossy()
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

