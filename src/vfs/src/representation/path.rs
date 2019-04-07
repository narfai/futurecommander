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

use std::cmp::Ordering;
use std::path::{ PathBuf, Path };
use std::ffi::{ OsStr };
use std::hash::{ Hash, Hasher };
use std::path::MAIN_SEPARATOR;

use crate::VfsError;

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

#[derive(Clone, Debug)]
pub struct VirtualPath {
    pub identity: PathBuf,
    pub source: Option<PathBuf>,
    pub kind: Kind
}

impl Eq for VirtualPath {}

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

    //Casts / Move
    pub fn into_identity(self) -> PathBuf {
        self.identity
    }

    pub fn into_source(self) -> Option<PathBuf> {
        self.source
    }

    pub fn into_kind(self) -> Kind {
        self.kind
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

    //Constructors
    pub fn root() -> Result<VirtualPath, VfsError> {
        VirtualPath::from(VirtualPath::root_identity(), None, Kind::Directory)
    }

    pub fn root_identity() -> PathBuf {
        PathBuf::from(MAIN_SEPARATOR.to_string())
    }

    pub fn from(identity: PathBuf, source: Option<PathBuf>, kind: Kind) -> Result<VirtualPath, VfsError> {
        match identity.is_relative() && (identity != PathBuf::new()) {
            true => return Err(VfsError::IsRelativePath(identity.to_path_buf())),
            false => Ok(Self::_from(identity, source, kind))
        }
    }

    fn _from(identity: PathBuf, source: Option<PathBuf>, kind: Kind) -> VirtualPath {
        VirtualPath {
            identity,
            source,
            kind
        }
    }

    pub fn from_path(path: &Path) -> Result<VirtualPath, VfsError> {
        VirtualPath::from(path.to_path_buf(), None, Kind::Unknown)
    }

    pub fn from_path_buf(path: PathBuf) -> Result<VirtualPath, VfsError> {
        VirtualPath::from(path, None, Kind::Unknown)
    }

    pub fn from_str(path: &str) -> Result<VirtualPath, VfsError> {
        VirtualPath::from(PathBuf::from(path), None, Kind::Unknown)
    }

    //Domain
    pub fn as_parent(&self) -> Option<&Path> {
        self.identity.parent()
    }

    pub fn to_parent(&self) -> Option<PathBuf> {
        match self.identity.parent() {
            Some(parent) => Some(parent.to_path_buf()),
            None => None
        }
    }

    pub fn into_parent(self) -> Option<PathBuf> {
        match self.identity.parent() {
            Some(parent) => Some(parent.to_path_buf()),
            None => None
        }
    }

    pub fn file_name(&self) -> Result<&OsStr, VfsError> {
        match self.identity.file_name() {
            Some(filename) => Ok(filename),
            None => Err(VfsError::IsDotName(self.identity.to_path_buf()))
        }
    }

    pub fn join(&self, node_name: &OsStr) -> Result<VirtualPath, VfsError>  {
        VirtualPath::from_path_buf(self.identity.join(node_name))
    }

    pub fn replace_parent(path: &Path, new_parent: &Path) -> PathBuf {
        match path.parent(){
            Some(parent) => {
                let stripped = path.strip_prefix(parent).unwrap(); //Do not handle ".." file names
                new_parent.join(stripped).to_path_buf()
            },
            None => new_parent.join(path).to_path_buf()
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

    pub fn with_owned_source(self, new_source: Option<PathBuf>) -> VirtualPath {
        Self::_from(
            self.identity,
            match new_source {
                Some(source) => Some(source),
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

    pub fn with_file_name(self, filename: &OsStr) -> VirtualPath  {
        Self::_from(
            self.to_identity().with_file_name(filename),
            self.source,
            self.kind
        )
    }

    pub fn depth(&self) -> usize{
        self.identity.components().into_iter().count()
    }

    pub fn get_parent_or_root(identity: &Path) -> PathBuf {
        match identity.parent() {
            Some(parent) => parent.to_path_buf(),
            None => VirtualPath::root_identity()
        }
    }

    pub fn is_contained_by(&self, other: &VirtualPath) -> bool {
        for ancestor in self.identity.ancestors() {
            if other.as_identity() == ancestor {
                return true;
            }
        }
        false
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
