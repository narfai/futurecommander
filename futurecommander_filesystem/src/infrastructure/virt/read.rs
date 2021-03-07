// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{ path::{ Path } };
use futurecommander_representation::{
    VirtualPath,
    VirtualState
};
use crate::{
    Kind,
    QueryError,
    EntryAdapter,
    EntryCollection,
    Entry
};
use super::super::{
    ReadableFileSystem,
    FileSystemAdapter,
    VirtualFileSystem,
    VirtualStatus
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
}

impl ReadableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    type Item = EntryAdapter<VirtualStatus>;
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

        let mut entry_collection = EntryCollection::new();

        let real_path = directory.as_source().unwrap_or_else(|| directory.as_identity());
        if real_path.exists() {
            match real_path.read_dir() {
                Ok(results) => {
                    for result in results {
                        match result {
                            Ok(result) => {
                                let result_path = result.path();
                                let mut virtual_identity = VirtualPath::from_path(result.path().as_path())?
                                    .with_source(Some(result_path.as_path()))
                                    .with_new_identity_parent(directory.as_identity())
                                    .with_kind(Kind::from_path(result_path.as_path()));

                                if let Some(source) = directory.as_source() {
                                    virtual_identity = virtual_identity.with_new_source_parent(source);
                                }

                                let entry_adapter = self.status(virtual_identity.as_identity())?;

                                if entry_adapter.exists() {
                                    entry_collection.add(entry_adapter);
                                }
                            },
                            Err(error) => return Err(QueryError::from(error))
                        };
                    }
                },
                Err(error) => return Err(QueryError::from(error))
            }
        }

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

    fn is_directory_empty(&self, path: &Path) -> Result<bool, QueryError> {
        let status = self.status(path)?;
        Ok(
            status.is_dir() && self.read_dir(path)?.iter().next().is_none()
        )
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        sample::Samples,
        WriteableFileSystem,
        Entry
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
                            Some(a_path),
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
                            Some(b_path),
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
                            Some(b_path),
                            Kind::File
                        ).unwrap()
                    )
                )
            )
        );
    }

    #[test]
    fn resolve() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());

        let b = sample_path.join("B");
        let ab = sample_path.join("A/B");
        let abcdef = sample_path.join("A/B/C/D/E/F");

        vfs.bind_directory_to_directory(b.as_path(), ab.as_path()).unwrap();

        let virtual_state = vfs.as_inner().virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );
        assert_eq!(
            b.join("C/D/E/F").as_path(),
            virtual_state.resolve(abcdef.as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn resolve_through() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());

        let b = sample_path.join("B");

        let ab = sample_path.join("A/B");
        let bd = sample_path.join("B/D");

        vfs.bind_directory_to_directory(b.as_path(), ab.as_path()).unwrap();
        vfs.bind_directory_to_directory(ab.as_path(), bd.join("B").as_path()).unwrap();
        let virtual_state = vfs.as_inner().virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(bd.join("B").as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn stat_none_if_deleted() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let f = sample_path.join("F");

        assert!(vfs.status(f.as_path()).unwrap().exists());

        vfs.remove_file(f.as_path()).unwrap();

        assert!(! vfs.status(f.as_path()).unwrap().exists());
    }

    #[test]
    fn stat_virtual() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let z = sample_path.join("Z");

        vfs.create_empty_directory(z.as_path()).unwrap();

        let stated = vfs.status(z.as_path())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated.to_kind(), Kind::Directory);
        assert_eq!(stated.as_identity(), z);
        assert!(stated.as_source().is_none())
    }

    #[test]
    fn stat_real() {
        let sample_path = Samples::static_samples_path();
        let vfs = FileSystemAdapter(VirtualFileSystem::default());
        let a = sample_path.join("A");

        let stated = vfs.status(a.as_path())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated.to_kind(), Kind::Directory);
        assert_eq!(stated.as_identity(), a.as_path());
        assert_eq!(stated.as_source(), Some(a.as_path()))
    }

    #[test]
    fn stat_related() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
        let abdg = sample_path.join("A/B/D/G");//Note : should exists in samples


        vfs.bind_directory_to_directory(
            sample_path.join("B").as_path(),
            sample_path.join("A/B").as_path()
        ).unwrap();

        let stated = vfs.status(abdg.as_path())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated.to_kind(), Kind::Directory);
        assert_eq!(stated.as_identity(), abdg.as_path());
        assert_eq!(stated.as_source(), Some(sample_path.join("B/D/G").as_path()))
    }
}
