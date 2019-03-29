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

use std::path::{ Path, PathBuf };
use std::ffi::{ OsStr };
//use std::fs::ReadDir;

use crate::{ VirtualPath, VirtualKind, VirtualChildrenIterator };

pub trait Entry {
    fn path(&self) -> &Path;
    fn to_path(&self) -> PathBuf;
    fn name(&self) -> Option<&OsStr>;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
}

#[derive(Debug)]
pub struct Node<T>(pub T);

impl Entry for Node<VirtualPath> {
    fn path(&self) -> &Path {
        self.0.as_identity()
    }

    fn to_path(&self) -> PathBuf {
        self.0.to_identity()
    }

    fn name(&self) -> Option<&OsStr> {
        self.0.as_identity().file_name()
    }

    fn is_dir(&self) -> bool {
        match self.0.to_kind() {
            VirtualKind::Directory => true,
            _ => false
        }
    }

    fn is_file(&self) -> bool {
        match self.0.to_kind() {
            VirtualKind::File => true,
            _ => false
        }
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
}

#[derive(Debug)]
pub struct NodeIterator<I>(pub I);

impl <I> Iterator for NodeIterator<I>
    where I: Iterator<Item = VirtualPath> {

    type Item = Node<VirtualPath>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(virtual_identity) => Some(Node(virtual_identity)),
            None => None
        }
    }
}


