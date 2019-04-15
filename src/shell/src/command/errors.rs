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
use futurecommander_vfs::VfsError;

#[derive(Debug)]
pub enum CommandError {
    Exit,
    VfsError(VfsError),
    ArgumentMissing(String, String, String),
    InvalidCommand,
    IsNotADirectory(PathBuf),
    DoesNotExists(PathBuf),
    CwdIsInside(PathBuf),
    CustomError(String)
}

impl From<VfsError> for CommandError {
    fn from(error: VfsError) -> Self {
        CommandError::VfsError(error)
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::Exit => write!(f, "Exit program"),
            CommandError::VfsError(error) => write!(f, "Vfs Error: {}", error),
            CommandError::ArgumentMissing(command, argument, usage) => write!(f, "{} missing {} argument \n {}", command, argument, usage),
            CommandError::InvalidCommand => write!(f, "Invalid command"),
            CommandError::IsNotADirectory(path) => write!(f, "{} is not a directory", path.as_os_str().to_string_lossy()),
            CommandError::DoesNotExists(path) => write!(f, "{} does not exists", path.as_os_str().to_string_lossy()),
            CommandError::CwdIsInside(path) => write!(f, "current working directory is inside {}", path.as_os_str().to_string_lossy()),
            CommandError::CustomError(custom_message) => write!(f, "Custom error message {}", custom_message)
        }
    }
}

impl error::Error for CommandError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            CommandError::VfsError(err) => Some(err),
            _ => None
        }
    }
}
