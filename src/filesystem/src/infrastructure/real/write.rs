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
    path::{ Path },
    io::{
        BufReader, BufWriter, Error,
        prelude::*
    },
    fs::{
        File,
        create_dir,
        rename,
        remove_file,
        remove_dir
    }
};

use crate::{
    port::{
        WriteableFileSystem,
        FileSystemAdapter
    },
    infrastructure::{
        errors::{ InfrastructureError },
        real::{
            RealFileSystem
        }
    }
};

impl FileSystemAdapter<RealFileSystem> {
    fn _copy_file(&self, src: &Path, dst: &Path, on_read: &dyn Fn(usize)) -> Result<usize, Error> {
        File::open(src)
            .and_then(|src_file| Ok(BufReader::with_capacity(self.0.read_buffer_size,src_file)))
            .and_then(|reader|
                File::create(dst)
                    .and_then(|dst_file| Ok((reader, BufWriter::with_capacity(self.0.write_buffer_size,dst_file) ) ) )
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

    fn safe_parent(&self, path: &Path) -> Result<(), InfrastructureError> {
        match path.parent() {
            Some(parent) =>
                if ! parent.exists() {
                    Err(InfrastructureError::ParentDoesNotExists(parent.to_path_buf()))
                } else if !parent.is_dir() {
                    Err(InfrastructureError::ParentIsNotADirectory(parent.to_path_buf()))
                } else {
                    Ok(())
                }
            None => {
                Ok(())
            }
        }
    }

    fn safe_file_translation(&self, source: &Path, destination: &Path) -> Result<(), InfrastructureError> {
        self.safe_parent(destination)?;

        if !source.exists() {
            return Err(InfrastructureError::SourceDoesNotExists(source.to_path_buf()));
        }

        if destination.exists() && ! destination.is_file() {
            return Err(InfrastructureError::DestinationIsNotAFile(destination.to_path_buf()));
        }

        if source.exists() && ! source.is_file() {
            return Err(InfrastructureError::SourceIsNotAFile(source.to_path_buf()));
        }
        Ok(())
    }
}

impl WriteableFileSystem for FileSystemAdapter<RealFileSystem> {
    //Write real specialization
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        self.safe_parent(path)?;
        create_dir(path)?;
        Ok(())
    }

    fn create_empty_file(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        self.safe_parent(path)?;
        File::create(path)?;
        Ok(())
    }

    fn copy_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>{
        self.safe_file_translation(source, destination)?;
        self._copy_file(source, destination, &|_|{})?;
        Ok(())
    }

    fn move_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>{
        self.safe_file_translation(source, destination)?;
        match rename(source, destination) {
            Err(error) => {
                println!("WARNING FALLBACK TO COPY / REMOVE {}", error);
                self.copy_file_to_file(source, destination)?;
                self.remove_file(source)
            },
            Ok(_) => Ok(())
        }
    }

    fn bind_directory_to_directory(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError> {
        self.safe_parent(destination)?;

        if !source.exists() {
            return Err(InfrastructureError::SourceDoesNotExists(source.to_path_buf()));
        }

        if destination.exists() {
            return Err(InfrastructureError::DestinationAlreadyExists(destination.to_path_buf()));
        }

        if source.exists() && ! source.is_dir() {
            return Err(InfrastructureError::SourceIsNotADirectory(source.to_path_buf()));
        }

        self.create_empty_directory(destination)
    }

    fn remove_file(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        if ! path.exists() {
            return Err(InfrastructureError::PathDoesNotExists(path.to_path_buf()));
        }
        remove_file(path)?;
        Ok(())
    }

    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>{
        if ! path.exists() {
            return Err(InfrastructureError::PathDoesNotExists(path.to_path_buf()));
        }
        remove_dir(path)?;
        Ok(())
    }
}



#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ sample::Samples };

    #[test]
    pub fn create_empty_file() {
        let chroot = Samples::init_simple_chroot("create_empty_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        fs.create_empty_file(chroot.join("FILE").as_path()).unwrap();

        assert!(chroot.join("FILE").exists());
        assert!(chroot.join("FILE").is_file());
    }

    #[test]
    pub fn create_empty_directory() {
        let chroot = Samples::init_simple_chroot("create_empty_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        fs.create_empty_directory(chroot.join("DIRECTORY").as_path()).unwrap();

        assert!(chroot.join("DIRECTORY").exists());
        assert!(chroot.join("DIRECTORY").is_dir());
    }

    #[test]
    pub fn copy_file_to_file() {
        let chroot = Samples::init_simple_chroot("copy_file_to_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        fs.copy_file_to_file(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("COPIED").as_path()
        ).unwrap();

        assert!(chroot.join("COPIED").exists());
        assert!(chroot.join("COPIED").is_file());
        assert!(chroot.join("COPIED").metadata().unwrap().len() > 1);
    }

    #[test]
    pub fn remove_file() {
        let chroot = Samples::init_simple_chroot("remove_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        fs.remove_file(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    pub fn remove_empty_directory() {
        let chroot = Samples::init_simple_chroot("remove_empty_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        fs.create_empty_directory(chroot.join("TEST").as_path()).unwrap();

        assert!(chroot.join("TEST").exists());

        fs.remove_empty_directory(chroot.join("TEST").as_path()).unwrap();

        assert!(!chroot.join("TEST").exists());
    }


    #[test]
    pub fn move_file_to_file() {
        let chroot = Samples::init_simple_chroot("move_file_to_file");

        let mut fs = FileSystemAdapter(RealFileSystem::default());

        fs.move_file_to_file(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("MOVED").as_path()
        ).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("MOVED").exists());
    }
}

