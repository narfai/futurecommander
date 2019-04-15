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

use crate::VfsError;
use crate::file_system::RealFileSystem;
use crate::operation::{Operation, CopyOperation };

impl CopyOperation {
    pub fn copy_real_children(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        for result in self.source().read_dir()? {
            let child = result?.path();
            let new_destination = self.destination().join(child.strip_prefix(self.source()).unwrap());

            CopyOperation::new(
                child.as_path(),
                new_destination.as_path(),
                self.merge(),
                self.overwrite()
            ).execute(fs)?;
        }
        Ok(())
    }

    pub fn copy_file(&self, fs: &mut RealFileSystem) -> Result<(), VfsError>{
        match fs.copy_file_to_file(
            self.source(),
            self.destination(),
            &|_s| { /*println!("{} {}", self.destination().file_name().unwrap().to_string_lossy(), _s / 1024)*/ },
            self.overwrite()
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(VfsError::from(error))
        }
    }
}


impl Operation<RealFileSystem> for CopyOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        let source = self.source();
        let destination = self.destination();

        if ! source.exists() {
            return Err(VfsError::DoesNotExists(source.to_path_buf()));
        }

        if source.is_dir() && destination.is_file() {
            return Err(VfsError::Custom("Cannot copy directory to the path of existing file".to_string())); //Error dir to existing file
        }

        if source.is_file() && destination.is_dir() {
            return Err(VfsError::Custom("Cannot copy file to the path existing directory".to_string()));
        }

        if source.is_dir() {
            if destination.exists() {
                if !self.merge() {
                    return Err(VfsError::Custom("Merge is not allowed".to_string()))
                }
                self.copy_real_children(fs)
            } else {
                fs.create_directory(destination, false)?;
                self.copy_real_children(fs)
            }
        } else if destination.exists() {
            if !self.overwrite() {
                return Err(VfsError::Custom("Overwrite is not allowed".to_string()));
            }
            self.copy_file(fs)
        } else {
            self.copy_file(fs)
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ Samples };

    #[test]
    fn copy_operation_dir(){
        let chroot = Samples::init_simple_chroot("copy_operation_dir");
        let mut fs = RealFileSystem::default();

        let operation = CopyOperation::new(
            chroot.join("RDIR").as_path(),
            chroot.join("COPIED").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("COPIED/RFILEA").exists());
    }

    #[test]
    fn copy_operation_dir_merge_overwrite(){
        let chroot = Samples::init_simple_chroot("copy_operation_dir_merge_overwrite");
        let mut fs = RealFileSystem::default();

        let operation = CopyOperation::new(
            chroot.join("RDIR").as_path(),
            chroot.join("RDIR2").as_path(),
            true,
            true
        );

        operation.execute(&mut fs).unwrap();

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
        let mut fs = RealFileSystem::default();

        let operation = CopyOperation::new(
            chroot.join("RDIR/RFILEB").as_path(),
            chroot.join("RDIR2/RFILEB").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

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
        let mut fs = RealFileSystem::default();

        let operation = CopyOperation::new(
            chroot.join("RDIR/RFILEB").as_path(),
            chroot.join("RDIR2/RFILEB").as_path(),
            false,
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(chroot.join("RDIR/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert_eq!(
            chroot.join("RDIR/RFILEB").metadata().unwrap().len(),
            chroot.join("RDIR2/RFILEB").metadata().unwrap().len()
        )
    }

    //TODO replace len tests with md5 sum for copy & move
}
