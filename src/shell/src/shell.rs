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
    env::{ current_dir },
    path::{ Path, PathBuf },
    io::{ stdin, stdout, Write }
};

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

use crate::{
    helper::VirtualHelper,
    command::*,
    errors::ShellError
};

pub struct Shell {
    cwd: PathBuf,
    container: Container,
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            cwd: current_dir().unwrap(),
            container: Container::new(),
        }
    }
}


impl Shell {
    fn send_matches<W: Write>(&mut self, matches: &ArgMatches, out: &mut W) -> Result<(), ShellError> {
        match matches.subcommand() {
            ("exit", Some(_matches)) => Err(CommandError::Exit),
            ("cd",   Some(matches))  => self.cd(matches),
            ("debug_status",   Some(matches))  =>
                match matches.value_of("path") {
                    Some(string_path) => {
                        let path = absolute(self.cwd.as_path(), Path::new(string_path));
                        println!("STATUS : {:?}", self.container.status(path.as_path())?);
                        Ok(())
                    },
                    None => Err(CommandError::InvalidCommand)
                },
            ("debug_container",     Some(_matches)) => { println!("{:#?}", self.container); Ok(()) },
            ("debug_add_state",     Some(_matches)) => unimplemented!(),
            ("debug_sub_state",     Some(_matches)) => unimplemented!(),
            ("debug_transaction",   Some(_matches)) => unimplemented!(),
            ("pwd",         Some(_matches)) => { println!("{}", self.cwd.to_string_lossy()); Ok(()) },
            ("reset",       Some(_matches)) => { self.container.reset(); writeln!(out, "Virtual state is now empty")?;  Ok(()) },
            ("ls",          Some(matches)) => Command::<ListCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(out, &mut self.container)),
            ("cp",          Some(matches)) => Command::<CopyCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.container)),
            ("mv",          Some(matches)) => Command::<MoveCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.container)),
            ("rm",          Some(matches)) => Command::<RemoveCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.container)),
            ("mkdir",       Some(matches)) => Command::<NewDirectoryCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.container)),
            ("touch",       Some(matches)) => Command::<NewFileCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.container)),
            ("tree",        Some(matches)) => Command::<TreeCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(out,&mut self.container)),
            ("save",        Some(matches)) => Command::<SaveCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.container)),
            ("import",        Some(matches)) => Command::<ImportCommand>::initialize(&self.cwd, matches)
                .and_then(|c| c.execute(&mut self.container)),
            ("apply",        Some(_matches)) => self.apply(),
            _ => Err(CommandError::InvalidCommand)
        }?;
        Ok(())
    }

    pub fn run_single<T, W : Write, E: Write>(&mut self, args: T, out: &mut W, err: &mut E) -> Result<(), ShellError> where T : Iterator<Item = String> {
        let yaml = load_yaml!("clap.yml");
        let matches = &App::from_yaml(yaml).get_matches_from_safe(args.skip(1)).unwrap();

        let mut current_state_file = None;

        if matches.value_of("state").is_some() {
            let path = Command::<ImportCommand>::extract_path_from_args(&self.cwd, matches, "state").unwrap();

            if path.exists() {
                Command(InitializedImportCommand {
                    path: path.clone()
                }).execute(&mut self.container)?;
            }
            current_state_file = Some(path);
        }

        match self.send_matches(&matches, out) {
            Ok(_) => { /*SUCCESS*/ },
            Err(error) =>
                match error {
                    ShellError::Command(CommandError::InvalidCommand) => writeln!(err, "{} {}", error, matches.usage())?,
                    ShellError::Command(CommandError::ArgumentMissing(command, _, _)) => {
                        match App::from_yaml(yaml).get_matches_from_safe(vec![command, "--help".to_string()]) {
                            Ok(_) => {},
                            Err(error) => write!(err, "{}", error)?
                        }
                    },
                    error => writeln!(err, "Unhandled error : {}", error)?
                }
        }

        if current_state_file.is_some() && matches.is_present("write_state") {
            Command(InitializedSaveCommand {
                path: current_state_file.unwrap(),
                overwrite: true
            }).execute(&mut self.container)?;
        }
        Ok(())
    }

    pub fn run_readline<W: Write, E: Write>(&mut self, out: &mut W, err: &mut E) -> Result<(), ShellError> {
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
            read_line_editor.set_helper(Some(VirtualHelper::new(&self.container, self.cwd.to_path_buf())));
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
                                        writeln!(out, "{}", line)?;
                                    }
                                },
                                _ =>
                                    match self.send_matches(&matches, out) {
                                        Ok(_) => { /*SUCCESS*/ },
                                        Err(error) =>
                                            match error {
                                                ShellError::Command(CommandError::Exit) => break,
                                                ShellError::Command(CommandError::InvalidCommand) => {
                                                    eprintln!("{} {}", error, matches.usage());
                                                },
                                                ShellError::Command(CommandError::ArgumentMissing(command, _, _)) => {
                                                    //Trick to get proper subcommand help
                                                    match App::from_yaml(yaml).get_matches_from_safe(vec![command, "--help".to_string()]) {
                                                        Ok(_) => {},
                                                        Err(error) => { writeln!(err, "{}", error)?; }
                                                    };
                                                },
                                                error => { writeln!(err, "Error : {}", error)?; }
                                            }
                                    }
                            }
                        Err(error) => { writeln!(err, "Error: {}", error)?; }
                    }
                },
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(error) => {
                    writeln!(err, "Error: {:?}", error)?;
                    break
                }
            }
            writeln!(out)?;
        }
        Ok(())
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
                                match self.send_matches(&matches, &mut std::io::stdout()) {
                                    Ok(_)      => {/*SUCCESS*/},
                                    Err(error) =>
                                        match error {
                                            ShellError::Command(CommandError::InvalidCommand) => eprintln!("{} {}", error, matches.usage()),
                                            ShellError::Command(CommandError::ArgumentMissing(command, _, _)) => {
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

                match self.container.status(path.as_path())?.into_inner().into_existing_virtual() {
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
        match self.container.apply() {
            Ok(_) => Ok(()),
            Err(error) => Err(CommandError::from(error))
        }
    }
}

