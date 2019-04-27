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
    io,
    error,
    fmt,
    path::{ PathBuf }
};

use crate::{
    errors::{ QueryError },
};

//Representation error convenient re-export
pub use crate::infrastructure::virt::representation::errors::RepresentationError;

#[derive(Debug)]
pub enum InfrastructureError {
    Io(io::Error),
    Representation(RepresentationError),
    Query(QueryError),
    PathDoesNotExists(PathBuf),
    ParentDoesNotExists(PathBuf),
    ParentIsNotADirectory(PathBuf),
    SourceDoesNotExists(PathBuf),
    SourceIsNotADirectory(PathBuf),
    SourceIsNotAFile(PathBuf),
    DestinationIsNotAFile(PathBuf),
    DestinationAlreadyExists(PathBuf),
    DirectoryIsNotEmpty(PathBuf),
    Custom(String)
}

impl From<io::Error> for InfrastructureError {
    fn from(error: io::Error) -> Self {
        InfrastructureError::Io(error)
    }
}

impl From<RepresentationError> for InfrastructureError {
    fn from(error: RepresentationError) -> Self {
        InfrastructureError::Representation(error)
    }
}

impl From<QueryError> for InfrastructureError {
    fn from(error: QueryError) -> Self {
        InfrastructureError::Query(error)
    }
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfrastructureError::Io(error) => write!(f, "Io error {}", error),
            InfrastructureError::Representation(error) => write!(f, "Representation error {}", error),
            InfrastructureError::Query(error) => write!(f, "Query error {}", error),
            InfrastructureError::PathDoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            InfrastructureError::ParentDoesNotExists(path) => write!(f, "Parent path {} does not exists", path.to_string_lossy()),
            InfrastructureError::ParentIsNotADirectory(path) => write!(f, "Parent path {} is not a directory", path.to_string_lossy()),
            InfrastructureError::SourceDoesNotExists(path) => write!(f, "Source path {} does not exists", path.to_string_lossy()),
            InfrastructureError::SourceIsNotADirectory(path) => write!(f, "Source path {} is not a directory", path.to_string_lossy()),
            InfrastructureError::SourceIsNotAFile(path) => write!(f, "Source path {} is not a file", path.to_string_lossy()),
            InfrastructureError::DestinationIsNotAFile(path) => write!(f, "Destination path {} is not a file", path.to_string_lossy()),
            InfrastructureError::DestinationAlreadyExists(path) => write!(f, "Destination path {} already exists", path.to_string_lossy()),
            InfrastructureError::DirectoryIsNotEmpty(path) => write!(f, "Directory {} is not empty", path.to_string_lossy()),
            InfrastructureError::Custom(message) => write!(f, "Custom message {}", message),
        }
    }
}


impl error::Error for InfrastructureError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            InfrastructureError::Io(err) => Some(err),
            InfrastructureError::Representation(err) => Some(err),
            InfrastructureError::Query(err) => Some(err),
            _ => None
        }
    }
}
