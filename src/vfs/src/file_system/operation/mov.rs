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

/* I EXPECT THE PATH destination TO EXISTS WITH SOURCE source */

use crate::VfsError;
use crate::file_system::RealFileSystem;
use crate::operation::{ Operation, MoveOperation };

impl Operation<RealFileSystem> for MoveOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        if ! self.source().exists() {
            return Err(VfsError::DoesNotExists(self.source().to_path_buf()));
        }

        if self.source().is_dir() && self.destination().is_dir() && self.merge() {
            for result in self.source().read_dir()? {
                let child = result?.path();
                let new_destination = self.destination().join(child.strip_prefix(self.source()).unwrap());
                MoveOperation::new(
                    child.as_path(),
                    new_destination.as_path(),
                    self.merge(),
                    self.overwrite()
                ).execute(fs)?;
            }
            fs.remove_directory(self.source())?;
            Ok(())
        } else {
            match fs.move_to(self.source(), self.destination(), self.overwrite()) {
                Ok(_) => Ok(()),
                Err(error) => Err(VfsError::from(error))
            }
        }
        //TODO switch to copy + remove if error
    }
}


#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Samples};

    #[test]
    fn move_operation_dir(){
        let chroot = Samples::init_simple_chroot("move_operation_dir");
        let mut fs = RealFileSystem::default();

        let operation = MoveOperation::new(
            chroot.join("RDIR").as_path(),
            chroot.join("MOVED").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(chroot.join("MOVED").exists());
        assert!(chroot.join("MOVED/RFILEA").exists());
        assert!(chroot.join("MOVED/RFILEB").exists());
    }

    #[test]
    fn move_operation_dir_merge_overwrite(){
        let chroot = Samples::init_simple_chroot("move_operation_dir_merge_overwrite");
        let mut fs = RealFileSystem::default();

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let operation = MoveOperation::new(
            chroot.join("RDIR").as_path(),
            chroot.join("RDIR2").as_path(),
            true,
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEC").exists());
        assert_eq!(
            a_len,
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }

    #[test]
    fn move_operation_file(){
        let chroot = Samples::init_simple_chroot("move_operation_file");
        let mut fs = RealFileSystem::default();

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let operation = MoveOperation::new(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("MOVED").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("MOVED").exists());
        assert_eq!(
            a_len,
            chroot.join("MOVED").metadata().unwrap().len()
        )
    }

    #[test]
    fn move_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("move_operation_file_overwrite");
        let mut fs = RealFileSystem::default();

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let operation = MoveOperation::new(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("RDIR2/RFILEA").as_path(),
            false,
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert_eq!(
            a_len,
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }
}
