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

use std::env::current_exe;

use std::path::{ PathBuf, Path };

use vfs::{ VirtualPath, VirtualFileSystem, VfsError };
use crate::command::{ InitializedCommand };
use crate::command::CommandError;
use crate::command::copy::{ InitializedCopyCommand };
use crate::command::mov::InitializedMoveCommand;
use crate::command::new_directory::InitializedNewDirectoryCommand;
use crate::command::new_file::InitializedNewFileCommand;
use crate::command::remove::InitializedRemoveCommand;
//TODO
//use crate::command::list::InitializedListCommand;
//use crate::command::tree::InitializedTreeCommand;

pub fn get_sample_path() -> PathBuf {
    current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples")
}

#[cfg(test)]
mod virtual_shell_tests {
    use super::*;

    #[test]
    fn virtual_shell_system_rm(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let b_path = sample_path.join(&Path::new("B"));

        let remove_b = InitializedRemoveCommand {
            path: b_path.to_path_buf()
        };

        remove_b.execute(&mut vfs).unwrap();

        assert!(!vfs.exists(b_path.as_path()).unwrap());
    }

    #[test]
    fn virtual_shell_system_cp_only(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let copy_a_to_b = InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            name: None
        };

        copy_a_to_b.execute(&mut vfs).unwrap();

        let copy_ab_to_abd = InitializedCopyCommand {
            source: sample_path.join("A/B/D"),
            destination: sample_path.join("A"),
            name: None
        };

        copy_ab_to_abd.execute(&mut vfs).unwrap();

        match vfs.read_dir(sample_path.join(&Path::new("A/D")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D/G"))).unwrap());
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D/E"))).unwrap());
            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn virtual_shell_system_cp_preserve_source_and_node_kind(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let copy_b_to_a = InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            name: None
        };

        copy_b_to_a.execute(&mut vfs).unwrap();

        let copy_f_to_b = InitializedCopyCommand {
            source: sample_path.join("F"),
            destination: sample_path.join("B"),
            name: None
        };

        copy_f_to_b.execute(&mut vfs).unwrap();

        let copy_bf_to_bde = InitializedCopyCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E"),
            name: None
        };

        copy_bf_to_bde.execute(&mut vfs).unwrap();

        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))).unwrap());

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn virtual_shell_system_mv(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let move_f_to_a = InitializedMoveCommand {
            source: sample_path.join(&Path::new("F")),
            destination: sample_path.join("A"),
            name: None
        };

        move_f_to_a.execute(&mut vfs).unwrap();

        let move_af_to_b = InitializedMoveCommand {
            source: sample_path.join("A/F"),
            destination: sample_path.join("B"),
            name: None
        };

        move_af_to_b.execute(&mut vfs).unwrap();

        let move_bf_to_bde = InitializedMoveCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E"),
            name: None
        };

        move_bf_to_bde.execute(&mut vfs).unwrap();

        assert!(
            vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))).unwrap())
        );
        assert!(
            !vfs.read_dir(sample_path.join(&Path::new("A/")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/F"))).unwrap()) //Parent
        );
        assert!(
            !vfs.read_dir(sample_path.join(&Path::new("B")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/F"))).unwrap())
        );
    }

    #[test]
    fn virtual_shell_system_mkdir(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let new_bde_mkdired = InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("B/D/E/MKDIRED"))
        };

        new_bde_mkdired.execute(&mut vfs).unwrap();

        let virtual_children = vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()).unwrap();

        assert!(virtual_children.len() > 0);
        assert!(
            virtual_children.contains(
                &VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/MKDIRED"))).unwrap()
            )
        );
    }

    #[test]
    fn virtual_shell_system_touch(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();


        let new_bde_touched = InitializedNewFileCommand {
            path: sample_path.join(&Path::new("B/D/E/TOUCHED"))
        };

        new_bde_touched.execute(&mut vfs).unwrap();

        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                assert!(
                    virtual_children.contains(
                        &VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/TOUCHED"))).unwrap()
                    )
                );

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    //Error testing

    #[test]
    fn virtual_shell_copy_or_move_directory_into_itself_must_not_be_allowed(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let source = sample_path.join(&Path::new("B"));
        let destination = sample_path.join("B/D");

        let move_b_to_bd = InitializedMoveCommand {
            source: source.clone(),
            destination: destination.clone(),
            name: None
        };

        match move_b_to_bd.execute(&mut vfs){
            Err(CommandError::VfsError(VfsError::CopyIntoItSelft(err_source, err_destination))) => {
                assert_eq!(source, err_source);
                assert_eq!(destination, err_destination);
            },
            Err(unwanted_error) => panic!("{}", unwanted_error),
            Ok(_) => panic!("Should not be able to move into itself")
        };
    }

    //TODO test mkdir Z / touch TEST / cp TEST Z/ / tree
    //TODO test cp B A/ / cp A APRIME / tree
    //TODO test mv B A/ / mv A APRIME / tree
}
