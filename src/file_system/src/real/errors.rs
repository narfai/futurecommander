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
    io, error, fmt,
    path::{ PathBuf }
};

#[derive(Debug)]
pub enum RealError {
    IoError(io::Error),
    AlreadyExists(PathBuf),
    SourceDoesNotExists(PathBuf),
    OverwriteDirectoryWithFile(PathBuf, PathBuf),
    CopyDirectoryIntoFile(PathBuf, PathBuf),
    ParentDoesNotExists(PathBuf),
    ParentIsNotADirectory(PathBuf),
    MergeNotAllowed(PathBuf, PathBuf),
    OverwriteNotAllowed(PathBuf, PathBuf),
    DirectoryIsNotEmpty(PathBuf),
    DoesNotExists(PathBuf),
    SourceIsNotAFile(PathBuf)
}

impl From<io::Error> for RealError {
    fn from(error: io::Error) -> Self {
        RealError::IoError(error)
    }
}

impl error::Error for RealError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            RealError::IoError(err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for RealError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RealError::IoError(error) => write!(f, "IO error: {}", error),
            RealError::AlreadyExists(path) => write!(f, "Path {} already exists", path.to_string_lossy()),
            RealError::ParentDoesNotExists(parent) => write!(f, "Parent {} does not exists", parent.to_string_lossy()),
            RealError::ParentIsNotADirectory(parent) => write!(f, "Parent {} is not a directory", parent.to_string_lossy()),
            RealError::SourceDoesNotExists(source) => write!(f, "Source {} does not exists", source.to_string_lossy()),
            RealError::OverwriteDirectoryWithFile(source, dst) => write!(f, "Cannot overwrite directory {} with file {}", source.to_string_lossy(), dst.to_string_lossy()),
            RealError::CopyDirectoryIntoFile(source, dst) => write!(f, "Cannot copy directory {} into file {}", source.to_string_lossy(), dst.to_string_lossy()),
            RealError::OverwriteNotAllowed(source, dst) => write!(f, "Overwrite of {} into {} is not allowrd", source.to_string_lossy(), dst.to_string_lossy()),
            RealError::MergeNotAllowed(source, dst) => write!(f, "Merge of {} into {} is not allowed", source.to_string_lossy(), dst.to_string_lossy()),
            RealError::DirectoryIsNotEmpty(path) => write!(f, "Directory {} is not empty", path.to_string_lossy()),
            RealError::DoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            RealError::SourceIsNotAFile(source) => write!(f, "Source {} is not a file", source.to_string_lossy()),
        }
    }
}

