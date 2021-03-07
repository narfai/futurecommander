// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    fs::{ read_to_string },
    path::{ Path, PathBuf }
};

use clap::ArgMatches;

use futurecommander_filesystem::{ Container };

use crate::command::{
    Command,
    errors::CommandError
};

pub struct ImportCommand {}

impl Command<ImportCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedImportCommand>, CommandError> {
        Ok(
            Command(
                InitializedImportCommand {
                    path:
                    Self::extract_path_from_args(cwd, args, "path").unwrap_or_else(|_| cwd.to_path_buf().join(".fc.json"))
                }
            )
        )
    }
}

pub struct InitializedImportCommand {
    pub path: PathBuf
}

impl Command<InitializedImportCommand> {
    pub fn execute(self, container: &mut Container) -> Result<(), CommandError> {
        if ! self.0.path.exists() {
            return Err(CommandError::DoesNotExists(self.0.path));
        }

        container.emit_json(read_to_string(self.0.path.as_path())?)?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    use crate::{
        command::{
            InitializedSaveCommand,
            InitializedCopyCommand,
            AvailableGuard
        },
    };

    use futurecommander_filesystem::{
        sample::Samples,
        ReadableFileSystem,
        Entry
    };

    #[test]
    fn can_import_virtual_state_from_a_file(){
        let mut container = Container::new();
        let sample_path = Samples::init_advanced_chroot("can_import_virtual_state_from_a_file");
        let copy_command = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        copy_command.execute(&mut container).unwrap();

        let save_command = Command(InitializedSaveCommand {
            path: sample_path.join("virtual_state.json"),
            overwrite: false
        });

        save_command.execute(&mut container).unwrap();

        let import_command = Command(InitializedImportCommand {
            path: sample_path.join("virtual_state.json")
        });

        let mut container_b = Container::new();

        import_command.execute(&mut container_b).unwrap();

        assert!(!container_b.is_empty());
        let b_stat = container.status(sample_path.join("APRIME").as_path()).unwrap();
        assert!(b_stat.exists());
        assert!(b_stat.is_dir());
    }
}
