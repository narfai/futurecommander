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

    CopyIntoItSelf(PathBuf, PathBuf),

    MergeNotAllowed(PathBuf, PathBuf),
    OverwriteNotAllowed(PathBuf),
    DirectoryOverwriteNotAllowed(PathBuf),

    MergeFileWithDirectory(PathBuf, PathBuf),
    OverwriteDirectoryWithFile(PathBuf, PathBuf),

    CreateUnknown(PathBuf),
    DoesNotExists(PathBuf),
    DeleteRecursiveNotAllowed(PathBuf),
    SourceDoesNotExists(PathBuf),
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
            DomainError::CopyIntoItSelf(source, dst) => write!(f, "Cannot copy {} into itself {}", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::MergeNotAllowed(source, dst) => write!(f, "Merge of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::OverwriteNotAllowed(dst) => write!(f, "Overwrite of {} is not allowed", dst.to_string_lossy()),
            DomainError::DirectoryOverwriteNotAllowed(path) => write!(f, "Directory overwrite of {} is not allowed", path.to_string_lossy()),
            DomainError::MergeFileWithDirectory(source, destination) => write!(f, "Cannot merge file {} with directory {} is not allowed", source.to_string_lossy(), destination.to_string_lossy()),
            DomainError::OverwriteDirectoryWithFile(source, dst) => write!(f, "Cannot overwrite directory {} with file {}", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::CreateUnknown(path) => write!(f, "Cannot create unknown kind at path {}", path.to_string_lossy()),
            DomainError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            DomainError::DeleteRecursiveNotAllowed(path) => write!(f, "Delete recursively {} is not allowed", path.to_string_lossy()),
            DomainError::SourceDoesNotExists(source) => write!(f, "Source {} does not exists", source.to_string_lossy()),
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
