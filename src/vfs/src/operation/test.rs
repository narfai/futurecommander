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
use std::ffi::{ OsStr, OsString };

use crate::*;

#[cfg(test)]
mod operation_test {
    use super::*;

    #[test]
    fn virtual_entry_collection(){
        let mut children = VirtualChildren::new();
        children.insert(VirtualPath::from(PathBuf::from("/A"), None, VirtualKind::File).unwrap());
        children.insert(VirtualPath::from(PathBuf::from("/B"), None, VirtualKind::Directory).unwrap());

        assert!(NodeIterator(children.clone().into_iter()).any(
            |entry| {
                entry.name().is_some() && entry.name() == Some(OsStr::new("A")) && entry.is_file()
            })
        );
        assert!(NodeIterator(children.into_iter()).any(
            |entry| {
                entry.name().is_some() && entry.name() == Some(OsStr::new("B")) && entry.is_dir()
            })
        );
    }

    #[test]
    fn apply_virtual(){
        let mut apply : ApplyOperation<Box<WriteOperation<VirtualFileSystem>>> = ApplyOperation::new();

    }

    #[test]
    fn apply_real(){
        let mut apply : ApplyOperation<Box<WriteOperation<RealFileSystem>>> = ApplyOperation::new();
    }
}
