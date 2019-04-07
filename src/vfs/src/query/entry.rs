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

use std::cmp::Ordering;
use std::slice::Iter;
use std::vec::IntoIter as VecIntoIter;
use std::path::{ Path, PathBuf };
use std::ffi::{ OsStr };

pub trait Entry {
    fn path(&self) -> &Path;
    fn to_path(&self) -> PathBuf;
    fn name(&self) -> Option<&OsStr>;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn exists(&self) -> bool;
}

#[derive(Debug)]
pub struct Node<T>(pub T);

impl <T> Node<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn as_inner(&self) -> &T {
        &self.0
    }
}

impl Entry for Node<&Path> {
    fn path(&self) -> &Path {
        self.0.clone()
    }

    fn to_path(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    fn name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    fn exists(&self) -> bool { self.0.exists() }
}

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


impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.path().cmp(other.path())
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.path().cmp(other.path()))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.path().eq(other.path())
    }
}


