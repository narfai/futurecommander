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

use crate::{ VfsError, Kind };
use crate::file_system::RealFileSystem;
use crate::representation::VirtualPath;
use crate::operation::{Operation, CreateOperation };

impl Operation<RealFileSystem> for CreateOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        let path = self.path();

        if ! self.recursive() {
            let parent = VirtualPath::get_parent_or_root(path);
            if !parent.exists() {
                return Err(VfsError::DoesNotExists(path.to_path_buf()));
            } else if !parent.is_dir() {
                return Err(VfsError::IsNotADirectory(path.to_path_buf()));
            }
        }

        let result = match self.kind() {
            Kind::File => fs.create_file(path, self.overwrite()),
            Kind::Directory => fs.create_directory(path, self.recursive()),
            _ => Ok(())
        };

        match result {
            Err(error) => Err(VfsError::from(error)),
            Ok(_) => Ok(())
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Samples};

    #[test]
    fn create_operation_directory(){
        let chroot = Samples::init_simple_chroot("create_operation_directory");
        let mut fs = RealFileSystem::default();

        let operation = CreateOperation::new(
            chroot.join("CREATED").as_path(),
            Kind::Directory,
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(chroot.join("CREATED").is_dir());
        assert!(chroot.join("CREATED").exists());
    }


    #[test]
    fn create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("create_operation_directory_recursive");
        let mut fs = RealFileSystem::default();

        let operation = CreateOperation::new(
            chroot.join("CREATED/NESTED/DIRECTORY").as_path(),
            Kind::Directory,
            true,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(chroot.join("CREATED/NESTED/DIRECTORY").exists());
        assert!(chroot.join("CREATED/NESTED/DIRECTORY").is_dir());
    }

    #[test]
    fn create_operation_file(){
        let chroot = Samples::init_simple_chroot("create_operation_file");
        let mut fs = RealFileSystem::default();

        let operation = CreateOperation::new(
            chroot.join("CREATED").as_path(),
            Kind::File,
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(chroot.join("CREATED").exists());
        assert!(chroot.join("CREATED").is_file());
    }

    #[test]
    fn create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("create_operation_file_overwrite");
        let mut fs = RealFileSystem::default();

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let operation = CreateOperation::new(
            chroot.join("RDIR/RFILEA").as_path(),
            Kind::File,
            false,
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert_ne!(a_len, chroot.join("RDIR/RFILEA").metadata().unwrap().len());
    }
}
