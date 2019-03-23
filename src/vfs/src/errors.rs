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
    DoesNotExists(PathBuf),
    AlreadyExists(PathBuf),
}

impl From<io::Error> for VfsError {
    fn from(error: io::Error) -> Self {
        VfsError::IoError(error)
    }
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VfsError::IoError(ref err) => write!(f, "IO error: {}", err),
            VfsError::HasNoSource(err) => write!(f, "Path {} : Has no source defined virtually", err.as_os_str().to_string_lossy()),
            VfsError::IsNotADirectory(err) => write!(f, "Path {} is not a directory", err.as_os_str().to_string_lossy()),
            VfsError::DoesNotExists(err) => write!(f, "Path {} does not exists", err.as_os_str().to_string_lossy()),
            VfsError::AlreadyExists(err) => write!(f, "Path {} : Already exists", err.as_os_str().to_string_lossy()),
        }
    }
}

impl error::Error for VfsError {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            VfsError::IoError(err) => Some(err),
            _ => None
        }
    }
}
