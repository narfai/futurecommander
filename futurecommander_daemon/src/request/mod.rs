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

mod list;

pub use self::{
    list::ListRequest
};

use crate:: {
    errors::DaemonError,
};

use std::{
    fmt::{
        Debug
    },
};

use futurecommander_filesystem::{
    Container
};

pub trait Request : Debug {
    fn header() -> RequestHeader where Self:Sized;
    fn process(&self, container: &mut Container) -> Result<Vec<u8>, DaemonError>;
    fn as_bytes(&self) -> Result<Vec<u8>, DaemonError>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, DaemonError> where Self: Sized;
}

pub enum RequestHeader {
    InvalidHeader,
    List
}

impl RequestHeader {
    pub fn list() -> &'static str { "LIST" }
}

impl From<u8> for RequestHeader {
    fn from(code: u8) -> RequestHeader {
        if (RequestHeader::List as u8) == code {
            RequestHeader::List
        } else {
            RequestHeader::InvalidHeader
        }
    }
}
