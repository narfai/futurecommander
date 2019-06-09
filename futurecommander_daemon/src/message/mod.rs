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

mod header;
mod message;
mod decoder;
mod encoder;

pub use std::{
    fmt::{ Debug }
};

pub use self::{
    header::Header,
    message::*
};

use tokio::{
    net::{ TcpListener, TcpStream },
    codec::{
        Framed
    },
    prelude::{
        Async,
        Poll,
        stream::{ Stream, empty },
        future::{ Future },
        *
    },
};

pub use crate::{
    State,
    errors::{
        DaemonError
    }
};

#[derive(Default)]
pub struct MessageCodec {
    consumer_index: usize,
    consumer_header: Option<Header>,
    consumer_length: Option<u64>
}

pub trait Message : Send + Sync + Debug {
    fn encode(&self) -> Result<Vec<u8>, DaemonError>;
    fn header(&self) -> Header;
    fn process(&self, state: State) -> MessageStream {
        Box::new(empty())
    }
}

pub type MessageStream = Box<Stream<Item=Box<Message>, Error=DaemonError> + Sync + Send>;
