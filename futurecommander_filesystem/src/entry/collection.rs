// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    slice::Iter,
    vec::IntoIter as VecIntoIter,
    iter::FromIterator,
    cmp::Ord
};
use super::{ Entry };

//TODO DELETE AND REPLACE BY FS ITERATORS
#[derive(Debug, Default)]
pub struct EntryCollection<T>(pub Vec<T>)
    where T: Entry;

impl <T: Entry> EntryCollection<T> {
    pub fn contains(&self, node: &dyn Entry) -> bool {
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