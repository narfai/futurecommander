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

mod connected;
mod connecting;
mod client;
mod sender;
pub mod tools;

pub use self::{
    client::Client,
    connected::ConnectedClient,
    connecting::ConnectingClient,
    sender::Sender
};

use std::{
    rc::{ Rc },
    cell:: { RefCell },
    collections::{
        vec_deque::VecDeque
    }
};

use tokio::{
    net::{ TcpStream },
    codec::{ Framed },
    prelude::stream::{ SplitStream }
};

use futurecommander_protocol::{
    Packet,
    PacketCodec,
    message::{ Message }
};

pub type Rx = SplitStream<Framed<TcpStream, PacketCodec>>;

type OnMessage = Rc<Fn(&Packet)>;
type ClientState = Rc<RefCell<VecDeque<Box<Message>>>>;
