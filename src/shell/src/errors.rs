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
    io,
    path::{ PathBuf }
};

use file_system::{
    DomainError,
    QueryError
};

use crate::{
    command::CommandError
};

#[derive(Debug)]
pub enum ShellError {
    Exit,
    Io(io::Error),
    Format(fmt::Error),
    Operation(DomainError),
    Query(QueryError),
    Command(CommandError),
}

impl From<DomainError> for ShellError {//TODO ex : filesystem::error
    fn from(error: DomainError) -> Self {
        ShellError::Operation(error)
    }
}

impl From<io::Error> for ShellError {//TODO <= apply that namespacing good practise every where ( and move errors into theirs respective domains )
    fn from(error: io::Error) -> Self {
        ShellError::Io(error)
    }
}

impl From<QueryError> for ShellError {//TODO ex : filesystem::query::error
    fn from(error: QueryError) -> Self {
        ShellError::Query(error)
    }
}

impl From<CommandError> for ShellError {//TODO ex : filesystem::query::error
    fn from(error: CommandError) -> Self {
        ShellError::Command(error)
    }
}

impl From<fmt::Error> for ShellError {//TODO ex : filesystem::query::error
    fn from(error: fmt::Error) -> Self {
        ShellError::Format(error)
    }
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Exit => write!(f, "Exit program"),
            ShellError::Operation(error) => write!(f, "Fs operation error: {}", error),
            ShellError::Io(error) => write!(f, "Input / output error : {}", error),
            ShellError::Query(error) => write!(f, "Fs query error: {}", error),
            ShellError::Format(error) => write!(f, "Format error : {}", error),
            ShellError::Command(error) => write!(f, "Command error : {}", error),
        }
    }
}

impl error::Error for ShellError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            ShellError::Io(err) => Some(err),
            ShellError::Operation(err) => Some(err),
            ShellError::Query(err) => Some(err),
            ShellError::Format(err) => Some(err),
            ShellError::Command(err) => Some(err),
            _ => None
        }
    }
}
