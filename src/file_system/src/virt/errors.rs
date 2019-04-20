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


use std::io;
use std::path::{ PathBuf };
use std::{ error, fmt };

use crate::{
    representation::errors::RepresentationError
};

#[derive(Debug)]
pub enum VirtualError {
    RepresentationError(RepresentationError),
    IoError(io::Error),
    CopyIntoItSelf(PathBuf, PathBuf),
    OverwriteDirectoryWithFile(PathBuf, PathBuf),
    CopyDirectoryIntoFile(PathBuf, PathBuf),
    MergeNotAllowed(PathBuf, PathBuf),
    OverwriteNotAllowed(PathBuf, PathBuf),
    ParentDoesNotExists(PathBuf),
    ParentIsNotADirectory(PathBuf),
    SourceDoesNotExists(PathBuf),
    DoesNotExists(PathBuf),
    IsNotADirectory(PathBuf),
    AddSubDanglingVirtualPath(PathBuf)
}

impl From<RepresentationError> for VirtualError {
    fn from(error: RepresentationError) -> Self {
        VirtualError::RepresentationError(error)
    }
}

impl From<io::Error> for VirtualError {
    fn from(error: io::Error) -> Self {
        VirtualError::IoError(error)
    }
}


impl fmt::Display for VirtualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtualError::RepresentationError(ref err) => write!(f, "Representation error: {}", err),
            VirtualError::CopyIntoItSelf(source, dst) => write!(f, "Cannot copy {} into itself {}", source.to_string_lossy(), dst.to_string_lossy()),
            VirtualError::OverwriteDirectoryWithFile(source, dst) => write!(f, "Cannot overwrite directory {} with file {}", source.to_string_lossy(), dst.to_string_lossy()),
            VirtualError::CopyDirectoryIntoFile(source, dst) => write!(f, "Cannot copy directory {} into file {}", source.to_string_lossy(), dst.to_string_lossy()),
            VirtualError::OverwriteNotAllowed(source, dst) => write!(f, "Overwrite of {} into {} is not allowrd", source.to_string_lossy(), dst.to_string_lossy()),
            VirtualError::MergeNotAllowed(source, dst) => write!(f, "Merge of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            VirtualError::ParentDoesNotExists(path) => write!(f, "Parent {} does not exists", path.to_string_lossy()),
            VirtualError::ParentIsNotADirectory(path) => write!(f, "Parent {} is not a directory", path.to_string_lossy()),
            VirtualError::SourceDoesNotExists(path) => write!(f, "Source {} does not exists", path.to_string_lossy()),
            VirtualError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            VirtualError::IoError(error) => write!(f, "IO error: {}", error),
            VirtualError::IsNotADirectory(path) => write!(f, "Path {} is not a directory", path.to_string_lossy()),
            VirtualError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            VirtualError::AddSubDanglingVirtualPath(path) => write!(f, "Path {} is present in both add and sub representations", path.to_string_lossy())
        }
    }
}

impl error::Error for VirtualError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            VirtualError::RepresentationError(err) => Some(err),
            VirtualError::IoError(err) => Some(err),
            _ => None
        }
    }
}
