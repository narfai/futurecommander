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

pub mod errors;
pub mod context;
pub mod request;
pub mod response;
pub mod tools;

pub mod message;// /!\ Highly experimental

pub use futurecommander_filesystem::SerializableEntry;

use std::{
    io::{
        prelude::*,
        Write,
        Stdin,
        Stdout,
        Stderr
    },
    sync::{
        Arc,
        Mutex
    },
    thread::{ spawn }
};

use tokio::{
    io,
    net::{ TcpStream },
    prelude::*,
    codec::{
        Framed
    }
};

use self::{
    request::RequestHeader,
    response::{
        Response,
        ErrorResponse,
        ResponseHeader,
        ResponseAdapter,
        ResponseStatus
    },
    errors::DaemonError,
    message::{
        MessageCodec
    }
};

use futurecommander_filesystem::{
    Container
};

pub struct Daemon<'a> {

    out: &'a mut Stdout,
    err: &'a mut Stderr,
    container: Container
}

impl <'a>Daemon<'a> {

    pub fn new(out: &'a mut Stdout, err: &'a mut Stderr) -> Daemon<'a> {
        Daemon {
            out,
            err,
            container: Container::default()
        }
    }

    fn process_request(payload: &[u8], container: &mut Container) -> Result<Box<Response>, DaemonError> {
        let request = RequestHeader::parse(&payload[..RequestHeader::len()])?
            .decode_adapter(&payload[RequestHeader::len()..])?;

        match request.process(container) {
            Ok(response) => Ok(response),
            Err(error) => Ok(
                Box::new(
                    ResponseAdapter::new(
                        request.id(),
                        ResponseStatus::Fail,
                        ResponseHeader::Error,
                        ErrorResponse::from(&error)
                    )
                )
            )
        }
    }

    fn write_response(&mut self, response: Box<Response>) -> Result<(), DaemonError> {
        unimplemented!()
    }

    fn emit(&mut self, payload: &[u8], ) -> Result<(), DaemonError>{
        unimplemented!()
    }

    pub fn next(&mut self, stdin: &Stdin) -> Result<(), DaemonError> {
        let mut inlock = stdin.lock();
        let mut outlock = self.out.lock();

        let length = {
            let buffer = inlock.fill_buf()?;
            let response = Self::process_request(buffer, &mut self.container)?;
            let binary_response = &response.encode()?;
            outlock.write_all(binary_response)?;
            outlock.flush()?;
            buffer.len()
        };

        inlock.consume(length);
        Ok(())
    }

    pub fn listen(mut self) {
        let addr = "127.0.0.1:7842".parse().unwrap();

        let mut container : Arc<Mutex<Container>> = Arc::default();
        let server = TcpStream::connect(&addr)
            .and_then( move |socket|{
                let framed = Framed::new(socket, MessageCodec::default());

                framed.for_each(move |message| {
                    let test = &mut container;
                    Ok(())
                }).map_err(|error| { error.into() })
            }).map_err(|error| { eprintln!("Server error {}", error); });

        println!("server running on localhost:6142");

        tokio::run(server);
    }

    pub fn run(mut self) {
        let stdin = std::io::stdin();

        loop {
            self.out.flush().unwrap();
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
