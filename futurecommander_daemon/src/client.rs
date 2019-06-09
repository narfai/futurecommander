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
    net::{ SocketAddr },
    path::{ Path, PathBuf },
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
    }
};

use tokio::{
    io,
    net::{ TcpListener, TcpStream },
    prelude::{
        Async,
        Poll,
//        stream::{ },
        future::{ Either, ok, lazy },
        *
    },
    codec::{
        Framed
    }
};

use crate::{
    errors::DaemonError,
    message::{
        PacketCodec,
        Header,
        Message,
        DirectoryOpen,
        DirectoryRead
    }
};

pub trait Server {

}

pub trait Client {

}

pub struct Peer<T> {
    socket_address: SocketAddr,
    inner: T
}

impl <T> Peer <T> {
    pub fn new(inner: T, address: String, port: u16) -> Result<Peer<T>, DaemonError> {
        Ok(
            Peer {
                inner,
                socket_address: format!("{}:{}", address, port).parse()?
            }
        )
    }
}

pub fn send(){
    let addr = "127.0.0.1:7842".parse().unwrap();

    let client = TcpStream::connect(&addr)
        .map_err(|error| error.into())
        .and_then(|socket| {
            let framed = Framed::new(socket, PacketCodec::default());
            let (tx, rx) = framed.split();
            println!("Stream splitted");
            let message = DirectoryOpen { path: PathBuf::from("/tmp2")};
            tx
                .send(message.encode().unwrap())
                .and_then(|_| rx
                    .into_future()
                    .map_err(|(e, _)| e)
                    .and_then(|(packet, _)|{
                        if let Some(packet) = packet {
                            println!("Packet received");
                            match packet.header() {
                                Header::DirectoryRead => match packet.parse::<DirectoryRead>() {
                                    Some(message) => {
                                        println!("Parse ok ! {:?}", message.entries)
                                    },
                                    None => {
                                        println!("Unable to parse DirectoryRead message");
                                    }
                                },
                                _ => { println!("Unhandled message"); }
                            };
                        } else {
                            println!("Packet is none");
                        }
                        Ok(())
                    })
                )

        }).map_err(|error| { eprintln!("{}", error); });

    tokio::run(client);
}
