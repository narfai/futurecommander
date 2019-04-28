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

use serde::{ Serialize, Deserialize };

use crate::{
    errors::{ DomainError },
    port::{
        Entry,
        ReadableFileSystem,
        Event,
        SerializableEvent,
        Atomic,
        AtomicTransaction
    }
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveEvent {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool //To honour overwrite or merge error, we should crawl recursively the entire vfs children of dst ...
}

impl MoveEvent {
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> MoveEvent {
        MoveEvent {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            merge,
            overwrite
        }
    }

    pub fn source(&self) -> &Path { self.source.as_path() }
    pub fn destination(&self) -> &Path { self.destination.as_path() }
    pub fn merge(&self) -> bool { self.merge }
    pub fn overwrite(&self) -> bool { self.overwrite }
}

#[typetag::serde]
impl SerializableEvent for MoveEvent {
    fn serializable(&self) -> Box<SerializableEvent> {
        Box::new(self.clone())
    }
}

impl <E, F> Event <E, F> for MoveEvent
    where F: ReadableFileSystem<Item=E>,
          E: Entry {

    fn atomize(&self, fs: &F) -> Result<AtomicTransaction, DomainError> {
        let source = fs.status(self.source())?;

        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(self.source().to_path_buf()))
        }

        let mut transaction = AtomicTransaction::default();
        let destination = fs.status(self.destination())?;

        if source.is_dir() && destination.is_contained_by(&source) {
            return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
        }

        if destination.exists() {
            if source.is_dir() {
                if destination.is_dir() {
                    if self.merge() {
                        for child in fs.read_dir(source.path())? {
                            transaction.merge(
                                MoveEvent::new(
                                    child.path(),
                                    destination.path()
                                        .join(child.name().unwrap())
                                        .as_path(),
                                    self.merge(),
                                    self.overwrite()
                                ).atomize(fs)?
                            );
                        }
                        transaction.add(Atomic::RemoveEmptyDirectory(source.to_path()));
                    } else {
                        return Err(DomainError::MergeNotAllowed(source.to_path(), destination.to_path()))
                    }
                } else {
                    return Err(DomainError::MergeFileWithDirectory(source.to_path(), destination.to_path()));
                }
            } else if source.is_file() {
                if destination.is_file() {
                    if self.overwrite() {
                        transaction.add(Atomic::RemoveFile(destination.to_path()));
                        transaction.add(Atomic::MoveFileToFile(source.to_path(), destination.to_path()));
                    } else {
                        return Err(DomainError::OverwriteNotAllowed(destination.to_path()))
                    }
                } else {
                    return Err(DomainError::OverwriteDirectoryWithFile(source.to_path(), destination.to_path()))
                }
            }
        } else if source.is_dir() {
            transaction.add(Atomic::CreateEmptyDirectory(destination.to_path()));
            for child in fs.read_dir(source.path())? {//TODO find a way to put back read_maintained ...
                transaction.merge(
                    MoveEvent::new(
                        child.path(),
                        destination.path()
                            .join(child.name().unwrap())
                            .as_path(),
                    self.merge(),
                    self.overwrite()
                ).atomize(fs)?
                );
            }
            transaction.add(Atomic::RemoveEmptyDirectory(source.to_path()));
        } else if source.is_file() {
            transaction.add(Atomic::MoveFileToFile(source.to_path(), destination.to_path()));
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
    fn move_operation_dir(){
        let chroot = Samples::init_simple_chroot("move_operation_dir");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        MoveEvent::new(
            chroot.join("RDIR").as_path(),
            chroot.join("MOVED").as_path(),
            false,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(chroot.join("MOVED").exists());
        assert!(chroot.join("MOVED/RFILEA").exists());
        assert!(chroot.join("MOVED/RFILEB").exists());
    }

    #[test]
    fn move_operation_dir_merge_overwrite(){
        let chroot = Samples::init_simple_chroot("move_operation_dir_merge_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let opcodes = MoveEvent::new(
            chroot.join("RDIR").as_path(),
            chroot.join("RDIR2").as_path(),
            true,
            true
        ).atomize(&fs)
            .unwrap();

        opcodes.apply(&mut fs)
            .unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEC").exists());
        assert_eq!(
            a_len,
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }

    #[test]
    fn move_operation_file(){
        let chroot = Samples::init_simple_chroot("move_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        MoveEvent::new(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("MOVED").as_path(),
            false,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("MOVED").exists());
        assert_eq!(
            a_len,
            chroot.join("MOVED").metadata().unwrap().len()
        )
    }

    #[test]
    fn move_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("move_operation_file_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        MoveEvent::new(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("RDIR2/RFILEA").as_path(),
            false,
            true
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert_eq!(
            a_len,
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }
}


#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod virtual_tests {
    use super::*;

    use crate::{
        sample::Samples,
        Kind,
        port::{
            FileSystemAdapter
        },
        infrastructure::{
            VirtualFileSystem
        }
    };

    #[test]
    fn virtual_move_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        MoveEvent::new(
            samples_path.join("A").as_path(),
            samples_path.join("Z").as_path(),
            false,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_directory_merge(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        //Avoid need of override because of .gitkeep file present in both directory
        let gitkeep = samples_path.join("B/.gitkeep");
        fs.as_inner_mut().mut_sub_state().attach(gitkeep.as_path(),Some(gitkeep.as_path()), Kind::File).unwrap();

        MoveEvent::new(
            samples_path.join("B").as_path(),
            samples_path.join("A").as_path(),
            true,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("B").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_file(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        MoveEvent::new(
            samples_path.join("F").as_path(),
            samples_path.join("Z").as_path(),
            false,
            false
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("F").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_file_overwrite(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        MoveEvent::new(
            samples_path.join("F").as_path(),
            samples_path.join("A/C").as_path(),
            false,
            true
        ).atomize(&fs)
            .unwrap()
            .apply(&mut fs)
            .unwrap();

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("F").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/C").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("A/C").as_path()).unwrap());
    }
}

