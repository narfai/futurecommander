// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    path:: { Path, PathBuf }
};


use serde::{ Serialize, Deserialize };

use crate::{
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveOperationDefinition {
    path: PathBuf,
    recursive: bool
}

impl RemoveOperationDefinition {
    pub fn new(path: &Path, recursive: bool) -> RemoveOperationDefinition {
        RemoveOperationDefinition {
            path: path.to_path_buf(),
            recursive
        }
    }

    pub fn path(&self) -> &Path { self.path.as_path() }
    pub fn recursive(&self) -> bool { self.recursive }
}

pub fn atomize<E: Entry, F: ReadableFileSystem<Item=E>>(definition: RemoveOperationDefinition, fs: &F, guard: &mut dyn Guard) -> Result<AtomicTransaction, DomainError> {
    let entry = fs.status(definition.path())?;
    let mut transaction = AtomicTransaction::default();

    if !entry.exists() {
        return Err(DomainError::DoesNotExists(definition.path().to_path_buf()))
    }

    if entry.is_file() {
        transaction.add(Atomic::RemoveFile(entry.path().to_path_buf()));
    } else if entry.is_dir() {
        let children = fs.read_dir(entry.path())?;

        if children.is_empty() {
            transaction.add(Atomic::RemoveEmptyDirectory(entry.path().to_path_buf()))
        } else if guard.authorize(Capability::Recursive, definition.recursive(), definition.path())? {
            for child in children {
                transaction.merge(
                    atomize(
                        RemoveOperationDefinition::new(
                            child.path(),
                            true
                        ),
                        fs,
                        guard
                    )?
                )
            }
            transaction.add(Atomic::RemoveEmptyDirectory(entry.path().to_path_buf()))
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
    fn remove_operation_file() {
        let chroot = Samples::init_simple_chroot("remove_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            RemoveOperationDefinition::new(
                chroot.join("RDIR/RFILEA").as_path(),
                false
            ),
            &fs,
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    fn remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("remove_operation_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            RemoveOperationDefinition::new(
                chroot.join("RDIR3").as_path(),
                false
            ),
            &fs,
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(!chroot.join("RDIR3").exists());
    }

    #[test]
    fn remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            RemoveOperationDefinition::new(
                chroot.join("RDIR").as_path(),
                true
            ),
            &fs,
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

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
        },
        capability::{
            ZealousGuard
        }
    };

    #[test]
    fn virtual_remove_operation_file() {
        let chroot = Samples::init_simple_chroot("virtual_remove_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            RemoveOperationDefinition::new(
                chroot.join("RDIR/RFILEA").as_path(),
                false
            ),
            &fs,
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR/RFILEA").as_path()).unwrap());
    }

    #[test]
    fn virtual_remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("virtual_remove_operation_directory");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            RemoveOperationDefinition::new(
                chroot.join("RDIR3").as_path(),
                false
            ),
            &fs,
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR3").as_path()).unwrap());
    }

    #[test]
    fn virtual_remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("virtual_remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            RemoveOperationDefinition::new(
                chroot.join("RDIR").as_path(),
                true
            ),
            &fs,
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR").as_path()).unwrap());
    }
}
