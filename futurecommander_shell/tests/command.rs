// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

extern crate futurecommander_shell;
extern crate futurecommander_filesystem;


#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod command_integration {
    use super::*;

    use std::{
        path::Path
    };

    use futurecommander_shell::{
        command::{
            Command,
            InitializedMoveCommand,
            InitializedNewDirectoryCommand,
            InitializedNewFileCommand,
            InitializedCopyCommand,
            AvailableGuard
        },
    };

    use futurecommander_filesystem::{
        sample::Samples,
        Container,
        EntryAdapter,
        ReadableFileSystem
    };

    #[test]
    fn virtual_shell_move_nested_virtual_identity() {
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let move_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        move_b_to_a.execute(&mut fs).unwrap();

        let move_a_as_aprime = Command(InitializedMoveCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        move_a_as_aprime.execute(&mut fs).unwrap();

        let collection_aprime = fs.read_dir(sample_path.join(&Path::new("APRIME")).as_path())
            .unwrap();

        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));
        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));

        let collection_aprime_b_d = fs.read_dir(sample_path.join(&Path::new("APRIME/B/D")).as_path())
            .unwrap();

        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/E").as_path())));
        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/G").as_path())));
    }


    #[test]
    fn virtual_shell_reference_virtual_children(){
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let mkdir_z = Command(InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("Z")),
            recursive: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        mkdir_z.execute(&mut fs).unwrap();

        let touch_test = Command(InitializedNewFileCommand {
            path: sample_path.join(&Path::new("TEST")),
            recursive: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        touch_test.execute(&mut fs).unwrap();

        let copy_test_to_z = Command(InitializedCopyCommand {
            source: sample_path.join("TEST"),
            destination: sample_path.join("Z"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        copy_test_to_z.execute(&mut fs).unwrap();

        assert!(
            fs.read_dir(sample_path.join(&Path::new("Z")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("Z/TEST").as_path()))
        );
    }
}
