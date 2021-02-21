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
    path::{ PathBuf, Path },
};

use serde::{ Serialize, Deserialize };


use crate::{
    errors::{DomainError},
    capability::{
        Capability,
        Guard,
        RegistrarGuard
    },
    port::{
        Entry,
        ReadableFileSystem,
        AtomicTransaction,
        Atomic
    }
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyOperationDefinition {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool
}

impl CopyOperationDefinition {
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> CopyOperationDefinition {
        CopyOperationDefinition {
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

pub fn atomize<E: Entry, F: ReadableFileSystem<Item=E>>(definition: CopyOperationDefinition, fs: &F, guard: &mut dyn Guard) -> Result<AtomicTransaction, DomainError> {
    let source = fs.status(definition.source())?;

    if !source.exists() {
        return Err(DomainError::SourceDoesNotExists(definition.source().to_path_buf()))
    }

    let mut transaction = AtomicTransaction::default();
    let destination = fs.status(definition.destination())?;

    if source.is_dir() && destination.is_contained_by(&source) {
        return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
    }

    if destination.exists() {
        if source.is_dir() {
            if destination.is_dir() {
                if guard.authorize(Capability::Merge, definition.merge(), definition.destination())? {
                    for child in fs.read_dir(source.path())? {
                        transaction.merge(
                            atomize(
                                CopyOperationDefinition::new(
                                    child.path(),
                                    destination.path()
                                        .join(child.name().unwrap())
                                        .as_path(),
                                        definition.merge(),
                                        definition.overwrite()
                                ),
                                fs, 
                                guard
                            )?
                        );
                    }
                }
            } else {
                return Err(DomainError::MergeFileWithDirectory(source.to_path(), destination.to_path()))
            }
        } else if source.is_file() {
            if destination.is_file() {
                if guard.authorize(Capability::Overwrite, definition.overwrite(), definition.destination())? {
                    transaction.add(Atomic::RemoveFile(destination.to_path()));
                    transaction.add(Atomic::CopyFileToFile {
                        source: source.to_path(),
                        destination: destination.to_path()
                    });
                }
            } else {
                return Err(DomainError::OverwriteDirectoryWithFile(source.to_path(), destination.to_path()))
            }
        }
    } else if source.is_dir() {
        transaction.add(Atomic::BindDirectoryToDirectory {
            source: source.to_path(),
            destination: destination.to_path()
        });
        for child in fs.read_maintained(source.path())? {
            transaction.merge(
                atomize(
                    CopyOperationDefinition::new(
                        child.path(),
                        destination.path()
                            .join(child.name().unwrap())
                            .as_path(),
                            definition.merge(),
                            definition.overwrite()                            
                    ),
                    fs, 
                    guard
                )?
            );
        }
    } else if source.is_file() {
        transaction.add(Atomic::CopyFileToFile{
            source: source.to_path(),
            destination: destination.to_path()
        });
    }
    Ok(transaction)
} 


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use super::*;

    use crate::{
        sample::Samples,
        infrastructure::RealFileSystem,
        port::{
            FileSystemAdapter
        },
        capability::{
            ZealousGuard
        }
    };

    #[test]
    fn copy_operation_dir(){
        let chroot = Samples::init_simple_chroot("copy_operation_dir");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            CopyOperationDefinition::new(
                chroot.join("RDIR").as_path(),
                chroot.join("COPIED").as_path(),
                false,
                false
            ), 
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("COPIED/RFILEA").exists());
    }

    #[test]
    fn copy_operation_dir_merge_overwrite(){
        let chroot = Samples::init_simple_chroot("copy_operation_dir_merge_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            CopyOperationDefinition::new(
                chroot.join("RDIR").as_path(),
                chroot.join("RDIR2").as_path(),
                true,
                true
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(chroot.join("RDIR/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEC").exists());
        assert_eq!(
            chroot.join("RDIR/RFILEA").metadata().unwrap().len(),
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }

    #[test]
    fn copy_operation_file(){
        let chroot = Samples::init_simple_chroot("copy_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            CopyOperationDefinition::new(
                chroot.join("RDIR/RFILEB").as_path(),
                chroot.join("RDIR2/RFILEB").as_path(),
                false,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(chroot.join("RDIR/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert_eq!(
            chroot.join("RDIR/RFILEB").metadata().unwrap().len(),
            chroot.join("RDIR2/RFILEB").metadata().unwrap().len()
        )
    }

    #[test]
    fn copy_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("copy_operation_file_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        atomize(
            CopyOperationDefinition::new(
                chroot.join("RDIR/RFILEB").as_path(),
                chroot.join("RDIR2/RFILEB").as_path(),
                false,
                true
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(chroot.join("RDIR/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert_eq!(
            chroot.join("RDIR/RFILEB").metadata().unwrap().len(),
            chroot.join("RDIR2/RFILEB").metadata().unwrap().len()
        )
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
        Kind,
        capability::{
            ZealousGuard
        }
    };

    #[test]
    fn virtual_copy_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            CopyOperationDefinition::new(
                samples_path.join("A").as_path(),
                samples_path.join("Z").as_path(),
                false,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_directory_merge(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        //Avoid need of override because of .gitkeep file present in both directory
        let gitkeep = samples_path.join("B/.gitkeep");
        fs.as_inner_mut().mut_sub_state().attach(gitkeep.as_path(),Some(gitkeep.as_path()), Kind::File).unwrap();

        atomize(
            CopyOperationDefinition::new(
                samples_path.join("B").as_path(),
                samples_path.join("A").as_path(),
                true,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_file(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            CopyOperationDefinition::new(
                samples_path.join("F").as_path(),
                samples_path.join("Z").as_path(),
                false,
                false
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
         .apply(&mut fs)
         .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_file_overwrite(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        atomize(
            CopyOperationDefinition::new(
                samples_path.join("F").as_path(),
                samples_path.join("A/C").as_path(),
                false,
                true
            ),
            &fs, 
            &mut ZealousGuard
        ).unwrap()
        .apply(&mut fs)
        .unwrap();

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/C").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("A/C").as_path()).unwrap());
    }
}
