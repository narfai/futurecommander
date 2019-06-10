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

use tokio::{
    prelude::*
};

use futurecommander_filesystem::{
    ReadableFileSystem,
    Container
};

use crate::{
    MessageStream,
    protocol::{
        errors::{ ProtocolError },
        message::{
            Message,
            DirectoryOpen,
            DirectoryRead,
        },
        Header,
        Packet
    }
};

#[derive(Default)]
pub struct Router {
    container: Container
}

impl Router {
    fn read_dir(&mut self, message: Option<DirectoryOpen>) -> Result<Box<Message>, ProtocolError> {
        if let Some(message) = message {
            Ok(
                Box::new(
                    DirectoryRead::from(
                        self.container.read_dir(message.path.as_path())?
                    )
                )
            )
        } else {
            Err(ProtocolError::MessageParsing)
        }
    }

    pub fn process(&mut self, packet: &Packet) -> MessageStream {
        match packet.header() {
            Header::DirectoryOpen =>
                Box::new(
                stream::once(
                    self.read_dir( packet.parse::<DirectoryOpen>() )
                    )
                ),
            _ => Box::new(stream::empty())
        }
    }
}
