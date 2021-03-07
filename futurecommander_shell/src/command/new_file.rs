// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::Path;
use std::path::PathBuf;

use clap::ArgMatches;

use futurecommander_filesystem::{
    Container,
    Kind,
    CreateOperationDefinition,
    Listener,
    FileSystemOperation
};

use crate::command::{
    Command,
    errors::CommandError,
    AvailableGuard
};

pub struct NewFileCommand {}

impl Command<NewFileCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedNewFileCommand>, CommandError> {
        Ok(
            Command(
                InitializedNewFileCommand {
                    path: Self::extract_path_from_args(cwd, args, "path")?,
                    recursive: args.is_present("recursive"),
                    overwrite: args.is_present("overwrite"),
                    guard: Self::extract_available_guard(args, "guard")?
                }
            )
        )
    }
}

pub struct InitializedNewFileCommand {
    pub path: PathBuf,
    pub recursive: bool,
    pub overwrite: bool,
    pub guard: AvailableGuard
}

impl Command<InitializedNewFileCommand> {
    pub fn execute(self, container: &mut Container) -> Result<(), CommandError> {
        container.emit(
            FileSystemOperation::create(
                CreateOperationDefinition::new(
                    self.0.path.as_path(),
                    Kind::File,
                    self.0.recursive,
                    self.0.overwrite
                )
            ),
            self.0.guard.to_guard()
        )?;
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use futurecommander_filesystem::{
        sample::Samples,
        ReadableFileSystem,
        EntryAdapter
    };

    #[test]
    fn touch(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let new_bde_touched = Command(InitializedNewFileCommand {
            path: sample_path.join(&Path::new("B/D/E/TOUCHED")),
            recursive: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        new_bde_touched.execute(&mut container).unwrap();

        assert!(
            container.read_dir(sample_path.join(&Path::new("B/D/E")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/TOUCHED").as_path()))
        );
    }
}
