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

use vfs::{ VirtualFileSystem, VfsError };
use std::path::{ Path, PathBuf };
use clap::ArgMatches;
use crate::path::{ absolute };

pub struct TreeCommand {
    path: PathBuf
}

impl crate::command::Command for TreeCommand {
    fn from_context(cwd : &Path, args: &ArgMatches) -> Self {
        Self {
            path: absolute(cwd, Path::new(args.value_of("path").unwrap_or(cwd.to_str().unwrap()).trim())),
        }
    }

    fn execute(&self, vfs: &mut VirtualFileSystem) {
        _tree(vfs, self.path.as_path(), None, true);
    }
}

fn _tree(vfs: &VirtualFileSystem, identity: &Path, depth_list: Option<Vec<bool>>, parent_last: bool){
    let file_name = match identity.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => "/".to_string()
    };

    if let Some(depth_list) = &depth_list {
        let mut depth_delimiter = "".to_string();
        for last in depth_list {
            if *last {
                depth_delimiter += "    ";
            } else {
                depth_delimiter += "│   ";
            }
        }
        println!(
            "{}{}── {}",
            depth_delimiter,
            match parent_last {
                false => "├",
                true => "└"
            },
            file_name
        );
    } else {
        println!("{}", file_name);
        println!("│");
    }

    match vfs.read_dir(identity) {
        Ok(children) => {
            let new_depth_list = match depth_list {
                Some(depth_list) => {
                    let mut new = depth_list.clone();
                    new.push(parent_last);
                    new
                },
                None => vec![]
            };
            let length = children.len();

            for (index, virtual_child) in children.iter().enumerate() {
                _tree(
                    vfs,
                    virtual_child.as_identity(),
                    Some(new_depth_list.clone()),
                    index == (length - 1)
                );
            }
        },
        Err(error) => match error {
            VfsError::DoesNotExists(_) | VfsError::IsNotADirectory(_) => {},
            error => eprintln!("{}", error)
        }
    }
}
