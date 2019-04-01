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

use clap::{ App, ArgMatches };

use vfs::{VirtualFileSystem, VirtualKind, ReadQuery, Virtual, StatusQuery, IdentityStatus };

use crate::path::absolute;
use crate::command::{ Command, CopyCommand, ListCommand, MoveCommand, NewDirectoryCommand, NewFileCommand, RemoveCommand, TreeCommand, ApplyCommand, CommandError };

pub struct Shell {
    cwd: PathBuf,
    vfs: VirtualFileSystem
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            cwd: env::current_dir().unwrap(),
            vfs: VirtualFileSystem::new()
        }
    }

    pub fn run(&mut self) {
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
                        match
                            match matches.subcommand() {
                                ("exit", Some(_matches)) => break,
                                ("cd",   Some(matches))  => self.cd(matches),
                                ("debug_status",   Some(matches))  =>
                                    match matches.value_of("path") {
                                        Some(string_path) => {
                                            let path = absolute(self.cwd.as_path(), Path::new(string_path));
                                            println!("STATUS : {:?}", Virtual(StatusQuery::new(path.as_path())).retrieve(&self.vfs));
                                            Ok(())
                                        },
                                        None => Err(CommandError::InvalidCommand)
                                    },
                                ("debug_virtual_state", Some(_matches)) => { println!("{:#?}", self.vfs.virtual_state().unwrap()); Ok(()) },
                                ("debug_add_state",     Some(_matches)) => { println!("{:#?}", self.vfs.add_state()); Ok(()) },
                                ("debug_sub_state",     Some(_matches)) => { println!("{:#?}", self.vfs.sub_state()); Ok(()) },
                                ("pwd",         Some(_matches)) => { println!("{}", self.cwd.to_string_lossy()); Ok(()) },
                                ("reset",       Some(_matches)) => { self.vfs.reset(); println!("Virtual state is now empty");  Ok(()) },
                                ("ls",          Some(matches)) => ListCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
                                ("cp",          Some(matches)) => CopyCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
                                ("mv",          Some(matches)) => MoveCommand::new(&self.cwd, matches).and_then(|c| c.execute(&mut self.vfs)),
                                ("rm",          Some(matches)) => RemoveCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
                                ("mkdir",       Some(matches)) => NewDirectoryCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
                                ("touch",       Some(matches)) => NewFileCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
                                ("tree",        Some(matches)) => TreeCommand::new(&self.cwd, matches).and_then(|c| c.execute(&mut self.vfs)),
                                ("apply",        Some(matches)) => ApplyCommand::new(&self.cwd, matches).and_then(|c| c.execute(&mut self.vfs)),
                                //Find out why this const / match syntax is invalid for webstorm
//                                (ListCommand::NAME,         Some(matches)) => ListCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)), //TODO create command!(MyCommand)
//                                (CopyCommand::NAME,         Some(matches)) => CopyCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
//                                (MoveCommand::NAME,         Some(matches)) => MoveCommand::new(&self.cwd, matches).and_then(|c| c.execute(&mut self.vfs)),
//                                (RemoveCommand::NAME,       Some(matches)) => RemoveCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
//                                (NewDirectoryCommand::NAME, Some(matches)) => NewDirectoryCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
//                                (NewFileCommand::NAME,      Some(matches)) => NewFileCommand::new(&self.cwd,matches).and_then(|c| c.execute(&mut self.vfs)),
//                                (TreeCommand::NAME,         Some(matches)) => TreeCommand::new(&self.cwd, matches).and_then(|c| c.execute(&mut self.vfs))
                                _ => Err(CommandError::InvalidCommand)
                            }
                            {
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

    fn cd(&mut self, matches: &ArgMatches) -> Result<(), CommandError> {
        match matches.value_of("path") {
            Some(string_path) => {
                let path = absolute(self.cwd.as_path(), Path::new(string_path));

                match Virtual(StatusQuery::new(path.as_path())).retrieve(&self.vfs) {
                    Ok(status) =>
                        match status.virtual_identity() {
                            Some(virtual_identity) =>
                                if virtual_identity.as_kind() == &VirtualKind::Directory {
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
}


