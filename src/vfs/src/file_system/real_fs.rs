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

use std::path::{ Path, Ancestors };
use std::ffi::{ OsStr, OsString };
//use crate::*;

pub struct RealFileSystem {
    dry: bool
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

    pub fn create(&self, path: &Path, recursively: bool) -> Result<(), IoError> {
        if path.is_dir() {
            self.create_directory(path, recursively)
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

    pub fn create_directory(&self, path: &Path, recursively: bool) -> Result<(), IoError> {
        if self.dry {
            println!("DRY : create directory {:?}", path);
        } else {
            if recursively {
                fn recursive_dir_creation(mut ancestors: &mut Ancestors) -> Result<(), IoError> {
                    if let Some(path) = ancestors.next() {
                        recursive_dir_creation(&mut ancestors)?;
                        create_dir(path)?;
                    }
                    Ok(())
                }
                let mut ancestors = path.ancestors();
                recursive_dir_creation(&mut ancestors)?;
            } else {
                create_dir(path)?;
            }
        }

        Ok(())
    }

    pub fn copy(&self, src: &Path, dst: &Path, on_read: &Fn(usize), merge: bool, overwrite: bool) -> Result<usize, IoError> {
        if ! src.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Source does not exists {:?}", src)))
        }

        match src.is_dir() {
            true => self.copy_directory_into_directory(src, dst, on_read, merge, overwrite),
            false =>
                match dst.is_dir() {
                    true => self.copy_file_into_directory(src, dst, on_read, overwrite),
                    false => self.copy_file_to_file(src, dst, on_read, overwrite)
                }
        }
    }

    pub fn copy_file_into_directory(&self, src: &Path, dst: &Path, on_read: &Fn(usize), overwrite: bool) -> Result<usize, IoError> {
        if ! src.is_file() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Source is not a file {:?}", src)));
        }

        if ! dst.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Destination does not exists {:?}", dst)));
        } else if ! dst.is_dir(){
            return Err(IoError::new(ErrorKind::InvalidData, format!("Destination is not a directory {:?}", dst)));
        }

        let new_destination = match src.file_name() {
            Some(file_name) => dst.join(file_name),
            None => return Err(IoError::new(ErrorKind::InvalidData, format!("Source file name is a dot path {:?}", src)))
        };

        if new_destination.exists() && ! overwrite {
            return Err(IoError::new(ErrorKind::InvalidData, format!("New destination already exists : {:?}", new_destination)));
        }

        self.copy_file_to_file(src, new_destination.as_path(), on_read, overwrite)
    }

    pub fn copy_directory_into_directory(&self, src: &Path, dst: &Path, on_read: &Fn(usize), merge: bool, overwrite: bool) -> Result<usize, IoError> {
        if ! src.is_dir() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Source is not a directory {:?}", src)))
        }

        if dst.exists() {
            if ! dst.is_dir() {
                return Err(IoError::new(ErrorKind::InvalidData, format!("Destination is not a directory {:?}", dst)));
            }
        } else {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Destination does not exists {:?}", dst)));
        }

        let mut read : usize = 0;

        for result in src.read_dir()? {
            let child = result?.path();
            let new_destination = dst.join(child.strip_prefix(src).unwrap());

            if new_destination.exists() && ! merge {
                return Err(IoError::new(ErrorKind::InvalidData, "New destination already exists - no merge"));
            }

            if new_destination.is_dir() {
                self.create_directory(new_destination.as_path(), false)?;
            }

            self.copy(child.as_path(), new_destination.as_path(), on_read, merge, overwrite)
                .and_then(|directory_read| {
                    read += directory_read;
                    Ok(())
                })?;
        }
        Ok(read)
    }

    pub fn copy_file_to_file(&self, src: &Path, dst: &Path, on_read: &Fn(usize), overwrite: bool) -> Result<usize, IoError>{
        if ! src.is_file() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Source is not a file {:?}", src)));
        }

        if overwrite {
            if ! dst.is_file() {
                return Err(IoError::new(ErrorKind::InvalidData, format!("Destination is not a file {:?}", dst)));
            }
        } else if dst.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Destination already exists {:?}", dst)));
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
