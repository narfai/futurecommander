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
use rustyline::config::OutputStreamType;
use rustyline::{ CompletionType, Config, EditMode, Editor };

use clap::{ App, ArgMatches };

use file_system::{
    Container,
    Kind,
    ReadableFileSystem,
    tools::{ absolute }
};

use crate::command::*;
use crate::helper::VirtualHelper;


pub struct Shell {
    cwd: PathBuf,
    fs: Container,
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            cwd: env::current_dir().unwrap(),
            fs: Container::new(),
        }
    }
}


impl Shell {
    fn send_matches(&mut self, matches: &ArgMatches) -> Result<(), CommandError>{
        match matches.subcommand() {
            ("exit", Some(_matches)) => Err(CommandError::Exit),
            ("cd",   Some(matches))  => self.cd(matches),
            ("debug_status",   Some(matches))  =>
                match matches.value_of("path") {
                    Some(string_path) => {
                        let path = absolute(self.cwd.as_path(), Path::new(string_path));
                        println!("STATUS : {:?}", self.fs.status(path.as_path())?);
                        Ok(())
                    },
                    None => Err(CommandError::InvalidCommand)
                },
            ("debug_virtual_state", Some(_matches)) => unimplemented!(),
            ("debug_add_state",     Some(_matches)) => unimplemented!(),
            ("debug_sub_state",     Some(_matches)) => unimplemented!(),
            ("debug_transaction",   Some(_matches)) => unimplemented!(),
            ("pwd",         Some(_matches)) => { println!("{}", self.cwd.to_string_lossy()); Ok(()) },
            ("reset",       Some(_matches)) => { self.fs.reset(); println!("Virtual state is now empty");  Ok(()) },
            ("ls",          Some(matches)) => Command::<ListCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("cp",          Some(matches)) => Command::<CopyCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("mv",          Some(matches)) => Command::<MoveCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("rm",          Some(matches)) => Command::<RemoveCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("mkdir",       Some(matches)) => Command::<NewDirectoryCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("touch",       Some(matches)) => Command::<NewFileCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("tree",        Some(matches)) => Command::<TreeCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("save",        Some(matches)) => Command::<SaveCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("import",        Some(matches)) => Command::<ImportCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.fs)),
            ("apply",        Some(_matches)) => self.apply(),
            _ => Err(CommandError::InvalidCommand)
        }
    }

    pub fn run_readline(&mut self) {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .output_stream(OutputStreamType::Stdout)
            .build();

        let yaml = load_yaml!("clap.yml");
        let mut history : Vec<String> = Vec::new();

        loop {
            let mut read_line_editor = Editor::with_config(config);
            for line in history.iter() { //@TODO find a better way
                read_line_editor.add_history_entry(line.to_string());
            }
            read_line_editor.set_helper(Some(VirtualHelper::new(&self.fs, self.cwd.to_path_buf())));
            let read_line = read_line_editor.readline(">> ");

            match read_line {
                Ok(input) => {
                    history.push(input.clone());
                    if let Some(first_char) = input.trim().chars().next()  {
                        if first_char == '#' {
                            continue;
                        }
                    }

                    let mut argv = Vec::new();
                    argv.extend(input.trim().split(' '));

                    match App::from_yaml(yaml).get_matches_from_safe(argv) {
                        Ok(matches) =>
                            match matches.subcommand_matches("history") {
                                Some(_) => {
                                    for line in history.iter() {
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
            println!();
        }
    }


    pub fn run_simple(&mut self) {
        let yaml = load_yaml!("clap.yml");
        let mut history : Vec<String> = Vec::new();
        print!("> ");
        loop {
            stdout().flush().unwrap();
            let mut input = String::new();
            if stdin().read_line(&mut input).is_ok() {
                println!();

                let trimmed = input.trim().to_string();
                history.push(trimmed.clone());

                let mut argv = Vec::new();
                argv.extend(trimmed.split(' '));

                if let Some(first_char) = trimmed.chars().next()  {
                    if first_char == '#' {
                        continue;
                    }
                }

                match App::from_yaml(yaml).get_matches_from_safe(argv) {
                    Ok(matches) =>
                        match matches.subcommand_matches("history") {
                            Some(_) => {
                                for line in history.iter() {
                                    println!("{}", line);
                                }
                            },
                            _ =>
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
                        }

                    Err(error) => eprintln!("Error: {}", error)
                }

                println!();
                print!("> ");
            }
        }
    }

    fn cd(&mut self, matches: &ArgMatches<'_>) -> Result<(), CommandError> {
        match matches.value_of("path") {
            Some(string_path) => {
                let path = absolute(self.cwd.as_path(), Path::new(string_path));

                match self.fs.status(path.as_path())?.into_inner().into_existing_virtual() {
                    Some(virtual_identity) =>
                        if virtual_identity.as_kind() == &Kind::Directory {
                            self.cwd = path;
                            Ok(())
                        } else {
                            Err(CommandError::IsNotADirectory(path.to_path_buf()))
                        },
                    None => Err(CommandError::DoesNotExists(path.to_path_buf())),
                }
            },
            None => Ok(())//TODO go to home directory ?
        }
    }

    fn apply(&mut self) -> Result<(), CommandError> {
        match self.fs.apply() {
            Ok(_) => Ok(()),
            Err(error) => Err(CommandError::from(error))
        }
    }
}
