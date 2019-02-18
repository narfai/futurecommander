use crate::delta::VirtualDelta;
use std::cmp::Ordering;
use std::path::{ PathBuf, Path };
use std::ffi::{ OsStr };

#[derive(Hash, Eq, Clone, Debug)]
pub struct VirtualPath {
    pub path: PathBuf,
    pub source: Option<PathBuf>
}

//TODO proper Error / Results implementation
//TODO proper [test] & main -> Result bubbling
/**
Virtual wrapper of PathBuf for keeping control over type & behaviors
PathBuf implementation will do the job for path components manipulation.
**/
impl VirtualPath {
    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn as_source_path(&self) -> Option<&Path> {
        match &self.source {
            Some(source) => Some(source.as_path()),
            None => None
        }
    }

    pub fn file_name(&self) -> &OsStr {
        self.path.file_name().unwrap()
    }

    pub fn join(&self, node_name: &OsStr) -> VirtualPath {
        VirtualPath::from_path_buf(self.path.join(node_name))
    }

    pub fn parent(&self) -> Option<VirtualPath> {
        match self.path.parent() {
            Some(parent) => Some(VirtualPath::from_path_buf(parent.to_path_buf())),
            None => None
        }
    }

    pub fn from_path_buf(path: PathBuf) -> VirtualPath {
        if path.is_relative() {
            panic!("Does not supports relative paths");
        }

        VirtualPath {
            path,
            source: None
        }
    }

    pub fn from_str(path: &str) -> VirtualPath {
        let path = PathBuf::from(path);
        if path.is_relative() {
            panic!("Does not supports relative paths");
        }
        VirtualPath {
            path,
            source: None
        }
    }

    pub fn from(path: PathBuf, source: Option<PathBuf>) -> VirtualPath {
        if path.is_relative() {
            panic!("Does not supports relative paths");
        }
        VirtualPath {
            path,
            source
        }
    }

    pub fn into_path_buf(self) -> PathBuf {
        PathBuf::from(self.path)
    }

    pub fn to_referent_source(&self, delta: &VirtualDelta) -> PathBuf {
        match delta.get(self) {
            Some(vpath) => match vpath.as_source_path() {
                Some(src) => src.to_path_buf(),
                None => vpath.clone().into_path_buf()
            },
            None => match self.as_source_path() {
                Some(src) => src.to_path_buf(),
                None => self.path.to_path_buf()
            }
        }
    }

    pub fn with_new_parent(&self, new_parent: &Path) -> VirtualPath {
        let stripped = self.path.as_path().strip_prefix(self.parent().unwrap().as_path()).unwrap();
        VirtualPath {
            path: new_parent.join(stripped).to_path_buf(),
            source: None
        }
    }
}

#[test]
fn is_path_buf_virtually_equal() {
    let vpath1 = VirtualPath::from_str("/intentionally/virtual/full/path");
    let vpath2 = VirtualPath::from_str("/intentionally/virtual/full/path");
    assert_eq!(vpath1.into_path_buf(), vpath2.into_path_buf());
}

#[test]
fn is_path_buf_parent_virtually_equal() {
    let parent = VirtualPath::from_str("/intentionally/virtual/full/");
    let child = VirtualPath::from_str("/intentionally/virtual/full/path");

    let child_parent = child.parent().unwrap();
    assert_eq!(parent.into_path_buf(), child_parent.into_path_buf());
}

//Rely on PathBuf implementation for identify & order VirtualPaths over Iterators
impl Ord for VirtualPath {
    fn cmp(&self, other: &VirtualPath) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for VirtualPath {
    fn partial_cmp(&self, other: &VirtualPath) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl PartialEq for VirtualPath {
    fn eq(&self, other: &VirtualPath) -> bool {
        self.path.eq(&other.path)
    }
}

#[test]
fn is_virtual_path_virtually_equal() {
    let vpath1 = VirtualPath::from_str("/intentionally/virtual/full/path");
    let vpath2 = VirtualPath::from_str("/intentionally/virtual/full/path");
    assert_eq!(vpath1, vpath2);
}

#[test]
fn is_virtual_path_parent_virtually_equal() {
    let parent = VirtualPath::from_str("/intentionally/virtual/full/");
    let child = VirtualPath::from_str("/intentionally/virtual/full/path");
    let child_parent = child.parent().unwrap();
    assert_eq!(parent, child_parent);
}


#[test]
fn is_virtual_path_source_equal() {
    let vpath1 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"), None);
    let vpath2 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"),Some(PathBuf::from("/another/source/path")));
    assert_eq!(vpath1, vpath2);
}
