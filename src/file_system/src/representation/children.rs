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

use std::collections::{
    HashSet,
    hash_set::Iter as HashSetIter
};

use crate::representation::{
    VirtualPath,
    VirtualDelta,
    errors::RepresentationError
};

#[derive(Debug, Clone, Default)]
pub struct VirtualChildren {
    set: HashSet<VirtualPath>
}

impl VirtualChildren {
    pub fn insert(&mut self, virtual_identity: VirtualPath) -> bool {
        self.set.insert(virtual_identity)
    }

    pub fn remove(&mut self, virtual_identity: &VirtualPath) -> bool {
        self.set.remove(virtual_identity)
    }

    pub fn get(&self, virtual_identity: &VirtualPath) -> Option<&VirtualPath> {
        self.set.get(&virtual_identity)
    }

    pub fn is_empty(&self) -> bool { self.set.is_empty() }

    pub fn contains(&self, virtual_identity: &VirtualPath) -> bool {
        self.set.contains(virtual_identity)
    }

    pub fn iter(&self) -> VirtualChildrenIterator<'_> {
        VirtualChildrenIterator::new(self.set.iter())
    }

    //TODO unused yet but seems useful at least for debugging
    pub fn into_delta(self) -> Result<VirtualDelta, RepresentationError> {
        let mut delta = VirtualDelta::default();
        for virtual_identity in self.iter() {
            delta.attach_virtual(&virtual_identity)?;
        }
        Ok(delta)
    }
}

#[derive(Debug, Clone)]
pub struct VirtualChildrenIterator<'a> {
    iter: HashSetIter<'a, VirtualPath>
}

impl <'a>VirtualChildrenIterator<'a> {
    pub fn new(iter: HashSetIter<'a, VirtualPath>) -> VirtualChildrenIterator<'_> {
        VirtualChildrenIterator {
            iter
        }
    }
}

impl <'a>Iterator for VirtualChildrenIterator<'a> {
    type Item = &'a VirtualPath;

    fn next(&mut self) -> Option<&'a VirtualPath> {
        self.iter.next()
    }
}
