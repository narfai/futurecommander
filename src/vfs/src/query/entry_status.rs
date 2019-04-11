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

use crate::Kind;
use crate::representation::{ VirtualPath, VirtualState };
use crate::query::{ Entry, EntryAdapter};

#[derive(Debug)]
pub struct VirtualStatus {
    pub state: VirtualState,
    pub identity: VirtualPath
}

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
            Some(identity) =>
                match identity.as_kind() {
                    Kind::Directory => true,
                    _ => false
                },
            None => false
        }
    }

    fn is_file(&self) -> bool {
        match self.0.as_existing_virtual() {
            Some(identity) =>
                match identity.as_kind() {
                    Kind::File => true,
                    _ => false
                },
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
}
