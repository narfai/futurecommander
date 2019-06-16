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
    prelude::{
        stream::{ Stream },
        *
    },
};

use futurecommander_protocol::{
    errors::{ ProtocolError },
    Packet
};

use crate::{
    Rx,
    ClientState,
    OnMessage
};

pub struct ConnectedClient {
    rx: Rx,
    state: ClientState,
    on_message: OnMessage
}

impl ConnectedClient {
    pub fn new(rx: Rx, state: ClientState, on_message: OnMessage) -> ConnectedClient {
        ConnectedClient {
            rx,
            state,
            on_message
        }
    }
}

impl Stream for ConnectedClient {
    type Item = Packet;
    type Error = ProtocolError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        for _ in 0..10 {
            match self.rx.poll()? {
                Async::Ready(Some(packet)) => { // We got a new packet in socket
                    println!("get packet");
                    self.on_message.as_ref()(&packet);
                },
                Async::Ready(None) => {
                    println!("Daemon disconnected");
                    return Ok(Async::Ready(None)); // Daemon disconnected
                },
                Async::NotReady => {} // Socket is not ready
            }
        }

        if !self.state.borrow().is_empty() {
            if let Some(message) = self.state.borrow_mut().pop_front() {
                return Ok(Async::Ready(Some(message.encode()?)));
            }
        }

        Ok(Async::NotReady)
    }
}
