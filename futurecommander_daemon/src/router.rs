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
  path::{ Path, PathBuf }
};

use tokio::{
    prelude::*
};

use futurecommander_filesystem::{
    ReadableFileSystem,
    Container,
    Kind,
    CreateEvent,
    Listener,
    Delayer,
    capability::{
        RegistrarGuard
    }
};

use crate::{
    MessageStream,
    protocol::{
        errors::{ ProtocolError },
        message::{
            Message,
            DirectoryOpen,
            DirectoryRead,
            DirectoryCreate,
            MessageError
        },
        Header,
        Packet
    },
    tools
};

#[derive(Default)]
pub struct Router {
    container: Container
}

impl Router {
    fn read_dir(&mut self, path: &Path) -> Result<Box<Message>, ProtocolError> {
        Ok(
            Box::new(
                DirectoryRead::from(
                    (path.to_path_buf(), self.container.read_dir(path)?)
                )
            )
        )
    }

    fn create(&mut self, path: &Path, recursive: bool, overwrite: bool) -> Result<(), ProtocolError> {
        let event = CreateEvent::new(
            path,
            Kind::Directory,
            recursive,
            overwrite
        );

        let guard = self.container.emit(&event, RegistrarGuard::default())?;
        self.container.delay(Box::new(event), guard);
        Ok(())
    }

    pub fn process(&mut self, packet: &Packet) -> MessageStream {
        match packet.header() {
            Header::DirectoryOpen =>
                Box::new(
                    stream::once(
                        packet.parse_result::<DirectoryOpen>()
                            .and_then(|packet| self.read_dir(packet.path.as_path()))
                    )
                ),
            Header::DirectoryCreate =>
                Box::new(
                    stream::once(
                        packet.parse_result::<DirectoryCreate>()
                            .and_then(|packet| {
                                self.create(packet.path.as_path(), packet.recursive, packet.overwrite)
                                    .and_then(|_| Ok(tools::get_parent_or_root(packet.path.as_path())))
                                    .and_then(|path| self.read_dir(path.as_path()))
                                    .or_else(|error| Ok(Box::new(MessageError::from(error))))
                            })
                    )
                ),
            _ => Box::new(stream::empty())
        }
    }
}
