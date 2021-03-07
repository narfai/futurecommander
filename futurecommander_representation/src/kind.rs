// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::{ PathBuf, Path };

#[derive(Clone, Debug, Copy)]
pub enum Kind {
    File,
    Directory,
    Unknown
}

impl Kind {
    pub fn from_path(path: &Path) -> Kind {
        if path.is_dir() {
            Kind::Directory
        } else if path.is_file() {
            Kind::File
        } else {
            Kind::Unknown
        }
    }

    pub fn from_path_buf(path: PathBuf) -> Kind {
        Self::from_path(path.as_path())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Kind) -> bool {
        match &self {
            Kind::File => match other {
                Kind::File => true,
                Kind::Directory => false,
                Kind::Unknown => false
            },
            Kind::Directory => match other {
                Kind::File => false,
                Kind::Directory => true,
                Kind::Unknown => false
            }
            Kind::Unknown => match other {
                Kind::File => false,
                Kind::Directory => false,
                Kind::Unknown => true
            }
        }
    }
}
