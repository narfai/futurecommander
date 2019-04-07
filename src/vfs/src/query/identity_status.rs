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


use crate::representation::{VirtualPath, Kind};
use crate::query::{ Entry, Node };

#[derive(Debug)]
pub enum IdentityStatus {
    Exists(VirtualPath), //Exists only in real FS
    ExistsVirtually(VirtualPath), //Directly added
    ExistsThroughVirtualParent(VirtualPath), //Indirectly added
    Replaced(VirtualPath),
    NotExists(VirtualPath), //Does not exists in virtual fs or real, indirectly or not
    Removed(VirtualPath), //Does exists in real fs and should be deleted
    RemovedVirtually(VirtualPath), //Does exists in virtual but is also virtually deleted
}

impl IdentityStatus {
    pub fn into_virtual(self) -> VirtualPath {
        match self {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity)
            | IdentityStatus::NotExists(virtual_identity)
            | IdentityStatus::Removed(virtual_identity)
            | IdentityStatus::RemovedVirtually(virtual_identity)
                => virtual_identity
        }
    }

    pub fn as_virtual(&self) -> &VirtualPath {
        match self {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity)
            | IdentityStatus::NotExists(virtual_identity)
            | IdentityStatus::Removed(virtual_identity)
            | IdentityStatus::RemovedVirtually(virtual_identity)
                => virtual_identity
        }
    }

    pub fn as_existing_virtual(&self) -> Option<&VirtualPath> {
        match self {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity)
                => Some(virtual_identity),

            IdentityStatus::NotExists(_) | IdentityStatus::Removed(_) | IdentityStatus::RemovedVirtually(_)
                => None
        }
    }

    pub fn into_existing_virtual(self) -> Option<VirtualPath> {
        match self {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity)
                => Some(virtual_identity),

            IdentityStatus::NotExists(_) | IdentityStatus::Removed(_) | IdentityStatus::RemovedVirtually(_)
                => None
        }
    }
}

impl Entry for Node<IdentityStatus> {
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
        match self.0 {
            IdentityStatus::Exists(_)
            | IdentityStatus::ExistsVirtually(_)
            | IdentityStatus::ExistsThroughVirtualParent(_)
            | IdentityStatus::Replaced(_) => true,

            IdentityStatus::NotExists(_)
            | IdentityStatus::Removed(_)
            | IdentityStatus::RemovedVirtually(_) => false
        }
    }
}
