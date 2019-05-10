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

use std::{
    ops::{ Add, Sub }
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

impl Sub<Capability> for Capabilities {
    type Output = Capabilities;

    fn sub(self, right_cap: Capability) -> Capabilities {
        Capabilities {
            merge: self.merge() && right_cap != Capability::Merge,
            overwrite: self.overwrite() && right_cap != Capability::Overwrite,
            recursive: self.recursive() && right_cap != Capability::Recursive,
        }
    }
}
