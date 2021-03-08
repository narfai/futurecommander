// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    io,
    error,
    fmt,
    path::PathBuf
};
use crate::{ InfrastructureError };
use super::{ QueryError };

#[derive(Debug)]
pub enum DomainError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    Infrastructure(InfrastructureError),
    Query(QueryError),
    CopyIntoItSelf(PathBuf, PathBuf),
    MergeNotAllowed(PathBuf),
    OverwriteNotAllowed(PathBuf),
    DirectoryOverwriteNotAllowed(PathBuf),
    MergeFileWithDirectory(PathBuf, PathBuf),
    OverwriteDirectoryWithFile(PathBuf, PathBuf),
    CreateUnknown(PathBuf),
    DoesNotExists(PathBuf),
    RecursiveNotAllowed(PathBuf),
    SourceDoesNotExists(PathBuf),
    UserCancelled,
    Custom(String)
}

impl From<serde_json::Error> for DomainError {
    fn from(error: serde_json::Error) -> Self {
        DomainError::JsonError(error)
    }
}

impl From<io::Error> for DomainError {
    fn from(error: io::Error) -> Self {
        DomainError::IoError(error)
    }
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
            DomainError::IoError(error) => write!(f, "I/O error {}", error),
            DomainError::JsonError(error) => write!(f, "Json error {}", error),
            DomainError::Infrastructure(error) => write!(f, "Infrastructure error {}", error),
            DomainError::Query(error) => write!(f, "Query error {}", error),
            DomainError::CopyIntoItSelf(source, dst) => write!(f, "Cannot copy {} into itself {}", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::MergeNotAllowed(dst) => write!(f, "Merge into {} is not allowed", dst.to_string_lossy()),
            DomainError::OverwriteNotAllowed(dst) => write!(f, "Overwrite of {} is not allowed", dst.to_string_lossy()),
            DomainError::DirectoryOverwriteNotAllowed(path) => write!(f, "Directory overwrite of {} is not allowed", path.to_string_lossy()),
            DomainError::MergeFileWithDirectory(source, destination) => write!(f, "Cannot merge file {} with directory {} is not allowed", source.to_string_lossy(), destination.to_string_lossy()),
            DomainError::OverwriteDirectoryWithFile(source, dst) => write!(f, "Cannot overwrite directory {} with file {}", source.to_string_lossy(), dst.to_string_lossy()),
            DomainError::CreateUnknown(path) => write!(f, "Cannot create unknown kind at path {}", path.to_string_lossy()),
            DomainError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            DomainError::RecursiveNotAllowed(path) => write!(f, "Delete recursively {} is not allowed", path.to_string_lossy()),
            DomainError::SourceDoesNotExists(source) => write!(f, "Source {} does not exists", source.to_string_lossy()),
            DomainError::UserCancelled => write!(f, "User cancelled operation"),
            DomainError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}


impl error::Error for DomainError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            DomainError::IoError(err) => Some(err),
            DomainError::JsonError(err) => Some(err),
            DomainError::Query(err) => Some(err),
            DomainError::Infrastructure(err) => Some(err),
            _ => None
        }
    }
}