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


use std::path::{ PathBuf };
use std::{ error, fmt };
use vfs::VfsError;

#[derive(Debug)]
pub enum CommandError {
    VfsError(VfsError),
    PathIsRelative(PathBuf),
}

impl From<VfsError> for CommandError {
    fn from(error: VfsError) -> Self {
        CommandError::VfsError(error)
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::VfsError(ref err) => write!(f, "Vfs Error: {}", err),
            CommandError::PathIsRelative(path) => write!(f, "Path is relative : {}", path),
        }
    }
}

impl error::Error for CommandError {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            CommandError::VfsError(err) => Some(err),
            _ => None
        }
    }
}
