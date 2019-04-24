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
    HybridFileSystem,
    operation::{
        Operation,
        MoveOperation
    },
    query::{
        Entry,
        Query,
        StatusQuery
    }
};

use crate::command::{
    Command,
    errors::CommandError
};

pub struct MoveCommand {}

impl Command<MoveCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedMoveCommand>, CommandError> {
        let (source, _source_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "source")?;
        let (destination, _destination_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "destination")?;

        Ok(
            Command(InitializedMoveCommand {
                source,
                destination
            })
        )
    }
}

pub struct InitializedMoveCommand {
    pub source: PathBuf,
    pub destination: PathBuf
}

impl Command<InitializedMoveCommand> {
    pub fn execute(&self, fs: &mut HybridFileSystem) -> Result<(), CommandError> {
        let source = StatusQuery::new(self.0.source.as_path()).retrieve(fs.vfs())?;
        let destination = StatusQuery::new(self.0.destination.as_path()).retrieve(fs.vfs())?;

        if ! source.exists() {
            return Err(CommandError::DoesNotExists(self.0.source.to_path_buf()));
        }

        let operation = if destination.exists() {
            if destination.is_dir() {
                MoveOperation::new(
                    self.0.source.as_path(),
                    self.0.destination
                        .join(self.0.source.file_name().unwrap())
                        .as_path(),
                    false,
                    false
                )
            } else if source.is_dir() {
                return Err(CommandError::DirectoryIntoAFile(source.to_path(), destination.to_path()))
            } else {
                return Err(CommandError::CustomError(format!("Overwrite {:?} {:?}", source.is_dir(), destination.is_dir()))) //OVERWRITE
            }
        } else {
            MoveOperation::new(
                self.0.source.as_path(),
                self.0.destination.as_path(),
                false,
                false
            )
        };


        match operation.execute(fs.mut_vfs()) {
            Ok(_) => {
                fs.mut_transaction().add_operation(Box::new(operation));
                Ok(())
            },
            Err(error) => Err(CommandError::from(error))
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use file_system::{
        Samples,
        BusinessError,
        query::{ ReadDirQuery, EntryAdapter }
    };

    #[test]
    fn mv(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::default();

        let move_f_to_a = Command(InitializedMoveCommand {
            source: sample_path.join(&Path::new("F")),
            destination: sample_path.join("A")
        });

        move_f_to_a.execute(&mut fs).unwrap();

        let move_af_to_b = Command(InitializedMoveCommand {
            source: sample_path.join("A/F"),
            destination: sample_path.join("B")
        });

        move_af_to_b.execute(&mut fs).unwrap();

        let move_bf_to_bde = Command(InitializedMoveCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E")
        });

        move_bf_to_bde.execute(&mut fs).unwrap();

        assert!(
            ReadDirQuery::new(sample_path.join(&Path::new("B/D/E")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/F").as_path()))
        );

        assert!(
            !ReadDirQuery::new(sample_path.join(&Path::new("A")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("A/F").as_path()))
        );

        assert!(
            !ReadDirQuery::new(sample_path.join(&Path::new("B")).as_path())
                .retrieve(fs.vfs())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/F").as_path()))
        );
    }

    //Error testing

    #[test]
    fn virtual_shell_move_directory_into_itself_must_not_be_allowed(){
        let sample_path = Samples::static_samples_path();
        let mut fs = HybridFileSystem::default();

        let source = sample_path.join(&Path::new("B"));
        let destination = sample_path.join("B/D");

        let move_b_to_bd = Command(InitializedMoveCommand {
            source: source.clone(),
            destination: destination.clone()
        });

        match move_b_to_bd.execute(&mut fs){
            Err(CommandError::Operation(BusinessError::CopyIntoItSelf(err_source, err_destination))) => {
                assert_eq!(source, err_source);
                assert_eq!(destination.join("B"), err_destination);
            },
            Err(unwanted_error) => panic!("{}", unwanted_error),
            Ok(_) => panic!("Should not be able to move into itself")
        };
    }
}
