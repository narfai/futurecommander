#[macro_use]
extern crate clap;

use std::path::{ PathBuf, Path, Component };
use std::fs::{ ReadDir, DirEntry };
use std::io::{stdin, stdout};

use clap::App;
use std::env;
mod path;
use path::absolute;




fn ls(path: PathBuf) {
    path.as_path().read_dir()
        .and_then(|results: ReadDir| {
            for result in results {
                println!("{:?}", result.unwrap());
            }
            Ok(())
        });
}


fn main() {
    let yaml = load_yaml!("clap.yml");
    let mut cwd: PathBuf = env::current_dir().unwrap();

    loop {
        print!("> ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        let mut argv = vec!["futurecommander"];
        argv.extend(command.split(" "));

        match App::from_yaml(yaml).get_matches_from_safe(argv) {
            Ok(matches) => {
                if let Some(matches) = matches.subcommand_matches("ls") {
                    let path = PathBuf::from(matches.value_of("path").unwrap_or(cwd.to_str().unwrap()));

                    ls(absolute(&cwd.as_path(), &path).to_path_buf());
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


