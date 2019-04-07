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

use std::vec::IntoIter;
use crate::file_system::{ RealFileSystem };
use crate::{ VfsError };
use crate::operation::Operation;

pub struct Transaction<T>(Vec<Box<dyn Operation<T>>>);

impl std::fmt::Debug for Transaction<RealFileSystem> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl <T> Transaction<T> {
    pub fn new() -> Transaction<T> {
        Transaction::<T>(Vec::new())
    }

    pub fn apply (&self, fs: &mut T) -> Result<(), VfsError>{
        for operation in self.0.iter() {
             operation.execute(fs)?
        }
        Ok(())
    }

    pub fn add_operation(&mut self, operation: Box<dyn Operation<T>>) {
        self.0.push(operation);
    }
}

impl <T>IntoIterator for Transaction<T> {
    type Item = Box<dyn Operation<T>>;
    type IntoIter = IntoIter<Box<dyn Operation<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
