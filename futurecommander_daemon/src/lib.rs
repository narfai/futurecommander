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
    io::{
        prelude::*,
        Write,
        Stdin
    },
    path::{ Path }
};

use serde::{ Serialize, Deserialize};

use bincode::{ deserialize, serialize };

use futurecommander_filesystem::{
    Container,
    ReadableFileSystem,
    tools::normalize
};

pub use futurecommander_filesystem::SerializableEntry;

mod errors;

pub type RequestId = Vec<u8>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Exit{
        id: RequestId
    },
    List {
        id: RequestId,
        path: String,
    },
    Status {
        id: RequestId,
        path: String,
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ResponseKind {
    Collection,
    Entry,
    String
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ResponseStatus {
    Success,
    Fail,
    Exit
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: RequestId,
    pub kind: ResponseKind,
    pub status: ResponseStatus,
    pub content: Vec<SerializableEntry>,
    pub error: Option<String>
}

impl Response {
    pub fn decode(payload: &[u8]) -> Result<Self, errors::DaemonError> {
        Ok(deserialize(payload)?)
    }

    pub fn encode(self) -> Result<Vec<u8>, errors::DaemonError> {
        Ok(serialize(&self)?)
    }
}

impl Request {
    pub fn process(&self, container: &mut Container) -> Result<Vec<u8>, errors::DaemonError> {
        let response = match self {
            Request::Exit { id } => {
                return Err(errors::DaemonError::Exit);
            },
            Request::List { path, id } => match container.read_dir(
                normalize(
                    Path::new(path)
                ).as_path()
            ){
                Ok(collection) =>
                    (Response {
                        id: id.clone(),
                        kind: ResponseKind::Collection,
                        status: ResponseStatus::Success,
                        content: collection
                            .into_iter()
                            .map(|entry| SerializableEntry::from(&entry))
                            .collect::<Vec<SerializableEntry>>(),
                        error: None
                    }).encode()?
                ,
                Err(error) =>
                    (Response {
                        id: id.clone(),
                        kind: ResponseKind::String,
                        status: ResponseStatus::Fail,
                        content: Vec::new(),
                        error: Some(format!("{}", errors::DaemonError::from(error)))
                    }).encode()?
            },
            Request::Status { path, id } => match container.status(
                normalize(
                    Path::new(path)
                ).as_path()
            ){
                Ok(entry) =>
                    (Response {
                        id: id.clone(),
                        kind: ResponseKind::Entry,
                        status: ResponseStatus::Success,
                        content: vec![ SerializableEntry::from(&entry) ],
                        error: None
                    }).encode()?
                ,
                Err(error) =>
                    (Response {
                        id: id.clone(),
                        kind: ResponseKind::String,
                        status: ResponseStatus::Fail,
                        content: Vec::new(),
                        error: Some(format!("{}", errors::DaemonError::from(error)))
                    }).encode()?
            }
        };
        Ok(response)
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, errors::DaemonError> {
        Ok(serialize(&self)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, errors::DaemonError> {
        Ok(deserialize(bytes)?)
    }
}

pub struct Daemon<'a, O: Write + 'a, E: Write + 'a> {
    out: &'a mut O,
    err: &'a mut E,
    container: Container
}

impl <'a, O: Write + 'a, E: Write + 'a>Daemon<'a, O, E> {
    pub fn new(out: &'a mut O, err: &'a mut E) -> Daemon<'a, O, E> {
        Daemon {
            out,
            err,
            container: Container::default()
        }
    }

    fn emit(&mut self, payload: &[u8]) -> Result<(), errors::DaemonError>{
        self.out.write(
            Request::from_bytes(payload)?
                .process(&mut self.container)?
                .as_slice()
        )?;
        Ok(())
    }

    pub fn next(&mut self, stdin: &Stdin) -> Result<(), errors::DaemonError> {
        let mut inlock = stdin.lock();

        let length = {
            self.out.flush()?;

            let buffer = inlock.fill_buf()?;

            self.emit(buffer)?;
            buffer.len()
        };

        inlock.consume(length);
        Ok(())
    }

    pub fn run(mut self) {
        let stdin = std::io::stdin();

        loop {
            let length = match self.next(&stdin) {
                Ok(_) => {},
                Err(errors::DaemonError::Exit) => {
                    break;
                }
                Err(error) => {
                    write!(self.err, "{}", error);
                    break;
                }
            };
        }
    }
}
