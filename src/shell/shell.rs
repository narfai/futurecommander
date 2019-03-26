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

use std::io::{stdin, stdout};
use std::io::Write;
use std::env;
use std::path::{ Path, PathBuf };

use clap::{App};

use vfs::{ VirtualFileSystem, VirtualKind };

use crate::path::absolute;
use crate::command::{ Command, CopyCommand, ListCommand, MoveCommand, NewDirectoryCommand, NewFileCommand, RemoveCommand, TreeCommand, CommandError };

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
                                ("cd", Some(matches))   => {
                                    if matches.is_present("path") {
                                        self.cd(Path::new(matches.value_of("path").unwrap()))
                                    }
                                    Ok(())
                                },
                                ("debug_virtual_state", Some(_matches))  => { println!("{:#?}", self.vfs.get_virtual_state()); Ok(()) },
                                ("debug_add_state",     Some(_matches))  => { println!("{:#?}", self.vfs.get_add_state()); Ok(()) },
                                ("debug_sub_state",     Some(_matches))  => { println!("{:#?}", self.vfs.get_sub_state()); Ok(()) }
                                (ListCommand::NAME,     Some(matches))  =>
                                    ListCommand::new(&self.cwd,matches)
                                        .and_then(|c| c.execute(&mut self.vfs)),
                                (CopyCommand::NAME,     Some(matches))  =>
                                    CopyCommand::new(&self.cwd,matches)
                                        .and_then(|c| c.execute(&mut self.vfs)),
                                (MoveCommand::NAME,     Some(matches))  =>
                                    MoveCommand::new(&self.cwd,matches)
                                        .and_then(|c| c.execute(&mut self.vfs)),
                                (RemoveCommand::NAME,    Some(matches))  =>
                                    RemoveCommand::new(&self.cwd,matches)
                                        .and_then(|c| c.execute(&mut self.vfs)),
                                (NewDirectoryCommand::NAME, Some(matches))  =>
                                    NewDirectoryCommand::new(&self.cwd,matches)
                                        .and_then(|c| c.execute(&mut self.vfs)),
                                (NewFileCommand::NAME, Some(matches))  =>
                                    NewFileCommand::new(&self.cwd,matches)
                                        .and_then(|c| c.execute(&mut self.vfs)),
                                (TreeCommand::NAME, Some(matches))  =>
                                    TreeCommand::new(&self.cwd, matches)
                                        .and_then(|c| c.execute(&mut self.vfs)),
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
                                        error => { eprintln!("{}", error) }
                                    }
                            }
                    Err(error) => eprintln!("{}", error)
                }

                println!("\n");
                print!("> ");
            }
        }
    }

    fn cd(&mut self, path: &Path) {
        let path = absolute(self.cwd.as_path(), path);
        if let Some(virtual_identity) = self.vfs.stat(path.as_path()) {
            if virtual_identity.as_kind() == &VirtualKind::Directory {
                self.cwd = path;
            } else {
                eprintln!("Error : {:?} is not a directory", path)
            }
        } else {
            eprintln!("Error : {:?} does not exists", path)
        }
    }
}


