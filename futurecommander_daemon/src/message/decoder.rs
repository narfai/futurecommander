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
    io::Cursor,
    mem::{ size_of }
};
use byteorder::{ NetworkEndian, ReadBytesExt};
use bincode::{ deserialize, serialize };

use bytes::{ BytesMut };
use tokio::{
    io,
    prelude::*,
    codec::{ Decoder },
};

use crate::{
    errors::{
        DaemonError
    },
    message::{
        MessageCodec,
        Header,
        Message
    }
};

impl Decoder for MessageCodec {
    type Item=Box<Message>;
    type Error=DaemonError;

    //TODO how to handle an error ?
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Box<Message>>, DaemonError> {
        //Parse header
        if self.consumer_header.is_none() {
            if (buf.len()) >= 1 {
                if let Some(first_byte) = buf[self.consumer_index..(self.consumer_index + 1)].first() {
                    self.consumer_header = Some(Header::parse(first_byte)?);
                    self.consumer_index += 1;
                }
            }
        }

        //Parse length
        if self.consumer_length.is_none() && self.consumer_header.is_some() {
            let u64_size = size_of::<u64>();
            if (self.consumer_index + buf.len()) >= u64_size {
                let mut cursor = Cursor::new(&buf[self.consumer_index..(self.consumer_index + u64_size)]);
                self.consumer_length = Some(cursor.read_u64::<NetworkEndian>().unwrap());
                self.consumer_index += u64_size;
            }
        }

        //Parse datagram
        if let Some(header) = self.consumer_header {
            if let Some(length) = self.consumer_length {
                if ((self.consumer_index + buf.len()) as u64) >= length {
                    let message = header.parse_message(
                        &buf[self.consumer_index..(self.consumer_index + (length as usize))]
                    )?;

                    self.consumer_header = None;
                    self.consumer_length = None;
                    self.consumer_index = 0;
                    return Ok(Some(message));
                }
            }
        }

        Ok(None)
    }
}

