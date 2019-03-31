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

use std::env::current_exe;
use std::path::PathBuf;
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

pub fn init_real_samples_idempotently(arbitrary_identifier: &str) -> PathBuf {
    let chroot = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap()
        .join(Path::new("samples").join(format!("real_tests_{}", arbitrary_identifier)));

    if chroot.exists() {
        remove_dir_all(chroot.as_path()).unwrap();
    }

    create_dir(chroot.as_path()).unwrap();
    assert!(chroot.exists());

    create_dir(chroot.join("RDIR")).unwrap();
    assert!(chroot.join("RDIR").exists());

    let mut file = File::create(chroot.join("RDIR").join("RFILEA")).unwrap();
    file.write_all(b"[A]Gummies candy biscuit jelly cheesecake. Liquorice gingerbread oat cake marzipan gummies muffin. Sweet liquorice dessert. Caramels chupa chups lollipop dragee gummies sesame snaps. Tootsie roll lollipop chocolate cake chocolate jelly jelly-o sesame snaps gummies. Topping topping bear claw candy canes bonbon muffin cupcake. Tart croissant liquorice croissant tootsie roll cupcake powder icing. Dessert souffle cake ice cream pie cookie. Brownie cotton candy pudding ice cream pudding cotton candy gingerbread gummi bears. Dragee biscuit croissant chocolate bar cheesecake marshmallow wafer macaroon. Sweet roll chupa chups gummi bears oat cake halvah marshmallow souffle pie. Jujubes pastry fruitcake macaroon jelly lemon drops chocolate cake chocolate cake."
    ).unwrap();
    assert!(chroot.join("RDIR").join("RFILEA").exists());

    let mut file = File::create(chroot.join("RDIR").join("RFILEB")).unwrap();

    file.write_all(b"[B]Gummies candy biscuit jelly cheesecake. Liquorice gingerbread oat cake marzipan gummies muffin. Sweet liquorice dessert. Caramels chupa chups lollipop dragee gummies sesame snaps. Tootsie roll lollipop chocolate cake chocolate jelly jelly-o sesame snaps gummies. Topping topping bear claw candy canes bonbon muffin cupcake. Tart croissant liquorice croissant tootsie roll cupcake powder icing. Dessert souffle cake ice cream pie cookie. Brownie cotton candy pudding ice cream pudding cotton candy gingerbread gummi bears. Dragee biscuit croissant chocolate bar cheesecake marshmallow wafer macaroon. Sweet roll chupa chups gummi bears oat cake halvah marshmallow souffle pie. Jujubes pastry fruitcake macaroon jelly lemon drops chocolate cake chocolate cake."
    ).unwrap();
    assert!(chroot.join("RDIR").join("RFILEB").exists());

    chroot
}


#[cfg(test)]
mod real_file_system_tests {
    use super::*;

    #[test]
    pub fn copy_file_to_file(){
        let chroot = init_real_samples_idempotently("copy_file_to_file");
        let fs = RealFileSystem::new(false);

        fs.copy_file_to_file(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("COPIED").as_path(),
            &|_read| { /*println!("read {}", read);*/ }
            ).unwrap();

        assert!(chroot.join("COPIED").exists());
        assert!(chroot.join("COPIED").is_file());
        assert!(chroot.join("COPIED").metadata().unwrap().len() > 1);
    }

    #[test]
    pub fn copy_file_to_directory(){
        let chroot = init_real_samples_idempotently("copy_file_to_directory");
        let fs = RealFileSystem::new(false);

        fs.copy_file_into_directory(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.as_path(),
            &|_read| { /*println!("read {}", read);*/ }
        ).unwrap();

        assert!(chroot.join("RFILEA").exists());
        assert!(chroot.join("RFILEA").is_file());
    }

    #[test]
    pub fn copy_directory_to_directory(){
        let chroot = init_real_samples_idempotently("copy_directory_to_directory");
        let fs = RealFileSystem::new(false);

        fs.copy_directory_into_directory(
            chroot.join("RDIR").as_path(),
            chroot.join("COPIED").as_path(),
            &|_read| { /*println!("read {}", read);*/ }
        ).unwrap();

        assert!(chroot.join("COPIED").exists());
        assert!(chroot.join("COPIED").is_dir());
        assert!(chroot.join("COPIED/RFILEA").exists());
        assert!(chroot.join("COPIED/RFILEB").exists());
    }

    #[test]
    pub fn create_file(){
        let chroot = init_real_samples_idempotently("create_file");
        let fs = RealFileSystem::new(false);

        fs.create_file(chroot.join("FILE").as_path()).unwrap();

        assert!(chroot.join("FILE").exists());
        assert!(chroot.join("FILE").is_file());
    }

    #[test]
    pub fn create_directory(){
        let chroot = init_real_samples_idempotently("create_directory");
        let fs = RealFileSystem::new(false);

        fs.create_directory(chroot.join("DIRECTORY").as_path()).unwrap();

        assert!(chroot.join("DIRECTORY").exists());
        assert!(chroot.join("DIRECTORY").is_dir());
    }

    #[test]
    pub fn remove_file(){
        let chroot = init_real_samples_idempotently("remove_file");
        let fs = RealFileSystem::new(false);

        fs.remove_file(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    pub fn remove_directory(){
        let chroot = init_real_samples_idempotently("remove_directory");
        let fs = RealFileSystem::new(false);

        fs.remove_directory(chroot.join("RDIR").as_path()).unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(!chroot.join("RDIR/RFILEB").exists());
    }
}
