// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::{ Path, PathBuf };

use clap::ArgMatches;

use futurecommander_filesystem::{
    Container,
    ReadableFileSystem,
    Entry,
    Capabilities
};

use crate::{
    command::{
        errors::CommandError,
        Command,
        AvailableGuard
    }
};

pub struct CopyCommand {}

impl Command<CopyCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedCopyCommand>, CommandError> {
        let (source, _source_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "source")?;
        let (destination, _destination_trailing) = Self::extract_path_and_trail_from_args(cwd, args, "destination")?;

        Ok(
            Command(InitializedCopyCommand {
                source,
                destination,
                merge: args.is_present("merge"),
                overwrite: args.is_present("overwrite"),
                guard: Self::extract_available_guard(args, "guard")?
            })
        )
    }
}

pub struct InitializedCopyCommand {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub merge: bool,
    pub overwrite: bool,
    pub guard: AvailableGuard
}

impl Command<InitializedCopyCommand> {
    pub fn execute(self, container: &mut Container) -> Result<(), CommandError> {
        let source = container.status(self.0.source.as_path())?;
        let destination = container.status(self.0.destination.as_path())?;

        if ! source.exists() {
            return Err(CommandError::DoesNotExists(self.0.source));
        }

        if destination.exists() {
            if destination.is_dir() {
                container.copy(
                    &self.0.source,
                    &self.0.destination.join(self.0.source.file_name().unwrap()),
                    &mut *self.0.guard.to_guard(Capabilities::new(self.0.merge, self.0.overwrite, false))
                )?;
            } else if source.is_dir() {
                return Err(CommandError::DirectoryIntoAFile(source.to_path(), destination.to_path()))
            } else {
                return Err(CommandError::CustomError(format!("Overwrite {:?} {:?}", source.is_dir(), destination.is_dir()))) //OVERWRITE
            }
        } else {
            container.copy(
                &self.0.source,
                &self.0.destination,
                &mut *self.0.guard.to_guard(Capabilities::new(self.0.merge, self.0.overwrite, false))
            )?;
        }
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use futurecommander_filesystem::{
        sample::Samples,
        EntryAdapter
    };

    #[test]
    fn cp_only(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let copy_a_to_b = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        copy_a_to_b.execute(&mut container).unwrap();

        let copy_ab_to_abd = Command(InitializedCopyCommand {
            source: sample_path.join("A/B/D"),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        copy_ab_to_abd.execute(&mut container).unwrap();

        let collection = container.read_dir(sample_path.join("A/D").as_path()).unwrap();

        assert!(!collection.is_empty());
        assert!(collection.contains(&EntryAdapter(sample_path.join("A/D/G").as_path())));
        assert!(collection.contains(&EntryAdapter(sample_path.join("A/D/E").as_path())));

    }

    #[test]
    fn cp_preserve_source_and_node_kind(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        copy_b_to_a.execute(&mut container).unwrap();

        let copy_f_to_b = Command(InitializedCopyCommand {
            source: sample_path.join("F"),
            destination: sample_path.join("B"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        copy_f_to_b.execute(&mut container).unwrap();

        let copy_bf_to_bde = Command(InitializedCopyCommand {
            source: sample_path.join("B/F"),
            destination: sample_path.join("B/D/E"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });

        copy_bf_to_bde.execute(&mut container).unwrap();

        let collection = container.read_dir(sample_path.join("B/D/E").as_path()).unwrap();
        assert!(!collection.is_empty());
        assert!(collection.contains(&EntryAdapter(sample_path.join("B/D/E/F").as_path())));
    }

    #[test]
    fn virtual_shell_copy_nested_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        copy_b_to_a.execute(&mut container).unwrap();

        let copy_a_as_aprime = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        copy_a_as_aprime.execute(&mut container).unwrap();

        let collection_aprime = container.read_dir(sample_path.join(&Path::new("APRIME")).as_path())
            .unwrap();

        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));
        assert!(collection_aprime.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));

        let collection_aprime_b_d = container.read_dir(sample_path.join(&Path::new("APRIME/B/D")).as_path())
            .unwrap();

        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/E").as_path())));
        assert!(collection_aprime_b_d.contains(&EntryAdapter(sample_path.join("APRIME/B/D/G").as_path())));
    }

    #[test]
    fn virtual_shell_copy_nested_deep_through_virtual_identity(){
        let sample_path = Samples::static_samples_path();
        let mut container = Container::new();

        let copy_b_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("B"),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        copy_b_to_a.execute(&mut container).unwrap();

        let copy_a_as_aprime = Command(InitializedCopyCommand {
            source: sample_path.join("A"),
            destination: sample_path.join("APRIME"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        copy_a_as_aprime.execute(&mut container).unwrap();

        let copy_aprime_as_abeta = Command(InitializedCopyCommand {
            source: sample_path.join("APRIME"),
            destination: sample_path.join("ABETA"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        copy_aprime_as_abeta.execute(&mut container).unwrap();

        let copy_abeta_to_a = Command(InitializedCopyCommand {
            source: sample_path.join("ABETA"),
            destination: sample_path.join("A"),
            merge: false,
            overwrite: false,
            guard: AvailableGuard::Zealed
        });
        copy_abeta_to_a.execute(&mut container).unwrap();

        let collection_a_abeta = container.read_dir(sample_path.join(&Path::new("APRIME")).as_path())
            .unwrap();

        assert!(collection_a_abeta.contains(&EntryAdapter(sample_path.join("APRIME/C").as_path())));
        assert!(collection_a_abeta.contains(&EntryAdapter(sample_path.join("APRIME/B").as_path())));

        let collection_aprime_a_abeta_b_d = container.read_dir(sample_path.join("A/ABETA/B/D").as_path())
            .unwrap();

        assert!(collection_aprime_a_abeta_b_d.contains(&EntryAdapter(sample_path.join("A/ABETA/B/D/E").as_path())));
        assert!(collection_aprime_a_abeta_b_d.contains(&EntryAdapter(sample_path.join("A/ABETA/B/D/G").as_path())));
    }
}
