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

//
//pub struct Connected {
//    tx: Tx,
//    rx: Rx
//}
//
//impl Connected {
//    pub fn new(tx: Tx, rx: Rx) -> Connected {
//        Connected {
//            tx,
//            rx
//        }
//    }
//
//    pub fn send(packet: Packet){
//
//    }
//}
//
//impl Future for Connected {
//    type Item = ();
//    type Error = DaemonError;
//
//    fn poll(&mut self) -> Poll<(), DaemonError> {
//
//        unimplemented!()
//    }
//}


//pub struct  Client {
////        on_packet: Box<Fn(Packet) + Send>,
////        priority: usize,
//    stream: Client,
//    socket_address: SocketAddr
//}

//impl Client {
//    pub fn new(address: String, port: u16, stream: Client) -> Result<Client, DaemonError> {
//        Client {
////            on_packet,
////            priority,
//            stream,
//            socket_address: format!("{}:{}", address, port).parse()?
//        }
//    }
//
//    pub fn connect(&mut self) -> Box<Future<Item = Client, Error = DaemonError>>{
////        let mut runtime = Runtime::new().unwrap();
//        TcpStream::connect(&self.socket_address)
//            .map_err(|error| DaemonError::from(error))
//            .and_then(move |socket| {
//                let framed = Framed::new(socket, PacketCodec::default());
//                let (tx, rx) = framed.split();
//                tx.send_all(self.stream)
//                    .and_then(|res| {
//                        Ok(Box::new(self.stream))
//                    })
//            })
////            .map_err(|error| { eprintln!("{}", error); });
//
////        runtime.spawn(client);
////        runtime.run().unwrap();
////
////        Ok(Client())
//    }
//}

//pub enum Client {
////    Connecting {
////        on_packet: Box<Fn(Packet) + Send>,
////        priority: usize
////    },
////    Listening {
////
////    }
//}
//
//impl Client {
//    pub fn new(on_packet: Box<Fn(Packet, priority) + Send>, priority: usize) -> Client {
//        Client::Connecting {
//            on_packet,
//            priority
//        }
//    }
//}
//
//impl Future for Client {
//    type
//}

//impl Stream for Client {
//    type Item = Packet;
//    type Error = DaemonError;
//
//    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
//        loop {
//            match self {
//                Client::Connecting { on_packet, priority } => {
//
//                }
//                Client::Listening {} => {
//                    // Keep trying to write the buffer to the socket as long as the
//                    // buffer has more bytes available for consumption
//                    while data.has_remaining() {
//                        try_ready!(socket.write_buf(data));
//                    }
//                    return Ok(Async::Ready(()));
//                }
//            }
//        }
////        for i in 0..self.priority {
////            match self.rx.poll()? {
////                Async::Ready(Some(packet)) => { // We got a new packet in input socket
////                    let callback = &self.on_packet;
////                    callback(packet)
////                },
////                Async::Ready(None) => {
////                    return Ok(Async::Ready(None)); // Client disconnected
////                },
////                Async::NotReady => {} // Socket is not ready
////            }
////        }
////
////        for i in 0..self.priority {
////            match self.out.poll()? {
////                Async::Ready(Some(message)) => { // We got a new message in output stream
////                    return Ok(Async::Ready(Some(message.encode()?)));
////                },
////                Async::Ready(None) => {
////                    return Ok(Async::Ready(None)); // Consumer disconnected
////                },
////                Async::NotReady => {} // Stream is not ready
////            }
////        }
//
//        Ok(Async::NotReady)
//    }
//}
