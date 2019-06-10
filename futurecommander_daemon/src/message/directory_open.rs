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
    path::{ PathBuf, Path }
};

use crate::{
    errors::DaemonError,
    State,
    Message,
    MessageStream,
    DirectoryRead,
    Packet,
    Header,
};

use tokio::{
    prelude::*
};

use futurecommander_filesystem::{
    ReadableFileSystem
};

use serde::{ Serialize, Deserialize };
use bincode::{ serialize };


#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryOpen {
    pub path: PathBuf
}

impl Message for DirectoryOpen {
    fn encode(&self) -> Result<Packet, DaemonError> {
        Ok(Packet::new(Header::DirectoryOpen, serialize(&self)?))
    }

    fn process(&self, state: State) -> MessageStream {
        fn read_dir(state: State, path: &Path) -> Result<Box<Message>, DaemonError> {
            Ok(
                Box::new(
                    DirectoryRead::from(
                        state.lock().unwrap().read_dir(path)?
                    )
                )
            )
        }

        Box::new(
            stream::once(
                read_dir(state, self.path.as_path())
            )
        )
    }
}
