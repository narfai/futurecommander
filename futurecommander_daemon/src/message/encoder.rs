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

use bincode::{ deserialize, serialize };

use byteorder::{ NetworkEndian, WriteBytesExt };
use bytes::{ BytesMut, BufMut };
use tokio::{
    io,
    prelude::*,
    codec::{ Encoder },
};

use crate::{
    errors::{
        DaemonError
    },
    message::{
        MessageCodec,
        Message
    }
};

impl Encoder for MessageCodec {
    type Item=Box<Message>;
    type Error=DaemonError;

    fn encode(&mut self, message: Box<Message>, buf: &mut BytesMut) -> Result<(), DaemonError> {
        let mut encoded = vec![message.header() as u8];
        let mut datagram = message.encode()?;
        let mut length = vec![];

        length.write_u64::<NetworkEndian>(datagram.len() as u64).unwrap();

        encoded.append(&mut length);
        encoded.append(&mut datagram);

        buf.reserve(encoded.len());

        buf.put(encoded);

        Ok(())
    }
}
