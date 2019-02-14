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
//use vfs::{Vfs, VirtualPath};

fn ls(path: PathBuf) {
    path.as_path().read_dir()
        .and_then(|results: ReadDir| {
            for result in results {
                println!("{:?}", result.unwrap());
            }
            Ok(())
        }).unwrap();
}


fn main() {
    let yaml = load_yaml!("clap.yml");
    let mut cwd: PathBuf = env::current_dir().unwrap();
//    let mut vfs = Vfs::new();
//    VirtualPath::add(
//        &mut vfs, &VirtualPath::from_path(&(Path::new("/")).to_path_buf(), true),
//       None
//    );
//    VirtualPath::add(
//        &mut vfs, &VirtualPath::from_path(&(Path::new("/testA")).to_path_buf(), true),
//        Some(&VirtualPath::from_path(&(Path::new("/")).to_path_buf(), true))
//    );
//    VirtualPath::add(
//        &mut vfs, &VirtualPath::from_path(&(Path::new("/testA/testB")).to_path_buf(), false)
//    );
//    VirtualPath::add(
//        &mut vfs, &VirtualPath::from_path(&(Path::new("/testA/testC")).to_path_buf(), false)
//    );
//    VirtualPath::add(
//        &mut vfs, &VirtualPath::from_path(&(Path::new("/testD/testE")).to_path_buf(), false)
//    );
//    let root = VirtualPath::add(vfs, );
//    let path_to_ls = VirtualPath::from_path(PathBuf::from_str("/"))
//    let root = vfs.add(VirtualPath::from_path(PathBuf::from_str("/")), None, true);
//    vfs.add(VirtualPath::from_path(PathBuf::from_str("/virtual_path"), Some(&root), true);
//    let child_two = vfs.add(Some(&root), Some(OsStr::new("virtual_directory_2")), true);
//    vfs.add(Some(&child_two), Some(OsStr::new("virtual_subdirectory_1")), true);
//    vfs.add(Some(&child_two), Some(OsStr::new("virtual_subdirectory_2")), true);
//    vfs.add(Some(&child_two), Some(OsStr::new("virtual_subdirectory_3")), true);

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
//                        let vpath = VirtualPath::from_path(&path, true);
//                        println!("VPath {:?}", vpath);
//                        if let Some(children) = VirtualPath::children(&mut vfs, &vpath) {
//                            for child in children {
//                                println!("VChild {:?}", child);
//                            }
//                        }
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
}


#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

