// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod domain;
pub use self::domain::DomainError;

mod query;
pub use self::query::QueryError;


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod query_errors {
    use std::{
        error,
        path::{
            Path,
            PathBuf
        }
    };
    use crate::{
        Kind,
        sample::Samples,
        infrastructure::{
            FileSystemAdapter,
            ReadableFileSystem,
            RealFileSystem,
            VirtualFileSystem
        }
    };
    use super::*;

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


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod domain_errors {
    use std::{
        error,
        path::{ PathBuf }
    };
    use crate::{
        Kind,
        sample::Samples,
        infrastructure::{
            FileSystemAdapter,
            RealFileSystem,
            VirtualFileSystem
        },
        operation::{
            OperationGenerator,
            OperationGeneratorInterface,
            CreateRequest,
            RemoveRequest,
            MoveRequest,
            CopyRequest
        }
    };
    use super::*;

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
            &OperationGenerator::new(
                MoveRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                MoveRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_directory_overwrite_not_allowed() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let to_overwrite = sample_path.join("A");

        let expected_error = DomainError::DirectoryOverwriteNotAllowed(to_overwrite.clone());

        assert_two_errors_equals(
            &OperationGenerator::new(
                CreateRequest::new(
                    to_overwrite.to_path_buf(),
                    Kind::Directory
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CreateRequest::new(
                    to_overwrite.to_path_buf(),
                    Kind::Directory
                )
            ).next(&rfs).err().unwrap(),
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
            &OperationGenerator::new(
                MoveRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                MoveRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
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
            &OperationGenerator::new(
                MoveRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                MoveRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
            &expected_error
        );
    }

    #[test]
    fn error_create_unknown() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let rfs = FileSystemAdapter(RealFileSystem::default());

        let dummy = sample_path.join("A/UNKNOW");

        let expected_error = DomainError::CreateUnknown(dummy.clone());

        assert_two_errors_equals(
            &OperationGenerator::new(
                CreateRequest::new(
                    dummy.to_path_buf(),
                    Kind::Unknown
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CreateRequest::new(
                    dummy.to_path_buf(),
                    Kind::Unknown
                )
            ).next(&rfs).err().unwrap(),
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
            &OperationGenerator::new(
                RemoveRequest::new(
                    not_exists.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                RemoveRequest::new(
                    not_exists.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
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
            &OperationGenerator::new(
                MoveRequest::new(
                    not_existing_source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                MoveRequest::new(
                    not_existing_source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    not_existing_source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&rfs).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &OperationGenerator::new(
                CopyRequest::new(
                    not_existing_source.to_path_buf(),
                    destination.to_path_buf()
                )
            ).next(&vfs).err().unwrap(),
            &expected_error
        );
    }
}
