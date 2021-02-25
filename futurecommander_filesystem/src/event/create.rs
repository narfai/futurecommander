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
    path:: { Path, PathBuf, Ancestors }
};

use serde::{ Serialize, Deserialize };

use crate::{
    Kind,
    errors::{ DomainError },
    capability::{
        Guard,
        Capability
    },
    port::{
        Entry,
        ReadableFileSystem,
        Atomic,
        AtomicTransaction
    },
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOperationDefinition {
    path: PathBuf,
    kind: SerializableKind,
    recursive: bool,
    overwrite: bool
}

impl CreateOperationDefinition {
    pub fn new(path: &Path, kind: Kind, recursive: bool, overwrite: bool) -> CreateOperationDefinition {
        CreateOperationDefinition {
            path: path.to_path_buf(),
            kind: kind.into(),
            recursive,
            overwrite
        }
    }

    pub fn path(&self) -> &Path { self.path.as_path() }
    pub fn kind(&self) -> Kind { self.kind.into() }
    pub fn recursive(&self) -> bool { self.recursive }
    pub fn overwrite(&self) -> bool { self.overwrite }
}

fn recursive_dir_creation<E, F> (fs: &F, mut ancestors: &mut Ancestors<'_>) -> Result<AtomicTransaction, DomainError>
    where F: ReadableFileSystem<Item=E>,
          E: Entry {

    let mut transaction = AtomicTransaction::default();
    if let Some(path) = ancestors.next() {
        let entry = fs.status(path)?;
        if !entry.exists() {
            transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
            transaction.add(Atomic::CreateEmptyDirectory(entry.to_path()));
        }
    }
    Ok(transaction)
}

pub fn atomize<E: Entry, F: ReadableFileSystem<Item=E>>(definition: CreateOperationDefinition, fs: &F, guard: &mut dyn Guard) -> Result<AtomicTransaction, DomainError> {
    let entry = fs.status(definition.path())?;
    let mut transaction = AtomicTransaction::default();
    let mut ancestors = definition.path().ancestors();
    ancestors.next();

    match definition.kind() {
        Kind::Directory => {
            if entry.exists() {
                return Err(DomainError::DirectoryOverwriteNotAllowed(entry.to_path()))
            } else {
                if definition.recursive() {
                    transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
                }
                transaction.add(Atomic::CreateEmptyDirectory(entry.to_path()));
            }
        },
        Kind::File => {
            if entry.exists() {
                if guard.authorize(Capability::Overwrite, definition.overwrite(), definition.path())? {
                    if definition.recursive() {
                        transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
                    }
                    transaction.add(Atomic::RemoveFile(entry.to_path()));
                    transaction.add(Atomic::CreateEmptyFile(entry.to_path()));
                }
            } else {
                transaction.add(Atomic::CreateEmptyFile(entry.to_path()));
            }
        },
        Kind::Unknown => {
            return Err(DomainError::CreateUnknown(entry.to_path()))
        }
    }
    Ok(transaction)
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

        atomize(
            CreateOperationDefinition::new(
                chroot.join("CREATED").as_path(),
                Kind::Directory,
                false,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(chroot.join("CREATED").is_dir());
        assert!(chroot.join("CREATED").exists());
    }


    #[test]
    fn create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            CreateOperationDefinition::new(
                chroot.join("CREATED/NESTED/DIRECTORY").as_path(),
                Kind::Directory,
                true,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(chroot.join("CREATED/NESTED/DIRECTORY").exists());
        assert!(chroot.join("CREATED/NESTED/DIRECTORY").is_dir());
    }

    #[test]
    fn create_operation_file(){
        let chroot = Samples::init_simple_chroot("create_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            CreateOperationDefinition::new(
                chroot.join("CREATED").as_path(),
                Kind::File,
                false,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(chroot.join("CREATED").exists());
        assert!(chroot.join("CREATED").is_file());
    }

    #[test]
    fn create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("create_operation_file_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        atomize(
            CreateOperationDefinition::new(
                chroot.join("RDIR/RFILEA").as_path(),
                Kind::File,
                false,
                true
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

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

        atomize(
            CreateOperationDefinition::new(
                chroot.join("CREATED").as_path(),
                Kind::Directory,
                false,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED").as_path()).unwrap());
    }


    #[test]
    fn virtual_create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            CreateOperationDefinition::new(
                chroot.join("CREATED/NESTED/DIRECTORY").as_path(),
                Kind::Directory,
                true,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            CreateOperationDefinition::new(
                chroot.join("CREATED").as_path(),
                Kind::File,
                false,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(chroot.join("CREATED").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file_overwrite");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            CreateOperationDefinition::new(
                chroot.join("RDIR/RFILEA").as_path(),
                Kind::File,
                false,
                true
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        let entry = fs.status(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(entry.exists());
        assert!(entry.is_file());
    }
}
