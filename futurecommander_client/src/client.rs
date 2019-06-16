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
    path::{ PathBuf },
    rc::{ Rc }
};

use tokio::{
    runtime::current_thread::{ Runtime },
    net::{ TcpStream },
    prelude::{ * }
};

use crate::{
    OnMessage,
    ClientState,
    ConnectingClient,
    tools::parse_address
};

use futurecommander_protocol::{
    errors::ProtocolError,
    message::{
        DirectoryOpen
    }
};

pub struct Client {
    socket_address: SocketAddr,
    state: ClientState
}

impl Client {
    pub fn new(socket_address: SocketAddr) -> Client {
        Client {
            socket_address,
            state: Rc::default()
        }
    }

    pub fn connect(&self, on_message: OnMessage) -> ConnectingClient {
        ConnectingClient::new(
            TcpStream::connect(&self.socket_address),
            self.state.clone(),
            on_message
        )
    }

    pub fn listen(address: Option<&str>, port: Option<u16>) -> Result<(), ProtocolError> {
        let mut runtime = Runtime::new().unwrap();

        let on_message : OnMessage = Rc::new(|packet| {
            println!("{:?}", packet.decode());
        });

        let client = Client::new(parse_address(address, port));

        runtime.spawn(
            client.connect(on_message) // TODO allow consumer to route I/O however he wants
                .and_then(|sender| {
                    //Here would need a lazy future to allow js to send whatever he wants
                    sender.send(Box::new(DirectoryOpen { path: PathBuf::from("/tmp2")}));
                    sender.send(Box::new(DirectoryOpen { path: PathBuf::from("/home/narfai")}));
                    sender.send(Box::new(DirectoryOpen { path: PathBuf::from("/home/narfai/tmp")}));
                    Ok(())
                })
                .map_err(|err| { eprintln!("{}", err ); })
        );

        runtime.run().unwrap();

        Ok(())
    }
}
