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
use std::ffi::OsStr;

use futurecommander_representation::{
    VirtualPath,
    VirtualState
};

use crate::{
    Kind,
    port::{
        Entry,
        EntryAdapter
    }
};

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualStatus {
    pub state: VirtualState,
    pub identity: VirtualPath
}

impl Eq for VirtualStatus {}

impl VirtualStatus {
    pub fn new(state: VirtualState, identity: VirtualPath) -> VirtualStatus {
        VirtualStatus {
            state,
            identity
        }
    }

    pub fn into_virtual(self) -> VirtualPath {
        self.identity
    }

    pub fn as_virtual(&self) -> &VirtualPath {
        &self.identity
    }

    pub fn as_existing_virtual(&self) -> Option<&VirtualPath> {
        match self.state {
            VirtualState::Exists
            | VirtualState::ExistsVirtually
            | VirtualState::ExistsThroughVirtualParent
            | VirtualState::Replaced
            => Some(&self.identity),

            VirtualState::NotExists | VirtualState::Removed | VirtualState::RemovedVirtually
            => None
        }
    }

    pub fn into_existing_virtual(self) -> Option<VirtualPath> {
        match self.state {
            VirtualState::Exists
            | VirtualState::ExistsVirtually
            | VirtualState::ExistsThroughVirtualParent
            | VirtualState::Replaced
            => Some(self.identity),

            VirtualState::NotExists | VirtualState::Removed | VirtualState::RemovedVirtually
            => None
        }
    }

    pub fn state(&self) -> VirtualState {
        self.state
    }
}


impl Entry for EntryAdapter<VirtualStatus> {
    fn path(&self) -> &Path {
        self.0.as_virtual().as_identity()
    }

    fn to_path(&self) -> PathBuf {
        self.0.as_virtual().to_identity()
    }

    fn name(&self) -> Option<&OsStr> {
        self.path().file_name()
    }

    fn is_dir(&self) -> bool {
        match self.0.as_existing_virtual() {
            Some(identity) => matches!(identity.as_kind(), Kind::Directory),
            None => false
        }
    }

    fn is_file(&self) -> bool {
        match self.0.as_existing_virtual() {
            Some(identity) => matches!(identity.as_kind(), Kind::File),
            None => false
        }
    }

    fn exists(&self) -> bool {
        match self.0.state() {
            VirtualState::Exists
            | VirtualState::ExistsVirtually
            | VirtualState::ExistsThroughVirtualParent
            | VirtualState::Replaced => true,

            VirtualState::NotExists
            | VirtualState::Removed
            | VirtualState::RemovedVirtually => false
        }
    }

    fn is_virtual(&self) -> bool {
        match self.0.state() {
            VirtualState::ExistsVirtually
            | VirtualState::ExistsThroughVirtualParent
            | VirtualState::RemovedVirtually
            | VirtualState::Replaced => true,

            VirtualState::Exists
            | VirtualState::NotExists
            | VirtualState::Removed => false
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_adapter_virtual_status_directory() {
        let a_path = PathBuf::from("/MOCK");
        let a_status = VirtualStatus::new(
            VirtualState::Exists,
            VirtualPath::from(
                a_path.clone(),
                Some(a_path.clone()),
                Kind::Directory
            ).unwrap()
        );

        let a = EntryAdapter(a_status.clone());
        assert!(a.exists());
        assert!(a.is_dir());
        assert!(!a.is_file());
        assert!(!a.is_virtual());
        assert_eq!(a.to_path(), a_path);
        assert_eq!(a.path(), a_path.as_path());
        assert_eq!(a.name(), Some(OsStr::new("MOCK")));
        assert_eq!(a.into_inner(), a_status);
    }


    #[test]
    fn entry_adapter_virtual_status_file() {
        let a_path = PathBuf::from("/MOCK");
        let a_status = VirtualStatus::new(
            VirtualState::Exists,
            VirtualPath::from(
                a_path.clone(),
                Some(a_path.clone()),
                Kind::File
            ).unwrap()
        );

        let a = EntryAdapter(a_status.clone());
        assert!(a.exists());
        assert!(!a.is_dir());
        assert!(a.is_file());
        assert!(!a.is_virtual());
        assert_eq!(a.to_path(), a_path);
        assert_eq!(a.path(), a_path.as_path());
        assert_eq!(a.name(), Some(OsStr::new("MOCK")));
        assert_eq!(a.into_inner(), a_status);
    }

    fn _mock_virtual_status_entry(status: VirtualState, kind: Kind) -> EntryAdapter<VirtualStatus>{
        EntryAdapter(
            VirtualStatus::new(
                status,
                VirtualPath::from(
                    PathBuf::from("/MOCK"),
                    Some(PathBuf::from("/MOCK")),
                    kind
                ).unwrap()
            )
        )
    }


    #[test]
    fn entry_adapter_virtual_status_not_exists() {
        let a = _mock_virtual_status_entry(VirtualState::NotExists, Kind::Unknown);
        assert!(!a.exists());
        assert!(!a.is_dir());
        assert!(!a.is_file());
        assert!(!a.is_virtual());
    }

    #[test]
    fn entry_adapter_virtual_status_removed() {
        let a = _mock_virtual_status_entry(VirtualState::Removed, Kind::Unknown);
        assert!(!a.exists());
        assert!(!a.is_dir());
        assert!(!a.is_file());
        assert!(!a.is_virtual());
    }

    #[test]
    fn entry_adapter_virtual_status_removed_virtually() {
        let a = _mock_virtual_status_entry(VirtualState::RemovedVirtually, Kind::Unknown);
        assert!(!a.exists());
        assert!(!a.is_dir());
        assert!(!a.is_file());
        assert!(a.is_virtual());
    }

    #[test]
    fn entry_adapter_virtual_status_exists_virtually() {
        let a = _mock_virtual_status_entry(VirtualState::ExistsVirtually, Kind::Directory);
        assert!(a.exists());
        assert!(a.is_dir());
        assert!(!a.is_file());
        assert!(a.is_virtual());
    }

    #[test]
    fn entry_adapter_virtual_status_exists_through_virtual_parent() {
        let a = _mock_virtual_status_entry(VirtualState::ExistsThroughVirtualParent, Kind::File);
        assert!(a.exists());
        assert!(!a.is_dir());
        assert!(a.is_file());
        assert!(a.is_virtual());
    }

    #[test]
    fn entry_adapter_virtual_status_replaced() {
        let a = _mock_virtual_status_entry(VirtualState::Replaced, Kind::File);
        assert!(a.exists());
        assert!(!a.is_dir());
        assert!(a.is_file());
        assert!(a.is_virtual());
    }
}
