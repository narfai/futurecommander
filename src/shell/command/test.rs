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

#[allow(unused_imports)]
use crate::command::InitializedCommand;

use std::path::{ PathBuf, Path };
use std::ffi::{ OsString };
use vfs::*;
use crate::command::CommandError;
use crate::command::copy::{ InitializedCopyCommand };
use crate::command::mov::InitializedMoveCommand;
use crate::command::new_directory::InitializedNewDirectoryCommand;
use crate::command::new_file::InitializedNewFileCommand;
use crate::command::remove::InitializedRemoveCommand;
//use crate::command::list::InitializedListCommand;
//use crate::command::tree::InitializedTreeCommand;

pub struct Samples;

impl Samples {
    pub fn sample_path() -> PathBuf {
        current_exe().unwrap()
            .parent().unwrap() //project root
            .parent().unwrap() //target
            .parent().unwrap() //debug
            .parent().unwrap() //deps
            .join("samples")
            .to_path_buf()
    }

    pub fn static_samples_path() -> PathBuf {
        let sample_path = Self::sample_path().join("static_samples");
        assert!(sample_path.join("A").exists());
        assert!(sample_path.join("B").exists());
        assert!(sample_path.join("F").exists());
        assert!(sample_path.join("B/D").exists());
        assert!(sample_path.join("B/D/E").exists());
        assert!(sample_path.join("B/D/G").exists());

        assert!(sample_path.join("A").is_dir());
        assert!(sample_path.join("B").is_dir());
        assert!(sample_path.join("F").is_file());
        assert!(sample_path.join("B/D").is_dir());
        assert!(sample_path.join("B/D/E").is_dir());
        assert!(sample_path.join("B/D/G").is_dir());
        sample_path
    }
}

#[cfg(test)]
mod virtual_shell_tests {
    use super::*;

    #[test]
    fn rm(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let b_path = sample_path.join(&Path::new("B"));

        let remove_b = InitializedRemoveCommand {
            path: b_path.to_path_buf()
        };

        remove_b.execute(&mut vfs).unwrap();

        assert!(!Virtual(StatusQuery::new(b_path.as_path()))
            .retrieve(&vfs)
            .unwrap()
            .exists()
        )
    }

