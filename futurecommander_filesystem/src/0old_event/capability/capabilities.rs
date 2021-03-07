// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    ops::{ Add }
};

use serde::{ Serialize, Deserialize };

use crate::{
    event::capability::{
        Capability
    }
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Capabilities {
    merge: bool,
    overwrite: bool,
    recursive: bool
}

impl Capabilities {
    pub fn merge(self) -> bool {
        self.merge
    }

    pub fn overwrite(self) -> bool {
        self.overwrite
    }

    pub fn recursive(self) -> bool {
        self.recursive
    }

    pub fn authorize(self, capability: Capability) -> bool {
        match capability {
            Capability::Merge => self.merge(),
            Capability::Recursive => self.recursive(),
            Capability::Overwrite => self.overwrite()
        }
    }
}

impl Add<Capability> for Capabilities {
    type Output = Capabilities;

    fn add(self, right_cap: Capability) -> Capabilities {
        Capabilities {
            merge: self.merge() || right_cap == Capability::Merge,
            overwrite: self.overwrite() || right_cap == Capability::Overwrite,
            recursive: self.recursive() || right_cap == Capability::Recursive,
        }
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_capability() {
        let mut caps = Capabilities::default();
        caps = caps + Capability::Overwrite;
        assert!(caps.authorize(Capability::Overwrite));
        assert!(!caps.authorize(Capability::Merge));
        assert!(!caps.authorize(Capability::Recursive));

        caps = caps + Capability::Merge;
        assert!(caps.authorize(Capability::Overwrite));
        assert!(caps.authorize(Capability::Merge));
        assert!(!caps.authorize(Capability::Recursive));

        caps = caps + Capability::Recursive;
        assert!(caps.authorize(Capability::Overwrite));
        assert!(caps.authorize(Capability::Merge));
        assert!(caps.authorize(Capability::Recursive));
    }
}
