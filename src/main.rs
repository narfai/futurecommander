#[macro_use]
extern crate clap;

use std::path::{ PathBuf, Path, Component };
use std::fs::{ ReadDir, DirEntry };
use std::io::{stdin, stdout};

use std::ffi::{ OsString, OsStr };

use clap::App;
use std::env;
mod path;
use path::absolute;

mod vfs;
use vfs::{VirtualFileSystem, VirtualPath};

//TODO proper shell / file ui representation
fn main() {
    let yaml = load_yaml!("clap.yml");
    let mut cwd: PathBuf = env::current_dir().unwrap();
    let mut vfs = VirtualFileSystem::new();

    loop {
        let mut input = String::new();
        print!("> ");
        if let Ok(_) =  stdin().read_line(&mut input) {
            let mut argv = vec!["futurecommander"];
            argv.extend(input.trim().split(" "));

            match App::from_yaml(yaml).get_matches_from_safe(argv) {
                Ok(matches) => {
                    if let Some(matches) = matches.subcommand_matches("exit") {
                        break;
                    }

                    if let Some(matches) = matches.subcommand_matches("ls") {
                        let path = Path::new(matches.value_of("path").unwrap_or(cwd.to_str().unwrap()));

                        vfs.read(path);

                        let state = vfs.get_state();

                        let children = state.children(&VirtualPath::from_path_buf(path.to_path_buf())).unwrap();
                        for child in children {
                            println!("VChild {:?}", child);
                        }

                    } else if let Some(matches) = matches.subcommand_matches("cwd") {
                        println!("{:?}", cwd)

                    } else if let Some(matches) = matches.subcommand_matches("cd") {
                        let path = Path::new(matches.value_of("path").unwrap());

                        vfs.read(path);

                        let state = vfs.get_state();

                        if state.is_directory(&VirtualPath::from_path_buf(path.to_path_buf()),) {
                            cwd = absolute(&cwd.as_path(), &path).to_path_buf();
                            println!("{:?}", cwd);
                        } else {
                            println!("Target does not exists or is not a directory");
                        }

                    } else {
                        println!("Unknown command");

                    }
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
    }
}
