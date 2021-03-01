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
fn rnd() {
    let chroot = Samples::init_simple_chroot("virtual_remove_operation_directory_recursive");
    let mut fs = FileSystemAdapter(VirtualFileSystem::default());

    let def = RemoveOperationDefinition::new(
        chroot.join("RDIR").as_path(),
        true
    );
    

    /* 
    atomize(
        def,
        &fs, 
        &mut ZealousGuard
    ).unwrap()
     .apply(&mut fs)
     .unwrap();
    */
    assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR").as_path()).unwrap());
}

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
    scheduling: Scheduling, 
    transaction: AtomicTransaction,
    source: PathBuf,
    destination: PathBuf
}

pub struct Operation {
    choice: Choice,
    children: Vec<Operation>
}

impl Operation {
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

    pub fn new<E: Entry, F: ReadableFileSystem<Item=E>>(definition: CopyOperationDefinition, fs: &F) -> Result<Self, DomainError> {
        let source = fs.status(definition.source())?;
        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(definition.source().to_path_buf()))
        }

        let destination = fs.status(definition.destination())?;
        if source.is_dir() && destination.is_contained_by(&source) {
            return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
        }

        let scheduling = Self::schedule(&source, &destination)?;

        Ok(Operation{ 
            choice: Choice{ 
                scheduling, 
                transaction: Self::transaction(&scheduling, &source, &destination), 
                source: source.to_path(), 
                destination: destination.to_path() 
            },
            children: match scheduling {
                Scheduling::DirectoryMerge => {
                    let mut children = Vec::new();
                    for child in fs.read_dir(source.path())? {
                        children.push(
                            Operation::new(
                                CopyOperationDefinition::new(
                                    child.path(),                     
                                    destination.path().join(child.name().unwrap()).as_path(), 
                                    definition.merge(), 
                                    definition.overwrite()
                                ), 
                                fs
                            )?
                        )
                    }
                    children  
                },
                Scheduling::DirectoryCopy => {
                    let mut children = Vec::new();
                    for child in fs.read_maintained(source.path())? {
                        children.push(
                            Operation::new(
                                CopyOperationDefinition::new(
                                    child.path(),
                                    destination.path()
                                        .join(child.name().unwrap())
                                        .as_path(),
                                        definition.merge(),
                                        definition.overwrite()                            
                                ), 
                                fs
                            )?
                        )
                    }
                    children
                },
                _ => Vec::new()
            }            
        })
    }

    pub fn choices<'a>(&'a self) -> Box<dyn Iterator<Item = &Choice> + 'a> {
        Box::new(
            iter::once(&self.choice)
                .chain(
                    self.children
                        .iter()
                        .map(|operation| operation.choices())
                        .flatten()
                )
        )
    }    
}
