/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommanderVfs.
 *
 * FutureCommanderVfs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommanderVfs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommanderVfs.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::{ Path };
use std::ffi::{ OsStr, OsString };
use crate::{ VirtualDelta, VirtualChildren, VirtualChildrenIterator, VirtualPath, VirtualKind, VfsError, IdentityStatus };
use crate::operation::{ Virtual, Copy, Remove, Create, Status, ReadDir, ReadOperation, WriteOperation, NodeIterator, Entry };
use std::collections::hash_set::IntoIter as HashSetIntoIter;

//TODO Wrapper Historized Vfs

#[derive(Debug)]
pub struct VirtualFileSystem {
    add: VirtualDelta,
    sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            add: VirtualDelta::new(),
            sub: VirtualDelta::new()
        }
    }

//    //TODO @deprecated
//    pub fn status(&self, path: &Path) -> Result<IdentityStatus, VfsError> {
//        Virtual(Status::new(path)).retrieve(self)
//    }
//
//    //TODO @deprecated
//    pub fn stat(&self, path: &Path) -> Result<Option<VirtualPath>, VfsError> {
//        Ok(
//            Virtual(Status::new(path))
//                .retrieve(self)?
//                .virtual_identity()
//        )
//
//    }
//
//    //TODO @deprecated
//    pub fn exists(&self, path: &Path) -> Result<bool, VfsError> {
//        Ok(
//            Virtual(Status::new(path))
//                .retrieve(self)?
//                .exists()
//        )
//    }
//
//    //TODO @deprecated
//    pub fn read_dir(&self, path: &Path) -> Result<NodeIterator<HashSetIntoIter<VirtualPath>>, VfsError> {
//        Virtual(ReadDir::new(path)).retrieve(self)
//    }
//
//    //TODO @deprecated
//    pub fn create(&mut self, identity: &Path, kind: VirtualKind) -> Result<(), VfsError>{
//        Virtual(Create::new(identity, kind)).execute(self)
//    }
//
//    //TODO @deprecated
//    pub fn remove(&mut self, identity: &Path) -> Result<(), VfsError>{
//        Virtual(Remove::new(identity)).execute(self)
//    }
//
//    //TODO @deprecated
//    pub fn copy(&mut self, source: &Path, destination: &Path, with_name: Option<OsString>) -> Result<(), VfsError>{
//        Virtual(Copy::new(source, destination, with_name)).execute(self)
//    }

    pub fn reset(&mut self) {
        self.add = VirtualDelta::new();
        self.sub = VirtualDelta::new();
    }

    pub fn is_empty(&self) -> bool {
        self.add.is_empty() && self.sub.is_empty()
    }

    pub fn mut_add_state(&mut self) -> &mut VirtualDelta {
        &mut self.add
    }

    pub fn mut_sub_state(&mut self) -> &mut VirtualDelta {
        &mut self.sub
    }

    pub fn add_state(&self) -> VirtualDelta {
        self.add.clone()
    }

    pub fn sub_state(&self) -> VirtualDelta {
        self.sub.clone()
    }

    pub fn virtual_state(&self) -> Result<VirtualDelta, VfsError> { &self.add - &self.sub }
}

/*
TODO : https://trello.com/c/ocihsIuv/29-as-human-i-can-apply-virtual-file-system-to-real-file-system-in-order-to-get-them-both-into-the-closer-possible-state
walk over vfs virtual path which have a source. for each of them, sorted by path depth asc :
ExistsVirtually(VirtualPath), => copy recursively source path to identity path ( with handling of name change ) then remove childs from add
Exists(VirtualPath), => Do nothing
ExistsThroughVirtualParent(VirtualPath), => Do nothing
NotExists, => Do nothing
Deleted, => Do nothing
RemovedVirtually, => Do nothing ?

walk over subs virtual path which have a source. for each of them, sorted by path depth asc :
ExistsVirtually(VirtualPath), => copy recursively source path to identity path ( with handling of name change ) then remove vpath with are childs by source from add
Exists(VirtualPath), => Delete recursively the source path, then remove vpath with are childs by source from sub
ExistsThroughVirtualParent(VirtualPath), => Do nothing
NotExists, => Do nothing
Deleted, => Do nothing
RemovedVirtually, => Do nothing ?

*/
