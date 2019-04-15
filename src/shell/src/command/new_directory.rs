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

use futurecommander_vfs::{
    HybridFileSystem,
    Kind,
    operation::{
        Operation,
        CreateOperation
    },
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
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let operation = CreateOperation::new(
            self.0.path.as_path(),
            Kind::Directory
        );

        match operation.execute(fs.mut_vfs()) {
            Ok(_)       => {
                fs.mut_transaction().add_operation(Box::new(operation.clone()));
                Ok(())
            }
            Err(error)  => Err(CommandError::from(error))
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use futurecommander_vfs::{
        Samples,
        query::{Query, ReadDirQuery, EntryAdapter}
    };

    #[test]
    fn mkdir(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::default();

        let new_bde_mkdired = Command(InitializedNewDirectoryCommand {
            path: sample_path.join(&Path::new("B/D/E/MKDIRED"))
        });

        new_bde_mkdired.execute(&mut fs).unwrap();

        assert!(
            ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/MKDIRED").as_path()))
        );
    }
}
