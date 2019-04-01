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

use std::slice::Iter;
use crate::{ Real, VfsError };
use crate::file_system::{ VirtualFileSystem, RealFileSystem };
use crate::operation::WriteOperation;

pub struct Apply<T>(Vec<T>);

impl <T> Apply<Box<WriteOperation<T>>> {
    pub fn new() -> Self {
        Apply(Vec::<Box<WriteOperation<T>>>::new())
    }
}

impl <T> WriteOperation<T> for Apply<Box<WriteOperation<T>>> { //TODO Real<Apply<...>> ?
    fn execute(&self, fs: &mut T) -> Result<(), VfsError> {
        for operation in self.0.iter() {
            operation.execute(fs)?;
        }
        Ok(())
    }
}

