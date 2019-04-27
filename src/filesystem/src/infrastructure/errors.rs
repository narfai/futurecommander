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
    path::{ PathBuf }
};

use crate::{
    errors::{ QueryError },
};

//Representation error convenient re-export
pub use crate::infrastructure::virt::representation::errors::RepresentationError;

#[derive(Debug)]
pub enum InfrastructureError {
    Io(io::Error),
    Representation(RepresentationError),
    Query(QueryError),
    PathDoesNotExists(PathBuf),
    ParentDoesNotExists(PathBuf),
    ParentIsNotADirectory(PathBuf),
    SourceDoesNotExists(PathBuf),
    SourceIsNotADirectory(PathBuf),
    SourceIsNotAFile(PathBuf),
    DestinationIsNotAFile(PathBuf),
    DestinationAlreadyExists(PathBuf),
    DirectoryIsNotEmpty(PathBuf),
    Custom(String)
}

impl From<io::Error> for InfrastructureError {
    fn from(error: io::Error) -> Self {
        InfrastructureError::Io(error)
    }
}

impl From<RepresentationError> for InfrastructureError {
    fn from(error: RepresentationError) -> Self {
        InfrastructureError::Representation(error)
    }
}

impl From<QueryError> for InfrastructureError {
    fn from(error: QueryError) -> Self {
        InfrastructureError::Query(error)
    }
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfrastructureError::Io(error) => write!(f, "Io error {}", error),
            InfrastructureError::Representation(error) => write!(f, "Representation error {}", error),
            InfrastructureError::Query(error) => write!(f, "Query error {}", error),
            InfrastructureError::PathDoesNotExists(path) => write!(f, "Path {} does not exists", path.to_string_lossy()),
            InfrastructureError::ParentDoesNotExists(path) => write!(f, "Parent path {} does not exists", path.to_string_lossy()),
            InfrastructureError::ParentIsNotADirectory(path) => write!(f, "Parent path {} is not a directory", path.to_string_lossy()),
            InfrastructureError::SourceDoesNotExists(path) => write!(f, "Source path {} does not exists", path.to_string_lossy()),
            InfrastructureError::SourceIsNotADirectory(path) => write!(f, "Source path {} is not a directory", path.to_string_lossy()),
            InfrastructureError::SourceIsNotAFile(path) => write!(f, "Source path {} is not a file", path.to_string_lossy()),
            InfrastructureError::DestinationIsNotAFile(path) => write!(f, "Destination path {} is not a file", path.to_string_lossy()),
            InfrastructureError::DestinationAlreadyExists(path) => write!(f, "Destination path {} already exists", path.to_string_lossy()),
            InfrastructureError::DirectoryIsNotEmpty(path) => write!(f, "Directory {} is not empty", path.to_string_lossy()),
            InfrastructureError::Custom(message) => write!(f, "Custom message {}", message),
        }
    }
}

