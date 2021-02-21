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

use std::path::Path;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::{
    command::{
        errors::CommandError,
        Command,
        AvailableGuard
    }
};

use futurecommander_filesystem::{
    Container,
    Kind,
    CreateOperationDefinition,
    Listener,
    Delayer,
    FileSystemOperation
};

pub struct NewDirectoryCommand {}

impl Command<NewDirectoryCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedNewDirectoryCommand>, CommandError> {
        Ok(
            Command(
                InitializedNewDirectoryCommand {
                    path: Self::extract_path_from_args(cwd, args, "path")?,
                    recursive: args.is_present("recursive"),
                    overwrite: args.is_present("overwrite"),
                    guard: Self::extract_available_guard(args, "guard")?
                }
            )
        )
    }
}

pub struct InitializedNewDirectoryCommand {
    pub path: PathBuf,
    pub recursive: bool,
    pub overwrite: bool,
    pub guard: AvailableGuard

}

impl Command<InitializedNewDirectoryCommand> {
    pub fn execute(self, container: &mut Container) -> Result<(), CommandError> {
        let event = FileSystemOperation::create(
            CreateOperationDefinition::new(
                self.0.path.as_path(),
                Kind::Directory,
                self.0.recursive,
                self.0.overwrite
            )
        );

        let guard = container.emit(&event, self.0.guard.registrar())?;
        container.delay(event, guard);
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use futurecommander_filesystem::{
        sample::Samples,
        EntryAdapter,
        ReadableFileSystem
    };

    #[test]
    fn mkdir(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let new_bde_mkdired = Command(InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("B/D/E/MKDIRED")),
            recursive: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        new_bde_mkdired.execute(&mut container).unwrap();

        assert!(
            container.read_dir(sample_path.join(&Path::new("B/D/E")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/MKDIRED").as_path()))
        );
    }
}
