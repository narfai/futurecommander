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

#[macro_use]
extern crate clap;

use std::path::{ PathBuf, Path };
use std::io::{stdin, stdout};
use std::io::Write;

use self::clap::App;
use std::env;
mod path;
use path::absolute;
use vfs::{VirtualFileSystem};
use vfs::{ cp, rm, touch, mkdir, ls, mv };

//TODO proper shell / file ui representation
fn main() {
    let yaml = load_yaml!("clap.yml");
    let mut cwd: PathBuf = env::current_dir().unwrap();
    let mut vfs = VirtualFileSystem::new();

    print!("> ");
    loop {
        stdout().flush();
        let mut input = String::new();
        if let Ok(_) =  stdin().read_line(&mut input) {
            println!("\n");
            let mut argv = vec!["futurecommander"];
            argv.extend(input.trim().split(" "));

            match App::from_yaml(yaml).get_matches_from_safe(argv) {
                Ok(matches) => {
                    if let Some(_) = matches.subcommand_matches("exit") {
                        break;
                    }
                    if let Some(matches) = matches.subcommand_matches("debug_virtual_state") {
                        println!("{:#?}", vfs.get_virtual_state());
                    } else if let Some(matches) = matches.subcommand_matches("debug_add_state") {
                        println!("{:#?}", vfs.get_add_state());
                    } else if let Some(matches) = matches.subcommand_matches("debug_sub_state") {
                        println!("{:#?}", vfs.get_sub_state());
                    } else if let Some(matches) = matches.subcommand_matches("ls") {
                        let identity = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap_or(cwd.to_str().unwrap())));
                        ls(&mut vfs, identity.as_path())
                    } else if let Some(matches) = matches.subcommand_matches("cd") {
                        let path = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap()));

                        if (
                            vfs.exists_virtually(path.as_path()) && vfs.is_directory_virtually(path.as_path()).unwrap_or(false))
                            || (path.exists() && path.is_dir()
                        ) {
                            cwd = path;
                        } else {
                            println!("Error : invalid path")
                        }
                    } else if let Some(matches) = matches.subcommand_matches("cp") {
                        let source = absolute(cwd.as_path(),Path::new(matches.value_of("source").unwrap()));
                        let destination = absolute(cwd.as_path(), Path::new(matches.value_of("destination").unwrap()));
                        cp(
                            &mut vfs,
                            source.as_path(),
                            destination.as_path()
                        );
                    } else if let Some(matches) = matches.subcommand_matches("mv") {
                        let source = absolute(cwd.as_path(),Path::new(matches.value_of("source").unwrap()));
                        let destination = absolute(cwd.as_path(), Path::new(matches.value_of("destination").unwrap()));
                        mv(
                            &mut vfs,
                            source.as_path(),
                            destination.as_path()
                        );
                    } else if let Some(matches) = matches.subcommand_matches("rm") {
                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
                        rm(&mut vfs, path.as_path());
                    } else if let Some(matches) = matches.subcommand_matches("mkdir") {
                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
                        mkdir(&mut vfs, path.as_path());

                    } else if let Some(matches) = matches.subcommand_matches("touch") {
                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
                        touch(&mut vfs, path.as_path());
                    }

                },
                Err(error) => {
                    println!("{}", error);
                }
            }

            println!("\n");
            print!("> ");
        }
    }
}
