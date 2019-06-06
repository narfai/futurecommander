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

use serde::{ Serialize, Deserialize };
use bincode::{ deserialize, serialize };

use futurecommander_filesystem::{
    SerializableEntry
};

use crate::{
    response::{
        Response,
        ResponseAdapter,
        SerializableResponse,
        EntriesResponse
    },
    errors::DaemonError
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum ResponseHeader {
    Entries,
    Error
}

//TODO Header trait
impl Eq for ResponseHeader {}

//TODO from string to string

impl ResponseHeader {
    pub fn len() -> usize {
        1 as usize
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, DaemonError> {
        if let Some(byte) = bytes.first() {
            match byte {
                b if b == &(ResponseHeader::Entries as u8) => Ok(ResponseHeader::Entries),
                b if b == &(ResponseHeader::Error as u8) => Ok(ResponseHeader::Error),
                _ => Err(DaemonError::InvalidResponse)
            }
        } else {
            Err(DaemonError::InvalidResponse)
        }
    }

    pub fn decode_adapter(self, bytes: &[u8]) -> Result<Box<SerializableResponse>, DaemonError> {
        match self {
            ResponseHeader::Entries => {
                let response: ResponseAdapter<EntriesResponse> = deserialize(bytes)?;
                Ok(response.serializable())
            },
            ResponseHeader::Error => {
                let response: ResponseAdapter<EntriesResponse> = deserialize(bytes)?;
                Ok(response.serializable())
            },
        }
    }
}

