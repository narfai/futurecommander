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
    io::{
        BufReader, BufWriter, Error, ErrorKind,
        prelude::*
    },
    fs::{ File, rename, create_dir, remove_dir_all, remove_file },
    path::{ Path, Ancestors }
};

use crate::{
    real::errors::RealError
};

const READ_BUFFER_SIZE: usize = 10_485_760; //10 Mo
const WRITE_BUFFER_SIZE: usize = 2_097_152; //2 Mo

#[derive(Debug, Default)]
pub struct RealFileSystem {}

impl RealFileSystem {
    pub fn default() -> RealFileSystem {
        RealFileSystem {}
    }

    pub fn remove_file(&self, path: &Path) -> Result<(), RealError> {
        remove_file(path)?;
        Ok(())
    }

    pub fn remove_directory(&self, path: &Path) -> Result<(), RealError> {//TODO remove_dir if force true
        remove_dir_all(path)?;
        Ok(())
    }

    pub fn create_file(&self, path: &Path, overwrite: bool) -> Result<(), RealError> {
        if !overwrite && path.exists() {
            return Err(RealError::AlreadyExists(path.to_path_buf()));
        }
        File::create(path)?; Ok(())
    }

    pub fn create_directory(&self, path: &Path, recursively: bool) -> Result<(), RealError> {
        if recursively {
            fn recursive_dir_creation(mut ancestors: &mut Ancestors<'_>) -> Result<(), RealError> {
                if let Some(path) = ancestors.next() {
                    if ! path.exists() {
                        recursive_dir_creation(&mut ancestors)?;
                        create_dir(path)?;
                    }
                }
                Ok(())
            }
            let mut ancestors = path.ancestors();
            recursive_dir_creation(&mut ancestors)
        } else {
            create_dir(path)?;
            Ok(())
        }
    }

    pub fn copy_file_to_file(&self, src: &Path, dst: &Path, on_read: &dyn Fn(usize), overwrite: bool) -> Result<usize, RealError>{
        if ! src.exists() {
            return Err(RealError::SourceDoesNotExists(src.to_path_buf()));
        }

        if ! src.is_file() { //TODO @symlink
            return Err(RealError::SourceIsNotAFile(src.to_path_buf()));
        }

        if dst.exists() {
            if overwrite {
                if !dst.is_file() { //TODO @symlink
                    return Err(RealError::OverwriteDirectoryWithFile(src.to_path_buf(), dst.to_path_buf()));
                }
            } else {
                return Err(RealError::OverwriteNotAllowed(src.to_path_buf(), dst.to_path_buf()));
            }
        }

        self.create_file(dst, overwrite)?;
        match self._copy_file(src, dst, on_read) {
            Ok(size) => Ok(size),
            Err(error) => Err(RealError::from(error))
        }
    }

    fn _copy_file(&self, src: &Path, dst: &Path, on_read: &dyn Fn(usize)) -> Result<usize, Error> {
        File::open(src)
            .and_then(|src_file| Ok(BufReader::with_capacity(READ_BUFFER_SIZE,src_file)))
            .and_then(|reader|
                File::create(dst)
                    .and_then(|dst_file| Ok((reader, BufWriter::with_capacity(WRITE_BUFFER_SIZE,dst_file) ) ) )
            )
            .and_then(|(mut reader, mut writer)| {
                let mut read = 0;
                loop {
                    match {
                        reader.fill_buf()
                            .and_then(|buffer| {
                                writer.write(&buffer)
                                    .and(Ok(buffer.len()))
                            })
                    } {
                        Ok(length) => {
                            if length == 0 {
                                break;
                            }
                            read += length;
                            on_read(read);
                            reader.consume(length);
                        }
                        Err(kind) => return Err(kind)
                    }
                }
                writer.flush()
                    .and(Ok(read))
            })
    }


    //@TODO find a way to detect if file coming from different resource if true then copy + rm instead
    pub fn move_to(&self, src: &Path, dst: &Path, overwrite: bool) -> Result<(), RealError> {
        if ! overwrite && dst.exists() {
            return Err(RealError::OverwriteNotAllowed(src.to_path_buf(), dst.to_path_buf()));
        }
        rename(src, dst)?;
        Ok(())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ Samples };

