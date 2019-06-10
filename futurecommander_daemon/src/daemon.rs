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
    sync::{ Arc },
    collections::{ vec_deque::VecDeque }
};

use tokio::{
    net::{ TcpListener, TcpStream },
    prelude::*,
    codec::{ Framed }
};

use crate::{
    errors::DaemonError,
    Packet,
    State,
    Rx,
    MessageStream,
    PacketCodec
};

pub struct Daemon {
    rx: Rx,
    state: State,
    replies: VecDeque<MessageStream>
}

impl Daemon {
    pub fn new(rx: Rx, state: State) -> Daemon {
        Daemon {
            rx,
            state,
            replies: VecDeque::new(),
        }
    }

    pub fn process(state: State, socket: TcpStream){
        let (tx, rx) = Framed::new(socket, PacketCodec::default()).split();

        let task = tx
            .send_all(Daemon::new(rx, state.clone()))
            .then(|res| {
                if let Err(e) = res {
                    println!("failed to process connection; error = {:?}", e);
                }

                Ok(())
            });

        tokio::spawn(task);
    }

    pub fn listen() {
        let addr = "127.0.0.1:7842".parse().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        let mut state : State = Arc::default();
        let server = listener.incoming()
            .map_err(|e| println!("failed to accept socket; error = {:?}", e))
            .for_each(move |socket| {
                Self::process(Arc::clone(&state), socket);
                Ok(())
            });

        println!("server running on localhost:7842");

        tokio::run(server);
    }
}

impl Stream for Daemon {
    type Item = Packet;
    type Error = DaemonError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        println!("initial poll call");
        match self.rx.poll()? {
            Async::Ready(Some(packet)) => { // We got a new packet in socket
                self.replies.push_front(
                    packet.decode()?
                        .process(self.state.clone())
                )
            },
            Async::Ready(None) => {
                return Ok(Async::Ready(None)); // Client disconnected
            },
            Async::NotReady => {} // Socket is not ready
        };

        if !self.replies.is_empty() {
            let mut cleanup_ids: Vec<usize> = Vec::new();
            for (id, stream) in self.replies.iter_mut().take(10).enumerate() {
                match stream.poll()? {
                    Async::Ready(Some(reply)) => { //Message processing yield some reply
                        return Ok(Async::Ready(Some(reply.encode()?)));
                    },
                    Async::Ready(None) => { //Message processing done
                        cleanup_ids.push(id);
                    },
                    Async::NotReady => {} //Message processing still running
                };
            }

            while let Some(id) = cleanup_ids.pop() {
                self.replies.remove(id);
            }
        }

        //If still replies to process or messages to send
        if !self.replies.is_empty() {
            task::current().notify(); // Notify re-scheduling of the task
        }

        Ok(Async::NotReady)
    }
}
