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
mod context;

pub use futurecommander_filesystem::SerializableEntry;

pub use self::{
    request::{
        Request,
        RequestHeader,
        RequestAdapter
    },
    response::{
        Response,
        ResponseKind,
        ResponseStatus
    },
    context::{
        ContextType,
        Context
    },
    errors::DaemonError
};

use std::{
    io::{
        prelude::*,
        Write,
        Stdin
    }
};

use futurecommander_filesystem::{
    Container
};

pub struct Daemon<'a, O, E>
    where O: Write + 'a,
          E: Write + 'a {

    out: &'a mut O,
    err: &'a mut E,
    container: Container
}

impl <'a, O, E>Daemon<'a, O, E>
    where O: Write + 'a,
          E: Write + 'a {

    pub fn new(out: &'a mut O, err: &'a mut E) -> Daemon<'a, O, E> {
        Daemon {
            out,
            err,
            container: Container::default()
        }
    }

    fn emit(&mut self, payload: &[u8]) -> Result<(), DaemonError>{
        let response = RequestHeader::parse(&payload[..RequestHeader::len()])?
            .decode_adapter(&payload[RequestHeader::len()..])?
            .process(&mut self.container)?;

        self.out.write_all(&response)?;
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
            match self.next(&stdin) {
                Ok(_) => {},
                Err(DaemonError::Exit) => {
                    break;
                }
                Err(error) => {
                    write!(self.err, "{}", error).unwrap();
                    break;
                }
            };
        }
    }
}
