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

mod directory_read;
mod directory_open;

pub use std::{
    fmt::{ Debug }
};

pub use self::{
    directory_open::DirectoryOpen,
    directory_read::DirectoryRead
};

use tokio::{
    prelude::*,
};

pub use crate::{
    errors::DaemonError,
    State,
    Packet
};

pub trait Message : Send + Sync + Debug {
    fn encode(&self) -> Result<Packet, DaemonError>;
    fn process(&self, state: State) -> MessageStream {
        Box::new(stream::empty())
    }
}

pub type MessageStream = Box<Stream<Item=Box<Message>, Error=DaemonError> + Sync + Send>;
