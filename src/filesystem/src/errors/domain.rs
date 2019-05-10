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
    errors::{
        QueryError
    },
    infrastructure::{
        errors::InfrastructureError
    },
};

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


#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod errors_tests {
    use super::*;

    use crate::{
        Kind,
        sample::Samples,
        event::*,
        capability::*,
        port::{
            FileSystemAdapter,
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
    fn error_query_error(){
        let query_emock = QueryError::IsNotADirectory(PathBuf::from("/TEST"));
        assert_two_errors_equals(
            &DomainError::from(query_emock),
            &DomainError::Query(QueryError::IsNotADirectory(PathBuf::from("/TEST")))
        );
    }

    #[test]
    fn error_infrastucture_error(){
        let query_emock = QueryError::IsNotADirectory(PathBuf::from("/TEST"));
        assert_two_errors_equals(
            &DomainError::from(query_emock),
            &DomainError::Query(QueryError::IsNotADirectory(PathBuf::from("/TEST")))
        );
    }

    #[test]
    fn error_copy_into_itself() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());
        let source = sample_path.join("B");
        let destination = sample_path.join("B/D/B");

        let expected_error = DomainError::CopyIntoItSelf(source.clone(), destination.clone());
        assert_two_errors_equals(
            &MoveEvent::new(
            source.as_path(),
            destination.as_path(),
            true,
            false
                ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &MoveEvent::new(
            source.as_path(),
            destination.as_path(),
            true,
            false
                ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
            source.as_path(),
            destination.as_path(),
            true,
            false
                ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
            source.as_path(),
            destination.as_path(),
            true,
            false
                ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_merge_not_allowed() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let source = sample_path.join("B");
        let destination = sample_path.join("A");

        let expected_error = DomainError::MergeNotAllowed(destination.clone());
        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                false,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                false,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                false,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                false,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_overwrite_not_allowed() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let source = sample_path.join("F");
        let destination = sample_path.join("A/C");

        let expected_error = DomainError::OverwriteNotAllowed(destination.clone());
        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CreateEvent::new(
                destination.as_path(),
                Kind::File,
                false,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CreateEvent::new(
                destination.as_path(),
                Kind::File,
                false,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_directory_overwrite_not_allowed() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let to_overwrite = sample_path.join("A/C");

        let expected_error = DomainError::DirectoryOverwriteNotAllowed(to_overwrite.clone());

        assert_two_errors_equals(
            &CreateEvent::new(
                to_overwrite.as_path(),
                Kind::Directory,
                false,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CreateEvent::new(
                to_overwrite.as_path(),
                Kind::Directory,
                false,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_merge_file_with_directory() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let source = sample_path.join("B");
        let destination = sample_path.join("A/C");

        let expected_error = DomainError::MergeFileWithDirectory(source.clone(), destination.clone());
        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_overwrite_directory_with_file() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());


        let source = sample_path.join("A/C");
        let destination = sample_path.join("B");

        let expected_error = DomainError::OverwriteDirectoryWithFile(source.clone(), destination.clone());
        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                true
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &MoveEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                true
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                true
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                source.as_path(),
                destination.as_path(),
                true,
                true
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_create_unknown() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let dummy = sample_path.join("A");

        let expected_error = DomainError::CreateUnknown(dummy.clone());

        assert_two_errors_equals(
            &CreateEvent::new(
                dummy.as_path(),
                Kind::Unknown,
                false,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CreateEvent::new(
                dummy.as_path(),
                Kind::Unknown,
                false,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_does_not_exists() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let not_exists = sample_path.join("NOTEXISTS");

        let expected_error = DomainError::DoesNotExists(not_exists.clone());

        assert_two_errors_equals(
            &RemoveEvent::new(
                not_exists.as_path(),
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &RemoveEvent::new(
                not_exists.as_path(),
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_delete_recursive_not_allowed() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let not_empty_dir = sample_path.join("A");

        let expected_error = DomainError::RecursiveNotAllowed(not_empty_dir.clone());

        assert_two_errors_equals(
            &RemoveEvent::new(
                not_empty_dir.as_path(),
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &RemoveEvent::new(
                not_empty_dir.as_path(),
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

    }

    #[test]
    fn error_source_does_not_exists() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let not_existing_source = sample_path.join("NOTEXISTS");
        let destination = sample_path.join("NEW");

        let expected_error = DomainError::SourceDoesNotExists(not_existing_source.clone());
        assert_two_errors_equals(
            &MoveEvent::new(
                not_existing_source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &MoveEvent::new(
                not_existing_source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                not_existing_source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&vfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &CopyEvent::new(
                not_existing_source.as_path(),
                destination.as_path(),
                true,
                false
            ).atomize(&rfs, &mut ZealedGuard).err().unwrap(),
            &expected_error
        );
    }
}
