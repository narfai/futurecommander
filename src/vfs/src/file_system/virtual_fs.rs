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

use crate::{ VirtualDelta, VfsError };

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

    pub fn reset(&mut self) {
        self.add = VirtualDelta::new();
        self.sub = VirtualDelta::new();
    }

    pub fn has_addition(&self) -> bool { !self.add.is_empty() }

    pub fn has_subtraction(&self) -> bool { !self.sub.is_empty() }

    pub fn mut_add_state(&mut self) -> &mut VirtualDelta {
        &mut self.add
    }

    pub fn mut_sub_state(&mut self) -> &mut VirtualDelta {
        &mut self.sub
    }

    pub fn add_state(&self) -> &VirtualDelta {
        &self.add
    }

    pub fn sub_state(&self) -> &VirtualDelta {
        &self.sub
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
