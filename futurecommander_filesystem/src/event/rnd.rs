use std::{
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

/* struct Choice {
    capability: Capability,
    default: bool,
    path: PathBuf
}

enum Operation {
    Paused,
    Processing
} */

/* fn atomize<E: Entry, F: ReadableFileSystem<Item=E>>(fs: &F) -> Operation {
    unimplemented!()
} */

#[test]
fn rnd() {
    let chroot = Samples::init_simple_chroot("virtual_remove_operation_directory_recursive");
    let mut fs = FileSystemAdapter(VirtualFileSystem::default());

    let def = RemoveOperationDefinition::new(
        chroot.join("RDIR").as_path(),
        true
    );

    /* let operation = Operation::Initialized;

    loop {
        match atomize(&fs) {
            Operation::Processed(transac) => transac.apply(&mut fs).unwrap(),
            Operation::WaitingForChoice(choice) => break
        }
    } */

    //1 OperationDefiniton -> 0..n User Choices -> Transaction<1..n Atomics>
    
    //1 L'utilisateur demande une operation
    //2 Le fs applique les transactions jusqu'a l'apparition d'un choix ou que l'operation soit terminée
    //3 L'utilisateur résout un choix
    //4 Le fs applique les transactions jusqu'a l'apparition d'un choix ou que l'operation soit terminée

    /*
        Les lectures doivent s'effectuer le plus tard possible
        Les écritures doivent s'effectuer le plus tôt possible
        Les opérations doivent pouvoir être intérrompues ou reprises de manière asynchrone
        Les opérations doivent se mettre en pause toutes seules en attente d'un choix de la part de l'utilisateur de manière asynchrone
        Faire un iterateur recursif ( le chainer optionnellement )!! 
        il devrait renvoyer des tuples (transaction, requête de choix) ? => comment renvoyer la réponse du choix pour relancer l'itérateur ?
        réécrire les fonctions "atomize" de manière plus fonctionnelle et peut-être faire des fonctions de haut ordre pour les différentes étapes temporelles
        stream duplex ? 
    */

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
/* 
struct OperationStateIterator<E: Entry> {
    source: Entry,
    destination: Entry,
} */
 
//Should be Eq / Hashable et timestampé
/* struct Choice {
    capability: Capability,
    default: bool,
    target: PathBuf
}

enum OperationState {
    Progress(AtomicTransaction),
    WaitingForChoice(Choice)
} */
//TODO Make atomize functions a recursive Iterator next() -> Option<OperationState>
//Iterator would contains a reference to choices and will next() -> OperationState::WaitingForChoice

#[test]
fn poc(){

}


pub struct Operation<E: Entry> {
    source: E,
    destination: E,
    transaction: Vec<AtomicTransaction>,
    children: Vec<Operation<E>>,
    scheduling: Scheduling
}

pub enum Scheduling {
    DirectoryMerge,
    FileOverwrite,
    FileCopy,
    DirectoryCopy
}

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

impl <E: Entry> Operation<E> {
    pub fn new<F: ReadableFileSystem<Item=E>>(definition: CopyOperationDefinition, fs: &F) -> Result<Self, DomainError> {
        let source = fs.status(definition.source())?;

        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(definition.source().to_path_buf()))
        }

        let destination = fs.status(definition.destination())?;

        if source.is_dir() && destination.is_contained_by(&source) {
            return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
        }

        let mut transaction = AtomicTransaction::default();
        let mut children: Vec<Operation<E>> = Vec::new();
        let scheduling = schedule(&source, &destination)?;

        match scheduling {
            Scheduling::FileCopy => {
                transaction.add(
                    Atomic::CopyFileToFile{
                        source: source.to_path(),
                        destination: destination.to_path()
                    }
                );
            },
            Scheduling::FileOverwrite => {
                transaction.add(Atomic::RemoveFile(destination.to_path()));
                transaction.add(Atomic::CopyFileToFile {
                    source: source.to_path(),
                    destination: destination.to_path()
                });
            },
            Scheduling::DirectoryMerge => {
                for child in fs.read_dir(source.path())? {
                    children.push(Operation::new(
                        CopyOperationDefinition::new(
                            child.path(),                     
                            destination.path().join(child.name().unwrap()).as_path(), 
                            definition.merge(), 
                            definition.overwrite()
                        ), 
                        fs
                    )?)
                }
            },
            Scheduling::DirectoryCopy => {
                transaction.add(Atomic::BindDirectoryToDirectory {
                    source: source.to_path(),
                    destination: destination.to_path()
                });
                for child in fs.read_maintained(source.path())? {
                    children.push(Operation::new(
                        CopyOperationDefinition::new(
                            child.path(),
                            destination.path()
                                .join(child.name().unwrap())
                                .as_path(),
                                definition.merge(),
                                definition.overwrite()                            
                        ), 
                        fs
                    )?)
                }
            }
        }

        Ok(Operation{ scheduling, source, destination, transaction: vec![transaction], children })
    }

    pub fn transactions<'a>(&'a self) -> Box<Iterator<Item = &AtomicTransaction> + 'a> {
        Box::new(
            self.transaction
                .iter()
                .chain(
                    self.children
                        .iter()
                        .map(|operation| operation.transactions())
                        .flatten()
                )
        )
    }


// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
    
}

/* 
pub fn apply<E: Entry, F: ReadableFileSystem<Item=E>>(definition: CopyOperationDefinition, fs: &mut F) -> Box<dyn Iterator<Item=Choice<E, F>>> {
    let source = fs.status(definition.source()).unwrap();

    /* if !source.exists() {
        return Err(DomainError::SourceDoesNotExists(definition.source().to_path_buf()))
    } */

    let destination = fs.status(definition.destination()).unwrap();

    /* if source.is_dir() && destination.is_contained_by(&source) {
        return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
    } */

    
    Box::new(match schedule(source, destination) {
        DirectoryMerge => fs.read_dir(source.path()).unwrap()
            .iter()
            .map(
                |child| apply(CopyOperationDefinition::new(
                    child.path(),                     
                    destination.path().join(child.name().unwrap()).as_path(), 
                    definition.merge(), 
                    definition.overwrite()
                ), fs)
            ),     
        DirectoryCopy => unimplemented!(),
        FileOverwrite => vec!(
            Choice { 
                callback: Box::new(|inner_fs| { 
                    inner_fs.remove_file(destination)?;
                    inner_fs.copy_file_to_file(source, destination)?;
                    Ok(())
                })
            }
        ).iter(),
        FileCopy => vec!(
            Choice {
                callback: Box::new(|inner_fs| { 
                    inner_fs.copy_file_to_file(source, destination)?;
                    Ok(())
                }) 
            }
        ).iter(),
    }
    )
} */