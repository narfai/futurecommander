// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN


use std::{
    error,
    fmt,
    io
};

use futurecommander_filesystem::{
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

impl From<DomainError> for ShellError {
    fn from(error: DomainError) -> Self {
        ShellError::Operation(error)
    }
}

impl From<io::Error> for ShellError {
    fn from(error: io::Error) -> Self {
        ShellError::Io(error)
    }
}

impl From<QueryError> for ShellError {
    fn from(error: QueryError) -> Self {
        ShellError::Query(error)
    }
}

impl From<CommandError> for ShellError {
    fn from(error: CommandError) -> Self {
        ShellError::Command(error)
    }
}

impl From<fmt::Error> for ShellError {
    fn from(error: fmt::Error) -> Self {
        ShellError::Format(error)
    }
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Exit => write!(f, "Exit program"),
            ShellError::Operation(error) => write!(f, "container operation error: {}", error),
            ShellError::Io(error) => write!(f, "Input / output error : {}", error),
            ShellError::Query(error) => write!(f, "container query error: {}", error),
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
