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
use std::{
    path::{ PathBuf }
};

use futurecommander_filesystem::{
    SerializableEntry,
    EntryCollection,
    Entry
};

use serde::{ Serialize, Deserialize };
use bincode::{ serialize };

use crate::{
    errors::ProtocolError,
    message::{
        Message
    },
    Packet,
    Header,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageError {
    pub message: String
}

impl Message for MessageError {
    fn encode(&self) -> Result<Packet, ProtocolError> {
        Ok(Packet::new(Header::MessageError, serialize(&self)?))
    }

    fn header(&self) -> Header {
        Header::MessageError
    }
}

impl From<ProtocolError> for MessageError {
    fn from(error: ProtocolError) -> Self {
        MessageError {
            message: error.to_string()
        }
    }
}
