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

pub mod errors;
pub mod tools;

// /!\ Highly experimental
pub mod server;
pub mod client;
pub mod message;

pub use futurecommander_filesystem::SerializableEntry;

use std::{
    io::{
        prelude::*,
        Write,
        Stdin,
        Stdout,
        Stderr
    },
    sync::{
        Arc,
        Mutex
    }
};

use self::{
    errors::DaemonError
};

pub use self::server::{ State };

use futurecommander_filesystem::{
    Container
};

pub struct Daemon<'a> {

    out: &'a mut Stdout,
    err: &'a mut Stderr,
    container: Container
}

impl <'a>Daemon<'a> {

    pub fn new(out: &'a mut Stdout, err: &'a mut Stderr) -> Daemon<'a> {
        Daemon {
            out,
            err,
            container: Container::default()
        }
    }

    pub fn run(mut self) {
       unimplemented!();
    }
}
