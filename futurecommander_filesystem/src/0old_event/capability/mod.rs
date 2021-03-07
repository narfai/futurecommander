// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod capabilities;
mod guard;
mod registrar_guard;

use std::{
    fmt::{ Display, Formatter, Result as FmtResult }
};

pub use self::{
    capabilities::Capabilities,
    guard::{ Guard, ZealousGuard, BlindGuard, QuietGuard },
    registrar_guard::{ RegistrarGuard }
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


