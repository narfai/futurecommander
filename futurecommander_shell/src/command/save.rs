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

use std::{
    fs::File,
    io::prelude::*,
    path::{ Path, PathBuf }
};

use clap::ArgMatches;

use futurecommander_filesystem::{ Container };

use crate::command::{
    Command,
    errors::CommandError
};

pub struct SaveCommand {}

impl Command<SaveCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedSaveCommand>, CommandError> {
        Ok(
            Command(
                InitializedSaveCommand {
                    path:
                    Self::extract_path_from_args(cwd, args, "path").unwrap_or_else(|_| cwd.to_path_buf().join(".fc.json")),
                    overwrite: args.is_present("overwrite")
                }
            )
        )
    }
}

pub struct InitializedSaveCommand {
    pub path: PathBuf,
    pub overwrite: bool
}

impl Command<InitializedSaveCommand> {
    pub fn execute(self, container: &mut Container) -> Result<(), CommandError> {
        if ! self.0.overwrite && self.0.path.exists() {
            return Err(CommandError::AlreadyExists(self.0.path));
        }
        let mut file = File::create(self.0.path.as_path())?;
        file.write_all(container.to_json()?.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    use std::{
        fs::read_to_string
    };

    use crate::{
        command::{
            InitializedCopyCommand,
            AvailableGuard
        },
    };

    use futurecommander_filesystem::{
        sample::Samples
    };

    #[test]
    fn can_export_virtual_state_into_a_file(){
        let mut container = Container::new();
        let sample_path = Samples::init_advanced_chroot("can_export_virtual_state_into_a_file");
        let copy_command = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Interactive
        });

        copy_command.execute(&mut container).unwrap();

        let save_command = Command(InitializedSaveCommand {
            path: sample_path.join("virtual_state.json"),
            overwrite: false
        });

        save_command.execute(&mut container).unwrap();

        let expected : String = format!(
            "[{{\"Copy\":[{{\"source\":\"{}\",\"destination\":\"{}\",\"merge\":false,\"overwrite\":false}},{{}}]}}]",
            sample_path.join("A").to_string_lossy(),
            sample_path.join("APRIME").to_string_lossy(),
        );

        assert!(sample_path.join("virtual_state.json").exists());

        assert_eq!(read_to_string(sample_path.join("virtual_state.json")).unwrap(), expected);
    }
}
