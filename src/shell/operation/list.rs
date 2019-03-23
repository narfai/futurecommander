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

use futurecommandervfs::VirtualFileSystem;
use std::path::{ Path, PathBuf };
use clap::ArgMatches;
use crate::path::absolute;

pub struct ListOperation {
    path: PathBuf
}

impl crate::operation::Operation for ListOperation {
    fn from_context(cwd : &Path, args: &ArgMatches) -> Self {
        Self {
            path: absolute(cwd, Path::new(args.value_of("path").unwrap_or(cwd.to_str().unwrap()))),
        }
    }

    fn execute(&self, vfs: &mut VirtualFileSystem) {
        match vfs.read_dir(self.path.as_path()) {
            Ok(virtual_children) => {
                if virtual_children.len() != 0 {
                    for child in virtual_children {
                        println!("{:?} {:?}", child, vfs.exists(child.as_identity()));
                    }
                } else {
                    println!("Directory is empty");
                }

            },
            Err(error) => println!("Error : {}", error)
        }
    }
}
