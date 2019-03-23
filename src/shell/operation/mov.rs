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

/**
renommer un dossier c'est : create dst, copy source destination, remove source
*/

use futurecommandervfs::VirtualFileSystem;
use std::path::{ Path, PathBuf };
use clap::ArgMatches;
use crate::path::{ absolute, normalize };

pub struct MoveOperation {
    source: PathBuf,
    destination: PathBuf
}

impl MoveOperation {
    pub fn new(source: &Path, destination: &Path) -> Self {
        MoveOperation {
            source: normalize(source),
            destination: normalize(destination)
        }
    }
}

impl crate::operation::Operation for MoveOperation {
    fn from_context(cwd: &Path, args: &ArgMatches) -> Self {
        Self {
            source: absolute(cwd, Path::new(args.value_of("source").unwrap())),
            destination: absolute(cwd, Path::new(args.value_of("destination").unwrap())),
        }
    }

    fn execute(&self, vfs: &mut VirtualFileSystem) {
        match vfs.copy(
            self.source.as_path(),
            self.destination.as_path()
        ) {
            Ok(_) => match vfs.remove(self.source.as_path()) {
                Ok(_) => { println!("MOVE {:?} to {:?}", self.source, self.destination); },
                Err(error) => println!("MOVE::REMOVE {:?}", error)
            },
            Err(error) => println!("MOVE::COPY {:?}", error)
        };
    }
}