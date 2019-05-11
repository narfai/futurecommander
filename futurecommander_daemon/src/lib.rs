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
    io::Write
};

use futurecommander_filesystem::{
    Container
};

pub struct Daemon<'a, O: Write + 'a, E: Write + 'a> {
    out: &'a mut O,
    err: &'a mut E,
    container: Container
}

impl <'a, O: Write + 'a, E: Write + 'a>Daemon<'a, O, E> {
    pub fn new(out: &'a mut O, err: &'a mut E) -> Daemon<'a, O, E> {
        Daemon {
            out,
            err,
            container: Container::default()
        }
    }

    pub fn run(self) {
        let stdin = std::io::stdin();
        write!(self.out, "INIT").unwrap();
        //https://stackoverflow.com/questions/53545792/reading-raw-bytes-from-standard-input-in-rust
        //bincode decoding
        //bincode encoding
        loop {
            self.out.flush().unwrap();
            let mut event = String::new();
            stdin.read_line(&mut event).unwrap();

            match event.trim() {
                "exit" => break,
                "test" => { write!(self.out, "TEST").unwrap(); },
                "test_error" => { write!(self.err, "TESTERROR").unwrap(); },
                _ => {}
            }
        }
    }
}
