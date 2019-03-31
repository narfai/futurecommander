/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommanderVfs.
 *
 * FutureCommanderVfs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommanderVfs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommanderVfs.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::fs::{ File, create_dir, remove_dir_all, remove_file };

use std::io::prelude::*;
use std::io::{ BufReader, BufWriter, Error as IoError, ErrorKind };

const READ_BUFFER_SIZE: usize = 8;
const WRITE_BUFFER_SIZE: usize = 8;

use std::path::{ Path };
use std::ffi::{ OsStr, OsString };
//use crate::*;

pub struct RealFileSystem {
    dry: bool
    /*
    ==Options==
    ecrase or ignore existing file destination
    merge or ignore existing directory destination
    */
}

impl RealFileSystem {
    pub fn new(dry: bool) -> RealFileSystem {
        RealFileSystem {
            dry
        }
    }

    pub fn remove(&self, path: &Path) -> Result<(), IoError> {
        if path.is_dir() {
            self.remove_directory(path)
        } else {
            self.remove_file(path)
        }
    }

    pub fn remove_file(&self, path: &Path) -> Result<(), IoError> {
        if self.dry {
            println!("DRY : remove file {:?}", path);

        } else {
            remove_file(path)?;
        }
        Ok(())
    }

    pub fn remove_directory(&self, path: &Path) -> Result<(), IoError> {
        if self.dry {
            println!("DRY : remove directory recursivelly {:?}", path);
        } else {
            remove_dir_all(path)?;
        }
        Ok(())
    }

    pub fn create(&self, path: &Path) -> Result<(), IoError> {
        if path.is_dir() {
            self.create_directory(path)
        } else {
            self.create_file(path)
        }
    }

    pub fn create_file(&self, path: &Path) -> Result<(), IoError> {
        if path.exists() {
            return Err(IoError::new(ErrorKind::AlreadyExists, "Create file : path already exists"));
        }

        if self.dry {
            println!("DRY : create_file {:?}", path);
        } else {
            File::create(path)?;
        }

        Ok(())
    }

    //TODO handle mkdir -p with create_recursive_directory or smth
    pub fn create_directory(&self, path: &Path) -> Result<(), IoError> {
        if self.dry {
            println!("DRY : create directory {:?}", path);
        } else {
            create_dir(path)?;
        }

        Ok(())
    }

    pub fn copy(&self, src: &Path, dst: &Path, on_read: &Fn(usize)) -> Result<usize, IoError> {
        if ! src.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, "Source does not exists"))
        }

        match src.is_dir() {
            true =>
                match dst.is_dir() {
                    true => self.copy_directory_into_directory(src, dst, on_read),
                    false =>
                        match dst.exists() {
                            true => return Err(IoError::new(ErrorKind::InvalidData, "Destination is not a directory")),
                            false => return Err(IoError::new(ErrorKind::InvalidData, "Destination does not exists")),
                        }
                },
            false =>
                match dst.is_dir() {
                    true => self.copy_file_into_directory(src, dst, on_read),
                    false =>
                        match dst.exists() {
                            false => self.copy_file_to_file(src, dst, on_read),
                            true =>  return Err(IoError::new(ErrorKind::InvalidData, "Destination already exists")),//TODO optionnaly allow file erase
                        }
                }
        }
    }

    pub fn copy_file_into_directory(&self, src: &Path, dst: &Path, on_read: &Fn(usize)) -> Result<usize, IoError> {
        let new_destination = match src.file_name() {
            Some(file_name) => dst.join(file_name),
            None => return Err(IoError::new(ErrorKind::InvalidData, "Source file name is a dot path"))
        };

        if new_destination.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("New destination already exists : {:?}", new_destination)));//TODO optionnaly allow directory merge
        }

        self.copy_file_to_file(src, new_destination.as_path(), on_read)
    }

    pub fn copy_directory_into_directory(&self, src: &Path, dst: &Path, on_read: &Fn(usize)) -> Result<usize, IoError> {
        let mut read : usize = 0;

        self.create_directory(dst)?;//TODO don't do that for directory merge

        for result in src.read_dir()? {
            let child = result?.path();

            let new_destination = dst.join(child.strip_prefix(src).unwrap());

            self.copy(child.as_path(), new_destination.as_path(), on_read)
                .and_then(|directory_read| {
                    read += directory_read;
                    Ok(())
                })?;
        }
        Ok(read)
    }

    pub fn copy_file_to_file(&self, src: &Path, dst: &Path, on_read: &Fn(usize)) -> Result<usize, IoError>{
        if src.is_dir() {
            return Err(IoError::new(ErrorKind::InvalidData, "Source is not a file"));
        }

        if dst.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, "Destination already exists"));
        }

        if self.dry {
            println!("DRY : copy file from {:?} to {:?}", &src, &dst); Ok(0 as usize)
        } else {
            self.create_file(dst)?;
            self._copy_file(src, dst, on_read)
        }
    }

    fn _copy_file(&self, src: &Path, dst: &Path, on_read: &Fn(usize)) -> Result<usize, IoError> {
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
}
