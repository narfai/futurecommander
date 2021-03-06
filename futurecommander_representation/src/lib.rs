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

#[cfg(test)]
mod tests;

pub mod errors;

mod path;
mod delta;
mod state;
mod kind;

pub use self::{
    kind::Kind,
    path::VirtualPath,
    delta::VirtualDelta,
    state::VirtualState
};

use std::collections::{
    HashSet
};

pub type VirtualChildren = HashSet<VirtualPath>;
