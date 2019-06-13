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

use wasm_bindgen::{ prelude::* };

use std::{
    path::{ PathBuf }
};

use bytes::{
    BytesMut
};

use futurecommander_protocol::{
    PacketCodec,
    message::{
        Message,
        DirectoryOpen,
        DirectoryRead
    },
};

use crate::{
    errors::AddonError,
    MessageDelta
};

#[wasm_bindgen]
pub struct ProtocolCodec {
    codec : PacketCodec
}

#[wasm_bindgen]
impl ProtocolCodec {

    #[wasm_bindgen(constructor)]
    pub fn new() -> ProtocolCodec {
        ProtocolCodec {
            codec: PacketCodec::default()
        }
    }

    pub fn read_dir(&self) -> Result<Box<[u8]>, JsValue> {
        // TODO encode with context as previous poc
        let message = DirectoryOpen { path: PathBuf::from("/tmp2") };
        message.encode()
            .and_then(|packet| {
                let mut buffer = BytesMut::new();
                packet.write(&mut buffer)
                    .and_then(|_|
                        Ok(buffer.freeze().to_vec())
                    )
            })
            .map_err(|error| AddonError::from(error).into())
            .map(|raw| raw.into_boxed_slice())
    }

    pub fn decode(&mut self, read_buffer: &[u8]) -> Result<MessageDelta, JsValue> {
        let mut codec = PacketCodec::default();
        codec.next(&mut BytesMut::from(read_buffer))
            .map_err(|error| AddonError::from(error).into())
            .and_then(|maybe_packet| Ok(MessageDelta { maybe_packet }))
    }
}
