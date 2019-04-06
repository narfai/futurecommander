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

use crate::VirtualPath;

#[derive(Debug)]
pub enum IdentityStatus {
    Exists(VirtualPath), //Exists only in real FS
    ExistsVirtually(VirtualPath), //Directly added
    ExistsThroughVirtualParent(VirtualPath), //Indirectly added
    Replaced(VirtualPath),
    NotExists, //Does not exists in virtual fs or real, indirectly or not
    Removed, //Does exists in real fs and should be deleted
    RemovedVirtually, //Does exists in virtual but is also virtually deleted
}

impl IdentityStatus {
    pub fn into_virtual_identity(self) -> Option<VirtualPath> {
        match self {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity) => Some(virtual_identity),

            IdentityStatus::NotExists
            | IdentityStatus::Removed
            | IdentityStatus::RemovedVirtually => None
        }
    }

    pub fn as_virtual_identity(&self) -> Option<&VirtualPath> {
        match self {
            IdentityStatus::Exists(virtual_identity)
            | IdentityStatus::ExistsVirtually(virtual_identity)
            | IdentityStatus::ExistsThroughVirtualParent(virtual_identity)
            | IdentityStatus::Replaced(virtual_identity) => Some(virtual_identity),

            IdentityStatus::NotExists
            | IdentityStatus::Removed
            | IdentityStatus::RemovedVirtually => None
        }
    }

    pub fn exists(&self) -> bool {
        match &self.as_virtual_identity() {
            Some(_) => true,
            None => false
        }
    }
}
