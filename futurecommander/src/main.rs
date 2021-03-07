// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    env,
    io::{ Write }
};

use futurecommander_shell::{ Shell };

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
    } else {
        shell.run_single(env::args(), &mut stdout, &mut stderr)
            .unwrap_or_else(|error| {
                write!(&mut stderr, "{}", error).unwrap();
            });
    };
}
