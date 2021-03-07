// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

#[macro_use]
extern crate clap;

extern crate rustyline;

mod shell;
mod helper;

pub mod command;
pub mod tools;

pub use self::helper::*;
pub use self::shell::Shell;

pub mod errors;

