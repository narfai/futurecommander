use std::{
    path::{ Path, PathBuf, Ancestors },
    iter::{
        Rev
    },
    slice::{
        Iter
    }
};

use serde::{ Serialize, Deserialize };

use crate::{
    Kind,
    errors::DomainError,
    infrastructure::errors::InfrastructureError,
    port::{
        ReadableFileSystem,
        WriteableFileSystem,
        Atomic,
        Entry,
        Operation,
        OperationGenerator
    }
};

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum SerializableKind {
    File,
    Directory,
    Unknown
}

impl From<Kind> for SerializableKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::File => SerializableKind::File,
            Kind::Directory => SerializableKind::Directory,
            Kind::Unknown => SerializableKind::Unknown,
        }
    }
}

impl From<SerializableKind> for Kind {
    fn from(kind: SerializableKind) -> Self {
        match kind {
            SerializableKind::File => Kind::File,
            SerializableKind::Directory => Kind::Directory,
            SerializableKind::Unknown => Kind::Unknown,
        }
    }
}

#[derive(Clone)]
pub struct CreateBatchDefinition {
    path: PathBuf,
    kind: SerializableKind
}

impl CreateBatchDefinition {
    pub fn new(path: PathBuf, kind: Kind) -> Self {
        CreateBatchDefinition { path, kind: kind.into() }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CreateScheduling {
    FileCreation,
    DirectoryCreation,
    FileCreationOverwrite,
    DirectoryCreationOverwrite,
}

pub struct CreateOperation {
    scheduling: CreateScheduling,
    transaction: Vec<Atomic>,
    definition: CreateBatchDefinition
}

impl CreateOperation {
    fn schedule<F: ReadableFileSystem>(fs: &F, definition: &CreateBatchDefinition) -> Result<CreateScheduling, DomainError> {
        let entry = fs.status(&definition.path)?;

        if entry.exists() {
            if entry.is_dir() {
                return Err(DomainError::DirectoryOverwriteNotAllowed(entry.to_path()))
            } else if !entry.is_file() {
                return Err(DomainError::CreateUnknown(entry.to_path()))
            }
        }

        match definition.kind.into() {
            Kind::Directory => {
                if entry.exists() && entry.is_file() {
                    Ok(CreateScheduling::DirectoryCreationOverwrite)
                } else {
                    Ok(CreateScheduling::DirectoryCreation)
                }
            },
            Kind::File => {
                if entry.exists() && entry.is_file() {
                    Ok(CreateScheduling::FileCreationOverwrite)
                } else {
                    Ok(CreateScheduling::FileCreation)
                }
            },
            Kind::Unknown => {
                Err(DomainError::CreateUnknown(entry.to_path()))
            }
        }
    }

    fn transaction(scheduling: &CreateScheduling, definition: &CreateBatchDefinition) -> Vec<Atomic> {
        match scheduling {
            CreateScheduling::FileCreation => vec![
                Atomic::CreateEmptyFile(definition.path.to_path_buf())
            ],
            CreateScheduling::DirectoryCreation => vec![
                Atomic::CreateEmptyDirectory(definition.path.to_path_buf())
            ],
            CreateScheduling::DirectoryCreationOverwrite => vec![
                Atomic::RemoveFile(definition.path.to_path_buf()),
                Atomic::CreateEmptyDirectory(definition.path.to_path_buf())
            ],
            CreateScheduling::FileCreationOverwrite => vec![
                Atomic::RemoveFile(definition.path.to_path_buf()),
                Atomic::CreateEmptyFile(definition.path.to_path_buf())
            ],
            _ => Vec::new()
        }
    }

    fn new(scheduling: CreateScheduling, definition: CreateBatchDefinition) -> Self {
        CreateOperation {
            scheduling,
            transaction: Self::transaction(&scheduling, &definition),
            definition
        }
    }
}

impl Operation for CreateOperation {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.transaction.iter() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}

pub enum CreateOperationGeneratorState {
    Uninitialized,
    ParentOperation(Vec<PathBuf>),
    SelfOperation,
    Terminated
}

pub struct CreateOperationGenerator {
    definition: CreateBatchDefinition,
    state: CreateOperationGeneratorState
}

impl CreateOperationGenerator {
    pub fn new(definition: CreateBatchDefinition) -> Self {
        CreateOperationGenerator {
            state: CreateOperationGeneratorState::Uninitialized,
            definition
        }
    }
}

impl <'a, E: Entry> OperationGenerator<E> for CreateOperationGenerator {
    type Item = CreateOperation;
    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        match &mut self.state {
            CreateOperationGeneratorState::Uninitialized => {
                let mut ancestors = self.definition.path.ancestors();
                ancestors.next();
                self.state = CreateOperationGeneratorState::ParentOperation(
                    ancestors.map(|path| path.to_path_buf()).collect()
                );
                self.next(fs)
            },
            CreateOperationGeneratorState::ParentOperation(ancestors)=> {
                if let Some(path) = ancestors.pop(){
                    let entry = fs.status(&path)?;
                    if !entry.exists() {
                        let definition = CreateBatchDefinition::new(entry.to_path(), Kind::Directory);
                        return Ok(
                            Some(
                                CreateOperation::new(
                                    CreateOperation::schedule(fs, &definition)?,
                                    definition
                                )
                            )
                        );
                    }
                } else {
                    self.state = CreateOperationGeneratorState::SelfOperation
                }
                self.next(fs)
            },
            CreateOperationGeneratorState::SelfOperation => {
                self.state = CreateOperationGeneratorState::Terminated;
                Ok(
                    Some(
                        CreateOperation::new(
                            CreateOperation::schedule(fs, &self.definition)?,
                            self.definition.clone()
                        )
                    )
                )
            },
            CreateOperationGeneratorState::Terminated => Ok(None)
        }
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use super::*;

    use crate::{
        sample::Samples,
        port::{
            FileSystemAdapter
        },
        infrastructure::{
            RealFileSystem
        },
        capability::{
            ZealousGuard
        }
    };

    #[test]
    fn create_operation_directory(){
        let chroot = Samples::init_simple_chroot("create_operation_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("CREATED"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("CREATED").is_dir());
        assert!(chroot.join("CREATED").exists());
    }


    #[test]
    fn create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("CREATED/NESTED/DIRECTORY"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("CREATED/NESTED/DIRECTORY").exists());
        assert!(chroot.join("CREATED/NESTED/DIRECTORY").is_dir());
    }

    #[test]
    fn create_operation_file(){
        let chroot = Samples::init_simple_chroot("create_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("CREATED"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("CREATED").exists());
        assert!(chroot.join("CREATED").is_file());
    }

    #[test]
    fn create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("create_operation_file_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("RDIR/RFILEA"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert_ne!(a_len, chroot.join("RDIR/RFILEA").metadata().unwrap().len());
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod virtual_tests {
    use super::*;

    use crate::{
        sample::Samples,
        port::{
            FileSystemAdapter
        },
        infrastructure::{
            VirtualFileSystem
        },
        capability::{
            ZealousGuard
        }
    };


    #[test]
    fn virtual_create_operation_directory(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("CREATED"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED").as_path()).unwrap());
    }


    #[test]
    fn virtual_create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("CREATED/NESTED/DIRECTORY"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("CREATED"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(chroot.join("CREATED").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file_overwrite");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = CreateOperationGenerator::new(
            CreateBatchDefinition::new(
                chroot.join("RDIR/RFILEA"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        let entry = fs.status(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(entry.exists());
        assert!(entry.is_file());
    }
}
