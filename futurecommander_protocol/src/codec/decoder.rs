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

use bytes::{ Bytes, BytesMut };
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

impl PacketCodec {
    pub fn parse_header(index: usize, buf: &BytesMut) -> Result<Option<(usize, Header)>, ProtocolError> {
        let header_end = index + Header::length();
        if (buf.len()) >= header_end {
            if let Some(first_byte) = buf[index..header_end].first() {
                return Ok(
                    Some(
                        ( header_end, Header::parse(first_byte)? )
                    )
                );
            }
        }
        Ok(None)
    }

    pub fn parse_length(index: usize, buf: &BytesMut) -> Result<Option<(usize, u64)>, ProtocolError> {
        let u64_size = size_of::<u64>();
        let length_end = index + u64_size;
        if buf.len() >= length_end {
            let mut cursor = Cursor::new(&buf[index..length_end]);
            return Ok(
                Some(
                    ( length_end, cursor.read_u64::<NetworkEndian>().unwrap() )
                )
            );
        }
        Ok(None)
    }

    pub fn parse_packet(index: usize, buf: &BytesMut, header: Header, length: u64) -> Result<Option<(usize, Packet)>, ProtocolError> {
        let datagram_end = index + (length as usize);
        if buf.len() >= datagram_end {
            let packet = Packet::from((
                header,
                &buf[index..datagram_end]
            ));
            return Ok(Some((datagram_end, packet)));
        }
        Ok(None)
    }

    pub fn next(&mut self, buf: &BytesMut) -> Result<Option<Packet>, ProtocolError> {
        for _ in 0..3 {
            if let Some(header) = self.consumer_header {
                if let Some(length) = self.consumer_length {
                    if let Some((new_index, packet)) = Self::parse_packet(self.consumer_index, buf, header, length)? {
                        self.consumer_index = new_index;
                        self.consumer_header = None;
                        self.consumer_length = None;
                        return Ok(Some(packet));
                    }
                } else if let Some((new_index, length)) = Self::parse_length(self.consumer_index, buf)? {
                    self.consumer_index = new_index;
                    self.consumer_length = Some(length);
                }
            } else if let Some((new_index, header)) = Self::parse_header(self.consumer_index, buf)? {
                self.consumer_index = new_index;
                self.consumer_header = Some(header);
            }
        }

        Ok(None)
    }
}

impl Decoder for PacketCodec {
    type Item=Packet;
    type Error=ProtocolError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Packet>, ProtocolError> {
        self.next(buf)
    }
}

