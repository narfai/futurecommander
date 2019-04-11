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
                new_destination.as_path()
            ).execute(fs)?;
        }
        Ok(())
    }

    pub fn copy_file(&self, fs: &mut RealFileSystem) -> Result<(), VfsError>{
        match fs.copy_file_to_file(
            self.source(),
            self.destination(),
            &|_s| { println!("{} {}", self.destination().file_name().unwrap().to_string_lossy(), _s / 1024)},
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

        match source.is_dir() {
            true =>
                match destination.exists() {
                    true => {
                        if ! self.merge() {
                            return Err(VfsError::Custom("Merge is not allowed".to_string()))
                        }
                        self.copy_real_children(fs)
                    },
                    false => {
                        fs.create_directory(destination, false)?;
                        self.copy_real_children(fs)
                    } //dir to dir
                },
            false =>
                match destination.exists() {
                    true => {
                        if !self.overwrite() {
                            return Err(VfsError::Custom("Overwrite is not allowed".to_string()));
                        }
                        self.copy_file(fs)
                    },
                    false => self.copy_file(fs)
                },
        }
    }
}
