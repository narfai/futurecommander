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


use std::{
    error,
    fmt,
    io::Error,
    path::{ PathBuf }
};

use futurecommander_filesystem::{
    DomainError,
    QueryError
};

#[derive(Debug)]
pub enum CommandError {
    Exit,
    Io(Error),
    Operation(DomainError),
    Query(QueryError),
    Format(fmt::Error),
    ArgumentMissing(String, String, String),
    InvalidCommand,
    AlreadyExists(PathBuf),
    IsNotADirectory(PathBuf),
    DoesNotExists(PathBuf),
    CwdIsInside(PathBuf),
    CustomError(String),
    DirectoryIntoAFile(PathBuf, PathBuf),
    InvalidGuard(String)
}

impl From<DomainError> for CommandError {
    fn from(error: DomainError) -> Self {
        CommandError::Operation(error)
    }
}

impl From<Error> for CommandError {
    fn from(error: Error) -> Self {
        CommandError::Io(error)
    }
}

impl From<QueryError> for CommandError {
    fn from(error: QueryError) -> Self {
        CommandError::Query(error)
    }
}

impl From<fmt::Error> for CommandError {
    fn from(error: fmt::Error) -> Self {
      CommandError::Format(error)
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::Exit => write!(f, "Exit program"),
            CommandError::Operation(error) => write!(f, "container operation error: {}", error),
            CommandError::Io(error) => write!(f, "Input / output error : {}", error),
            CommandError::Query(error) => write!(f, "container query error: {}", error),
            CommandError::Format(error) => write!(f, "Format error : {}", error),
            CommandError::ArgumentMissing(command, argument, usage) => write!(f, "{} missing {} argument \n {}", command, argument, usage),
            CommandError::InvalidCommand => write!(f, "Invalid command"),
            CommandError::AlreadyExists(path) => write!(f, "Path {} already exists", path.to_string_lossy()),
            CommandError::IsNotADirectory(path) => write!(f, "{} is not a directory", path.to_string_lossy()),
            CommandError::DoesNotExists(path) => write!(f, "{} does not exists", path.to_string_lossy()),
            CommandError::CwdIsInside(path) => write!(f, "current working directory is inside {}", path.to_string_lossy()),
            CommandError::DirectoryIntoAFile(src, dst) => write!(f, "Directory {} into a file {}", src.to_string_lossy(), dst.to_string_lossy()),
            CommandError::CustomError(custom_message) => write!(f, "Custom error message {}", custom_message),
            CommandError::InvalidGuard(guard) => write!(f, "Invalid guard {}", guard),
        }
    }
}

impl error::Error for CommandError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            CommandError::Io(err) => Some(err),
            CommandError::Operation(err) => Some(err),
            CommandError::Query(err) => Some(err),
            CommandError::Format(err) => Some(err),
            _ => None
        }
    }
}
