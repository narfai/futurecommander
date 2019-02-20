use std::cmp::Ordering;
use std::path::{ PathBuf, Path };
use std::ffi::{ OsStr };
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone, Debug)]
pub struct VirtualPath {
    pub identity: PathBuf,
    pub source: Option<PathBuf>
}

//TODO proper Error / Results implementation
//TODO proper [test] & main -> Result bubbling
/**
Virtual wrapper of PathBuf for keeping control over type & behaviors
PathBuf implementation will do the job for path components manipulation.
**/
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

    pub fn as_referent_source(&self) -> &Path {
        match &self.source {
            Some(source) => source.as_path(),
            None => self.identity.as_path()
        }
    }

    //Casts / Move
    pub fn into_identity(self) -> PathBuf {
        self.identity
    }

    pub fn into_source(self) -> Option<PathBuf> {
        self.source
    }

    pub fn into_referent_source(self) -> PathBuf {
        match self.source {
            Some(source) => source,
            None => self.identity
        }
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

    pub fn to_referent_source(&self) -> PathBuf {
        match &self.source {
            Some(source) => source.to_path_buf(),
            None => self.to_identity()
        }
    }

    //Constructors
    pub fn from(identity: PathBuf, source: Option<PathBuf>) -> VirtualPath {
        if identity.is_relative() {
            panic!("Does not supports relative paths");
        }
        VirtualPath {
            identity,
            source
        }
    }

    pub fn from_path(path: &Path) -> VirtualPath {
        VirtualPath::from(path.to_path_buf(), None)
    }

    pub fn from_path_buf(path: PathBuf) -> VirtualPath {
        VirtualPath::from(path, None)
    }

    pub fn from_str(path: &str) -> VirtualPath {
        let identity = PathBuf::from(path);
        VirtualPath::from(identity, None)
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

    pub fn file_name(&self) -> &OsStr {
        self.identity.file_name().unwrap() //Do not handle ".." file names
    }

    pub fn join(&self, node_name: &OsStr) -> VirtualPath {
        VirtualPath::from_path_buf(self.identity.join(node_name))
    }

    pub fn with_new_parent(self, new_parent: &Path) -> VirtualPath {
        match self.identity.parent() {
            Some(parent) => {
                let stripped = self.identity.as_path().strip_prefix(parent).unwrap(); //Do not handle ".." file names
                VirtualPath::from(new_parent.join(stripped).to_path_buf(), None)
            },
            None => self
        }
    }

    pub fn with_source(self, new_source: Option<&Path>) -> VirtualPath {
        VirtualPath::from(
            self.into_identity(),
            match new_source {
                Some(source) => Some(source.to_path_buf()),
                None => None
            }
        )
    }

    pub fn depth(&self) -> usize{
        self.identity.components().into_iter().count()
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
