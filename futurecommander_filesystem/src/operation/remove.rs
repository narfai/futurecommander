use std::{
    path::{ Path, PathBuf },
    iter
};

use crate::{
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

pub struct RemoveBatchDefinition {
    path: PathBuf
}

impl RemoveBatchDefinition {
    pub fn new(path: PathBuf) -> Self {
        RemoveBatchDefinition { path }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RemoveScheduling {
    FileRemoval,
    EmptyDirectoryRemoval,
    RecursiveDirectoryRemoval
}

pub struct RemoveOperation {
    pub scheduling: RemoveScheduling,
    pub transaction: Vec<Atomic>,
    pub path: PathBuf
}

impl Operation for RemoveOperation {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.transaction.iter() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}

impl RemoveOperation {
    fn schedule<F: ReadableFileSystem>(fs: &F, path: &Path) -> Result<RemoveScheduling, DomainError> {
        let entry = fs.status(path)?;

        if !entry.exists() {
            return Err(DomainError::DoesNotExists(path.to_path_buf()))
        }

        if entry.is_file() {
            Ok(RemoveScheduling::FileRemoval)
        } else if entry.is_dir() {
            if fs.is_directory_empty(entry.path())? {
                Ok(RemoveScheduling::EmptyDirectoryRemoval)
            } else {
                Ok(RemoveScheduling::RecursiveDirectoryRemoval)
            }
        } else {
            return Err(DomainError::Custom(String::from("Unknown node type")))
        }
    }

    fn transaction(scheduling: &RemoveScheduling, path: &Path) -> Vec<Atomic> {
        match scheduling {
            RemoveScheduling::FileRemoval => vec![
                Atomic::RemoveFile(path.to_path_buf())
            ],
            RemoveScheduling::EmptyDirectoryRemoval|RemoveScheduling::RecursiveDirectoryRemoval => vec![
                Atomic::RemoveEmptyDirectory(path.to_path_buf())
            ],
            _ => Vec::new()
        }
    }

    fn new(scheduling: RemoveScheduling, path: &Path) -> Self {
        RemoveOperation {
            transaction: Self::transaction(&scheduling, &path),
            scheduling,
            path: path.to_path_buf()
        }
    }
}

pub enum RemoveOperationGeneratorState<'a, E: Entry> {
    Uninitialized,
    ChildrenOperation {
        scheduling: RemoveScheduling,
        children_iterator: Box<dyn Iterator<Item = E> + 'a>,
        opt_operation_generator: Option<Box<RemoveOperationGenerator<'a, E>>>
    },
    SelfOperation(RemoveScheduling),
    Terminated
}

pub struct RemoveOperationGenerator<'a, E: Entry + 'a> {
    pub definition: RemoveBatchDefinition,
    pub state: RemoveOperationGeneratorState<'a, E>
}

impl <'a, E: Entry + 'a>RemoveOperationGenerator<'a, E> {
    pub fn new(definition: RemoveBatchDefinition) -> Self {
        RemoveOperationGenerator {
            state: RemoveOperationGeneratorState::Uninitialized,
            definition
        }
    }
}

impl <'a, E: Entry> OperationGenerator<E> for RemoveOperationGenerator<'a, E> {
    type Item = RemoveOperation;
    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        match &mut self.state {
            RemoveOperationGeneratorState::Uninitialized => {
                let scheduling = RemoveOperation::schedule(fs, &self.definition.path)?;
                match scheduling {
                    RemoveScheduling::RecursiveDirectoryRemoval => {
                        self.state = RemoveOperationGeneratorState::ChildrenOperation{
                            scheduling,
                            children_iterator: Box::new(fs.read_dir(&self.definition.path)?.into_iter()),
                            opt_operation_generator: None
                        };
                    }
                    _ => { self.state = RemoveOperationGeneratorState::SelfOperation(scheduling); }
                }
                self.next(fs)
            },
            RemoveOperationGeneratorState::ChildrenOperation{ scheduling, children_iterator, opt_operation_generator } => {
                if let Some(operation_generator) = opt_operation_generator {
                    if let Some(operation) = operation_generator.next(fs)? {
                        return Ok(Some(operation));
                    }
                }
                if let Some(entry) = children_iterator.next() {
                    *opt_operation_generator = Some(Box::new(
                        RemoveOperationGenerator::new(
                            RemoveBatchDefinition::new(entry.to_path())
                        )
                    ));
                } else {
                    self.state = RemoveOperationGeneratorState::SelfOperation(*scheduling);
                }
                self.next(fs)
            },
            RemoveOperationGeneratorState::SelfOperation(scheduling) => {
                let _scheduling = scheduling.clone();
                self.state = RemoveOperationGeneratorState::Terminated;
                Ok(Some(RemoveOperation::new(_scheduling, &self.definition.path)))
            },
            RemoveOperationGeneratorState::Terminated => Ok(None)
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
        }
    };

    #[test]
    fn operation_remove_operation_file() {
        let chroot = Samples::init_simple_chroot("operation_remove_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = RemoveOperationGenerator::new(RemoveBatchDefinition::new(
            chroot.join("RDIR/RFILEA")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    fn operation_remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("operation_remove_operation_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = RemoveOperationGenerator::new(RemoveBatchDefinition::new(
            chroot.join("RDIR3")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR3").exists());
    }

    #[test]
    fn operation_remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("operation_remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = RemoveOperationGenerator::new(RemoveBatchDefinition::new(
            chroot.join("RDIR")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR").exists());
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
        }
    };

    #[test]
    fn operation_virtual_remove_operation_file() {
        let chroot = Samples::init_simple_chroot("operation_virtual_remove_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = RemoveOperationGenerator::new(RemoveBatchDefinition::new(
            chroot.join("RDIR/RFILEA")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR/RFILEA").as_path()).unwrap());
    }

    #[test]
    fn operation_virtual_remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("operation_virtual_remove_operation_directory");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = RemoveOperationGenerator::new(RemoveBatchDefinition::new(
            chroot.join("RDIR3")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR3").as_path()).unwrap());
    }

    #[test]
    fn operation_virtual_remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("operation_virtual_remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = RemoveOperationGenerator::new(RemoveBatchDefinition::new(
            chroot.join("RDIR")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR").as_path()).unwrap());
    }
}
