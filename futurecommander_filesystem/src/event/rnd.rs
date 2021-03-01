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
        DomainError,
    },
    sample::Samples,
    event::{
        CopyOperationDefinition,
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

    let mut it = CopyOperationGenerator::new(samples_path.join("B"), samples_path.join("A"));

    while let Some(Ok(choice)) = it.next(&fs) {
        println!("Iterator : {:?}", choice);
        choice.transaction.apply(&mut fs);
    }

    assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
    assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
}

#[derive(Copy, Clone, Debug)]
pub enum Scheduling {
    DirectoryMerge,
    FileOverwrite,
    FileCopy,
    DirectoryCopy
}

#[derive(Debug)]
pub struct CopyOperation {
    pub scheduling: Scheduling,
    pub transaction: AtomicTransaction,
    pub source: PathBuf,
    pub destination: PathBuf
}

impl CopyOperation {
    pub fn schedule<E: Entry>(source: &E, destination: &E) -> Result<Scheduling, DomainError> {
        if destination.exists() {
            if source.is_dir() {
                if destination.is_dir() {
                    Ok(Scheduling::DirectoryMerge)
                } else {
                    Err(DomainError::MergeFileWithDirectory(source.to_path(), destination.to_path()))
                }
            } else if source.is_file() {
                if destination.is_file() {
                    Ok(Scheduling::FileOverwrite)
                } else {
                    Err(DomainError::OverwriteDirectoryWithFile(source.to_path(), destination.to_path()))
                }
            } else {
                Err(DomainError::Custom(String::from("Unknown node source type")))
            }
        } else if source.is_dir() {
            Ok(Scheduling::DirectoryCopy)
        } else if source.is_file() {
            Ok(Scheduling::FileCopy)
        } else {
            Err(DomainError::Custom(String::from("Unknown node source type")))
        }
    }

    pub fn transaction<E: Entry>(scheduling: &Scheduling, source: &E, destination: &E) -> AtomicTransaction {
        match scheduling {
            Scheduling::FileCopy =>
                AtomicTransaction::new(vec![
                    Atomic::CopyFileToFile{
                        source: source.to_path(),
                        destination: destination.to_path()
                    }
                ])
            ,
            Scheduling::FileOverwrite =>
                AtomicTransaction::new(vec![
                    Atomic::RemoveFile(destination.to_path()),
                    Atomic::CopyFileToFile {
                        source: source.to_path(),
                        destination: destination.to_path()
                    }
                ])
            ,
            Scheduling::DirectoryCopy =>
                AtomicTransaction::new(vec![Atomic::BindDirectoryToDirectory {
                    source: source.to_path(),
                    destination: destination.to_path()
                }])
            ,
            _ => AtomicTransaction::default()
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

        Ok(CopyOperation {
                transaction: Self::transaction(&scheduling, &source, &destination),
                scheduling,
                source: source.to_path(),
                destination: destination.to_path()
        })
    }
}

pub struct CopyOperationGenerator<'a, E: Entry + 'a> {
    source: PathBuf,
    destination: PathBuf,

    send_itself: bool,
    children_iterator: Box<dyn Iterator<Item = E> + 'a>,
    choice_generator: Option<Box<CopyOperationGenerator<'a, E>>>
}

impl <'a, E: Entry + 'a >CopyOperationGenerator<'a, E> {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        CopyOperationGenerator {
            source,
            destination,
            send_itself: false,
            children_iterator: Box::new(iter::empty()),
            choice_generator: None,
        }
    }

    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Option<Result<CopyOperation, DomainError>> {
        if !self.send_itself {
            match CopyOperation::new(fs, &self.source, &self.destination) {
                Ok(choice) => {
                    self.send_itself = true;
                    match choice.scheduling {
                        Scheduling::DirectoryMerge => {
                            match fs.read_dir(&self.source) {
                                Ok(collection) => {
                                    self.children_iterator = Box::new(collection.into_iter());
                                },
                                Err(err) => return Some(Err(err.into()))
                            }
                        },
                        Scheduling::DirectoryCopy => {
                            match fs.read_maintained(&self.source) {
                                Ok(collection) => {
                                    self.children_iterator = Box::new(collection.into_iter());
                                },
                                Err(err) => return Some(Err(err.into()))
                            }
                        },
                        _ => {}
                    }
                    Some(Ok(choice))
                },
                Err(err) => Some(Err(err))
            }
        } else {
            if let Some(choice_iter) = &mut self.choice_generator {
                if let Some(choice) = choice_iter.next(fs) {
                    return Some(choice)
                }
            }
            if let Some(entry) = self.children_iterator.next() {
                self.choice_generator = Some(Box::new(
                    CopyOperationGenerator::new(
                        entry.to_path(),
                        self.destination.join(entry.name().unwrap()).to_path_buf()
                    )
                ));
                self.next(fs)
            } else {
                None
            }

        }
    }
}