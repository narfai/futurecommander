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

use std::io::{ stdin, stdout };
use std::io::Write;
use std::env;
use std::path::{ Path, PathBuf };

use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{ App, ArgMatches };

use vfs::{
    HybridFileSystem,
    Kind,
    query::{Query, StatusQuery }
};

use crate::path::absolute;
use crate::command::{ Command, CopyCommand, ListCommand, MoveCommand, NewDirectoryCommand, NewFileCommand, RemoveCommand, TreeCommand, CommandError };

pub struct Shell {
    cwd: PathBuf,
    fs: HybridFileSystem,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            cwd: env::current_dir().unwrap(),
            fs: HybridFileSystem::new(),
        }
    }

    fn send_matches(&mut self, matches: &ArgMatches) -> Result<(), CommandError>{
        match matches.subcommand() {
            ("exit", Some(_matches)) => Err(CommandError::Exit),
            ("cd",   Some(matches))  => self.cd(matches),
            ("debug_status",   Some(matches))  =>
                match matches.value_of("path") {
                    Some(string_path) => {
                        let path = absolute(self.cwd.as_path(), Path::new(string_path));
                        println!("STATUS : {:?}", StatusQuery::new(path.as_path()).retrieve(self.fs.vfs()));
                        Ok(())
                    },
                    None => Err(CommandError::InvalidCommand)
                },
            ("debug_virtual_state", Some(_matches)) => { println!("{:#?}", self.fs.vfs().virtual_state().unwrap()); Ok(()) },
            ("debug_add_state",     Some(_matches)) => { println!("{:#?}", self.fs.vfs().add_state()); Ok(()) },
            ("debug_sub_state",     Some(_matches)) => { println!("{:#?}", self.fs.vfs().sub_state()); Ok(()) },
            ("debug_transaction",   Some(_matches)) => { println!("{:#?}", self.fs.transaction()); Ok(()) },
            ("pwd",         Some(_matches)) => { println!("{}", self.cwd.to_string_lossy()); Ok(()) },
            ("reset",       Some(_matches)) => { self.fs.reset(); println!("Virtual state is now empty");  Ok(()) },
            ("ls",          Some(matches)) => Command::<ListCommand>::new(&self.cwd,matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("cp",          Some(matches)) => Command::<CopyCommand>::new(&self.cwd,matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("mv",          Some(matches)) => Command::<MoveCommand>::new(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("rm",          Some(matches)) => Command::<RemoveCommand>::new(&self.cwd,matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("mkdir",       Some(matches)) => Command::<NewDirectoryCommand>::new(&self.cwd,matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("touch",       Some(matches)) => Command::<NewFileCommand>::new(&self.cwd,matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("tree",        Some(matches)) => Command::<TreeCommand>::new(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("apply",        Some(_matches)) => self.apply(),
            _ => Err(CommandError::InvalidCommand)
        }
    }

    pub fn run_readline(&mut self) {
        let yaml = load_yaml!("clap.yml");
        let mut read_line_editor = Editor::<()>::new();
        print!(">  ");
        loop {
            let read_line = read_line_editor.readline("> ");
            match read_line {
                Ok(input) => {
                    read_line_editor.add_history_entry(input.as_ref());
                    if let Some(first_char) = input.trim().chars().next()  {
                        if first_char == '#' {
                            continue;
                        }
                    }

                    let mut argv = Vec::new();
                    argv.extend(input.trim().split(" "));

                    match App::from_yaml(yaml).get_matches_from_safe(argv) {
                        Ok(matches) =>
                            match matches.subcommand_matches("history") {
                                Some(_) => {
                                    for line in read_line_editor.history() {
                                        println!("{}", line);
                                    }
                                },
                                _ =>
                                    match self.send_matches(&matches) {
                                        Ok(_) => { /*SUCCESS*/ },
                                        Err(error) =>
                                            match error {
                                                CommandError::Exit => break,
                                                CommandError::InvalidCommand => eprintln!("{} {}", error, matches.usage()),
                                                CommandError::ArgumentMissing(command, _, _) => {
                                                    //Trick to get proper subcommand help
                                                    match App::from_yaml(yaml).get_matches_from_safe(vec![command, "--help".to_string()]) {
                                                        Ok(_) => {},
                                                        Err(error) => eprintln!("{}", error)
                                                    };
                                                },
                                                error => { eprintln!("Error : {}", error) }
                                            }
                                    }
                            }
                        Err(error) => eprintln!("Error: {}", error)
                    }
                },
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break
                }
            }
            println!("\n");
        }
    }


    pub fn run_simple(&mut self) {
        let yaml = load_yaml!("clap.yml");
        print!("> ");
        loop {
            stdout().flush().unwrap();
            let mut input = String::new();
            if let Ok(_) = stdin().read_line(&mut input) {
                println!("\n");
                let mut argv = Vec::new();
                argv.extend(input.trim().split(" "));

                match App::from_yaml(yaml).get_matches_from_safe(argv) {
                    Ok(matches) =>
                        match self.send_matches(&matches) {
                            Ok(_)      => {/*SUCCESS*/},
                            Err(error) =>
                                match error {
                                    CommandError::InvalidCommand => eprintln!("{} {}", error, matches.usage()),
                                    CommandError::ArgumentMissing(command, _, _) => {
                                        //Trick to get proper subcommand help
                                        match App::from_yaml(yaml).get_matches_from_safe(vec![command, "--help".to_string()]) {
                                            Ok(_) => {},
                                            Err(error) => eprintln!("{}", error)
                                        };
                                    },
                                    error => { eprintln!("Error : {}", error) }
                                }
                        }
                    Err(error) => eprintln!("Error: {}", error)
                }

                println!("\n");
                print!("> ");
            }
        }
    }

    fn cd(&mut self, matches: &ArgMatches<'_>) -> Result<(), CommandError> {
        match matches.value_of("path") {
            Some(string_path) => {
                let path = absolute(self.cwd.as_path(), Path::new(string_path));

                match StatusQuery::new(path.as_path()).retrieve(&self.fs.vfs()) {
                    Ok(status) =>
                        match status.into_inner().into_existing_virtual() {
                            Some(virtual_identity) =>
                                if virtual_identity.as_kind() == &Kind::Directory {
                                    self.cwd = path;
                                    Ok(())
                                } else {
                                    Err(CommandError::IsNotADirectory(path.to_path_buf()))
                                },
                            None => Err(CommandError::DoesNotExists(path.to_path_buf())),
                        },
                    Err(error) => Err(CommandError::from(error))
                }
            },
            None => Ok(())//TODO go to home directory ?
        }
    }

    fn apply(&mut self) -> Result<(), CommandError> {
        match self.fs.apply() {
            Ok(_) => Ok(()),
            Err(error) => return Err(CommandError::from(error))
        }
    }
}
