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

mod decoder;
mod encoder;

use crate::{
    Header
};

#[derive(Default)]
pub struct PacketCodec {
    consumer_index: usize,
    consumer_header: Option<Header>,
    consumer_length: Option<u64>
}

impl PacketCodec {
    pub fn index(&self) -> usize {
        self.consumer_index
    }

    pub fn header(&self) -> Option<Header> {
        self.consumer_header
    }

    pub fn length(&self) -> Option<u64> {
        self.consumer_length
    }
}