    #[test]
    fn cp_only(){
        let sample_path = Samples::static_samples_path();
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

        let read_dir = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("A/D")).as_path()));

        match read_dir.retrieve(&vfs) {
            Ok(virtual_children) => {
                let collection = virtual_children.collection();
                assert!(collection.len() > 0);
                assert!(collection.contains(&Node(VirtualPath::from_path_buf(sample_path.join("A/D/G")).unwrap())));
                assert!(collection.contains(&Node(VirtualPath::from_path_buf(sample_path.join("A/D/E")).unwrap())));
            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn cp_preserve_source_and_node_kind(){
        let sample_path = Samples::static_samples_path();
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

        let read_dir = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path()));

        match read_dir.retrieve(&vfs) {
            Ok(virtual_children) => {
                let collection = virtual_children.collection();
                assert!(collection.len() > 0);
                collection.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))).unwrap()));

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn mv(){
        let sample_path = Samples::static_samples_path();
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
            Virtual(ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path()))
                .retrieve(&vfs)
                .unwrap()
                .collection()
                .contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))).unwrap()))
        );

        assert!(
            !Virtual(ReadDirQuery::new(sample_path.join(&Path::new("A")).as_path()))
                .retrieve(&vfs)
                .unwrap()
                .collection()
                .contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("A/F"))).unwrap()))
        );

        assert!(
            !Virtual(ReadDirQuery::new(sample_path.join(&Path::new("B")).as_path()))
                .retrieve(&vfs)
                .unwrap()
                .collection()
                .contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("B/F"))).unwrap()))
        );
    }

    #[test]
    fn mkdir(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let new_bde_mkdired = InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("B/D/E/MKDIRED"))
        };

        new_bde_mkdired.execute(&mut vfs).unwrap();

        assert!(
            Virtual(ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path()))
                .retrieve(&vfs)
                .unwrap()
                .collection()
                .contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/MKDIRED"))).unwrap()))
        );
    }

    #[test]
    fn touch(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();


        let new_bde_touched = InitializedNewFileCommand {
            path: sample_path.join(&Path::new("B/D/E/TOUCHED"))
        };

        new_bde_touched.execute(&mut vfs).unwrap();

        assert!(
            Virtual(ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path()))
                .retrieve(&vfs)
                .unwrap()
                .collection()
                .contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/TOUCHED"))).unwrap()))
        );
    }

    #[test]
    fn virtual_shell_reference_virtual_children(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let mkdir_z = InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("Z"))
        };
        mkdir_z.execute(&mut vfs).unwrap();

        let touch_test = InitializedNewFileCommand {
            path: sample_path.join(&Path::new("TEST"))
        };
        touch_test.execute(&mut vfs).unwrap();

        let copy_test_to_z = InitializedCopyCommand {
            source: sample_path.join("TEST"),
            destination: sample_path.join("Z"),
            name: None
        };
        copy_test_to_z.execute(&mut vfs).unwrap();

        assert!(
            Virtual(ReadDirQuery::new(sample_path.join(&Path::new("Z")).as_path()))
                .retrieve(&vfs)
                .unwrap()
                .collection()
                .contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("Z/TEST"))).unwrap()))
        );
    }

    #[test]
    fn virtual_shell_copy_nested_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let copy_b_to_a = InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            name: None
        };
        copy_b_to_a.execute(&mut vfs).unwrap();

        let copy_a_as_aprime = InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.clone(),
            name: Some(OsString::from("APRIME"))
        };
        copy_a_as_aprime.execute(&mut vfs).unwrap();

        let collection_aprime = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("APRIME")).as_path()))
            .retrieve(&vfs)
            .unwrap()
            .collection();

        assert!(collection_aprime.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/C"))).unwrap())));
        assert!(collection_aprime.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/B"))).unwrap())));

        let collection_aprime_b_d = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("APRIME/B/D")).as_path()))
            .retrieve(&vfs)
            .unwrap()
            .collection();

        assert!(collection_aprime_b_d.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/B/D/E"))).unwrap())));
        assert!(collection_aprime_b_d.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/B/D/G"))).unwrap())));
    }

    #[test]
    fn virtual_shell_move_nested_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let move_b_to_a = InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            name: None
        };
        move_b_to_a.execute(&mut vfs).unwrap();

        let move_a_as_aprime = InitializedMoveCommand {
            source: sample_path.join("A"),
            destination: sample_path.clone(),
            name: Some(OsString::from("APRIME"))
        };
        move_a_as_aprime.execute(&mut vfs).unwrap();

        let collection_aprime = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("APRIME")).as_path()))
            .retrieve(&vfs)
            .unwrap()
            .collection();

        assert!(collection_aprime.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/C"))).unwrap())));
        assert!(collection_aprime.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/B"))).unwrap())));

        let collection_aprime_b_d = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("APRIME/B/D")).as_path()))
            .retrieve(&vfs)
            .unwrap()
            .collection();

        assert!(collection_aprime_b_d.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/B/D/E"))).unwrap())));
        assert!(collection_aprime_b_d.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/B/D/G"))).unwrap())));
    }


    #[test]
    fn virtual_shell_copy_nested_virtual_identity_deep_through(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let copy_b_to_a = InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            name: None
        };
        copy_b_to_a.execute(&mut vfs).unwrap();

        let copy_a_as_aprime = InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.clone(),
            name: Some(OsString::from("APRIME"))
        };
        copy_a_as_aprime.execute(&mut vfs).unwrap();

        let copy_aprime_as_abeta = InitializedCopyCommand {
            source: sample_path.join("APRIME"),
            destination: sample_path.clone(),
            name: Some(OsString::from("ABETA"))
        };
        copy_aprime_as_abeta.execute(&mut vfs).unwrap();

        let copy_abeta_to_a = InitializedCopyCommand {
            source: sample_path.join("ABETA"),
            destination: sample_path.join("A"),
            name: None
        };
        copy_abeta_to_a.execute(&mut vfs).unwrap();

        let collection_a_abeta = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("APRIME")).as_path()))
            .retrieve(&vfs)
            .unwrap()
            .collection();

        assert!(collection_a_abeta.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/C"))).unwrap())));
        assert!(collection_a_abeta.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("APRIME/B"))).unwrap())));

        let collection_aprime_a_abeta_b_d = Virtual(ReadDirQuery::new(sample_path.join(&Path::new("A/ABETA/B/D")).as_path()))
            .retrieve(&vfs)
            .unwrap()
            .collection();

        assert!(collection_aprime_a_abeta_b_d.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("A/ABETA/B/D/E"))).unwrap())));
        assert!(collection_aprime_a_abeta_b_d.contains(&Node(VirtualPath::from_path_buf(sample_path.join(&Path::new("A/ABETA/B/D/G"))).unwrap())));
    }

    //Error testing

    #[test]
    fn virtual_shell_copy_or_move_directory_into_itself_must_not_be_allowed(){
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let source = sample_path.join(&Path::new("B"));
        let destination = sample_path.join("B/D");

        let move_b_to_bd = InitializedMoveCommand {
            source: source.clone(),
            destination: destination.clone(),
            name: None
        };

        match move_b_to_bd.execute(&mut vfs){
            Err(CommandError::VfsError(VfsError::CopyIntoItSelf(err_source, err_destination))) => {
                assert_eq!(source, err_source);
                assert_eq!(destination, err_destination);
            },
            Err(unwanted_error) => panic!("{}", unwanted_error),
            Ok(_) => panic!("Should not be able to move into itself")
        };
    }
}
