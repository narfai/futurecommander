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
    path::PathBuf
};

use crate::{
    representation::errors::RepresentationError,
    query::QueryError
};

#[derive(Debug)]
pub enum OperationError {
    Io(io::Error),
    Representation(RepresentationError),
    Query(QueryError),

    //Virtual Only
    CopyIntoItSelf(PathBuf, PathBuf),

    //Real Only
    AlreadyExists(PathBuf),
    DirectoryIsNotEmpty(PathBuf),
    SourceIsNotAFile(PathBuf),

    //Common
    OverwriteDirectoryWithFile(PathBuf, PathBuf),
    CopyDirectoryIntoFile(PathBuf, PathBuf),
    MergeNotAllowed(PathBuf, PathBuf),
    OverwriteNotAllowed(PathBuf, PathBuf),
    ParentDoesNotExists(PathBuf),
    ParentIsNotADirectory(PathBuf),
    SourceDoesNotExists(PathBuf),
    DoesNotExists(PathBuf),
    Custom(String)
}

impl From<io::Error> for OperationError {
    fn from(error: io::Error) -> Self {
        OperationError::Io(error)
    }
}

impl From<RepresentationError> for OperationError {
    fn from(error: RepresentationError) -> Self {
        OperationError::Representation(error)
    }
}

impl From<QueryError> for OperationError {
    fn from(error: QueryError) -> Self {
        OperationError::Query(error)
    }
}


impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationError::Io(error) => write!(f, "Io error {}", error),
            OperationError::Representation(error) => write!(f, "Representation error {}", error),
            OperationError::Query(error) => write!(f, "Query error {}", error),
            //Virtual Only
            OperationError::CopyIntoItSelf(source, dst) => write!(f, "Cannot copy {} into itself {}", source.to_string_lossy(), dst.to_string_lossy()),

            //Real Only
            OperationError::AlreadyExists(path) => write!(f, "Path {} already exists", path.to_string_lossy()),
            OperationError::DirectoryIsNotEmpty(path) => write!(f, "Directory {} is not empty", path.to_string_lossy()),
            OperationError::SourceIsNotAFile(source) => write!(f, "Source {} is not a file", source.to_string_lossy()),

            //Common
            OperationError::ParentDoesNotExists(parent) => write!(f, "Parent {} does not exists", parent.to_string_lossy()),
            OperationError::ParentIsNotADirectory(parent) => write!(f, "Parent {} is not a directory", parent.to_string_lossy()),
            OperationError::SourceDoesNotExists(source) => write!(f, "Source {} does not exists", source.to_string_lossy()),
            OperationError::OverwriteDirectoryWithFile(source, dst) => write!(f, "Cannot overwrite directory {} with file {}", source.to_string_lossy(), dst.to_string_lossy()),
            OperationError::CopyDirectoryIntoFile(source, dst) => write!(f, "Cannot copy directory {} into file {}", source.to_string_lossy(), dst.to_string_lossy()),
            OperationError::OverwriteNotAllowed(source, dst) => write!(f, "Overwrite of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            OperationError::MergeNotAllowed(source, dst) => write!(f, "Merge of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            OperationError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            OperationError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}


impl error::Error for OperationError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            OperationError::Io(err) => Some(err),
            OperationError::Representation(err) => Some(err),
            OperationError::Query(err) => Some(err),
            _ => None
        }
    }
}
