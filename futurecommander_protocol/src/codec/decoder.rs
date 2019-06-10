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

use bytes::{ BytesMut };
use tokio_codec::{
    Decoder
};

use crate::{
    errors::{
        ProtocolError
    },
    PacketCodec,
    Header,
    Packet
};
// TODO problem is somehow, tokio consume the buffer but addon dont ( js has to do ) => tokio keeps up to 220K afaik
// TODO Split those 3 parsing function naivily then use it directly trough addon bounday
impl Decoder for PacketCodec {
    type Item=Packet;
    type Error=ProtocolError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Packet>, ProtocolError> {
        println!("BUFFER SIZE {}", buf.len());
        //Parse header
        if self.consumer_header.is_none() {
            let header_pos = self.consumer_index + 1;
            if (buf.len()) >= header_pos {
                if let Some(first_byte) = buf[self.consumer_index..header_pos].first() {
                    self.consumer_header = Some(Header::parse(first_byte)?);
                    self.consumer_index += 1;
                }
            }
        }

        //Parse length
        if self.consumer_length.is_none() && self.consumer_header.is_some() {
            let u64_size = size_of::<u64>();
            let length_pos = self.consumer_index + u64_size;
            if buf.len() >= length_pos {
                let mut cursor = Cursor::new(&buf[self.consumer_index..length_pos]);
                self.consumer_length = Some(cursor.read_u64::<NetworkEndian>().unwrap());
                self.consumer_index += u64_size;
            }
        }

        //Parse datagram
        if let Some(header) = self.consumer_header {
            if let Some(length) = self.consumer_length {
                let datagram_pos = self.consumer_index + (length as usize);
                if buf.len() >= datagram_pos {
                    let packet = Packet::from((
                        header,
                        &buf[self.consumer_index..datagram_pos]
                    ));

                    self.consumer_header = None;
                    self.consumer_length = None;
                    self.consumer_index += length as usize;
                    return Ok(Some(packet));
                }
            }
        }

        Ok(None)
    }
}

