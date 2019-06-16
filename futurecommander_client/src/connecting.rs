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

use tokio::{
    runtime::current_thread::{ TaskExecutor },
    net::{ tcp::ConnectFuture },
    prelude::{
        stream::{ Stream },
        *
    },
    codec::{
        Framed
    }
};

use futurecommander_protocol::{
    errors::ProtocolError,
    PacketCodec
};

use crate::{
    ClientState,
    OnMessage,
    Sender,
    ConnectedClient
};

pub struct ConnectingClient {
    connect: ConnectFuture,
    state: ClientState,
    on_message: OnMessage
}

impl ConnectingClient {
    pub fn new(connect: ConnectFuture, state: ClientState, on_message: OnMessage) -> ConnectingClient {
        ConnectingClient {
            connect,
            state,
            on_message
        }
    }
}

impl Future for ConnectingClient {
    type Item = Sender;
    type Error = ProtocolError;

    fn poll(&mut self) -> Poll<Sender, ProtocolError> {
        match self.connect.poll() {
            Ok(Async::Ready(socket)) => {
                let framed = Framed::new(socket, PacketCodec::default());
                let (tx, rx) = framed.split();
                let task = tx.send_all(
                    ConnectedClient::new(
                        rx,
                        self.state.clone(),
                        self.on_message.clone()
                    )
                ).then(|res| {
                    if let Err(e) = res {
                        println!("failed to process connection; error = {:?}", e);
                    }

                    Ok(())
                });

                TaskExecutor::current()
                    .spawn_local(Box::new(task))
                    .unwrap();
                return Ok(Async::Ready(Sender::new(self.state.clone())));
            },
            Ok(Async::NotReady) => { println!("Not ready"); },
            Err(error) => { return Err(ProtocolError::from(error)) }
        }

        Ok(Async::NotReady)
    }
}
