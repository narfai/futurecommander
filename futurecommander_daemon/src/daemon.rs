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
    sync::{ Arc, Mutex },
    collections::{ vec_deque::VecDeque }
};

use tokio::{
    net::{ TcpListener, TcpStream },
    prelude::*,
    codec::{ Framed }
};

use crate::{
    Router,
    Rx,
    MessageStream,
    tools::parse_address,
    protocol::{
        errors::{ ProtocolError },
        Packet,
        PacketCodec,
    }
};

pub struct Daemon {
    rx: Rx,
    router: Arc<Mutex<Router>>,
    replies: VecDeque<MessageStream>,
    tx_count: u128,
    rx_count: u128,
}

impl Daemon {
    pub fn new(rx: Rx) -> Daemon {
        Daemon {
            rx,
            router: Arc::default(),
            replies: VecDeque::new(),
            tx_count: 0,
            rx_count: 0,
        }
    }

    pub fn process(socket: TcpStream){
        let (tx, rx) = Framed::new(socket, PacketCodec::default()).split();

        let task = tx
            .send_all(Daemon::new(rx))
            .then(|res| {
                if let Err(e) = res {
                    println!("failed to process connection; error = {:?}", e);
                }

                Ok(())
            });

        tokio::spawn(task);
    }

    pub fn listen(address: Option<&str>, port: Option<u16>) -> Result<(), ProtocolError> {
        let listener = TcpListener::bind(&parse_address(address, port))?;

        let server = listener.incoming()
            .map_err(|e| println!("failed to accept socket; error = {:?}", e))
            .for_each(move |socket| {
                Self::process(socket);
                Ok(())
            });

        println!("server running on localhost:7842");

        tokio::run(server);

        Ok(())
    }
}

impl Stream for Daemon {
    type Item = Packet;
    type Error = ProtocolError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.rx.poll()? {
            Async::Ready(Some(packet)) => { // We got a new packet in socket
                self.rx_count += 1;
                println!("INCOMING MESSAGE #{} {:?}", self.rx_count, packet.header());
                self.replies.push_front(
                    self.router
                            .clone()
                            .lock()
                            .unwrap()
                            .process(&packet)
                )
            },
            Async::Ready(None) => {
                return Ok(Async::Ready(None)); // Client disconnected
            },
            Async::NotReady => {} // Socket is not ready
        };

        if !self.replies.is_empty() {
            let mut cleanup_ids: Vec<usize> = Vec::new();
            for (id, stream) in self.replies.iter_mut().take(100).enumerate() {
                match stream.poll()? {
                    Async::Ready(Some(reply)) => { //Message processing yield some reply
                        self.tx_count += 1;
                        println!("REPLY MESSAGE #{} {:?}", self.tx_count, reply.header());
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
