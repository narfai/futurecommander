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

use std::path::{ PathBuf, Path };

#[derive(Clone, Debug, Copy)]
pub enum Kind {
    File,
    Directory,
    Unknown
}

impl Kind {
    pub fn from_path(path: &Path) -> Kind {
        match path.is_dir() {
            true => Kind::Directory,
            false =>
                match path.is_file() {
                    true => Kind::File,
                    false => Kind::Unknown
                }
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
