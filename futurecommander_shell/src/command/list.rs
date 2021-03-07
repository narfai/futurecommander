// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    io::Write,
    path::{ Path, PathBuf }
};

use clap::ArgMatches;

use futurecommander_filesystem::{
    ReadableFileSystem,
    Container,
    Entry
};

use crate::command::{
    Command,
    errors::CommandError,
    add_red_color
};

pub struct ListCommand {}

impl Command<ListCommand> {
    pub fn initialize(cwd: &Path, args: &ArgMatches<'_>) -> Result<Command<InitializedListCommand>, CommandError> {
        Ok(
            Command(
                InitializedListCommand {
                    path:
                        Self::extract_path_from_args(cwd, args, "path").unwrap_or_else(|_| cwd.to_path_buf())
                }
            )
        )
    }
}

pub struct InitializedListCommand {
    pub path: PathBuf
}

impl Command<InitializedListCommand> {
    pub fn execute<W : Write>(self, out: &mut W, container: &mut Container) -> Result<(), CommandError> {
        let collection = container.read_dir(self.0.path.as_path())?;
        if ! collection.is_empty() {
            for child in collection.sort().into_iter() {
                let output = format!(
                    "{}    {}",
                   if child.is_dir() {
                       "Directory"
                   } else if child.is_file() {
                       "File     "
                   } else {
                       "Unknown  "
                   },
                    child.name().unwrap().to_string_lossy(),

                );
                writeln!(
                    out,
                    "{}",
                    if child.is_virtual() {
                        add_red_color(&output)
                    } else {
                        output
                    }
                )?;
            }
        } else {
            writeln!(out, "Directory is empty")?;
        }
        Ok(())
    }
}

