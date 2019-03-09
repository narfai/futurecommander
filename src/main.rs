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
//use vfs::cp;
//use vfs::rm;
//use vfs::touch;
//use vfs::mkdir;
//use vfs::ls;
//use vfs::mv;
//use vfs::tree;

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
                    if let Some(matches) = matches.subcommand_matches("ls") {
                        let identity = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap_or(cwd.to_str().unwrap())));
                        match vfs.read_dir(identity.as_path()) {
                            Ok(virtual_children) => {
                                for child in virtual_children {
                                    println!("{:?}", child);
                                }
                            },
                            Err(error) => println!("Error : {}", error)
                        }
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
                    }
//                    if let Some(_matches) = matches.subcommand_matches("debug_vfs_state") {
//                        println!("{:#?}", vfs.get_state());
//                    } else if let Some(_matches) = matches.subcommand_matches("debug_add_state") {
//                        println!("{:#?}", vfs.get_add_state());
//                    } else if let Some(_matches) = matches.subcommand_matches("debug_sub_state") {
//                        println!("{:#?}", vfs.get_sub_state());
//                    } else if let Some(_matches) = matches.subcommand_matches("debug_real_state") {
//                        println!("{:#?}", vfs.get_real_state());
//                    } else if let Some(_matches) = matches.subcommand_matches("debug_node_state") {
//                        let identity = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap_or(cwd.to_str().unwrap())));
//                        println!("{:?}", vfs.get_node_state(identity.as_path()));
//                    } else
////                        println!("{:#?}", vfs);
//                    } else if let Some(matches) = matches.subcommand_matches("tree") {
//                        let path = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap_or(cwd.to_str().unwrap())));
//                        tree(&mut vfs,path.as_path());
//                    } else if let Some(matches) = matches.subcommand_matches("cp") {
//                        let source = absolute(cwd.as_path(),Path::new(matches.value_of("source").unwrap()));
//                        let destination = absolute(cwd.as_path(), Path::new(matches.value_of("destination").unwrap()));
//                        cp(
//                            &mut vfs,
//                            source.as_path(),
//                            destination.as_path()
//                        );
//                    } else if let Some(matches) = matches.subcommand_matches("mv") {
//                        let source = absolute(cwd.as_path(),Path::new(matches.value_of("source").unwrap()));
//                        let destination = absolute(cwd.as_path(), Path::new(matches.value_of("destination").unwrap()));
//                        mv(
//                            &mut vfs,
//                            source.as_path(),
//                            destination.as_path()
//                        );
//                    } else if let Some(matches) = matches.subcommand_matches("rm") {
//                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
//                        rm(&mut vfs, path.as_path());
//                    } else if let Some(matches) = matches.subcommand_matches("mkdir") {
//                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
//                        mkdir(&mut vfs, path.as_path());
//
//                    } else if let Some(matches) = matches.subcommand_matches("touch") {
//                        let path = absolute(cwd.as_path(),Path::new(matches.value_of("path").unwrap()));
//                        touch(&mut vfs, path.as_path());
//
//                    } else if let Some(matches) = matches.subcommand_matches("cd") {
//                        let path = absolute(cwd.as_path(), Path::new(matches.value_of("path").unwrap()));
//
//                        match vfs.virtualize(path.as_path()).exp_is_directory(path.as_path()) {
//                            Some(true) => cwd = path,
//                            Some(false) => { println!("Is not a directory") }
//                            None => { println!("Does not exists") }
//                        }
//                    } else {
//                        println!("Unknown command");
//                    }
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
