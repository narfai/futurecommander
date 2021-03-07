// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    io,
    error,
    fmt,
    path::PathBuf
};
use crate::{ RepresentationError };

#[derive(Debug)]
pub enum QueryError {
    Io(io::Error),
    Representation(RepresentationError),
    AddSubDanglingVirtualPath(PathBuf),
    IsNotADirectory(PathBuf),
    ReadTargetDoesNotExists(PathBuf)
}

impl From<io::Error> for QueryError {
    fn from(error: io::Error) -> Self {
        QueryError::Io(error)
    }
}

impl From<RepresentationError> for QueryError {
    fn from(error: RepresentationError) -> Self {
        QueryError::Representation(error)
    }
}


impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::Io(error) => write!(f, "Query io error {}", error),
            QueryError::Representation(error) => write!(f, "Query representation error {}", error),
            QueryError::AddSubDanglingVirtualPath(path) => write!(f, "Path {} is present in both add and sub representations", path.to_string_lossy()),
            QueryError::IsNotADirectory(path) => write!(f, "Path {} is not a directory", path.to_string_lossy()),
            QueryError::ReadTargetDoesNotExists(path) => write!(f, "Read target {} does not exists", path.to_string_lossy()),
        }
    }
}


impl error::Error for QueryError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            QueryError::Io(err) => Some(err),
            QueryError::Representation(err) => Some(err),
            _ => None
        }
    }
}