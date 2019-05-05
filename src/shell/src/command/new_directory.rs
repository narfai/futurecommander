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
use crate::command::{ Command };
use crate::command::errors::CommandError;

use file_system::{
    Container,
    Kind,
    CreateEvent,
    Listener,
    Delayer
};

pub struct NewDirectoryCommand {}

impl Command<NewDirectoryCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedNewDirectoryCommand>, CommandError> {
        Ok(
            Command(
                InitializedNewDirectoryCommand {
                    path: Self::extract_path_from_args(cwd, args, "path")?
                }
            )
        )
    }
}

pub struct InitializedNewDirectoryCommand {
    pub path: PathBuf
}

impl Command<InitializedNewDirectoryCommand> {
    pub fn execute(self, fs: &mut Container) -> Result<(), CommandError> {
        let event = CreateEvent::new(
            self.0.path.as_path(),
            Kind::Directory,
            false,
            false
        );

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
        EntryAdapter,
        ReadableFileSystem
    };

    #[test]
    fn mkdir(){
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let new_bde_mkdired = Command(InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("B/D/E/MKDIRED"))
        });

        new_bde_mkdired.execute(&mut fs).unwrap();

        assert!(
            fs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/MKDIRED").as_path()))
        );
    }
}
