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
use crate::operation::{Operation, MoveOperation };

impl Operation<RealFileSystem> for MoveOperation {
    fn execute(&self, fs: &mut RealFileSystem) -> Result<(), VfsError> {
        if ! self.source().exists() {
            return Err(VfsError::DoesNotExists(self.source().to_path_buf()));
        }

        match fs.move_to(self.source(), self.destination(), self.overwrite()) {
            Ok(_) => Ok(()),
            Err(error) => Err(VfsError::from(error))
        }
        //TODO switch to copy + remove if error
    }
}
