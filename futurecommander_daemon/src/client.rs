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
    rc::{ Rc },
    cell:: { RefCell },
    collections::{
        vec_deque::VecDeque
    }
};

use tokio::{
    runtime::current_thread::{ Runtime, TaskExecutor },
    net::{ TcpStream, tcp::ConnectFuture },
    prelude::{
        stream::{ Stream, SplitSink, SplitStream },
        *
    },
    codec::{
        Framed
    }
};

use crate::{
    tools::parse_address,
    protocol::{
        errors::{ ProtocolError },
        message::{
            Message,
            DirectoryOpen
        },
        PacketCodec,
        Packet
    }
};

//TODO Move it to a separated crate

pub type Rx = SplitStream<Framed<TcpStream, PacketCodec>>;

type OnMessage = Rc<Fn(&Packet)>;
type ClientState = Rc<RefCell<VecDeque<Box<Message>>>>;

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
            client.connect(on_message)
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

pub struct Sender {
    state: ClientState
}

impl Sender {
    pub fn new(state: ClientState) -> Sender {
        Sender {
            state
        }
    }

    pub fn send(&self, message: Box<Message>) {
        self.state.borrow_mut().push_back(message);
        task::current().notify(); //NOTIFY WORKS
    }
}

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
        for i in 0..10 {
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
