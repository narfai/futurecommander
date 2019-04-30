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
mod shell {
    use super::*;


    use std::{
        process::{
            Command
        },
        thread,
        io::Result as IoResult
    };

    use assert_cmd::prelude::*;
    use predicates::prelude::*;

    use file_system::{
        sample::Samples
    };

    fn _debug_command(command: &mut Command) -> IoResult<()> {
        println!("{}", String::from_utf8_lossy(&command.output()?.stdout));
        Ok(())
    }

    #[test]
    #[ignore]
    fn regular_list() -> Result<(), Box<std::error::Error>> {
        let sample_path = Samples::static_samples_path();
        let mut command = Command::cargo_bin("futurecommander")?;
        //Mystic macro behavior - expect both 1 and 2 arguments
        let predicate = predicate::str::contains("Directory    A\nDirectory    B\nFile         F");
        command.arg("ls")
            .arg(sample_path)
            .assert()
            .success()
            .stdout(predicate);
        Ok(())
    }


    #[test]
    #[ignore]
    fn regular_tree() -> Result<(), Box<std::error::Error>> {
        let sample_path = Samples::static_samples_path();
        let mut command = Command::cargo_bin("futurecommander")?;
        let expected = predicate::str::contains("A\n│\n├── .gitkeep\n└── C");
        command.arg("tree")
            .arg(sample_path.join("A"))
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    #[ignore]
    fn regular_new_directory() -> Result<(), Box<std::error::Error>> {
        let sample_path = Samples::init_advanced_chroot("shell_regular_new_directory");
        let state_file = sample_path.join("state.json");
        let mut mkdir_command = Command::cargo_bin("futurecommander")?;
        let expected = predicate::str::contains("");
        mkdir_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("-w")
            .arg("mkdir")
            .arg(sample_path.join("NEWDIRECTORY"))
            .assert()
            .success()
            .stdout(expected);

        let mut ls_command = Command::cargo_bin("futurecommander")?;
        let predicate = predicate::str::contains("NEWDIRECTORY");
        ls_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("ls")
            .arg(sample_path)
            .assert()
            .success()
            .stdout(predicate);

        Ok(())
    }

    #[test]
    #[ignore]
    fn regular_new_file() -> Result<(), Box<std::error::Error>> {
        let sample_path = Samples::init_advanced_chroot("shell_regular_new_file");
        let state_file = sample_path.join("state.json");
        let mut mkdir_command = Command::cargo_bin("futurecommander")?;
        let expected = predicate::str::contains("");
        mkdir_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("-w")
            .arg("touch")
            .arg(sample_path.join("NEWFILE"))
            .assert()
            .success()
            .stdout(expected);

        let mut ls_command = Command::cargo_bin("futurecommander")?;
        let predicate = predicate::str::contains("NEWFILE");
        ls_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("ls")
            .arg(sample_path)
            .assert()
            .success()
            .stdout(predicate);

        Ok(())
    }

    #[test]
    #[ignore]
    fn regular_remove() -> Result<(), Box<std::error::Error>> {
        let sample_path = Samples::init_advanced_chroot("shell_regular_remove");
        let state_file = sample_path.join("state.json");
        let mut rm_command = Command::cargo_bin("futurecommander")?;
        let expected = predicate::str::contains("");
        rm_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("-w")
            .arg("rm")
            .arg(sample_path.join("F"))
            .assert()
            .success()
            .stdout(expected);

        let mut ls_command = Command::cargo_bin("futurecommander")?;
        let predicate = predicate::str::contains("File         F").not();
        ls_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("ls")
            .arg(sample_path)
            .assert()
            .success()
            .stdout(predicate);

        Ok(())
    }

    #[test]
    #[ignore]
    fn regular_mov() -> Result<(), Box<std::error::Error>> {
        let sample_path = Samples::init_advanced_chroot("shell_regular_mov");
        let state_file = sample_path.join("state.json");
        let mut mov_command = Command::cargo_bin("futurecommander")?;
        let expected = predicate::str::contains("");
        mov_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("-w")
            .arg("mv")
            .arg(sample_path.join("A"))
            .arg(sample_path.join("B"))
            .assert()
            .success()
            .stdout(expected);

        let mut ls_command = Command::cargo_bin("futurecommander")?;
        let predicate = predicate::str::contains("Directory    A").not();
        ls_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("ls")
            .arg(sample_path.as_path())
            .assert()
            .success()
            .stdout(predicate);

        let mut ls_command = Command::cargo_bin("futurecommander")?;
        let predicate = predicate::str::contains("Directory    A");
        ls_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("ls")
            .arg(sample_path.join("B"))
            .assert()
            .success()
            .stdout(predicate);

        Ok(())
    }

    #[test]
    #[ignore]
    fn regular_copy() -> Result<(), Box<std::error::Error>> {
        let sample_path = Samples::init_advanced_chroot("shell_regular_copy");
        let state_file = sample_path.join("state.json");
        let mut copy_command = Command::cargo_bin("futurecommander")?;
        let expected = predicate::str::contains("");
        copy_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("-w")
            .arg("cp")
            .arg(sample_path.join("A"))
            .arg(sample_path.join("B"))
            .assert()
            .success()
            .stdout(expected);

        let mut ls_command = Command::cargo_bin("futurecommander")?;
        let predicate = predicate::str::contains("Directory    A");
        ls_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("ls")
            .arg(sample_path.as_path())
            .assert()
            .success()
            .stdout(predicate);

        let mut ls_command = Command::cargo_bin("futurecommander")?;
        let predicate = predicate::str::contains("Directory    A");
        ls_command
            .arg(format!("-s {}", state_file.to_string_lossy()))
            .arg("ls")
            .arg(sample_path.join("B"))
            .assert()
            .success()
            .stdout(predicate);

        Ok(())
    }
}
