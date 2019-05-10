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

use std::{
    slice::Iter,
    vec::IntoIter as VecIntoIter,
    iter::FromIterator,
    cmp::Ord
};

use crate::{
    port::{
        Entry
    }
};

#[derive(Debug, Default)]
pub struct EntryCollection<T>(pub Vec<T>)
    where T: Entry;

impl <T: Entry> EntryCollection<T> {
    pub fn contains(&self, node: &Entry) -> bool {
        for owned_node in self.0.iter() {
            if owned_node.path() == node.path() {
                return true;
            }
        }
        false
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

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn sort(mut self) -> Self {
        self.0.sort_by(|left, right| left.path().cmp(right.path()));
        self
    }
}

impl <T: Entry>IntoIterator for EntryCollection<T> {
    type Item = T;
    type IntoIter = VecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl <T: Entry>FromIterator<T> for EntryCollection<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = EntryCollection::new();
        for i in iter {
            c.add(i);
        }
        c
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        path::Path
    };

    use crate::{
        port::{
            EntryAdapter
        }
    };


    #[test]
    fn entry_collection_contains() {
        let mut c = EntryCollection::new();
        c.add(EntryAdapter(Path::new("A")));

        assert!(c.contains(&EntryAdapter(Path::new("A"))));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn entry_collection_is_empty() {
        let c = EntryCollection::<EntryAdapter<&Path>>::new();

        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }
}
