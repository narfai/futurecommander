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

use crate::{
    errors::ProtocolError,
    message::Message,
    context::{
        ContextMessage,
        ContextContainer
    },
    Packet,
    Header,
};

use serde::{ Serialize, Deserialize };
use bincode::{ serialize };


#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryOpen {
    pub path: PathBuf
}

impl Message for DirectoryOpen {
    fn encode(&self) -> Result<Packet, ProtocolError> {
        Ok(Packet::new(Header::DirectoryOpen, serialize(&self)?))
    }

    fn header(&self) -> Header {
        Header::DirectoryOpen
    }
}

impl ContextMessage for DirectoryOpen {
    fn from_context(context: &ContextContainer) -> Result<Box<ContextMessage>, ProtocolError> where Self: Sized {
        Ok(
            Box::new(
                DirectoryOpen {
                    path: PathBuf::from(
                        context.get("path")?
                            .to_string()?
                    ),
                }
            )
        )
    }
}
