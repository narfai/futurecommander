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

use bincode::{ deserialize, serialize };
use serde::{
    Serialize,
    Deserialize
};

use crate::{
    errors::{
        DaemonError
    },
    message::{
        Header,
        Message
    }
};

use futurecommander_filesystem::{
    SerializableEntry
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryOpen {
    pub path: PathBuf
}

impl Message for DirectoryOpen {
    fn encode(self) -> Result<Vec<u8>, DaemonError> {
        Ok(serialize(&self)?)
    }

    fn header(&self) -> Header {
        Header::DirectoryOpen
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryRead {
    pub entries: Option<Vec<SerializableEntry>>
}

impl Message for DirectoryRead {
    fn encode(self) -> Result<Vec<u8>, DaemonError> {
        Ok(serialize(&self)?)
    }

    fn header(&self) -> Header {
        Header::DirectoryRead
    }
}
