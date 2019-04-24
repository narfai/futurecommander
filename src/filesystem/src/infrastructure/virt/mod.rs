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

pub mod entry_status;
pub mod read;
pub mod write;
pub mod representation;

use crate::infrastructure::virt::{
    representation::{
        VirtualDelta,
        errors::RepresentationError
    }
};

#[derive(Debug, Default)]
pub struct VirtualFileSystem {
    add: VirtualDelta,
    sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn reset(&mut self) {
        self.add = VirtualDelta::default();
        self.sub = VirtualDelta::default();
    }

    pub fn has_addition(&self) -> bool { !self.add.is_empty() }

    pub fn has_subtraction(&self) -> bool { !self.sub.is_empty() }

    pub fn is_empty(&self) -> bool { ! self.has_addition() && ! self.has_subtraction() }

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

    pub fn virtual_state(&self) -> Result<VirtualDelta, RepresentationError> { &self.add - &self.sub }

    pub fn reverse_state(&self) -> Result<VirtualDelta, RepresentationError> { &self.sub - &self.add }
}
