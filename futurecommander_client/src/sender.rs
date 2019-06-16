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
    prelude::{ * }
};

use crate::{
    ClientState
};

use futurecommander_protocol::{
    message::{
        Message
    }
};

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
