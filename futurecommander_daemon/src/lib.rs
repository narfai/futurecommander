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

mod errors;
mod request;
mod response;

pub use futurecommander_filesystem::SerializableEntry;

pub use self::{
    request::{
        Request,
        RequestHeader,
        ListRequest
    },
    response::{
        Response
    },
    errors::DaemonError
};

use std::{
    fmt::{
        Debug
    },
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
    Container
};

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

    fn emit(&mut self, payload: &[u8]) -> Result<(), DaemonError>{
        if let Some(header) = payload[..1].first() {
            match RequestHeader::from(*header) {
                RequestHeader::InvalidHeader => {},
                RequestHeader::List => {
                    self.out.write(
                        ListRequest::from_bytes(&payload[1..])?
                            .process(&mut self.container)?
                            .as_slice()
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn next(&mut self, stdin: &Stdin) -> Result<(), DaemonError> {
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
                Err(DaemonError::Exit) => {
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
