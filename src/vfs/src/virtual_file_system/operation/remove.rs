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
use crate::{ VirtualFileSystem, VfsError };
use crate::operation::{Operation, RemoveOperation };
use crate::query::{ Query, StatusQuery, VirtualStatus};
use crate::representation::VirtualState;

impl Operation<VirtualFileSystem> for RemoveOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        match StatusQuery::new(self.path()).retrieve(&fs)?.into_inner() {
            VirtualStatus{ state: VirtualState::Exists, identity }
            | VirtualStatus{ state: VirtualState::Replaced, identity }
            | VirtualStatus{ state: VirtualState::ExistsThroughVirtualParent, identity } => {
                fs.mut_sub_state().attach_virtual(&identity)?;
            },
            VirtualStatus{ state: VirtualState::ExistsVirtually, identity } => {
                fs.mut_add_state().detach(identity.as_identity())?;
                if let Some(source) = identity.as_source() {
                    if let VirtualStatus{
                        state: VirtualState::Replaced,
                        identity: virtual_path
                    } = StatusQuery::new(source).retrieve(&fs)?.into_inner() {
                        if fs.add_state().is_directory_empty(virtual_path.as_identity()) {
                            fs.mut_add_state().detach(virtual_path.as_identity())?;
                        }
                    }
                }
            }
            VirtualStatus{ state: VirtualState::NotExists, .. }
            | VirtualStatus{ state: VirtualState::Removed, .. }
            | VirtualStatus{ state: VirtualState::RemovedVirtually, .. } =>
                return Err(VfsError::DoesNotExists(self.path().to_path_buf()))
            ,
        }
        Ok(())
    }
}
