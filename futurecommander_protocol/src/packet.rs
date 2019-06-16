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
    mem::{ size_of }
};

use byteorder::{ NetworkEndian, WriteBytesExt };
use bytes::{ BytesMut, BufMut };
use bincode::{ deserialize };
use serde::{ Deserialize };

use crate::{
    errors::ProtocolError,
    message::{ Message },
    Header
};

pub struct Packet {
    header: Header,
    datagram: Vec<u8>
}

impl Packet {
    pub fn new(header : Header, datagram: Vec<u8>) -> Packet {
        Packet {
            header,
            datagram
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn length(&self) -> usize {
        1 + size_of::<u64>() + self.datagram.len()
    }
    pub fn decode(&self) -> Result<Box<Message>, ProtocolError> {
        Ok(self.header.parse_message(self.datagram.as_slice())?)
    }

    pub fn write(self, buf: &mut BytesMut) -> Result<(), ProtocolError> {
        let mut encoded = vec![self.header as u8];
        let mut datagram = self.datagram;
        let mut length = vec![];

        length.write_u64::<NetworkEndian>(datagram.len() as u64)?;

        encoded.append(&mut length);
        encoded.append(&mut datagram);

        buf.reserve(encoded.len());

        buf.put(encoded);
        Ok(())
    }

    pub fn parse<'a, T: Message + Deserialize<'a>>(&'a self) -> Option<T> {
        match deserialize(self.datagram.as_slice()) {
            Ok(message) => Some(message),
            Err(_) => None
        }
    }
}

impl From<(Header, &[u8])> for Packet {
    fn from(payload : (Header, &[u8])) -> Packet {
        Packet {
            header: payload.0,
            datagram: payload.1.to_vec()
        }
    }
}
