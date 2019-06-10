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
    path::{ PathBuf, Path, MAIN_SEPARATOR },
    net::{ SocketAddr }
};

pub fn root_identity() -> PathBuf {
    PathBuf::from(MAIN_SEPARATOR.to_string())
}

pub fn get_parent_or_root(identity: &Path) -> PathBuf {
    match identity.parent() {
        Some(parent) => parent.to_path_buf(),
        None => root_identity()
    }
}

pub fn parse_address(address: Option<&str>, port: Option<u16>) -> SocketAddr {
    let address = address.unwrap_or("127.0.0.1");
    let port : u16 = port.unwrap_or(7842);
    format!("{}:{}", address, port).parse().unwrap()
}
