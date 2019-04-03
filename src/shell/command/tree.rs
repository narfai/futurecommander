/*
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

use vfs::{VirtualFileSystem, VfsError, Node, ReadQuery, ReadDirQuery};
use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::command::{ Command };
use crate::command::errors::CommandError;

pub struct TreeCommand {}

impl Command<TreeCommand> {
    pub const NAME : &'static str = "tree";

    pub fn new(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedTreeCommand>, CommandError> {
        Ok(
            Command(
                InitializedTreeCommand {
                    path: Self::extract_path_from_args(cwd, args, "path").unwrap_or(cwd.to_path_buf())
                }
            )
        )
    }
}

pub struct InitializedTreeCommand {
    pub path: PathBuf
}

impl Command<InitializedTreeCommand> {
    fn display_tree_line(depth_list: &Option<Vec<bool>>, parent_last: bool, file_name: String){
        if let Some(depth_list) = &depth_list {
            println!(
                "{}{}── {}",
                depth_list.
                    iter().fold(
                    "".to_string(),
                    |depth_delimiter, last|
                        depth_delimiter + match *last {
                            true  => "    ",
                            false => "│   "
                        }
                ),
                match parent_last {
                    false => "├",
                    true  => "└"
                },
                file_name
            );
        } else {
            println!("{}", file_name);
            println!("│");
        }
    }

    fn tree(vfs: &VirtualFileSystem, identity: &Path, depth_list: Option<Vec<bool>>, parent_last: bool) -> Result<(),VfsError>{
        let file_name = match identity.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => "/".to_string()
        };

        Self::display_tree_line(&depth_list, parent_last, file_name);

        match ReadDirQuery::new(identity).retrieve(&vfs) {
            Ok(children) => {
                let new_depth_list = match depth_list {
                    Some(depth_list) => {
                        let mut new = depth_list.clone();
                        new.push(parent_last);
                        new
                    },
                    None => vec![]
                };

                let collection = children.collection();
                let length = collection.len();
                for (index, Node(virtual_child)) in collection.into_iter().enumerate() {
                    if let Err(error) = Self::tree(
                        vfs,
                        virtual_child.as_identity(),
                        Some(new_depth_list.clone()),
                        index == (length - 1)
                    ) {
                        return Err(error);
                    }
                }
            },
            Err(error) => match error {
                VfsError::DoesNotExists(_) | VfsError::IsNotADirectory(_) => {},
                error => return Err(error)
            }
        }
        Ok(())
    }

    pub fn execute(&self, vfs: &mut VirtualFileSystem) -> Result<(), CommandError> {
        match Self::tree(vfs, self.0.path.as_path(), None, true) {
            Ok(_)       => Ok(()),
            Err(error)  => Err(CommandError::from(error))
        }
    }
}



