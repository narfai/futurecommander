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

use crate::{ VfsError };
use crate::operation::WriteOperation;

pub struct Transaction<T: ?Sized>(Vec<Box<dyn WriteOperation<T>>>);

impl <T> Transaction<T> {
    pub fn apply (&self, fs: &mut T) -> Result<(), VfsError>{
        for operation in self.0.iter() {
             operation.execute(fs)?
        }
        Ok(())
    }

    pub fn add_operation(&mut self, operation: Box<dyn WriteOperation<T>>) {
        self.0.push(operation);
    }
}
