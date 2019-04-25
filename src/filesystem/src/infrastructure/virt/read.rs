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

use std::{ path::{ Path }, vec::IntoIter };

use crate::{
    Kind,
    errors::{
      QueryError
    },
    port::{
        ReadableFileSystem,
        FileSystemAdapter,
        EntryAdapter,
        EntryCollection,
        Entry
    },
    infrastructure::virt::{
        VirtualFileSystem,
        entry_status::{ VirtualStatus },
        representation::{
            VirtualState,
            VirtualPath
        }
    }
};

impl FileSystemAdapter<VirtualFileSystem> {
    fn virtual_unknown(&self, path: &Path) -> Result<VirtualPath, QueryError>{
        match VirtualPath::from(
            path.to_path_buf(),
            Some(path.to_path_buf()),
            Kind::Unknown
        ) {
            Ok(virtual_identity) => Ok(virtual_identity),
            Err(error) => Err(QueryError::from(error))
        }
    }

    fn status_virtual(&self, path: &Path) -> Result<VirtualStatus, QueryError> {
        if self.0.sub_state().is_virtual(path)? {
            match self.0.add_state().get(path)? {
                Some(_virtual_state) => Err(QueryError::AddSubDanglingVirtualPath(path.to_path_buf())),
                None => Ok(VirtualStatus::new(VirtualState::RemovedVirtually, self.virtual_unknown(path)?))
            }
        } else {
            match self.0.add_state().get(path)? {//IN ADD AND NOT IN SUB
                Some(virtual_identity) =>
                    if path.exists() {
                        Ok(VirtualStatus::new(VirtualState::Replaced, virtual_identity.clone()))
                    } else {
                        Ok(VirtualStatus::new(VirtualState::ExistsVirtually, virtual_identity.clone()))
                    }
                None =>
                    match self.0.virtual_state()?.resolve(path)? {
                        Some(real_path) => {
                            if real_path.exists() {
                                Ok(
                                    VirtualStatus::new(
                                        VirtualState::ExistsThroughVirtualParent,
                                        VirtualPath::from(
                                            path.to_path_buf(),
                                            Some(real_path.clone()),
                                            Kind::from_path_buf(real_path)
                                        )?
                                    )
                                )
                            } else {
                                Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown(path)?))
                            }
                        },
                        None => Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown(path)?))//Got a virtual parent but does not exists
                    }

            }
        }
    }

    fn status_real(&self, path: &Path) -> Result<VirtualStatus, QueryError> {
        if self.0.sub_state().is_virtual(path)? {
            Ok(VirtualStatus::new(VirtualState::Removed, self.virtual_unknown(path)?))
        } else if path.exists() {
            Ok(
                VirtualStatus::new(
                    VirtualState::Exists,
                    VirtualPath::from(
                        path.to_path_buf(),
                        Some(path.to_path_buf()),
                        if path.is_dir()
                        { Kind::Directory }
                        else { Kind::File }
                    )?
                )
            )
        } else {
            Ok(VirtualStatus::new(VirtualState::NotExists, self.virtual_unknown(path)?))
        }

    }

    pub fn from_file_system(
        fs: &VirtualFileSystem,
        path: &Path,
        source: Option<&Path>,
        parent: Option<&Path>
    ) -> Result<EntryCollection<EntryAdapter<VirtualStatus>>, QueryError> {
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

                            if ! fs.sub_state().is_virtual(virtual_identity.as_identity())? {
                                entry_collection.add(EntryAdapter(VirtualStatus::new(VirtualState::Exists, virtual_identity)));
                            }
                        },
                        Err(error) => return Err(QueryError::from(error))
                    };
                }
                Ok(entry_collection)
            },
            Err(error) => Err(QueryError::from(error))
        }
    }
}

impl ReadableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    type Item = EntryAdapter<VirtualStatus>;
    //Read virtual specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Item>, QueryError> {
        let directory =
            match self.status(path)?
                .into_inner()
                .into_existing_virtual() {
                Some(virtual_identity) =>
                    match virtual_identity.as_kind() {
                        Kind::Directory => virtual_identity,
                        _ => return Err(QueryError::IsNotADirectory(path.to_path_buf()))
                    },
                None => return Err(QueryError::ReadTargetDoesNotExists(path.to_path_buf()))
            };

        let mut entry_collection = Self::from_file_system(
            &self.0,
            directory.as_source().unwrap_or_else(|| directory.as_identity()),
            directory.as_source(),
            Some(path)
        )?;

        if let Some(to_add_children) = self.0.add_state().children(directory.as_identity()) {
            for child in to_add_children.iter() {
                if ! self.0.sub_state().is_virtual(child.as_identity())? {
                    entry_collection.add(self.status(child.as_identity())?)
                }
            }
        }

        Ok(entry_collection)
    }

    fn status(&self, path: &Path) -> Result<Self::Item, QueryError> {
        if self.0.add_state().is_virtual(path)? {
            match self.status_virtual(path) {
                Ok(status) => Ok(EntryAdapter(status)),
                Err(error) => Err(error)
            }
        } else {
            match self.status_real(path) {
                Ok(status) => Ok(EntryAdapter(status)),
                Err(error) => Err(error)
            }
        }
    }

    fn read_maintained(&self, path: &Path) -> Result<EntryCollection<Self::Item>, QueryError> {
        Ok(
            self.read_dir(path)?.into_iter()
                .filter(|entry: &EntryAdapter<VirtualStatus>|{
                    entry.as_inner().state == VirtualState::ExistsVirtually
                }).collect()
        )
    }
}


#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        sample::Samples
    };

    #[test]
    fn status_query_relay_real_fs(){
        let static_samples = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());

        let entry = vfs.status(static_samples.as_path()).unwrap();
        assert_eq!(entry.as_inner().state(), VirtualState::Exists);
        assert!(entry.is_dir());
    }


    #[test]
    fn read_dir_query_relay_real_fs() {
        let static_samples = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());

        let collection = vfs.read_dir(static_samples.as_path()).unwrap();
        let a_path = static_samples.join("A");
        assert!(
            collection.contains(
                &EntryAdapter(
                    VirtualStatus::new(
                        VirtualState::Exists,
                        VirtualPath::from(
                            a_path.clone(),
                            Some(a_path.clone()),
                            Kind::Directory
                        ).unwrap()
                    )
                )
            )
        );

        let b_path = static_samples.join("B");
        assert!(
            collection.contains(
                &EntryAdapter(
                    VirtualStatus::new(
                        VirtualState::Exists,
                        VirtualPath::from(
                            b_path.clone(),
                            Some(b_path.clone()),
                            Kind::Directory
                        ).unwrap()
                    )
                )
            )
        );

        let b_path = static_samples.join("F");
        assert!(
            collection.contains(
                &EntryAdapter(
                    VirtualStatus::new(
                        VirtualState::Exists,
                        VirtualPath::from(
                            b_path.clone(),
                            Some(b_path.clone()),
                            Kind::File
                        ).unwrap()
                    )
                )
            )
        );
    }
}
