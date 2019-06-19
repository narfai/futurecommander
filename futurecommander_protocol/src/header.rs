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

use crate::{
    errors::{ ProtocolError },
    context::{ ContextMessage, ContextContainer },
    message::{
        Message,
        DirectoryOpen,
        DirectoryRead,
        DirectoryCreate,
        FileCreate,
        EntryRemove,
        EntryCopy,
        MessageError
    }
};

use bincode::{ deserialize };
use serde::{ Serialize, Deserialize };

use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    },
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Header {
    DirectoryOpen,
    DirectoryRead,
    DirectoryCreate,
    FileCreate,
    EntryRemove,
    EntryCopy,
    MessageError
}

impl Header {
    pub fn new(s: &str) -> Result<Self, ProtocolError> {
        match s {
            t if t == Header::DirectoryOpen.to_string() => Ok(Header::DirectoryOpen),
            t if t == Header::DirectoryRead.to_string() => Ok(Header::DirectoryRead),
            t if t == Header::DirectoryCreate.to_string() => Ok(Header::DirectoryCreate),
            t if t == Header::FileCreate.to_string() => Ok(Header::FileCreate),
            t if t == Header::EntryRemove.to_string() => Ok(Header::EntryRemove),
            t if t == Header::EntryCopy.to_string() => Ok(Header::EntryCopy),
            t if t == Header::MessageError.to_string() => Ok(Header::MessageError),
            _ => Err(ProtocolError::InvalidHeader)
        }
    }

    pub fn parse(byte: u8) -> Result<Self, ProtocolError> {
        match byte {
            b if b == (Header::DirectoryOpen as u8) => Ok(Header::DirectoryOpen),
            b if b == (Header::DirectoryRead as u8) => Ok(Header::DirectoryRead),
            b if b == (Header::DirectoryCreate as u8) => Ok(Header::DirectoryCreate),
            b if b == (Header::FileCreate as u8) => Ok(Header::FileCreate),
            b if b == (Header::EntryRemove as u8) => Ok(Header::EntryRemove),
            b if b == (Header::EntryCopy as u8) => Ok(Header::EntryCopy),
            b if b == (Header::MessageError as u8) => Ok(Header::MessageError),
            _ => Err(ProtocolError::InvalidHeader)
        }
    }

    pub const fn length() -> usize {
        1 as usize
    }

    pub fn parse_context(self, context: &ContextContainer) -> Result<Box<ContextMessage>, ProtocolError> {
        match self {
            Header::DirectoryOpen => Ok(DirectoryOpen::from_context(&context)?),
            Header::DirectoryCreate => Ok(DirectoryCreate::from_context(&context)?),
            Header::FileCreate => Ok(FileCreate::from_context(&context)?),
            Header::EntryRemove => Ok(EntryRemove::from_context(&context)?),
            Header::EntryCopy => Ok(EntryCopy::from_context(&context)?),
            _ => Err(ProtocolError::InvalidHeader)
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
