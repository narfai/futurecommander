// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod domain;
pub use self::domain::DomainError;

mod query;
pub use self::query::QueryError;


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod query_errors {
    use std::path::{
        Path,
        PathBuf
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
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                source.as_path(),
                destination.as_path(),
                true,
                false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    false,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    false,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    false,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    false,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::create(
                CreateOperationDefinition::new(
                    destination.as_path(),
                    Kind::File,
                    false,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::create(
                CreateOperationDefinition::new(
                    destination.as_path(),
                    Kind::File,
                    false,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::create(
                CreateOperationDefinition::new(
                    to_overwrite.as_path(),
                    Kind::Directory,
                    false,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::create(
                CreateOperationDefinition::new(
                    to_overwrite.as_path(),
                    Kind::Directory,
                    false,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    true
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    true
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    true
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    source.as_path(),
                    destination.as_path(),
                    true,
                    true
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::create(
                CreateOperationDefinition::new(
                    dummy.as_path(),
                    Kind::Unknown,
                    false,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::create(
                CreateOperationDefinition::new(
                    dummy.as_path(),
                    Kind::Unknown,
                    false,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::remove(
                RemoveOperationDefinition::new(
                    not_exists.as_path(),
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::remove(
                RemoveOperationDefinition::new(
                    not_exists.as_path(),
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &&FileSystemOperation::remove(
                RemoveOperationDefinition::new(
                    not_empty_dir.as_path(),
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &&FileSystemOperation::remove(
                RemoveOperationDefinition::new(
                    not_empty_dir.as_path(),
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
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
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    not_existing_source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::mov(
                MoveOperationDefinition::new(
                    not_existing_source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    not_existing_source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&vfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );

        assert_two_errors_equals(
            &FileSystemOperation::copy(
                CopyOperationDefinition::new(
                    not_existing_source.as_path(),
                    destination.as_path(),
                    true,
                    false
                )
            ).atomize(&rfs, Box::new(ZealousGuard)).err().unwrap(),
            &expected_error
        );
    }
}
