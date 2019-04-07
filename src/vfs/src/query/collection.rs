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
use std::vec::IntoIter as VecIntoIter;

use crate::query::Entry;

#[derive(Debug)]
pub struct EntryCollection<T>(pub Vec<T>)
    where T: Entry;

impl <T> EntryCollection<T> where T: Entry {
    pub fn contains(&self, node: &Entry) -> bool {
        for owned_node in self.0.iter() {
            if owned_node.path() == node.path() {
                return true;
            }
        }
        return false;
    }

    pub fn new() -> Self {
        EntryCollection(Vec::new())
    }

    pub fn add(&mut self, node: T) {
        self.0.push(node)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_iter(self) -> VecIntoIter<T> {
        self.0.into_iter()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }
}
