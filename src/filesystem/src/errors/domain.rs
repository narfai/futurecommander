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
    path::PathBuf
};

use crate::{
    errors::{
        QueryError
    },
    infrastructure::{
        errors::InfrastructureError
    },
};

#[derive(Debug)]
pub enum DomainError {
    Infrastructure(InfrastructureError),
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
    DeleteRecursiveNotAllowed(PathBuf),
    ParentDoesNotExists(PathBuf),
    ParentIsNotADirectory(PathBuf),
    SourceDoesNotExists(PathBuf),
    DoesNotExists(PathBuf),
    Custom(String)
}

impl From<QueryError> for DomainError {
    fn from(error: QueryError) -> Self {
        DomainError::Query(error)
    }
}

impl From<InfrastructureError> for DomainError {
    fn from(error: InfrastructureError) -> Self {
        DomainError::Infrastructure(error)
    }
}


impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::Infrastructure(error) => write!(f, "Infrastructure error {}", error),
            DomainError::Query(error) => write!(f, "Query error {}", error),
            //Virtual Only
            DomainError::CopyIntoItSelf(source, dst) => write!(f, "Cannot copy {} into itself {}", source.to_string_lossy(), dst.to_string_lossy()),

            //Real Only
            DomainError::AlreadyExists(path) => write!(f, "Path {} already exists", path.to_string_lossy()),
            DomainError::DirectoryIsNotEmpty(path) => write!(f, "Directory {} is not empty", path.to_string_lossy()),
            DomainError::SourceIsNotAFile(source) => write!(f, "Source {} is not a file", source.to_string_lossy()),

            //Common
            DomainError::ParentDoesNotExists(parent) => write!(f, "Parent {} does not exists", parent.to_string_lossy()),
            DomainError::ParentIsNotADirectory(parent) => write!(f, "Parent {} is not a directory", parent.to_string_lossy()),
            DomainError::SourceDoesNotExists(source) => write!(f, "Source {} does not exists", source.to_string_lossy()),
            DomainError::OverwriteDirectoryWithFile(source, dst) => write!(f, "Cannot overwrite directory {} with file {}", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::CopyDirectoryIntoFile(source, dst) => write!(f, "Cannot copy directory {} into file {}", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::OverwriteNotAllowed(source, dst) => write!(f, "Overwrite of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::MergeNotAllowed(source, dst) => write!(f, "Merge of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::DeleteRecursiveNotAllowed(path) => write!(f, "Delete recursively {} is not allowed", path.to_string_lossy()),
            DomainError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            DomainError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}


impl error::Error for DomainError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            DomainError::Query(err) => Some(err),
            DomainError::Infrastructure(err) => Some(err),
            _ => None
        }
    }
}
