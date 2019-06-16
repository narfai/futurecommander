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
    env,
    io::{ Write }
};

use futurecommander_shell::{ Shell };
use futurecommander_daemon::{ Daemon };
use futurecommander_client::{ Client };

fn main() {
    let mut shell = Shell::default();
    let args : Vec<String> = env::args().skip(1).collect();

    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    if args.is_empty() {
        shell.run_readline(&mut stdout, &mut stderr)
            .unwrap_or_else(|error| {
                write!(&mut stderr, "{}", error).unwrap();
            });
    } else if args[0].trim() == "daemon" {
        Daemon::listen(None, None)
            .unwrap_or_else(|error| {
                write!(&mut stderr, "{}", error).unwrap();
            });
    } else if args[0].trim() == "client" {
        Client::listen(None, None)
            .unwrap_or_else(|error| {
                write!(&mut stderr, "{}", error).unwrap();
            });
    } else {
        shell.run_single(env::args(), &mut stdout, &mut stderr)
            .unwrap_or_else(|error| {
                write!(&mut stderr, "{}", error).unwrap();
            });
    };
}
