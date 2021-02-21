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

use std::path::{ Path, PathBuf };

use clap::ArgMatches;

use futurecommander_filesystem::{
    Container,
    RemoveOperationDefinition,
    Listener,
    FileSystemOperation
};

use crate::command::{
    Command,
    errors::CommandError,
    AvailableGuard
};


pub struct RemoveCommand {}

impl Command<RemoveCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedRemoveCommand>, CommandError> {
        let path = Self::extract_path_from_args(cwd, args, "path")?;
        for ancestor in cwd.ancestors() {
            if path == ancestor {
                return Err(CommandError::CwdIsInside(path))
            }
        }

        Ok(
            Command(
                InitializedRemoveCommand {
                    path,
                    recursive: args.is_present("recursive"),
                    guard: Self::extract_available_guard(args, "guard")?
                }
            )
        )
    }
}

pub struct InitializedRemoveCommand {
    pub path: PathBuf,
    pub recursive: bool,
    pub guard: AvailableGuard
}

impl Command<InitializedRemoveCommand> {
    pub fn execute(self, container: &mut Container) -> Result<(), CommandError> {
        container.emit(
            FileSystemOperation::remove(
                RemoveOperationDefinition::new(self.0.path.as_path(), self.0.recursive)
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
        Entry
    };

    #[test]
    fn rm(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let b_path = sample_path.join(&Path::new("B"));

        // https://github.com/rust-lang/rust-clippy/issues/5595
        #[allow(clippy::redundant_clone)]
        let remove_b = Command(InitializedRemoveCommand {
             path: b_path.to_path_buf(),
            recursive: true,
            guard: AvailableGuard::Zealed
        });

        remove_b.execute(&mut container).unwrap();

        assert!(
            !container.status(b_path.as_path())
                .unwrap()
                .exists()
        )
    }
}
