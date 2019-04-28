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
    event::{
        Event
    },
    port::{
        Entry,
        ReadableFileSystem,
        Atomic,
        AtomicTransaction
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEvent {
    path: PathBuf,
    kind: Kind,
    recursive: bool,
    overwrite: bool
}

impl CreateEvent {
    pub fn new(path: &Path, kind: Kind, recursive: bool, overwrite: bool) -> CreateEvent {
        CreateEvent {
            path: path.to_path_buf(),
            kind,
            recursive,
            overwrite
        }
    }

    pub fn path(&self) -> &Path { self.path.as_path() }
    pub fn kind(&self) -> Kind { self.kind }
    pub fn recursive(&self) -> bool { self.recursive }
    pub fn overwrite(&self) -> bool { self.overwrite }
}


impl <E, F> Event <E, F> for CreateEvent
    where F: ReadableFileSystem<Item=E>,
          E: Entry {

    fn atomize(&self, fs: &F) -> Result<AtomicTransaction, DomainError> {
        let entry = fs.status(self.path())?;
        let mut transaction = AtomicTransaction::default();

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

        let mut ancestors = self.path().ancestors();
        ancestors.next();

        match self.kind() {
            Kind::Directory => {
                if entry.exists() {
                    return Err(DomainError::DirectoryOverwriteNotAllowed(entry.to_path()))
                } else {
                    if self.recursive() {
                        transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
                    }
                    transaction.add(Atomic::CreateEmptyDirectory(entry.to_path()));
                }
            },
            Kind::File => {
                if entry.exists() {
                    if self.overwrite() {
                        if self.recursive() {
                            transaction.merge(recursive_dir_creation(fs, &mut ancestors)?);
                        }
                        transaction.add(Atomic::RemoveFile(entry.to_path()));
                        transaction.add(Atomic::CreateEmptyFile(entry.to_path()));
                    } else {
                        return Err(DomainError::OverwriteNotAllowed(entry.to_path()))
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
}

//Infrastructure -> Domain integration tests

#[cfg_attr(tarpaulin, skip)]
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
    fn create_operation_directory(){
        let chroot = Samples::init_simple_chroot("create_operation_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        CreateEvent::new(
            chroot.join("CREATED").as_path(),
            Kind::Directory,
            false,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(chroot.join("CREATED").is_dir());
        assert!(chroot.join("CREATED").exists());
    }


    #[test]
    fn create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        CreateEvent::new(
            chroot.join("CREATED/NESTED/DIRECTORY").as_path(),
            Kind::Directory,
            true,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(chroot.join("CREATED/NESTED/DIRECTORY").exists());
        assert!(chroot.join("CREATED/NESTED/DIRECTORY").is_dir());
    }

    #[test]
    fn create_operation_file(){
        let chroot = Samples::init_simple_chroot("create_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        CreateEvent::new(
            chroot.join("CREATED").as_path(),
            Kind::File,
            false,
            false
        ).atomize(&fs)
            .unwrap()
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

        CreateEvent::new(
            chroot.join("RDIR/RFILEA").as_path(),
            Kind::File,
            false,
            true
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert_ne!(a_len, chroot.join("RDIR/RFILEA").metadata().unwrap().len());
    }
}


#[cfg_attr(tarpaulin, skip)]
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
    fn virtual_create_operation_directory(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        CreateEvent::new(
            chroot.join("CREATED").as_path(),
            Kind::Directory,
            false,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED").as_path()).unwrap());
    }


    #[test]
    fn virtual_create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let opcodes = CreateEvent::new(
            chroot.join("CREATED/NESTED/DIRECTORY").as_path(),
            Kind::Directory,
            true,
            false
        ).atomize(&fs)
            .unwrap();

        opcodes.apply(&mut fs)
            .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        CreateEvent::new(
            chroot.join("CREATED").as_path(),
            Kind::File,
            false,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(chroot.join("CREATED").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file_overwrite");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let opcodes = CreateEvent::new(
            chroot.join("RDIR/RFILEA").as_path(),
            Kind::File,
            false,
            true
        ).atomize(&fs)
            .unwrap();

        opcodes.apply(&mut fs)
            .unwrap();

        let entry = fs.status(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(entry.exists());
        assert!(entry.is_file());
    }
}

