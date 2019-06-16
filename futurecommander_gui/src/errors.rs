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

use wasm_bindgen::{
    prelude::*
};

use futurecommander_protocol::{
    errors::{ ProtocolError }
};

use std::{
    error,
    fmt
};

#[derive(Debug)]
pub enum AddonError {
    InvalidRequest(String),
    InvalidArgument(String),
    JsonError(serde_json::Error),
    Protocol(ProtocolError),
    JsValue(JsValue)
}

impl From<serde_json::Error> for AddonError {
    fn from(error: serde_json::Error) -> Self {
        AddonError::JsonError(error)
    }
}

impl fmt::Display for AddonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddonError::InvalidRequest(rtype) => write!(f, "Invalid request type {}", rtype),
            AddonError::InvalidArgument(arg) => write!(f, "Invalid request argument {}", arg),
            AddonError::JsonError(error) => write!(f, "Json error {}", error),
            AddonError::Protocol(error) => write!(f, "Protocol error : {}", error),
            AddonError::JsValue(value) => write!(f, "JsValue : {:?}", value.as_string()),
        }
    }
}

impl From<AddonError> for JsValue {
    fn from(error: AddonError) -> Self {
        JsValue::from_str(format!("{}", error).as_str())
    }
}

impl From<JsValue> for AddonError {
    fn from(error: JsValue) -> Self {
        AddonError::JsValue(error)
    }
}

impl From<ProtocolError> for AddonError {
    fn from(error: ProtocolError) -> Self {
        AddonError::Protocol(error)
    }
}

impl error::Error for AddonError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            AddonError::Protocol(err) => Some(err),
            AddonError::JsonError(err) => Some(err),
            _ => None
        }
    }
}
