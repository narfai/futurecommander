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
//mod status;

pub use self::{
    list::ListAction,
//    status::StatusRequest
};

use serde::{
    Serialize,
    Deserialize
};

use bincode::{ deserialize, serialize };

use crate:: {
    errors::DaemonError,
    Context
};

use std::{
    fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult
    },
};

use futurecommander_filesystem::{
    Container
};

pub trait Request: Debug {
    fn process(&self, container: &mut Container) -> Result<Vec<u8>, DaemonError>;
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum RequestHeader {
    LIST,
//    Status
}

impl RequestHeader {
    pub fn encode_adapter(self, context: Context) -> Result<Vec<u8>, DaemonError> {
        let mut binary_request = vec![self as u8];
        match self {
            RequestHeader::LIST => {
                binary_request.append(
                    &mut serialize(&ListAction::adapter(context)?)?
                )
            }
        }
        Ok(binary_request)
    }

    pub fn decode_adapter(self, bytes: &[u8]) -> Result<Box<Request>, DaemonError> {
        match self {
            RequestHeader::LIST => {
                let request: RequestAdapter<ListAction> = deserialize(bytes)?;
                Ok(Box::new(request))
            }
        }
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, DaemonError> {
        if let Some(byte) = bytes.first() {
            match byte {
                b if b == &(RequestHeader::LIST as u8) => Ok(RequestHeader::LIST),
//                b if b == &(RequestHeader::Status as u8) => Ok(RequestHeader::Status),
                _ => Err(DaemonError::InvalidRequest)
            }
        } else {
            Err(DaemonError::InvalidRequest)
        }
    }

    pub fn len() -> usize {
        1 as usize
    }

    pub fn new(s: &str) -> Result<RequestHeader, DaemonError> {
        match s {
            t if t == RequestHeader::LIST.to_string() => Ok(RequestHeader::LIST),
            _ => Err(DaemonError::InvalidRequest)
        }
    }
}

impl Display for RequestHeader {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAdapter<T: Serialize>(pub T);
