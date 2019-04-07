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

use std::fs::{ File, rename, create_dir, remove_dir_all, remove_file };

use std::io::prelude::*;
use std::io::{ BufReader, BufWriter, Error as IoError, ErrorKind };

const READ_BUFFER_SIZE: usize = 64;
const WRITE_BUFFER_SIZE: usize = 64;

use std::path::{ Path, Ancestors };

#[derive(Debug)]
pub struct RealFileSystem {
    dry: bool
}

impl RealFileSystem {
    pub fn new(dry: bool) -> RealFileSystem {
        RealFileSystem {
            dry
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

    pub fn remove_directory(&self, path: &Path) -> Result<(), IoError> {//TODO remove_dir if force true
        if self.dry {
            println!("DRY : remove directory recursivelly {:?}", path);
        } else {
            remove_dir_all(path)?;
        }
        Ok(())
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
                fn recursive_dir_creation(mut ancestors: &mut Ancestors<'_>) -> Result<(), IoError> {
                    if let Some(path) = ancestors.next() {
                        if ! path.exists() {
                            recursive_dir_creation(&mut ancestors)?;
                            create_dir(path)?;
                        }
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

    pub fn copy_file_to_file(&self, src: &Path, dst: &Path, on_read: &dyn Fn(usize), overwrite: bool) -> Result<usize, IoError>{
        if ! src.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Source does not exists {:?}", src)));
        }

        if ! src.is_file() { //TODO @symlink
            return Err(IoError::new(ErrorKind::InvalidData, format!("Source is not a file {:?}", src)));
        }

        if overwrite {
            if ! dst.is_file() { //TODO @symlink
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

    fn _copy_file(&self, src: &Path, dst: &Path, on_read: &dyn Fn(usize)) -> Result<usize, IoError> {
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
    pub fn move_to(&self, src: &Path, dst: &Path, overwrite: bool) -> Result<(), IoError> {
        if ! overwrite && dst.exists() {
            return Err(IoError::new(ErrorKind::InvalidData, format!("Destination already exists {:?}", dst)));
        }
        rename(src, dst)
    }
}
