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
        State,
        MessageStream,
        Packet
    }
};

use tokio::{
    prelude::{
        stream::{ Stream, once },
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
        once(
            read_dir(state, self.path.as_path())
            )
            //DOES WORKS !
    //        iter_result(vec![
//                    read_dir(state.clone(), self.path.as_path()),
//                    read_dir(state, Path::new("/tmp2")),
//                ]
//            )
        )
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryRead {
    pub entries: Vec<SerializableEntry>
}

impl Message for DirectoryRead {
    fn encode(&self) -> Result<Packet, DaemonError> {
        Ok(Packet::new(Header::DirectoryRead, serialize(&self)?))
    }

    //TODO process could be stream for client ?
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
//
//impl From<Packet> for Option<DirectoryRead> {
//    fn from(packet: Packet) -> Option<DirectoryOpen> {
//
//    }
//}
