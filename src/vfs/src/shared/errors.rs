/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommanderVfs.
 *
 * FutureCommanderVfs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommanderVfs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommanderVfs.  If not, see <https://www.gnu.org/licenses/>.
 */


use std::io;
use std::path::{ PathBuf };
use std::{ error, fmt };

#[derive(Debug)]
pub enum VfsError {
    IoError(io::Error),
    HasNoSource(PathBuf),
    IsNotADirectory(PathBuf),
    IsNotAFile(PathBuf),
    DoesNotExists(PathBuf),
    AlreadyExists(PathBuf),
    VirtualParentIsAFile(PathBuf),
    SubDanglingVirtualPath(PathBuf),
    AddSubDanglingVirtualPath(PathBuf),
    IsRelativePath(PathBuf),
    IsDotName(PathBuf),
    CopyIntoItSelf(PathBuf, PathBuf),
    SourceDoesNotExists(PathBuf),
    Custom(String)
}

impl From<io::Error> for VfsError {
    fn from(error: io::Error) -> Self {
        VfsError::IoError(error)
    }
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VfsError::IoError(ref err)                  => write!(f, "IO error: {}", err),
            VfsError::HasNoSource(identity)             => write!(f, "Path {} has no source defined virtually", identity.as_os_str().to_string_lossy()),
            VfsError::IsNotADirectory(identity)         => write!(f, "Path {} is not a directory", identity.as_os_str().to_string_lossy()),
            VfsError::IsNotAFile(identity)              => write!(f, "Path {} is not a file", identity.as_os_str().to_string_lossy()),
            VfsError::DoesNotExists(identity)           => write!(f, "Path {} does not exists", identity.as_os_str().to_string_lossy()),
            VfsError::AlreadyExists(identity)           => write!(f, "Path {} already exists", identity.as_os_str().to_string_lossy()),
            VfsError::VirtualParentIsAFile(identity)    => write!(f, "Path {} virtual parent is a file", identity.as_os_str().to_string_lossy()),
            VfsError::AddSubDanglingVirtualPath(identity)     => write!(f, "Path {} dangling virtual path appears to be deleted but still in add delta", identity.as_os_str().to_string_lossy()),
            VfsError::SubDanglingVirtualPath(identity)     => write!(f, "Path {} dangling virtual path appears to deleted but does not exists on real fs", identity.as_os_str().to_string_lossy()),
            VfsError::IsRelativePath(identity)          => write!(f, "Path {} is relative", identity.as_os_str().to_string_lossy()),
            VfsError::IsDotName(identity)           =>  write!(f, "Path {} has a file name with dots", identity.as_os_str().to_string_lossy()),
            VfsError::CopyIntoItSelf(source, destination) =>  write!(f, "Cannot copy {} into itsef {}", source.as_os_str().to_string_lossy(), destination.as_os_str().to_string_lossy()),
            VfsError::SourceDoesNotExists(source) =>  write!(f, "Real source does not exists {}", source.as_os_str().to_string_lossy()),
            VfsError::Custom(custom_message) => write!(f, "Custom message {}", custom_message)
        }
    }
}

impl error::Error for VfsError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            VfsError::IoError(err) => Some(err),
            _ => None
        }
    }
}