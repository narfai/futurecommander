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
    error::Error,
    fmt
};

#[derive(Debug)]
pub enum ContextError {
    KeyDoesNotExists(String),
    CannotCast(String, String)
}


impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::KeyDoesNotExists(key) => write!(f, "Context key {} does not exists", key),
            ContextError::CannotCast(from, to) => write!(f, "Context cannot cast {} to {}", from, to),
        }
    }
}


impl Error for ContextError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            _ => None
        }
    }
}
