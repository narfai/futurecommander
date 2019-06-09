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
        Message,
        State
    }
};

use tokio::{
    prelude::{
        stream::{ Stream, empty },
        future::{ Future },
        *
    },
};

use futurecommander_filesystem::{
    EntryCollection,
    SerializableEntry,
    tools::normalize,
    ReadableFileSystem,
    Entry
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryOpen {
    pub path: PathBuf
}

impl Message for DirectoryOpen {
    fn encode(&self) -> Result<Vec<u8>, DaemonError> {
        Ok(serialize(&self)?)
    }

    fn header(&self) -> Header {
        Header::DirectoryOpen
    }

    fn process(&self, state: State) ->  Result<Box<Message>, DaemonError> {
        let collection = state.lock().unwrap().read_dir(
            normalize(
                self.path.as_path()
            ).as_path()
        )?;
        Ok(Box::new(DirectoryRead::from(collection)))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryRead {
    pub entries: Vec<SerializableEntry>
}

impl Message for DirectoryRead {
    fn encode(&self) -> Result<Vec<u8>, DaemonError> {
        Ok(serialize(&self)?)
    }

    fn header(&self) -> Header {
        Header::DirectoryRead
    }

    fn process(&self, state: State) ->  Result<Box<Message>, DaemonError> {
        unimplemented!();
    }
}

impl <T: Entry>From<EntryCollection<T>> for DirectoryRead {
    fn from(collection: EntryCollection<T>) -> DirectoryRead {
        DirectoryRead {
            entries: collection
                .into_iter()
                .map(|entry| SerializableEntry::from(&entry))
                .collect::<Vec<SerializableEntry>>()
        }
    }
}
