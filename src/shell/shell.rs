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

use futurecommandervfs::{VirtualFileSystem, VirtualKind};

use crate::path::absolute;
use crate::operation::{ Operation, CopyOperation, ListOperation, MoveOperation, NewDirectoryOperation, NewFileOperation, RemoveOperation };

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
        print!("> ");
        loop {
            stdout().flush().unwrap();
            let mut input = String::new();
            if let Ok(_) = stdin().read_line(&mut input) {
                println!("\n");
                let mut argv = vec!["futurecommander"];
                argv.extend(input.trim().split(" "));

                if self.send(argv).is_none() {
                    break;
                }

                println!("\n");
                print!("> ");
            }
        }
    }

    pub fn send(&mut self, argv: Vec<&str>) -> Option<()> {
        let yaml = load_yaml!("clap.yml");

        match App::from_yaml(yaml).get_matches_from_safe(argv) {
            Ok(matches) => {
                if let Some(_) = matches.subcommand_matches("exit") {
                    return None;
                } else if let Some(matches) = matches.subcommand_matches("cd") {
                    let path = absolute(self.cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
                    let state = self.vfs.get_virtual_state();
                    if let Some(virtual_identity) = self.vfs.stat(path.as_path()) {
                        if virtual_identity.as_kind() == &VirtualKind::Directory {
                            self.cwd = path;
                        } else {
                            println!("Error : {:?} is not a directory", path)
                        }
                    } else {
                        println!("Error : {:?} does not exists", path)
                    }
                } else if matches.subcommand_matches("debug_virtual_state").is_some() {
                    println!("{:#?}", self.vfs.get_virtual_state());
                } else if matches.subcommand_matches("debug_add_state").is_some() {
                    println!("{:#?}", self.vfs.get_add_state());
                } else if matches.subcommand_matches("debug_sub_state").is_some() {
                    println!("{:#?}", self.vfs.get_sub_state());
                } else if let Some(matches) = matches.subcommand_matches("cp") {
                    CopyOperation::from_context(self.cwd.as_path(), &matches)
                        .execute(&mut self.vfs);
                } else if let Some(matches) = matches.subcommand_matches("ls") {
                    ListOperation::from_context(self.cwd.as_path(), &matches)
                        .execute(&mut self.vfs);
                } else if let Some(matches) = matches.subcommand_matches("mv") {
                    MoveOperation::from_context(self.cwd.as_path(), &matches)
                        .execute(&mut self.vfs);
                } else if let Some(matches) = matches.subcommand_matches("mkdir") {
                    NewDirectoryOperation::from_context(self.cwd.as_path(), &matches)
                        .execute(&mut self.vfs);
                } else if let Some(matches) = matches.subcommand_matches("touch") {
                    NewFileOperation::from_context(self.cwd.as_path(), &matches)
                        .execute(&mut self.vfs);
                } else if let Some(matches) = matches.subcommand_matches("rm") {
                    RemoveOperation::from_context(self.cwd.as_path(), &matches)
                        .execute(&mut self.vfs);
                } else {
                    println!("No such command");
                }
                //TODO rename ? tree ?
            },
            Err(error) => {
                println!("{}", error);
            }
        }
        Some(())
    }
}
