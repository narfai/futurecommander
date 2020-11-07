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
    infrastructure::errors::RepresentationError
};

#[derive(Debug)]
pub enum QueryError {
    Io(io::Error),
    Representation(RepresentationError),
    AddSubDanglingVirtualPath(PathBuf),
    IsNotADirectory(PathBuf),
    ReadTargetDoesNotExists(PathBuf)
}

impl From<io::Error> for QueryError {
    fn from(error: io::Error) -> Self {
        QueryError::Io(error)
    }
}

impl From<RepresentationError> for QueryError {
    fn from(error: RepresentationError) -> Self {
        QueryError::Representation(error)
    }
}


impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::Io(error) => write!(f, "Query io error {}", error),
            QueryError::Representation(error) => write!(f, "Query representation error {}", error),
            QueryError::AddSubDanglingVirtualPath(path) => write!(f, "Path {} is present in both add and sub representations", path.to_string_lossy()),
            QueryError::IsNotADirectory(path) => write!(f, "Path {} is not a directory", path.to_string_lossy()),
            QueryError::ReadTargetDoesNotExists(path) => write!(f, "Read target {} does not exists", path.to_string_lossy()),
        }
    }
}


impl error::Error for QueryError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            QueryError::Io(err) => Some(err),
            QueryError::Representation(err) => Some(err),
            _ => None
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod errors_tests {
    use super::*;

    use std::{
        path::{
            Path,
            PathBuf
        }
    };

    use crate::{
        Kind,
        sample::Samples,
        port::{
            FileSystemAdapter,
            ReadableFileSystem
        },
        infrastructure::{
            RealFileSystem,
            VirtualFileSystem
        }
    };

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn error_is_not_a_directory() {
        let sample_path = Samples::init_advanced_chroot("error_is_not_a_directory");
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let is_not_a_directory = sample_path.join("F");
        let expected_error = QueryError::IsNotADirectory(is_not_a_directory.clone());

        assert_two_errors_equals(&vfs.read_dir(is_not_a_directory.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.read_dir(is_not_a_directory.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_read_target_does_not_exists() {
        let sample_path = Samples::init_advanced_chroot("error_read_target_does_not_exists");
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let not_exists = sample_path.join("NOTEXISTS");
        let expected_error = QueryError::ReadTargetDoesNotExists(not_exists.clone());

        assert_two_errors_equals(&vfs.read_dir(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.read_dir(not_exists.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_add_sub_dangling(){
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        vfs.as_inner_mut().mut_add_state().attach(Path::new("/TEST"), None, Kind::Directory).unwrap();
        vfs.as_inner_mut().mut_sub_state().attach(Path::new("/TEST"), None, Kind::Directory).unwrap();

        let expected_error = QueryError::AddSubDanglingVirtualPath(PathBuf::from("/TEST"));

        assert_two_errors_equals(&vfs.status(Path::new("/TEST")).err().unwrap(), &expected_error);
    }
}
