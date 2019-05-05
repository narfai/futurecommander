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
    CopyEvent,
    Listener,
    Delayer,
    ReadableFileSystem,
    Entry
};

use crate::command::{ Command };
use crate::command::errors::CommandError;


pub struct CopyCommand {}

impl Command<CopyCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedCopyCommand>, CommandError> {
        let (source, _source_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "source")?;
        let (destination, _destination_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "destination")?;

        Ok(
            Command(InitializedCopyCommand {
                source,
                destination
            })
        )
    }
}

pub struct InitializedCopyCommand {
    pub source: PathBuf,
    pub destination: PathBuf
}

impl Command<InitializedCopyCommand> {
    pub fn execute(self, fs: &mut Container) -> Result<(), CommandError> {
        let source = fs.status(self.0.source.as_path())?;
        let destination = fs.status(self.0.destination.as_path())?;

        if ! source.exists() {
            return Err(CommandError::DoesNotExists(self.0.source.to_path_buf()));
        }

        let event = if destination.exists() {
            if destination.is_dir() {
                CopyEvent::new(
                    self.0.source.as_path(),
                    self.0.destination
                        .join(self.0.source.file_name().unwrap())
                        .as_path(),
                    true,
                    false
                )
            } else if source.is_dir() {
                return Err(CommandError::DirectoryIntoAFile(source.to_path(), destination.to_path()))
            } else {
                return Err(CommandError::CustomError(format!("Overwrite {:?} {:?}", source.is_dir(), destination.is_dir()))) //OVERWRITE
            }
        } else {
            CopyEvent::new(self.0.source.as_path(), self.0.destination.as_path(), true, false)
        };

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
        EntryAdapter
    };

    #[test]
    fn cp_only(){
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let copy_a_to_b = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });

        copy_a_to_b.execute(&mut fs).unwrap();

        let copy_ab_to_abd = Command(InitializedCopyCommand {
            source: sample_path.join("A/B/D"),
            destination: sample_path.join("A")
        });

        copy_ab_to_abd.execute(&mut fs).unwrap();

        let collection = fs.read_dir(sample_path.join("A/D").as_path()).unwrap();

        assert!(!collection.is_empty());
        assert!(collection.contains(&EntryAdapter(sample_path.join("A/D/G").as_path())));
        assert!(collection.contains(&EntryAdapter(sample_path.join("A/D/E").as_path())));

    }

    #[test]
    fn cp_preserve_source_and_node_kind(){
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });

        copy_b_to_a.execute(&mut fs).unwrap();

        let copy_f_to_b = Command(InitializedCopyCommand {
            source: sample_path.join("F"),
            destination: sample_path.join("B")
        });

        copy_f_to_b.execute(&mut fs).unwrap();

        let copy_bf_to_bde = Command(InitializedCopyCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E")
        });

        copy_bf_to_bde.execute(&mut fs).unwrap();

        let collection = fs.read_dir(sample_path.join("B/D/E").as_path()).unwrap();
        assert!(!collection.is_empty());
        assert!(collection.contains(&EntryAdapter(sample_path.join("B/D/E/F").as_path())));
    }

    #[test]
    fn virtual_shell_copy_nested_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });
        copy_b_to_a.execute(&mut fs).unwrap();

        let copy_a_as_aprime = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME")
        });
        copy_a_as_aprime.execute(&mut fs).unwrap();

        let collection_aprime = fs.read_dir(sample_path.join(&Path::new("APRIME")).as_path())
            .unwrap();

        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));
        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));

        let collection_aprime_b_d = fs.read_dir(sample_path.join(&Path::new("APRIME/B/D")).as_path())
            .unwrap();

        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/E").as_path())));
        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/G").as_path())));
    }

    #[test]
    fn virtual_shell_copy_nested_deep_through_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut fs = Container::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A")
        });
        copy_b_to_a.execute(&mut fs).unwrap();

        let copy_a_as_aprime = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME").to_path_buf()
        });
        copy_a_as_aprime.execute(&mut fs).unwrap();

        let copy_aprime_as_abeta = Command(InitializedCopyCommand {
            source: sample_path.join("APRIME"),
            destination: sample_path.join("ABETA").clone()
        });
        copy_aprime_as_abeta.execute(&mut fs).unwrap();

        let copy_abeta_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("ABETA"),
            destination: sample_path.join("A")
        });
        copy_abeta_to_a.execute(&mut fs).unwrap();

        let collection_a_abeta = fs.read_dir(sample_path.join(&Path::new("APRIME")).as_path())
            .unwrap();

        assert!(collection_a_abeta.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));
        assert!(collection_a_abeta.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));

        let collection_aprime_a_abeta_b_d = fs.read_dir(sample_path.join("A/ABETA/B/D").as_path())
            .unwrap();

        assert!(collection_aprime_a_abeta_b_d.contains(&EntryAdapter(sample_path.join("A/ABETA/B/D/E").as_path())));
        assert!(collection_aprime_a_abeta_b_d.contains(&EntryAdapter(sample_path.join("A/ABETA/B/D/G").as_path())));
    }
}
