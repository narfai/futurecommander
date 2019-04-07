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
        if path.exists() {
            return Err(VfsError::AlreadyExists(path.to_path_buf()));
        }

        if ! self.recursive() {
            let parent = VirtualPath::get_parent_or_root(path);
            if !parent.exists() {
                return Err(VfsError::DoesNotExists(path.to_path_buf()));
            } else if !parent.is_dir() {
                return Err(VfsError::IsNotADirectory(path.to_path_buf()));
            }
        }

        let result = match self.kind() {
            Kind::File => fs.create_file(path),
            Kind::Directory => fs.create_directory(path, self.recursive()),
            _ => Ok(())
        };

        match result {
            Err(error) => Err(VfsError::from(error)),
            Ok(_) => Ok(())
        }
    }
}
