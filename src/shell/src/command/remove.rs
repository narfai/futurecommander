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

use futurecommander_vfs::{
    HybridFileSystem,
    operation::{
        Operation,
        RemoveOperation
    }
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
                    path
                }
            )
        )
    }
}

pub struct InitializedRemoveCommand {
    pub path: PathBuf
}

impl Command<InitializedRemoveCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let operation = RemoveOperation::new(self.0.path.as_path());

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
        query::{ Query, StatusQuery, Entry }
    };

    #[test]
    fn rm(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::default();

        let b_path = sample_path.join(&Path::new("B"));

        let remove_b = Command(InitializedRemoveCommand {
            path: b_path.to_path_buf()
        });

        remove_b.execute(&mut fs).unwrap();

        assert!(!StatusQuery::new(b_path.as_path())
            .retrieve(fs.vfs())
            .unwrap()
            .exists()
        )
    }
}
