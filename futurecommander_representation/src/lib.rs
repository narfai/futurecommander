// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

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