    #[test]
    pub fn create_file() {
        let chroot = Samples::init_simple_chroot("create_file");
        let fs = RealFileSystem::default();

        fs.create_file(chroot.join("FILE").as_path(), false).unwrap();

        assert!(chroot.join("FILE").exists());
        assert!(chroot.join("FILE").is_file());
    }

    #[test]
    pub fn create_file_overwrite() {
        let chroot = Samples::init_simple_chroot("create_file_overwrite");
        let fs = RealFileSystem::default();

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        fs.create_file(chroot.join("RDIR/RFILEA").as_path(), true).unwrap();

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert_ne!(a_len, chroot.join("RDIR/RFILEA").metadata().unwrap().len());
    }

    #[test]
    pub fn create_directory() {
        let chroot = Samples::init_simple_chroot("create_directory");
        let fs = RealFileSystem::default();

        fs.create_directory(chroot.join("DIRECTORY").as_path(), false).unwrap();

        assert!(chroot.join("DIRECTORY").exists());
        assert!(chroot.join("DIRECTORY").is_dir());
    }

    #[test]
    pub fn create_directory_recursively() {
        let chroot = Samples::init_simple_chroot("create_directory_recursively");
        let fs = RealFileSystem::default();

        fs.create_directory(chroot.join("DEEP/NESTED/DIRECTORY").as_path(), true).unwrap();

        assert!(chroot.join("DEEP/NESTED/DIRECTORY").exists());
        assert!(chroot.join("DEEP/NESTED/DIRECTORY").is_dir());
    }

    #[test]
    pub fn copy_file_to_file() {
        let chroot = Samples::init_simple_chroot("copy_file_to_file");
        let fs = RealFileSystem::default();

        fs.copy_file_to_file(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("COPIED").as_path(),
            &|_read| { /*println!("read {}", read);*/ },
            false
        ).unwrap();

        assert!(chroot.join("COPIED").exists());
        assert!(chroot.join("COPIED").is_file());
        assert!(chroot.join("COPIED").metadata().unwrap().len() > 1);
    }

    #[test]
    pub fn copy_file_to_file_overwrite() {
        let chroot = Samples::init_simple_chroot("copy_file_to_file_overwrite");
        let fs = RealFileSystem::default();

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        fs.copy_file_to_file(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("RDIR/RFILEB").as_path(),
            &|_read| { /*println!("read {}", read);*/ },
            true
        ).unwrap();

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("RDIR/RFILEB").is_file());
        assert_eq!(a_len, chroot.join("RDIR/RFILEB").metadata().unwrap().len());
    }

    #[test]
    pub fn remove_file() {
        let chroot = Samples::init_simple_chroot("remove_file");
        let fs = RealFileSystem::default();

        fs.remove_file(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    pub fn remove_directory_recursively() {
        let chroot = Samples::init_simple_chroot("remove_directory");
        let fs = RealFileSystem::default();

        fs.remove_directory(chroot.join("RDIR").as_path()).unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(!chroot.join("RDIR/RFILEB").exists());
    }


    #[test]
    pub fn move_to() {
        let chroot = Samples::init_simple_chroot("move_directory");

        let fs = RealFileSystem::default();

        fs.move_to(
            chroot.join("RDIR").as_path(),
            chroot.join("MOVED").as_path(),
            false
        ).unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(!chroot.join("RDIR/RFILEB").exists());

        assert!(chroot.join("MOVED").exists());
        assert!(chroot.join("MOVED/RFILEA").exists());
        assert!(chroot.join("MOVED/RFILEB").exists());
    }

    #[test]
    pub fn move_to_overwrite() {
        let chroot = Samples::init_simple_chroot("move_to_overwrite");

        let fs = RealFileSystem::default();

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        fs.move_to(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("RDIR/RFILEB").as_path(),
            true
        ).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("RDIR/RFILEB").exists());

        assert_eq!(a_len, chroot.join("RDIR/RFILEB").metadata().unwrap().len());
    }
}
