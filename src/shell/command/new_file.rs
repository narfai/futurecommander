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

use vfs::{ VirtualFileSystem, VirtualKind };
use std::path::{ Path, PathBuf };
use clap::ArgMatches;
use crate::path::{ absolute, normalize };

pub struct NewFileCommand {
    path: PathBuf
}

impl NewFileCommand {
    pub fn new(path: &Path) -> Self {
        NewFileCommand {
            path: normalize(path)
        }
    }
}


impl crate::command::Command for NewFileCommand {
    fn from_context(cwd : &Path, args: &ArgMatches) -> Self {
        Self {
            path: absolute(cwd, Path::new(args.value_of("path").unwrap().trim())),
        }
    }

    fn execute(&self, vfs: &mut VirtualFileSystem) {
        vfs.create(self.path.as_path(), VirtualKind::File).unwrap();
    }
}
