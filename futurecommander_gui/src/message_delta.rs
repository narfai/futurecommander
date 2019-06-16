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

use wasm_bindgen::{ prelude::* };

use crate::{
    errors::AddonError
};

use futurecommander_protocol::{
    Packet,
    Header,
    message::{
        DirectoryRead
    }
};

#[wasm_bindgen]
pub struct MessageDelta {

    #[wasm_bindgen(skip)]
    pub maybe_packet: Option<Packet>
}

#[wasm_bindgen]
impl MessageDelta {
    pub fn header(&self) -> JsValue {
        if let Some(packet) = &self.maybe_packet {
            match JsValue::from_serde(&packet.header()) {
                Ok(value) => value,
                Err(_) => JsValue::NULL
            }
        } else {
            JsValue::NULL
        }
    }

    pub fn len(&self) -> JsValue {
        if let Some(packet) = &self.maybe_packet {
            JsValue::from_f64(packet.length() as f64)
        } else {
            JsValue::NULL
        }
    }

    pub fn is_empty(&self) -> bool {
        self.maybe_packet.is_none()
    }

    pub fn parse(&self) -> Result<JsValue, JsValue> {
        if let Some(packet) = &self.maybe_packet {
            match packet.header() {
                Header::DirectoryRead => JsValue::from_serde(&packet.parse::<DirectoryRead>().unwrap())
                    .map_err(|error| AddonError::from(error).into())
                ,
                _ => Err(AddonError::InvalidArgument("Unsupported header".to_string()).into())
            }
        } else {
            Ok(JsValue::NULL)
        }
    }
}
