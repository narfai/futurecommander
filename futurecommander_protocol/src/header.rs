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

use crate::{
    errors::{ ProtocolError },
    message::{
        Message,
        DirectoryOpen,
        DirectoryRead
    }
};

use bincode::{ deserialize };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Header {
    DirectoryOpen,
    DirectoryRead
}

impl Header {
    pub fn parse(byte: &u8) -> Result<Self, ProtocolError> {
        match byte {
            b if b == &(Header::DirectoryOpen as u8) => Ok(Header::DirectoryOpen),
            b if b == &(Header::DirectoryRead as u8) => Ok(Header::DirectoryRead),
            _ => Err(ProtocolError::InvalidHeader)
        }
    }

    pub fn parse_message(self, datagram: &[u8]) -> Result<Box<dyn Message>, ProtocolError> {
        match self {
            Header::DirectoryOpen => {
                let message: DirectoryOpen = deserialize(datagram)?;
                Ok(Box::new(message))
            },
            Header::DirectoryRead => {
                let message: DirectoryRead = deserialize(datagram)?;
                Ok(Box::new(message))
            }
        }
    }
}
