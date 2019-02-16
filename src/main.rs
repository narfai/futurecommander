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
use vfs::{Vfs, VirtualPath};

fn main() {
    let yaml = load_yaml!("clap.yml");
    let mut cwd: PathBuf = env::current_dir().unwrap();
    let mut vfs = vfs::Vfs::new();

    loop {
        let mut input = String::new();
        print!("> ");
        if let Ok(_) =  stdin().read_line(&mut input) {
            let mut argv = vec!["futurecommander"];
            argv.extend(input.trim().split(" "));

            match App::from_yaml(yaml).get_matches_from_safe(argv) {
                Ok(matches) => {
                    if let Some(matches) = matches.subcommand_matches("ls") {
                        let path = PathBuf::from(matches.value_of("path").unwrap_or(cwd.to_str().unwrap()));
                        path.as_path().read_dir()
                            .and_then(|results: ReadDir| {
                                for result in results {
                                    let result= result.unwrap();
                                    vfs.attach(VirtualPath::from_path_buf(result.path(), result.path().is_dir()))
                                }
                                Ok(())
                            }).unwrap();

                        let children = vfs.children(VirtualPath::from_path_buf(path, true)).unwrap();
                        for child in children {
                            println!("VChild {:?}", child);
                        }

                    } else if let Some(matches) = matches.subcommand_matches("cwd") {
                        println!("{:?}", cwd)

                    } else if let Some(matches) = matches.subcommand_matches("cd") {
                        let path = Path::new(matches.value_of("path").unwrap());

                        if path.exists() && path.is_dir() {
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


#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

