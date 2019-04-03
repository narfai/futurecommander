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

use std::path::{ PathBuf, Path };

use crate::{ Virtual, VfsError };

use crate::representation::{ VirtualPath, VirtualKind, VirtualChildren };
use crate::file_system::{ VirtualFileSystem };
use crate::query::{ReadQuery, StatusQuery, NodeIterator };

use std::collections::hash_set::IntoIter as HashSetIntoIter;

pub struct ReadDirQuery {
    path: PathBuf
}

impl ReadDirQuery {
    pub fn new(path: &Path) -> ReadDirQuery {
        ReadDirQuery {
            path: path.to_path_buf()
        }
    }
}

impl ReadQuery<&VirtualFileSystem, NodeIterator<HashSetIntoIter<VirtualPath>>> for Virtual<ReadDirQuery> {
    fn retrieve(&self, fs: &VirtualFileSystem) -> Result<NodeIterator<HashSetIntoIter<VirtualPath>>, VfsError> {
        let stat_directory = Virtual(StatusQuery::new(self.0.path.as_path()));
        let directory = match stat_directory.retrieve(&fs)?.virtual_identity() {
            Some(virtual_identity) =>
                match virtual_identity.as_kind() {
                    VirtualKind::Directory => virtual_identity,
                    _ => return Err(VfsError::IsNotADirectory(self.0.path.to_path_buf()))
                },
            None => return Err(VfsError::DoesNotExists(self.0.path.to_path_buf()))
        };

        let mut real_children = VirtualChildren::from_file_system(
            directory.as_source().unwrap_or(directory.as_identity()),
            directory.as_source(),
            Some(self.0.path.as_path())
        )?;

        if let Some(to_add_children) = fs.add_state().children(directory.as_identity()) {
            real_children = &real_children + &to_add_children;
        }
        if let Some(to_del_children) = fs.sub_state().children(directory.as_identity()) {
            real_children = &real_children - &to_del_children;
        }

        Ok(NodeIterator(real_children.into_iter()))
    }
}
