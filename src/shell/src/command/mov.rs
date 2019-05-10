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
    MoveEvent,
    ReadableFileSystem,
    Entry,
    Listener,
    Delayer
};

use crate::command::{
    Command,
    errors::CommandError,
    AvailableGuard
};

pub struct MoveCommand {}

impl Command<MoveCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedMoveCommand>, CommandError> {
        let (source, _source_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "source")?;
        let (destination, _destination_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "destination")?;

        Ok(
            Command(InitializedMoveCommand {
                source,
                destination,
                merge: args.is_present("merge"),
                overwrite: args.is_present("overwrite"),
                guard: Self::extract_available_guard(args, "guard")?
            })
        )
    }
}

pub struct InitializedMoveCommand {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub merge: bool,
    pub overwrite: bool,
    pub guard: AvailableGuard
}

impl Command<InitializedMoveCommand> {
    pub fn execute(self, container: &mut Container) -> Result<(), CommandError> {
        let source = container.status(self.0.source.as_path())?;
        let destination = container.status(self.0.destination.as_path())?;

        if ! source.exists() {
            return Err(CommandError::DoesNotExists(self.0.source.to_path_buf()));
        }

        let event = if destination.exists() {
            if destination.is_dir() {
                MoveEvent::new(
                    self.0.source.as_path(),
                    self.0.destination
                        .join(self.0.source.file_name().unwrap())
                        .as_path(),
                    self.0.merge,
                    self.0.overwrite
                )
            } else if source.is_dir() {
                return Err(CommandError::DirectoryIntoAFile(source.to_path(), destination.to_path()))
            } else {
                return Err(CommandError::CustomError(format!("Overwrite {:?} {:?}", source.is_dir(), destination.is_dir()))) //OVERWRITE
            }
        } else {
            MoveEvent::new(
                self.0.source.as_path(),
                self.0.destination.as_path(),
                self.0.merge,
                self.0.overwrite
            )
        };

        let guard = container.emit(&event, self.0.guard.registrar())?;
        container.delay(Box::new(event), guard);
        Ok(())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use file_system::{
        sample::Samples,
        EntryAdapter
    };

    #[test]
    fn mv(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let move_f_to_a = Command(InitializedMoveCommand {
            source: sample_path.join(&Path::new("F")),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        move_f_to_a.execute(&mut container).unwrap();

        let move_af_to_b = Command(InitializedMoveCommand {
            source: sample_path.join("A/F"),
            destination: sample_path.join("B"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        move_af_to_b.execute(&mut container).unwrap();

        let move_bf_to_bde = Command(InitializedMoveCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        move_bf_to_bde.execute(&mut container).unwrap();

        assert!(
            container.read_dir(sample_path.join(&Path::new("B/D/E")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/D/E/F").as_path()))
        );

        assert!(
            !container.read_dir(sample_path.join(&Path::new("A")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("A/F").as_path()))
        );

        assert!(
            !container.read_dir(sample_path.join(&Path::new("B")).as_path())
                .unwrap()
                .contains(&EntryAdapter(sample_path.join("B/F").as_path()))
        );
    }
}