impl error::Error for InfrastructureError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            InfrastructureError::Io(err) => Some(err),
            InfrastructureError::Representation(err) => Some(err),
            InfrastructureError::Query(err) => Some(err),
            _ => None
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod errors_tests {
    use super::*;

    use crate::{
        sample::Samples,
        port::{
            FileSystemAdapter,
            WriteableFileSystem
        },
        infrastructure::{
            RealFileSystem,
            VirtualFileSystem
        }
    };

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    //Error testing
    #[test]
    fn error_path_does_not_exists() {
        let sample_path = Samples::init_advanced_chroot("error_path_does_not_exists");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let not_exists = sample_path.join("NOTEXISTS");
        let expected_error = InfrastructureError::PathDoesNotExists(not_exists.clone());
        assert_two_errors_equals(&vfs.remove_file(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.remove_empty_directory(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.remove_file(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.remove_empty_directory(not_exists.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_parent_does_not_exists() {
        let sample_path = Samples::init_advanced_chroot("error_parent_does_not_exists");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let existing_source = sample_path.join("F");
        let not_exists = sample_path.join("NOTPARENT/NOTCHILD");
        let expected_error = InfrastructureError::ParentDoesNotExists(not_exists.parent().unwrap().to_path_buf());

        assert_two_errors_equals(&vfs.create_empty_directory(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.create_empty_file(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.copy_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.move_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.bind_directory_to_directory(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.create_empty_directory(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.create_empty_file(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.copy_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.move_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.bind_directory_to_directory(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_parent_is_not_a_directory() {
        let sample_path = Samples::init_advanced_chroot("error_parent_is_not_a_directory");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let existing_source = sample_path.join("A");
        let not_exists = sample_path.join("F/NOTCHILD");
        let expected_error = InfrastructureError::ParentIsNotADirectory(not_exists.parent().unwrap().to_path_buf());

        assert_two_errors_equals(&vfs.create_empty_directory(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.create_empty_file(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.copy_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.move_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.bind_directory_to_directory(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.create_empty_directory(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.create_empty_file(not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.copy_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.move_file_to_file(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.bind_directory_to_directory(existing_source.as_path(), not_exists.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_source_does_not_exists() {
        let sample_path = Samples::init_advanced_chroot("error_source_does_not_exists");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let not_exists = sample_path.join("NOTEXISTS");
        let existing_destination = sample_path.join("A");
        let expected_error = InfrastructureError::SourceDoesNotExists(not_exists.clone());

        assert_two_errors_equals(&vfs.copy_file_to_file(not_exists.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.move_file_to_file(not_exists.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.bind_directory_to_directory(not_exists.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.copy_file_to_file(not_exists.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.move_file_to_file(not_exists.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.bind_directory_to_directory(not_exists.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_source_is_not_a_directory() {
        let sample_path = Samples::init_advanced_chroot("error_source_is_not_a_directory");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let new_destination = sample_path.join("NEW");
        let is_file = sample_path.join("F");
        let expected_error = InfrastructureError::SourceIsNotADirectory(is_file.clone());

        assert_two_errors_equals(&vfs.bind_directory_to_directory(is_file.as_path(), new_destination.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.bind_directory_to_directory(is_file.as_path(), new_destination.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_source_is_not_a_file() {
        let sample_path = Samples::init_advanced_chroot("error_source_is_not_a_file");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let not_a_file = sample_path.join("A");
        let new_destination = sample_path.join("NEW");
        let expected_error = InfrastructureError::SourceIsNotAFile(not_a_file.clone());

        assert_two_errors_equals(&vfs.copy_file_to_file(not_a_file.as_path(), new_destination.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.move_file_to_file(not_a_file.as_path(), new_destination.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.copy_file_to_file(not_a_file.as_path(), new_destination.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.move_file_to_file(not_a_file.as_path(), new_destination.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_destination_is_not_a_file() {
        let sample_path = Samples::init_advanced_chroot("error_destination_is_not_a_file");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let not_a_file = sample_path.join("A");
        let existing_source = sample_path.join("F");
        let expected_error = InfrastructureError::DestinationIsNotAFile(not_a_file.clone());

        assert_two_errors_equals(&vfs.copy_file_to_file(existing_source.as_path(), not_a_file.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&vfs.move_file_to_file(existing_source.as_path(), not_a_file.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.copy_file_to_file(existing_source.as_path(), not_a_file.as_path()).err().unwrap(), &expected_error);
        assert_two_errors_equals(&rfs.move_file_to_file(existing_source.as_path(), not_a_file.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_destination_already_exists() {
        let sample_path = Samples::init_advanced_chroot("error_destination_already_exists");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let existing_destination = sample_path.join("A");
        let existing_source = sample_path.join("B");
        let expected_error = InfrastructureError::DestinationAlreadyExists(existing_destination.clone());

        assert_two_errors_equals(&vfs.bind_directory_to_directory(existing_source.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.bind_directory_to_directory(existing_source.as_path(), existing_destination.as_path()).err().unwrap(), &expected_error);
    }

    #[test]
    fn error_directory_is_not_empty() {
        let sample_path = Samples::init_advanced_chroot("error_directory_is_not_empty");
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let mut rfs = FileSystemAdapter(RealFileSystem::default());

        let not_empty = sample_path.join("A");
        let expected_error = InfrastructureError::DirectoryIsNotEmpty(not_empty.clone());

        assert_two_errors_equals(&vfs.remove_empty_directory(not_empty.as_path()).err().unwrap(), &expected_error);

        assert_two_errors_equals(&rfs.remove_empty_directory(not_empty.as_path()).err().unwrap(), &expected_error);
    }
}
