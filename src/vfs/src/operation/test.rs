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

use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;
use std::path::{ Path, PathBuf };
use std::ffi::OsStr;
use crate::operation::{ Entry, Node, NodeIterator };
use crate::representation::{ VirtualDelta, VirtualChildren, VirtualPath, VirtualKind, VirtualChildrenIterator };

#[cfg(test)]
mod operation_test {
    use super::*;

    #[test]
    fn operation_virtual_entry_collection(){
        let mut children = VirtualChildren::new();
        children.insert(VirtualPath::from(PathBuf::from("/A"), None, VirtualKind::File).unwrap());
        children.insert(VirtualPath::from(PathBuf::from("/B"), None, VirtualKind::Directory).unwrap());
        let mut virtual_collection = NodeIterator(children.into_iter());

        assert!(virtual_collection.any(
            |entry|
                entry.name() == Some(&OsStr::new("A")) && entry.is_file()
            )
        );
        assert!(virtual_collection.any(
            |entry|
                entry.name() == Some(&OsStr::new("B")) && entry.is_dir()
            )
        );
    }
}
