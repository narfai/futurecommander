#[macro_use]
extern crate clap;

use std::path::{ PathBuf, Path };
use std::io::{stdin};

use self::clap::App;
use std::env;
mod path;
use path::absolute;
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
                    if let Some(_) = matches.subcommand_matches("exit") {
                        break;
                    }

                    if let Some(matches) = matches.subcommand_matches("ls") {
                        let path = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap_or(cwd.to_str().unwrap())));

                        println!("{:#?}", path);

                        vfs.read(path.as_path());

                        let state = vfs.get_state();

                        let cmp_path = VirtualPath::from_path_buf(path.to_path_buf());

                        if state.is_directory(&cmp_path) {
                            match state.children(&cmp_path) {
                                Some(children) => {
                                    for child in children {
                                        println!("{:?} VChild {:?}", if state.is_directory(&child) {"DIRECTORY"} else {"FILE"}, child);
                                    }
                                },
                                None => {
                                    println!("No children");
                                }
                            };
                        } else {
                            vfs.read(path.parent().unwrap());
                            let state = vfs.get_state();
                            println!("FILE VChild {:?}", state.get(&cmp_path));
                        }

                    } else if let Some(matches) = matches.subcommand_matches("cp") {
                        let source = absolute(cwd.as_path(),Path::new(matches.value_of("source").unwrap()));
                        let destination = absolute(cwd.as_path(), Path::new(matches.value_of("destination").unwrap()));
                        vfs.copy(
                            VirtualPath::from_path_buf(source),
                            VirtualPath::from_path_buf(destination)
                        );

                    } else if let Some(matches) = matches.subcommand_matches("rm") {
                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
                        vfs.rm(&VirtualPath::from_path_buf(path));

                    } else if let Some(matches) = matches.subcommand_matches("cd") {
                        let path = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap()));

                        vfs.read(path.as_path());

                        let state = vfs.get_state();

                        if state.is_directory(&VirtualPath::from_path_buf(path.to_path_buf()),) {
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
