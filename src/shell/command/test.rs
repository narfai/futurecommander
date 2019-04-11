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

use vfs::{
    HybridFileSystem,
    VfsError,
    query::{
        Query,
        Entry,
        StatusQuery,
        ReadDirQuery,
        EntryAdapter
    }
};

pub use crate::command::{
    Command,
    CommandError,
    copy            ::InitializedCopyCommand,
    list            ::InitializedListCommand,
    new_directory   ::InitializedNewDirectoryCommand,
    mov             ::InitializedMoveCommand,
    new_file        ::InitializedNewFileCommand,
    remove          ::InitializedRemoveCommand
};

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
        let mut fs = HybridFileSystem::new();

        let b_path = sample_path.join(&Path::new("B"));

        let remove_b = Command(InitializedRemoveCommand {
            path: b_path.to_path_buf()
        });

        remove_b.execute(&mut fs).unwrap();

        assert!(!StatusQuery::new(b_path.as_path())
            .retrieve(fs.vfs())
            .unwrap()
            .exists()
        )
    }

    #[test]
    fn cp_only(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let copy_a_to_b = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });

        copy_a_to_b.execute(&mut fs).unwrap();

        let copy_ab_to_abd = Command(InitializedCopyCommand {
            source: sample_path.join("A/B/D"),
            destination: sample_path.join("A")
        });

        copy_ab_to_abd.execute(&mut fs).unwrap();

        let read_dir = ReadDirQuery::new(sample_path.join("A/D").as_path());

        match read_dir.retrieve(fs.vfs()) {
            Ok(collection) => {
                assert!(collection.len() > 0);
                assert!(collection.contains(&EntryAdapter(sample_path.join("A/D/G").as_path())));
                assert!(collection.contains(&EntryAdapter(sample_path.join("A/D/E").as_path())));
            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn cp_preserve_source_and_node_kind(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });

        copy_b_to_a.execute(&mut fs).unwrap();

        let copy_f_to_b = Command(InitializedCopyCommand {
            source: sample_path.join("F"),
            destination: sample_path.join("B")
        });

        copy_f_to_b.execute(&mut fs).unwrap();

        let copy_bf_to_bde = Command(InitializedCopyCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E")
        });

        copy_bf_to_bde.execute(&mut fs).unwrap();

        let read_dir = ReadDirQuery::new(sample_path.join("B/D/E").as_path());

        match read_dir.retrieve(fs.vfs()) {
            Ok(collection) => {
                assert!(collection.len() > 0);
                assert!(collection.contains(&EntryAdapter(sample_path.join("B/D/E/F").as_path())));

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn mv(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let move_f_to_a = Command(InitializedMoveCommand {
            source: sample_path.join(&Path::new("F")),
            destination: sample_path.join("A")
        });

        move_f_to_a.execute(&mut fs).unwrap();

        let move_af_to_b = Command(InitializedMoveCommand {
            source: sample_path.join("A/F"),
            destination: sample_path.join("B")
        });

        move_af_to_b.execute(&mut fs).unwrap();

        let move_bf_to_bde = Command(InitializedMoveCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E")
        });

        move_bf_to_bde.execute(&mut fs).unwrap();

        assert!(
            ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/F").as_path()))
        );

        assert!(
            !ReadDirQuery::new(sample_path.join(&Path::new("A")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("A/F").as_path()))
        );

        assert!(
            !ReadDirQuery::new(sample_path.join(&Path::new("B")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/F").as_path()))
        );
    }

    #[test]
    fn mkdir(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let new_bde_mkdired = Command(InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("B/D/E/MKDIRED"))
        });

        new_bde_mkdired.execute(&mut fs).unwrap();

        assert!(
            ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/MKDIRED").as_path()))
        );
    }

    #[test]
    fn touch(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let new_bde_touched = Command(InitializedNewFileCommand {
            path: sample_path.join(&Path::new("B/D/E/TOUCHED"))
        });

        new_bde_touched.execute(&mut fs).unwrap();

        assert!(
            ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/TOUCHED").as_path()))
        );
    }

    #[test]
    fn virtual_shell_reference_virtual_children(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let mkdir_z = Command(InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("Z"))
        });
        mkdir_z.execute(&mut fs).unwrap();

        let touch_test = Command(InitializedNewFileCommand {
            path: sample_path.join(&Path::new("TEST"))
        });
        touch_test.execute(&mut fs).unwrap();

        let copy_test_to_z = Command(InitializedCopyCommand {
            source: sample_path.join("TEST"),
            destination: sample_path.join("Z")
        });
        copy_test_to_z.execute(&mut fs).unwrap();

        assert!(
            ReadDirQuery::new(sample_path.join(&Path::new("Z")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("Z/TEST").as_path()))
        );
    }

    #[test]
    fn virtual_shell_copy_nested_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });
        copy_b_to_a.execute(&mut fs).unwrap();

        let copy_a_as_aprime = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME")
        });
        copy_a_as_aprime.execute(&mut fs).unwrap();

        let collection_aprime = ReadDirQuery::new(sample_path.join(&Path::new("APRIME")).as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));
        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));

        let collection_aprime_b_d = ReadDirQuery::new(sample_path.join(&Path::new("APRIME/B/D")).as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/E").as_path())));
        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/G").as_path())));
    }

    #[test]
    fn virtual_shell_move_nested_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let move_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });
        move_b_to_a.execute(&mut fs).unwrap();

        let move_a_as_aprime = Command(InitializedMoveCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME")
        });
        move_a_as_aprime.execute(&mut fs).unwrap();

        let collection_aprime = ReadDirQuery::new(sample_path.join(&Path::new("APRIME")).as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));
        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));

        let collection_aprime_b_d = ReadDirQuery::new(sample_path.join(&Path::new("APRIME/B/D")).as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/E").as_path())));
        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/G").as_path())));
    }


    #[test]
    fn virtual_shell_copy_nested_deep_through_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });
        copy_b_to_a.execute(&mut fs).unwrap();

        let copy_a_as_aprime = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME").to_path_buf()
        });
        copy_a_as_aprime.execute(&mut fs).unwrap();

        let copy_aprime_as_abeta = Command(InitializedCopyCommand {
            source: sample_path.join("APRIME"),
            destination: sample_path.join("ABETA").clone()
        });
        copy_aprime_as_abeta.execute(&mut fs).unwrap();

        let copy_abeta_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("ABETA"),
            destination: sample_path.join("A")
        });
        copy_abeta_to_a.execute(&mut fs).unwrap();

        let collection_a_abeta = ReadDirQuery::new(sample_path.join(&Path::new("APRIME")).as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(collection_a_abeta.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));
        assert!(collection_a_abeta.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));

        let collection_aprime_a_abeta_b_d = ReadDirQuery::new(sample_path.join("A/ABETA/B/D").as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(collection_aprime_a_abeta_b_d.contains(&EntryAdapter(sample_path.join("A/ABETA/B/D/E").as_path())));
        assert!(collection_aprime_a_abeta_b_d.contains(&EntryAdapter(sample_path.join("A/ABETA/B/D/G").as_path())));
    }

    //Error testing

    #[test]
    fn virtual_shell_copy_or_move_directory_into_itself_must_not_be_allowed(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::new();

        let source = sample_path.join(&Path::new("B"));
        let destination = sample_path.join("B/D");

        let move_b_to_bd = Command(InitializedMoveCommand {
            source: source.clone(),
            destination: destination.clone()
        });

        match move_b_to_bd.execute(&mut fs){
            Err(CommandError::VfsError(VfsError::CopyIntoItSelf(err_source, err_destination))) => {
                assert_eq!(source, err_source);
                assert_eq!(destination.join("B"), err_destination);
            },
            Err(unwanted_error) => panic!("{}", unwanted_error),
            Ok(_) => panic!("Should not be able to move into itself")
        };
    }
}
