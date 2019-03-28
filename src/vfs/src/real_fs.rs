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
use std::path::PathBuf;
use std::fs::{ File, create_dir, remove_dir_all, remove_file };

use std::io::prelude::*;
use std::io::{ BufReader, BufWriter, Error as IoError, ErrorKind };

const READ_BUFFER_SIZE: usize = 8;
const WRITE_BUFFER_SIZE: usize = 8;

use std::path::{ Path };
use std::ffi::{ OsStr, OsString };
use crate::{ VirtualDelta, VirtualChildren, VirtualPath, VirtualKind, VfsError, IdentityStatus };
use crate::operation::{ Virtual, Copy, Remove, Create, Status, ReadDir, ReadOperation, WriteOperation };

pub struct RealFileSystem {
    count: i128,
    dry: bool
}

impl RealFileSystem {
    pub fn new() -> RealFileSystem {
        RealFileSystem {
            count: 0,
            dry: true
        }
    }

    pub fn real_mode(&mut self){
        self.dry = false;
    }

    pub fn increment(&mut self){
        self.count += 1;
    }

    pub fn remove_file(&mut self, path: &Path) -> Result<(), IoError> {
        if self.dry {
            println!("DRY : remove file {:?}", path);

        } else {
            remove_file(path)?;
        }
        self.increment();
        Ok(())
    }

    pub fn remove_directory(&mut self, path: &Path) -> Result<(), IoError> {
        if self.dry {
            println!("DRY : remove directory recursivelly {:?}", path);
        } else {
            remove_dir_all(path)?;
        }
        self.increment();
        Ok(())
    }

    pub fn create_file(&mut self, path: &Path) -> Result<(), IoError> {
        if path.exists() {
            return Err(IoError::new(ErrorKind::AlreadyExists, "Create file : path already exists"));
        }

        if self.dry {
            println!("DRY : create_file {:?}", path);
        } else {
            File::create(path)?;
        }

        self.increment();
        Ok(())
    }

    pub fn create_directory(&mut self, path: &Path) -> Result<(), IoError> {
        if self.dry {
            println!("DRY : create directory {:?}", path);
        } else {
            create_dir(path)?;
        }

        self.increment();
        Ok(())
    }

    pub fn copy_file(&mut self, src: &PathBuf, dst: &PathBuf, on_read: &Fn(usize)) -> Result<(), IoError>{
        if self.dry {
            println!("DRY : copy file from {:?} to {:?}", src.as_path(), dst.as_path());
        } else {
            self._copy_file(src, dst, on_read)?;
        }
        Ok(())
    }

    fn _copy_file(&mut self, src: &PathBuf, dst: &PathBuf, on_read: &Fn(usize)) -> Result<(), IoError> {
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
                writer.flush();
                self.increment();
                Ok(())
            })
    }
}
