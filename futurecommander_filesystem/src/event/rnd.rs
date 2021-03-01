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

}

#[derive(Copy, Clone)]
pub enum Scheduling {
    DirectoryMerge,
    FileOverwrite,
    FileCopy,
    DirectoryCopy
}

pub struct Choice {
    pub scheduling: Scheduling,
    pub transaction: AtomicTransaction,
    pub source: PathBuf,
    pub destination: PathBuf
}

impl Choice {
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

        Ok(Choice {
                transaction: Self::transaction(&scheduling, &source, &destination),
                scheduling,
                source: source.to_path(),
                destination: destination.to_path()
        })
    }
}

pub struct ChoiceIter<'a, E: Entry, F: ReadableFileSystem<Item=E>> {
    fs: &'a F,
    source: PathBuf,
    destination: PathBuf,
    send_itself: bool,

    children_iter: Box<dyn Iterator<Item = E> + 'a>,
    choice_iter: Box<dyn Iterator<Item = Result<Choice, DomainError>> + 'a>
}

impl <'a, E: Entry + 'a, F: ReadableFileSystem<Item=E>> Iterator for ChoiceIter<'a, E, F> {
    type Item = Result<Choice, DomainError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.send_itself {
            match Choice::new(self.fs, &self.source, &self.destination) {
                Ok(choice) => {
                    self.send_itself = false;
                    match choice.scheduling {
                        Scheduling::DirectoryMerge => {
                            match self.fs.read_dir(&self.source) {
                                Ok(collection) => {
                                    self.children_iter = Box::new(collection.into_iter());
                                },
                                Err(err) => return Some(Err(err.into()))
                            }
                        },
                        Scheduling::DirectoryCopy => {
                            match self.fs.read_maintained(&self.source) {
                                Ok(collection) => {
                                    self.children_iter = Box::new(collection.into_iter());
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
            if let Some(choice) = self.choice_iter.next() {
                Some(choice)
            } else {
                if let Some(entry) = self.children_iter.next() {
                    self.choice_iter = Box::new(
                        ChoiceIter {
                            fs: self.fs,
                            source: entry.to_path(),
                            send_itself: false,
                            destination: self.destination.join(entry.name().unwrap()).to_path_buf(),
                            children_iter: Box::new(iter::empty()),
                            choice_iter: Box::new(iter::empty())
                        }
                    );
                    self.next()
                } else {
                    None
                }
            }
        }
    }
}