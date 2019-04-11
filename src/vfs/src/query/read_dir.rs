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

#[allow(unused_imports)]
use crate::query::Entry;
use crate::{ VfsError, Kind };
use crate::virtual_file_system::{ VirtualFileSystem };
use crate::representation::{VirtualPath, VirtualChildren, VirtualState };
use crate::query::{Query, StatusQuery, EntryCollection, EntryAdapter, VirtualStatus};

pub struct ReadDirQuery {
    path: PathBuf
}

impl ReadDirQuery {
    pub fn new(path: &Path) -> ReadDirQuery {
        ReadDirQuery {
            path: path.to_path_buf()
        }
    }

    pub fn from_file_system(path: &Path, source: Option<&Path>, parent: Option<&Path>) -> Result<EntryCollection<EntryAdapter<VirtualStatus>>, VfsError> {
        if !path.exists() {
            return Ok(EntryCollection::new());
        }

        let mut entry_collection = EntryCollection::new();

        match path.read_dir() {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(result) => {
                            let result_path = result.path();
                            let mut virtual_identity = VirtualPath::from_path(result.path().as_path())?
                                .with_source(Some(result_path.as_path()))
                                .with_kind(Kind::from_path(result_path.as_path()));

                            if let Some(source) = source {
                                virtual_identity = virtual_identity.with_new_source_parent(source);
                            }

                            if let Some(parent) = parent {
                                virtual_identity = virtual_identity.with_new_identity_parent(parent);
                            }

                            entry_collection.add(EntryAdapter(VirtualStatus::new(VirtualState::Exists, virtual_identity)));
                        },
                        Err(error) => return Err(VfsError::from(error))
                    };
                }
                Ok(entry_collection)
            },
            Err(error) => Err(VfsError::from(error))
        }
    }
}

impl Query<&VirtualFileSystem> for ReadDirQuery {
    type Result = EntryCollection<EntryAdapter<VirtualStatus>>;

    fn retrieve(&self, fs: &VirtualFileSystem) -> Result<Self::Result, VfsError> {
        let directory =
            match StatusQuery::new(self.path.as_path())
                .retrieve(&fs)?
                .into_inner()
                .into_existing_virtual() {
                    Some(virtual_identity) =>
                        match virtual_identity.as_kind() {
                            Kind::Directory => virtual_identity,
                            _ => return Err(VfsError::IsNotADirectory(self.path.to_path_buf()))
                        },
                    None => return Err(VfsError::DoesNotExists(self.path.to_path_buf()))
        };

        let mut entry_collection = Self::from_file_system(
            directory.as_source().unwrap_or(directory.as_identity()),
            directory.as_source(),
            Some(self.path.as_path())
        )?;

        if let Some(to_add_children) = fs.add_state().children(directory.as_identity()) {
            let empty = VirtualChildren::new();
            let to_del_children = fs.sub_state()
                    .children(directory.as_identity())
                    .unwrap_or(&empty);

            for child in to_add_children.iter() {
                if ! to_del_children.contains(child) {
                    entry_collection.add(StatusQuery::new(child.as_identity()).retrieve(fs)?)
                }
            }
        }

        Ok(entry_collection)
    }
}
