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

#[allow(unused_imports)]
use std::collections::hash_map::{ DefaultHasher };

use std::{
    cmp::Ordering,
    path::{
        PathBuf,
        Path
    },
    str::{ FromStr },
    hash::{ Hash, Hasher }
};

use crate::{
    Kind,
    errors::RepresentationError
};


#[derive(Clone, Debug)]
pub struct VirtualPath {
    pub identity: PathBuf,
    pub source: Option<PathBuf>,
    pub kind: Kind
}

impl Eq for VirtualPath {}


impl FromStr for VirtualPath {
    type Err = RepresentationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VirtualPath::from(PathBuf::from(s), None, Kind::Unknown)
    }
}

//Rely on PathBuf implementation for identify & order VirtualPaths over Iterators
impl Ord for VirtualPath {
    fn cmp(&self, other: &VirtualPath) -> Ordering {
        self.identity.cmp(&other.identity)
    }
}

impl PartialOrd for VirtualPath {
    fn partial_cmp(&self, other: &VirtualPath) -> Option<Ordering> {
        Some(self.identity.cmp(&other.identity))
    }
}

impl PartialEq for VirtualPath {
    fn eq(&self, other: &VirtualPath) -> bool {
        self.identity.eq(&other.identity)
    }
}

impl Hash for VirtualPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identity.hash(state);
    }
}


/*
Virtual wrapper of PathBuf for keeping control over type & behaviors
PathBuf implementation will do the job for path components manipulation.
*/
impl VirtualPath {
    //Slices / Refs
    pub fn as_identity(&self) -> &Path {
        self.identity.as_path()
    }

    pub fn as_source(&self) -> Option<&Path> {
        match &self.source {
            Some(source) => Some(source.as_path()),
            None => None
        }
    }

    pub fn as_kind(&self) -> &Kind {
        &self.kind
    }

    //Conversions / Copy
    pub fn to_identity(&self) -> PathBuf {
        self.identity.to_path_buf()
    }

    pub fn to_source(&self) -> Option<PathBuf> {
        match &self.source {
            Some(source) => Some(source.to_path_buf()),
            None => None
        }
    }

    pub fn to_kind(&self) -> Kind {
        match self.kind {
            Kind::File => Kind::File,
            Kind::Directory => Kind::Directory,
            Kind::Unknown => Kind::Unknown
        }
    }

    pub fn from(identity: PathBuf, source: Option<PathBuf>, kind: Kind) -> Result<VirtualPath, RepresentationError> {
        if identity.is_relative() && (identity != PathBuf::new()) {
            Err(RepresentationError::IsRelativePath(identity))
        } else {
            Ok(Self::_from(identity, source, kind))
        }
    }

    fn _from(identity: PathBuf, source: Option<PathBuf>, kind: Kind) -> VirtualPath {
        VirtualPath {
            identity,
            source,
            kind
        }
    }

    pub fn from_path(path: &Path) -> Result<VirtualPath, RepresentationError> {
        VirtualPath::from(path.to_path_buf(), None, Kind::Unknown)
    }

    pub fn from_path_buf(path: PathBuf) -> Result<VirtualPath, RepresentationError> {
        VirtualPath::from(path, None, Kind::Unknown)
    }

    pub fn into_parent(self) -> Option<PathBuf> {
        match self.identity.parent() {
            Some(parent) => Some(parent.to_path_buf()),
            None => None
        }
    }

    pub fn replace_parent(path: &Path, new_parent: &Path) -> PathBuf {
        match path.parent(){
            Some(parent) => {
                let stripped = path.strip_prefix(parent).unwrap(); //Do not handle ".." file names
                new_parent.join(stripped)
            },
            None => new_parent.join(path)
        }
    }

    pub fn with_new_identity_parent(self, new_parent: &Path) -> VirtualPath  {
        Self::_from(
            Self::replace_parent(self.as_identity(), new_parent),
            self.source,
            self.kind
        )
    }

    pub fn with_new_source_parent(self, new_parent: &Path) -> VirtualPath  {
        Self::_from(
            self.identity,
            match &self.source {
                Some(source) => Some(Self::replace_parent(source.as_path(), new_parent)),
                None => None
            },
            self.kind
        )
    }

    pub fn with_source(self, new_source: Option<&Path>) -> VirtualPath  {
        Self::_from(
            self.identity,
            match new_source {
                Some(source) => Some(source.to_path_buf()),
                None => None
            },
            self.kind
        )
    }

    pub fn with_kind(self, kind: Kind) -> VirtualPath  {
        Self::_from(
            self.identity,
            self.source,
            kind
        )
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtually_equal() {
        let vpath1 = VirtualPath::from_str("/intentionally/virtual/full/path").unwrap();
        let vpath2 = VirtualPath::from_str("/intentionally/virtual/full/path").unwrap();
        assert_eq!(vpath1, vpath2);
    }

    #[test]
    fn parent_virtually_equal() {
        let parent = VirtualPath::from_str("/intentionally/virtual/full/").unwrap();
        let child = VirtualPath::from_str("/intentionally/virtual/full/path").unwrap();
        assert_eq!(parent, VirtualPath::from_path_buf(child.into_parent().unwrap()).unwrap());
    }

    #[test]
    fn still_equal_with_source_diff() {
        let vpath1 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            None,
            Kind::File
        ).unwrap();
        let vpath2 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            Some(PathBuf::from("/another/source/path")),
            Kind::File
        ).unwrap();
        assert_eq!(vpath1, vpath2);
    }

    #[test]
    fn hash_with_source_equal() {
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        let vpath1 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            None,
            Kind::File
        ).unwrap();
        let vpath2 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            Some(PathBuf::from("/another/source/path")),
            Kind::File
        ).unwrap();
        assert_eq!(
            calculate_hash(&vpath1),
            calculate_hash(&vpath2)
        );
    }
}
