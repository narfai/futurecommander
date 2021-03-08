// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use futurecommander_representation::{
    VirtualDelta,
    errors::RepresentationError
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

#[cfg(test)]
mod test {
    use std::path::{ Path };
    use crate::{ Kind };
    use super::*;

    #[test]
    fn reset_empty() {
        let mut vfs = VirtualFileSystem::default();

        vfs.mut_add_state().attach(Path::new("/virtualA"), None, Kind::Directory).unwrap();
        vfs.mut_add_state().attach(Path::new("/virtualB"), None, Kind::File).unwrap();
        vfs.mut_sub_state().attach(Path::new("/A"), None, Kind::Directory).unwrap();

        assert!(vfs.has_addition());
        assert!(vfs.has_subtraction());

        vfs.reset();

        assert!(!vfs.has_addition());
        assert!(!vfs.has_subtraction());
    }
}
