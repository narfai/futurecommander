use std::path::{ PathBuf, Path, Component };
use std::collections::{BTreeMap, HashSet };
use std::cmp::Ordering;

#[derive(Hash, Eq, Clone, Debug)]
pub struct VirtualPath {
    pub is_directory: bool,
    pub path: PathBuf
}

/**
After all ... All we need is to keep is_directory data.
PathBuf implementation will do the job for path components manipulation.
**/
impl VirtualPath {
    pub fn from_path_buf(path: PathBuf, is_directory: bool) -> VirtualPath {
        VirtualPath {
            is_directory,
            path
        }
    }

    pub fn from_str(path: &str, is_directory: bool) -> VirtualPath {
        VirtualPath {
            is_directory,
            path: PathBuf::from(path)
        }
    }

    pub fn into_path_buf(self) -> PathBuf {
        PathBuf::from(self.path)
    }

    pub fn is_dir(&self) -> bool {
        self.is_directory
    }

    pub fn parent(&self) -> Option<VirtualPath> {
        match self.path.parent() {
            Some(parent) => Some(VirtualPath::from_path_buf(parent.to_path_buf(), true)),
            None => None
        }
    }
}


#[test]
fn is_path_buf_virtually_equal() {
    let vpath1 = VirtualPath::from_str("/intentionally/wrong/full/path", true);
    let vpath2 = VirtualPath::from_str("/intentionally/wrong/full/path", true);
    assert_eq!(vpath1.into_path_buf(), vpath2.into_path_buf());
}

#[test]
fn is_path_buf_parent_virtually_equal() {
    let parent = VirtualPath::from_str("/intentionally/wrong/full/", true);
    let child = VirtualPath::from_str("/intentionally/wrong/full/path", true);

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
    let vpath1 = VirtualPath::from_str("/intentionally/wrong/full/path", true);
    let vpath2 = VirtualPath::from_str("/intentionally/wrong/full/path", true);
    assert_eq!(vpath1, vpath2);
}

#[test]
fn is_virtual_path_parent_virtually_equal() {
    let parent = VirtualPath::from_str("/intentionally/wrong/full/", true);
    let child = VirtualPath::from_str("/intentionally/wrong/full/path", true);
    let child_parent = child.parent().unwrap();
    assert_eq!(parent, child_parent);
}

#[derive(Debug)]
pub struct Vfs {
    //TODO may it could be BTreeMap<PathBuf, HashSet<VirtualPath>> as long as key path is *always* a directory
    pub hierarchy: BTreeMap<VirtualPath, HashSet<VirtualPath>>
}

impl Vfs {
    pub fn new() -> Vfs {
        Vfs {
            hierarchy: BTreeMap::new()
        }
    }

    pub fn attach(&mut self, vpath: VirtualPath) {
        if let Some(parent) = vpath.parent() {
            match self.hierarchy.get_mut(&parent) {
                Some(children) => {
                    match children.get(&vpath) {
                        Some(_) => { children.replace(vpath); },
                        None => { children.insert(vpath); }
                    }
                },
                None => {
                    let mut children: HashSet<VirtualPath> = HashSet::new();
                    children.insert(vpath);
                    self.hierarchy.insert(parent, children);
                }
            }
        } else {
            panic!("YOU'RE TRYING TO ATTACH THE ROOT")
        }
    }

    pub fn children(&self, parent: VirtualPath) -> Option<&HashSet<VirtualPath>> {
        match self.hierarchy.get(&parent) {
            Some(children) => {
                Some(&children)
            }
            None => None
        }
    }
}

#[test]
fn vfs_attach_child_to_root_then_find_it_in_children() {
    let mut vfs = Vfs::new();
    let vpath = VirtualPath::from_str("/wrong/path", true);
    let parent = VirtualPath::from_str("/wrong", true);

    vfs.attach(vpath.clone());

    let children= vfs.children(parent.clone()).unwrap();
    let same_vpath = children.get(&(vpath.clone())).unwrap();
    assert_eq!(vpath, same_vpath.clone());
}

#[test]
fn vfs_update_a_child() {
    let mut vfs = Vfs::new();
    {
        let vpath = VirtualPath::from_str("/wrong/path", true);
        vfs.attach(vpath.clone());

        let parent = vpath.parent().unwrap();
        let children = vfs.children(parent).unwrap();
        let same_vpath = children.get(&(vpath.clone())).unwrap();
        assert_eq!(vpath, same_vpath.clone());
        assert_eq!(same_vpath.is_dir(), true);
    }

    {
        let new_vpath = VirtualPath::from_str("/wrong/path", false);
        vfs.attach(new_vpath.clone());

        let parent = new_vpath.parent().unwrap();
        let children = vfs.children(parent).unwrap();
        let same_vpath = children.get(&(new_vpath.clone())).unwrap();

        assert_eq!(new_vpath, same_vpath.clone());
        assert_eq!(same_vpath.is_dir(), false);
    }
}


#[test]
fn vfs_is_consistent_over_async() {
    let mut vfs = Vfs::new();
    let child = VirtualPath::from_str("/wrong/path", false);
    vfs.attach(child.clone());
    let parent = VirtualPath::from_str("/wrong", true);
    vfs.attach(parent.clone());

    let owned_child = vfs.children(parent).unwrap().get(&(child)).unwrap();
    assert_eq!(child, owned_child.clone());
}

#[test]
#[should_panic]
fn vfs_do_not_ensure_parent_is_directory() {
    /*
        This test prove that vfs does not ensure consistency or fail if we add a file which is supposed to contain files.
        We'll see how serious it could be.
    */
    let mut vfs = Vfs::new();
    let child = VirtualPath::from_str("/wrong/path", false);
    vfs.attach(child.clone());
    let parent = VirtualPath::from_str("/wrong", false);
    vfs.attach(parent.clone());

    let owned_child = vfs.children(parent.clone()).unwrap().get(&(child)).unwrap();
    assert_eq!(child, owned_child.clone());

    let owned_parent = vfs.children(VirtualPath::from_str("/", true)).unwrap().get(&(parent)).unwrap();
    assert!(owned_parent.is_dir())
}
