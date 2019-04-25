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
    path:: { Path, PathBuf }
};

use crate::{
    Kind,
    errors::{ DomainError },
    port::{
        Entry,
        ReadableFileSystem,
        Event,
        Atomic,
        AtomicTransaction
    }
};

#[derive(Debug, Clone)]
pub struct RemoveEvent {
    path: PathBuf,
    recursive: bool
}

impl RemoveEvent {
    pub fn new(path: &Path, recursive: bool) -> RemoveEvent {
        RemoveEvent {
            path: path.to_path_buf(),
            recursive
        }
    }

    pub fn path(&self) -> &Path { self.path.as_path() }
    pub fn recursive(&self) -> bool { self.recursive }
}

impl <E, F> Event <E, F> for RemoveEvent where F: ReadableFileSystem<Item=E>, E: Entry {
    fn atomize(&self, fs: &F) -> Result<AtomicTransaction, DomainError> {
        let entry = fs.status(self.path())?;
        let mut transaction = AtomicTransaction::default();

        if !entry.exists() {
            return Err(DomainError::DoesNotExists(self.path().to_path_buf()))
        }

        if entry.is_file() {
            transaction.add(Atomic::RemoveFile(entry.path().to_path_buf()));
        } else if entry.is_dir() {
            let children = fs.read_dir(entry.path())?;

            if children.is_empty() {
                transaction.add(Atomic::RemoveEmptyDirectory(entry.path().to_path_buf()))
            } else if self.recursive(){
                for child in children {
                    transaction.merge(
                        RemoveEvent::new(
                            child.path(),
                            true
                        ).atomize(fs)?
                    )
                }
                transaction.add(Atomic::RemoveEmptyDirectory(entry.path().to_path_buf()))
            } else {
                return Err(DomainError::DeleteRecursiveNotAllowed(entry.path().to_path_buf()))
            }
        }

        Ok(transaction)
    }
}


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
    fn remove_operation_file() {
        let chroot = Samples::init_simple_chroot("remove_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        RemoveEvent::new(
            chroot.join("RDIR/RFILEA").as_path(),
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    fn remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("remove_operation_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        RemoveEvent::new(
            chroot.join("RDIR3").as_path(),
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!chroot.join("RDIR3").exists());
    }

    #[test]
    fn remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        RemoveEvent::new(
            chroot.join("RDIR").as_path(),
            true
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!chroot.join("RDIR").exists());
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
    fn virtual_remove_operation_file() {
        let chroot = Samples::init_simple_chroot("virtual_remove_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        RemoveEvent::new(
            chroot.join("RDIR/RFILEA").as_path(),
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR/RFILEA").as_path()).unwrap());
    }

    #[test]
    fn virtual_remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("virtual_remove_operation_directory");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        RemoveEvent::new(
            chroot.join("RDIR3").as_path(),
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR3").as_path()).unwrap());
    }

    #[test]
    fn virtual_remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("virtual_remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        RemoveEvent::new(
            chroot.join("RDIR").as_path(),
            true
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR").as_path()).unwrap());
    }
}

