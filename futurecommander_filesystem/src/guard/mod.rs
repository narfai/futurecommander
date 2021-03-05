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

mod capabilities;
mod guard;
//TODO mod registrar_guard;
mod strategy;

use std::{
    fmt::{ Display, Formatter, Result as FmtResult }
};

pub use self::{
    capabilities::Capabilities,
    guard::{ Guard, ZealousGuard, BlindGuard, QuietGuard },
    // registrar_guard::{ RegistrarGuard }
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Capability {
    Merge,
    Overwrite,
    Recursive
}

impl Display for Capability {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Capability::Merge => "merge",
                Capability::Recursive => "recursive",
                Capability::Overwrite => "overwrite"
            }
        )
    }
}

impl Eq for Capability {}


