#[macro_use]
extern crate clap;

use std::path::{ PathBuf, Path };
use std::io::{stdin};

use self::clap::App;
use std::env;
mod path;
use path::absolute;
use vfs::{VirtualFileSystem};

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
                    if let Some(_) = matches.subcommand_matches("exit") {
                        break;
                    }
                    if let Some(_matches) = matches.subcommand_matches("debug_vfs_state") {
                        println!("{:#?}", vfs.get_state());
                    } else if let Some(matches) = matches.subcommand_matches("ls") {
                        let path = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap_or(cwd.to_str().unwrap())));

                        println!("{:#?}", path);

                        match vfs.ls(path.as_path()) {
                            Some(results) => for child in results.into_iter() {
                                println!("{:?}", child);
                            },
                            None => println!("No children")
                        }

                    } else if let Some(matches) = matches.subcommand_matches("cp") {
                        let source = absolute(cwd.as_path(),Path::new(matches.value_of("source").unwrap()));
                        let destination = absolute(cwd.as_path(), Path::new(matches.value_of("destination").unwrap()));
                        vfs.copy(
                            source.as_path(),
                            destination.as_path()
                        );

                    } else if let Some(matches) = matches.subcommand_matches("rm") {
                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
                        vfs.rm(path.as_path());

                    } else if let Some(matches) = matches.subcommand_matches("cd") {
                        let path = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap()));

                        vfs.read_virtual(path.as_path());

                        let state = vfs.get_state();

                        if state.is_directory(path.as_path()) {
                            cwd = path;
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
