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

use crate::{ VfsError };
use crate::representation::{ VirtualDelta };

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

    pub fn reverse_state(&self) -> Result<VirtualDelta, VfsError> { &self.sub - &self.add }
}
