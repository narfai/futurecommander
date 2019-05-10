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
extern crate futurecommander_filesystem;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod shell {
    use super::*;

    use std::{
        str::from_utf8
    };

    use futurecommander_filesystem::{
        sample::Samples
    };

    use futurecommander_shell::{
        Shell
    };

    #[test]
    fn regular_list() {
        let sample_path = Samples::static_samples_path();
        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec!["futurecommander", "ls", sample_path.to_str().unwrap()];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        let expected = "Directory    A\nDirectory    B\nFile         F\n".to_string();
        assert_eq!(expected, from_utf8(&stdout).unwrap());
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());
    }


    #[test]
    fn regular_tree() {
        let sample_path = Samples::static_samples_path();
        let target = sample_path.join("A");

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec!["futurecommander", "tree", target.to_str().unwrap()];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        let expected = "A\n│\n├── .gitkeep\n└── C\n".to_string();
        assert_eq!(expected, from_utf8(&stdout).unwrap());
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());
    }

    #[test]
    fn regular_new_directory() {
        let sample_path = Samples::init_advanced_chroot("regular_new_directory");
        let state_file = sample_path.join("state.json");
        let state_arg = format!("-s {}", state_file.to_string_lossy());
        let target = sample_path.join("NEWDIRECTORY");

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "-w",
            "mkdir",
            target.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert_eq!("".to_string(), from_utf8(&stdout).unwrap());
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "ls",
            sample_path.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert!(from_utf8(&stdout).unwrap().contains("Directory    NEWDIRECTORY"));
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());
    }

    #[test]
    fn regular_new_file() {
        let sample_path = Samples::init_advanced_chroot("regular_new_file");
        let state_file = sample_path.join("state.json");
        let state_arg = format!("-s {}", state_file.to_string_lossy());
        let target = sample_path.join("NEWFILE");

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "-w",
            "touch",
            target.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert_eq!("".to_string(), from_utf8(&stdout).unwrap());
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "ls",
            sample_path.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert!(from_utf8(&stdout).unwrap().contains("File         NEWFILE"));
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());
    }

    #[test]
    fn regular_remove() {
        let sample_path = Samples::init_advanced_chroot("regular_remove");
        let state_file = sample_path.join("state.json");
        let state_arg = format!("-s {}", state_file.to_string_lossy());
        let target = sample_path.join("F");

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "-w",
            "rm",
            target.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert_eq!("".to_string(), from_utf8(&stdout).unwrap());
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "ls",
            sample_path.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert!(!from_utf8(&stdout).unwrap().contains("File         F"));
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());
    }

    #[test]
    fn regular_mov() {
        let sample_path = Samples::init_advanced_chroot("regular_mov");
        let state_file = sample_path.join("state.json");
        let state_arg = format!("-s {}", state_file.to_string_lossy());
        let source = sample_path.join("A");
        let destination = sample_path.join("B");

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "-w",
            "mv",
            source.to_str().unwrap(),
            destination.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert_eq!("".to_string(), from_utf8(&stdout).unwrap());
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "ls",
            sample_path.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert!(!from_utf8(&stdout).unwrap().contains("Directory    A"));
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());  let mut shell = Shell::default();

        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "ls",
            destination.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        println!("{:?}", from_utf8(&stdout).unwrap());

        assert!(from_utf8(&stdout).unwrap().contains("Directory    A"));
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());
    }

    #[test]
    fn regular_copy() {
        let sample_path = Samples::init_advanced_chroot("regular_copy");
        let state_file = sample_path.join("state.json");
        let state_arg = format!("-s {}", state_file.to_string_lossy());
        let source = sample_path.join("A");
        let destination = sample_path.join("B");

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "-w",
            "cp",
            source.to_str().unwrap(),
            destination.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert_eq!("".to_string(), from_utf8(&stdout).unwrap());
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());

        let mut shell = Shell::default();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "ls",
            sample_path.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        assert!(from_utf8(&stdout).unwrap().contains("Directory    A"));
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());  let mut shell = Shell::default();

        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let args = vec![
            "futurecommander",
            state_arg.as_str(),
            "ls",
            destination.to_str().unwrap()
        ];

        shell.run_single(
            args.iter().map(|s| s.to_string()),
            &mut stdout,
            &mut stderr
        ).unwrap();

        println!("{:?}", from_utf8(&stdout).unwrap());

        assert!(from_utf8(&stdout).unwrap().contains("Directory    A"));
        assert_eq!("".to_string(), from_utf8(&stderr).unwrap());
    }
}
