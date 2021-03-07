/*
    TLDR;
    The "read operations" must apply as late as possible.
    Les lectures doivent s'effectuer le plus tard possible
    réécrire les fonctions "atomize" de manière plus fonctionnelle et peut-être faire des fonctions de haut ordre pour les différentes étapes temporelles
    stream duplex ?

    # OK / Prooved
    Faire un iterateur recursif ( le chainer optionnellement )!!
    Les écritures doivent s'effectuer le plus tôt possible
    Les opérations doivent pouvoir être intérrompues ou reprises de manière asynchrone
    Les opérations doivent se mettre en pause toutes seules en attente d'un choix de la part de l'utilisateur de manière asynchrone
    //1 OperationDefiniton -> 0..n User Choices -> Transaction<1..n Atomics>

    //1 L'utilisateur demande une operation
    //2 Le fs applique les transactions jusqu'a l'apparition d'un choix ou que l'operation soit terminée
    //3 L'utilisateur résout un choix
    //4 Le fs applique les transactions jusqu'a l'apparition d'un choix ou que l'operation soit terminée

*/

use std::{
    iter,
    path:: { Path, PathBuf }
};

use crate::{
    errors::{
        DomainError
    },
    infrastructure::errors::{ InfrastructureError },
    sample::Samples,
    event::{
        RemoveOperationDefinition,
        capability::{
            Capability
        }
    },
    infrastructure::{
        VirtualFileSystem,

    },
    port::{
        Entry,
        WriteableFileSystem,
        ReadableFileSystem,
        FileSystemAdapter,
        Atomic,
        AtomicTransaction
    }
};

#[test]
fn poc(){
    let samples_path = Samples::static_samples_path();
    let mut fs = FileSystemAdapter(VirtualFileSystem::default());

    let mut generator = OperationGeneratorAdapter::new(CopyBatchDefinition::new(samples_path.join("B"), samples_path.join("A")));

    while let Some(operation) = generator.next(&fs).unwrap() {
        operation.apply(&mut fs).unwrap();
    }

    assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
    assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
}


// Interface
pub trait Operation {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> ;
}


pub trait OperationGenerator<E: Entry> {
    type Item: Operation;
    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError>;
}

// Adapter
pub struct OperationGeneratorAdapter<'a, E: Entry + 'a, T> {
    inner: T,
    sent_itself: bool,
    children_iterator: Box<dyn Iterator<Item = E> + 'a>,
    operation_generator: Option<Box<OperationGeneratorAdapter<'a, E, T>>>
}

impl <'a, E: Entry + 'a, T>OperationGeneratorAdapter<'a, E, T> {
    pub fn new(inner: T) -> Self {
        OperationGeneratorAdapter {
            inner,
            sent_itself: false,
            children_iterator: Box::new(iter::empty()),
            operation_generator: None,
        }
    }
}

// Concrete
#[derive(Copy, Clone, Debug)]
pub enum CopyScheduling {
    DirectoryMerge,
    FileOverwrite,
    FileCopy,
    DirectoryCopy
}

pub struct CopyOperation {
    pub scheduling: CopyScheduling,
    pub transaction: Vec<Atomic>,
    pub source: PathBuf,
    pub destination: PathBuf
}

impl CopyOperation {
    fn schedule<E: Entry>(source: &E, destination: &E) -> Result<CopyScheduling, DomainError> {
        if destination.exists() {
            if source.is_dir() {
                if destination.is_dir() {
                    Ok(CopyScheduling::DirectoryMerge)
                } else {
                    Err(DomainError::MergeFileWithDirectory(source.to_path(), destination.to_path()))
                }
            } else if source.is_file() {
                if destination.is_file() {
                    Ok(CopyScheduling::FileOverwrite)
                } else {
                    Err(DomainError::OverwriteDirectoryWithFile(source.to_path(), destination.to_path()))
                }
            } else {
                Err(DomainError::Custom(String::from("Unknown node source type")))
            }
        } else if source.is_dir() {
            Ok(CopyScheduling::DirectoryCopy)
        } else if source.is_file() {
            Ok(CopyScheduling::FileCopy)
        } else {
            Err(DomainError::Custom(String::from("Unknown node source type")))
        }
    }

    fn transaction<E: Entry>(scheduling: &CopyScheduling, source: &E, destination: &E) -> Vec<Atomic> {
        match scheduling {
            CopyScheduling::FileCopy => vec![
                Atomic::CopyFileToFile{
                    source: source.to_path(),
                    destination: destination.to_path()
                }
            ],
            CopyScheduling::FileOverwrite => vec![
                Atomic::RemoveFile(destination.to_path()),
                Atomic::CopyFileToFile {
                    source: source.to_path(),
                    destination: destination.to_path()
                }
            ],
            CopyScheduling::DirectoryCopy => vec![
                Atomic::BindDirectoryToDirectory {
                    source: source.to_path(),
                    destination: destination.to_path()
                }
            ],
            _ => Vec::new()
        }
    }

    pub fn new<F: ReadableFileSystem>(fs: &F, source_path: &Path, destination_path: &Path) -> Result<Self, DomainError> {
        let source = fs.status(source_path)?;
        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(source_path.to_path_buf()))
        }

        let destination = fs.status(destination_path)?;
        if source.is_dir() && destination.is_contained_by(&source) {
            return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
        }

        let scheduling = Self::schedule(&source, &destination)?;

        Ok(
            CopyOperation {
                transaction: Self::transaction(&scheduling, &source, &destination),
                scheduling,
                source: source.to_path(),
                destination: destination.to_path()
            }
        )
    }

    pub fn children<'a, E: Entry + 'a, F: ReadableFileSystem<Item=E>>(fs: &F, operation: &Self) -> Result<Option<Box<dyn Iterator<Item = E> + 'a>>, DomainError>{
        match operation.scheduling {
            CopyScheduling::DirectoryMerge => {
                Ok(Some(Box::new(fs.read_dir(&operation.source)?.into_iter())))
            },
            CopyScheduling::DirectoryCopy => {
                Ok(Some(Box::new(fs.read_maintained(&operation.source)?.into_iter())))
            },
            _ => Ok(None)
        }
    }
}

impl Operation for CopyOperation {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.transaction.iter() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}

pub struct CopyBatchDefinition {
    source: PathBuf,
    destination: PathBuf
}

impl CopyBatchDefinition {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        CopyBatchDefinition {
            source,
            destination
        }
    }
}

impl <'a, E: Entry> OperationGenerator<E> for OperationGeneratorAdapter<'a, E, CopyBatchDefinition> {
    type Item = CopyOperation;
    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        if let Some(operation_generator) = &mut self.operation_generator {
            if let Some(operation) = operation_generator.next(fs)? {
                return Ok(Some(operation))
            }
        }
        if !self.sent_itself {
            let operation = CopyOperation::new(fs, &self.inner.source, &self.inner.destination)?;
            self.sent_itself = true;
            match CopyOperation::children(fs, &operation)? {
                Some(children_iterator) => { self.children_iterator = children_iterator },
                None => {}
            }
            Ok(Some(operation))
        } else if let Some(entry) = self.children_iterator.next() {
            self.operation_generator = Some(Box::new(
                OperationGeneratorAdapter::new(
                    CopyBatchDefinition::new(
                        entry.to_path(),
                        self.inner.destination.join(entry.name().unwrap()).to_path_buf()
                    )
                )
            ));
            self.next(fs)
        } else {
            Ok(None)
        }
    }
}