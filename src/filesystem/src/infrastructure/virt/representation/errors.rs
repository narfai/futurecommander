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

use std::path::{ PathBuf };
use std::{ error, fmt };

#[derive(Debug)]
pub enum RepresentationError {
    AlreadyExists(PathBuf),
    VirtualParentIsAFile(PathBuf),
    DoesNotExists(PathBuf),
    IsNotADirectory(PathBuf),
    IsRelativePath(PathBuf),
}
impl fmt::Display for RepresentationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepresentationError::AlreadyExists(identity) => write!(f, "Identity {} already exists", identity.as_os_str().to_string_lossy()),
            RepresentationError::VirtualParentIsAFile(identity) => write!(f, "Identity parent {} is a file", identity.as_os_str().to_string_lossy()),
            RepresentationError::DoesNotExists(identity) => write!(f, "Identity {} does not exists", identity.as_os_str().to_string_lossy()),
            RepresentationError::IsNotADirectory(identity) => write!(f, "Identity {} is not a directory", identity.as_os_str().to_string_lossy()),
            RepresentationError::IsRelativePath(identity) => write!(f, "Path {} is relative", identity.as_os_str().to_string_lossy()),
        }
    }
}


impl error::Error for RepresentationError {}
