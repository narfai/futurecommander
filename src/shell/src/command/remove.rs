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

use file_system::{
    Container,
    RemoveEvent,
    Listener,
    Delayer
};

use crate::command::{
    Command,
    errors::CommandError
};


pub struct RemoveCommand {}

impl Command<RemoveCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedRemoveCommand>, CommandError> {
        let path = Self::extract_path_from_args(cwd, args, "path")?;
        for ancestor in cwd.ancestors() {
            if path == ancestor {
                return Err(CommandError::CwdIsInside(path.to_path_buf()))
            }
        }

        Ok(
            Command(
                InitializedRemoveCommand {
                    path,
                    recursive: args.is_present("recursive")
                }
            )
        )
    }
}

pub struct InitializedRemoveCommand {
    pub path: PathBuf,
    pub recursive: bool
}

impl Command<InitializedRemoveCommand> {
    pub fn execute(self, fs: &mut Container) -> Result<(), CommandError> {
        let event = RemoveEvent::new(self.0.path.as_path(), self.0.recursive);

        fs.emit(&event)?;
        fs.delay(Box::new(event));
        Ok(())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use file_system::{
        sample::Samples,
        ReadableFileSystem,
        Entry
    };

    #[test]
    fn rm(){
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let b_path = sample_path.join(&Path::new("B"));

        let remove_b = Command(InitializedRemoveCommand {
            path: b_path.to_path_buf(),
            recursive: true
        });

        remove_b.execute(&mut fs).unwrap();

        assert!(
            !fs.status(b_path.as_path())
                .unwrap()
                .exists()
        )
    }
}
