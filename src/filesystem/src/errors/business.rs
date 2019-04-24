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

//use crate::{
//    infrastructure::virt::representation::errors::RepresentationError,
//    query::QueryError
//};

#[derive(Debug)]
pub enum BusinessError {
    Io(io::Error),
//    Representation(RepresentationError),
//    Query(QueryError),

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

impl From<io::Error> for BusinessError {
    fn from(error: io::Error) -> Self {
        BusinessError::Io(error)
    }
}

//impl From<RepresentationError> for BusinessError {
//    fn from(error: RepresentationError) -> Self {
//        BusinessError::Representation(error)
//    }
//}
//
//impl From<QueryError> for BusinessError {
//    fn from(error: QueryError) -> Self {
//        BusinessError::Query(error)
//    }
//}


impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
//            BusinessError::Io(error) => write!(f, "Io error {}", error),
//            BusinessError::Representation(error) => write!(f, "Representation error {}", error),
//            BusinessError::Query(error) => write!(f, "Query error {}", error),
            //Virtual Only
            BusinessError::CopyIntoItSelf(source, dst) => write!(f, "Cannot copy {} into itself {}", source.to_string_lossy(), dst.to_string_lossy()),

            //Real Only
            BusinessError::AlreadyExists(path) => write!(f, "Path {} already exists", path.to_string_lossy()),
            BusinessError::DirectoryIsNotEmpty(path) => write!(f, "Directory {} is not empty", path.to_string_lossy()),
            BusinessError::SourceIsNotAFile(source) => write!(f, "Source {} is not a file", source.to_string_lossy()),

            //Common
            BusinessError::ParentDoesNotExists(parent) => write!(f, "Parent {} does not exists", parent.to_string_lossy()),
            BusinessError::ParentIsNotADirectory(parent) => write!(f, "Parent {} is not a directory", parent.to_string_lossy()),
            BusinessError::SourceDoesNotExists(source) => write!(f, "Source {} does not exists", source.to_string_lossy()),
            BusinessError::OverwriteDirectoryWithFile(source, dst) => write!(f, "Cannot overwrite directory {} with file {}", source.to_string_lossy(), dst.to_string_lossy()),
            BusinessError::CopyDirectoryIntoFile(source, dst) => write!(f, "Cannot copy directory {} into file {}", source.to_string_lossy(), dst.to_string_lossy()),
            BusinessError::OverwriteNotAllowed(source, dst) => write!(f, "Overwrite of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            BusinessError::MergeNotAllowed(source, dst) => write!(f, "Merge of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            BusinessError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            BusinessError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}


impl error::Error for BusinessError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            BusinessError::Io(err) => Some(err),
            BusinessError::Representation(err) => Some(err),
            BusinessError::Query(err) => Some(err),
            _ => None
        }
    }
}
