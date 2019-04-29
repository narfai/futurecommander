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

extern crate futurecommander_shell;
extern crate file_system;


#[cfg_attr(tarpaulin, skip)]
mod command_integration {
    use super::*;

    use std::{
        path::Path,
        fs::read_to_string
    };

    use futurecommander_shell::{
        command::{
            Command,
            InitializedMoveCommand,
            InitializedNewDirectoryCommand,
            InitializedNewFileCommand,
            InitializedCopyCommand,
            InitializedSaveCommand,
            InitializedImportCommand
        },
    };

    use file_system::{
        sample::Samples,
        Container,
        EntryAdapter,
        ReadableFileSystem,
        Entry
    };

    #[test]
    fn virtual_shell_move_nested_virtual_identity() {
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

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
            fs.read_dir(sample_path.join(&Path::new("Z")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("Z/TEST").as_path()))
        );
    }

    #[test]
    fn can_export_virtual_state_into_a_file(){
        let mut fs = Container::new();
        let sample_path = Samples::init_advanced_chroot("can_export_virtual_state_into_a_file");
        let copy_command = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME")
        });

        copy_command.execute(&mut fs).unwrap();

        let save_command = Command(InitializedSaveCommand {
            path: sample_path.join("virtual_state.json"),
            overwrite: false
        });

        save_command.execute(&mut fs).unwrap();

        let expected : String = format!(
            "[{{\"type\":\"CopyEvent\",\"source\":\"{}\",\"destination\":\"{}\",\"merge\":true,\"overwrite\":false}}]",
            sample_path.join("A").to_string_lossy(),
            sample_path.join("APRIME").to_string_lossy(),
        );

        assert!(sample_path.join("virtual_state.json").exists());

        assert_eq!(read_to_string(sample_path.join("virtual_state.json")).unwrap(), expected);
    }

    #[test]
    fn can_import_virtual_state_from_a_file(){
        let mut fs = Container::new();
        let sample_path = Samples::init_advanced_chroot("can_import_virtual_state_from_a_file");
        let copy_command = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME")
        });

        copy_command.execute(&mut fs).unwrap();

        let save_command = Command(InitializedSaveCommand {
            path: sample_path.join("virtual_state.json"),
            overwrite: false
        });

        save_command.execute(&mut fs).unwrap();

        let import_command = Command(InitializedImportCommand {
            path: sample_path.join("virtual_state.json")
        });

        let mut container_b = Container::new();

        import_command.execute(&mut container_b).unwrap();

        assert!(!container_b.is_empty());
        let b_stat = fs.status(sample_path.join("APRIME").as_path()).unwrap();
        assert!(b_stat.exists());
        assert!(b_stat.is_dir());
    }
}
