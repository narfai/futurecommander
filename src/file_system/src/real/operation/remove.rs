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

use crate::{
    OperationError,
    RealFileSystem,
    operation::{ Operation, RemoveOperation }
};

impl Operation<RealFileSystem> for RemoveOperation{
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), OperationError> {
        let path = self.path();
        if ! path.exists() {
            return Err(OperationError::DoesNotExists(path.to_path_buf()));
        }

        if path.is_dir() {
            if ! self.recursive() && self.path().read_dir()?.count() > 0 {
                return Err(OperationError::DirectoryIsNotEmpty(path.to_path_buf()));
            }

            fs.remove_directory(path)
        } else {
            fs.remove_file(path)
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ Samples };

    #[test]
    fn remove_operation_file() {
        let chroot = Samples::init_simple_chroot("remove_operation_file");
        let mut fs = RealFileSystem::default();

        let operation = RemoveOperation::new(
            chroot.join("RDIR/RFILEA").as_path(),
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    fn remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("remove_operation_directory");
        let mut fs = RealFileSystem::default();

        let operation = RemoveOperation::new(
            chroot.join("RDIR3").as_path(),
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(!chroot.join("RDIR3").exists());
    }

    #[test]
    fn remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("remove_operation_directory_recursive");
        let mut fs = RealFileSystem::default();

        let operation = RemoveOperation::new(
            chroot.join("RDIR").as_path(),
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(!chroot.join("RDIR").exists());
    }
}
