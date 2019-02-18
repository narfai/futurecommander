use std::path::{ PathBuf, Path, Component };
use std::collections::{BTreeMap, HashSet };
use std::cmp::Ordering;
use std::ops::{ Add, Sub };
use std::fs::{ ReadDir, DirEntry };
use std::env::current_exe;
use std::iter::FromIterator;
use std::ffi::{ OsString, OsStr };

/*
TODO Vfs Slices ? VirtualPath Slices ?
TODO plusieurs pistes pour éviter qu'un read event pourrisse le resultat des opérations :
- source tracking et déduire dans le vfs ce qui doit être update ou pas
- permettre de diff deux segments de deux vfs => VfsA.diff(VfsB) => ( stocker un tree avec le vfs reel, un autre avec le virtuel )
VfsDiff::Equal => both trees are different,
VfsDiff::Left | VfsDiff::Right |.
- union two vfs vfs + vfs = vfs

Idée : Se baser sur des hash de paths Sized, de manière a ne pas heaper du tout !
*/

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
        VirtualPath {
            path,
            source: None
        }
    }

    pub fn from_str(path: &str) -> VirtualPath {
        VirtualPath {
            path: PathBuf::from(path),
            source: None
        }
    }

    pub fn from(path: PathBuf, source: Option<PathBuf>) -> VirtualPath {
        VirtualPath {
            path,
            source
        }
    }

    pub fn into_path_buf(self) -> PathBuf {
        PathBuf::from(self.path)
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

#[derive(Debug, Clone)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<VirtualPath, HashSet<VirtualPath>>
}

//TODO custom arborescent debug
impl VirtualDelta {
    pub fn new() -> VirtualDelta {
        VirtualDelta {
            hierarchy: BTreeMap::new()
        }
    }

    //TODO -> Result
    pub fn attach(&mut self, vpath: VirtualPath, is_directory: bool) {
        let is_already_directory = self.is_directory(&vpath);
        if  is_already_directory && !is_directory {
            self.hierarchy.remove(&vpath);
        } else if !is_already_directory && is_directory {
            self.hierarchy.insert(vpath.clone(), HashSet::new());
        }

        if let Some(parent) = vpath.parent() {
            match self.hierarchy.get_mut(&parent) {
                Some(children) =>{
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
        }
    }

    //TODO -> Result
    pub fn detach(&mut self, vpath: &VirtualPath) {
        if let Some(parent) = vpath.parent() {
            match self.hierarchy.get_mut(&parent) {
                Some(children) => match children.get(vpath) {
                    Some(_) => { children.remove(vpath); },
                    None => {/*TODO should log debug : there is no such file defined in btree childs*/}
                },
                None => {/*TODO should log debug : the parent dir is empty*/}
            }
        }

        if self.is_directory(vpath) {
            self.hierarchy.remove(vpath);
        }
    }

    pub fn get(&self, path: &VirtualPath) -> Option<&VirtualPath> {
        match path.parent() {
            Some(parent) => match self.hierarchy.get(&parent) {
                Some(children) => match children.get(path) {
                    Some(child) => Some(&child),
                    None => None
                },
                None => None
            },
            None => None
        }
    }

    pub fn walk(&self, parent: &VirtualPath) -> HashSet<&VirtualPath>{
        let mut result: HashSet<&VirtualPath> = HashSet::new();
        if let Some(children) = self.children(&parent) {
            for child in children {
                result.insert(child);
                let subset = self.walk(child);
                for sub_child in subset {
                    result.insert(sub_child);
                }
            }
        }
        result
    }

    pub fn children(&self, parent: &VirtualPath) -> Option<&HashSet<VirtualPath>> {
        match self.hierarchy.get(parent) {
            Some(children) => {
                Some(&children)
            }
            None => None
        }
    }

    pub fn is_directory(&self, vpath: &VirtualPath) -> bool {
        self.hierarchy.contains_key(vpath)
    }

    pub fn is_file(&self, vpath: &VirtualPath) -> bool {
        if let Some(parent) = vpath.parent() {
            if let Some(children) = self.hierarchy.get(&parent) {
                if children.contains(vpath) {
                    return !self.is_directory(&vpath);
                }
            }
        }

        return false;
    }

    pub fn exists(&self, vpath: &VirtualPath) -> bool {
        self.is_directory(vpath) || self.is_file(vpath)
    }
}

#[test]
fn vfs_attach_child_to_root_then_find_it_in_children() {
    let mut vfs = VirtualDelta::new();
    let vpath = VirtualPath::from_str("/virtual/path");

    vfs.attach(vpath.clone(), true);

    let children= vfs.children(&VirtualPath::from_str("/virtual")).unwrap();
    let same_vpath = children.get(&(vpath.clone())).unwrap();
    assert_eq!(vpath, same_vpath.clone());
}


#[test]
fn vfs_is_consistent_over_async() {
    let mut vfs = VirtualDelta::new();

    let child = VirtualPath::from_str("/virtual/path");
    vfs.attach(child.clone(), false);

    let parent = VirtualPath::from_str("/virtual");
    vfs.attach(parent.clone(), true);

    let owned_child = vfs.children(&parent).unwrap().get(&(child)).unwrap();
    assert_eq!(child, owned_child.clone());
}

impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn add(self, right_vfs: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (parent, children) in &right_vfs.hierarchy {
            for child in children {
                result.attach(child.clone(), right_vfs.is_directory(&child));
            }
        }
        result
    }
}

impl <'a, 'b> Sub<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn sub(self, right_vfs: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (parent, children) in &right_vfs.hierarchy {
            for child in children {
                result.detach(&child);
            }
        }
        result
    }
}

#[test]
fn add_a_vfs_to_another(){
    let mut vfs_r = VirtualDelta::new();
    vfs_r.attach(VirtualPath::from_str("/R/to_replace"), false);
    vfs_r.attach(VirtualPath::from_str("/R/to_not_change"), false);
    vfs_r.attach(VirtualPath::from_str("/R/to_complete"), true);

    let mut vfs_ra = VirtualDelta::new();
    vfs_ra.attach(VirtualPath::from_str("/R/to_replace/A"), true);
    vfs_ra.attach(VirtualPath::from_str("/R/to_not_change"), false);
    vfs_ra.attach(VirtualPath::from_str("/R/to_complete/B"), false);

    let vfs_r_prime = &vfs_r + &vfs_ra;
    assert!(vfs_r_prime.is_directory(&VirtualPath::from_str("/R")));
    assert!(vfs_r_prime.is_directory(&VirtualPath::from_str("/R/to_replace")));
    assert!(vfs_r_prime.is_directory(&VirtualPath::from_str("/R/to_complete")));
    assert!(!vfs_r_prime.is_directory(&VirtualPath::from_str("/R/to_not_change")));
    assert!(vfs_r_prime.exists(&VirtualPath::from_str("/R/to_replace/A")));
    assert!(vfs_r_prime.exists(&VirtualPath::from_str("/R/to_complete/B")));
}

#[test]
fn substract_a_vfs_from_another(){
    let mut vfs_r = VirtualDelta::new();
    vfs_r.attach(VirtualPath::from_str("/R/to_remove"), true);
    vfs_r.attach(VirtualPath::from_str("/R/to_not_change"), false);
    vfs_r.attach(VirtualPath::from_str("/R/to_not_change_dir/to_remove"), false);

    let mut vfs_rs = VirtualDelta::new();
    vfs_rs.attach(VirtualPath::from_str("/R/to_remove"), true);
    vfs_rs.attach(VirtualPath::from_str("/R/to_not_change_dir/to_remove"), false);

    let vfs_r_prime = &vfs_r - &vfs_rs;
    assert!(vfs_r_prime.is_directory(&VirtualPath::from_str("/R")));
    assert!(!vfs_r_prime.is_directory(&VirtualPath::from_str("/R/to_not_change")));
    assert!(vfs_r_prime.is_directory(&VirtualPath::from_str("/R/to_not_change_dir")));
    assert!(!vfs_r_prime.exists(&VirtualPath::from_str("/R/to_remove")));
    assert!(!vfs_r_prime.exists(&VirtualPath::from_str("/R/to_not_change_dir/to_remove")));
}

#[derive(Debug)]
pub struct VirtualFileSystem {
    real: VirtualDelta,
    add: VirtualDelta,
    sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            real: VirtualDelta::new(),
            add: VirtualDelta::new(),
            sub: VirtualDelta::new()
        }
    }
    pub fn read(&mut self, path: &Path) {
        let state = self.get_state();
        let cmp_path = VirtualPath::from_path_buf(path.to_path_buf());
        let virtual_path = match state.get(&cmp_path) {
            Some(vpath) => match vpath.as_source_path() {
                Some(src) => VirtualPath::from_path_buf(src.to_path_buf()),
                None => vpath.clone()
            },
            None => cmp_path
        };

        if virtual_path.as_path().exists() {
            virtual_path.as_path().read_dir()
                .and_then(|results: ReadDir| {
                    for result in results {
                        let result = result?;
                        self.real.attach(VirtualPath::from_path_buf(result.path()), result.path().is_dir());
                    }
                    Ok(())
                }).unwrap();
        }
    }

    pub fn get_state(&self) -> VirtualDelta {
        &(&self.real - &self.sub) + &self.add
    }

    pub fn rm(&mut self, path: &VirtualPath) {
        self.read(path.as_path());
        let state = self.get_state();
        if state.exists(&path) {
            let mut sub_delta = VirtualDelta::new();
            for child in state.walk(path) {
                sub_delta.attach(child.clone(), state.is_directory(&child));
            }
            sub_delta.attach(path.clone(), state.is_directory(&path));
            self.sub = &self.sub + &sub_delta;
            self.add = &self.add - &sub_delta;
        } else {
            panic!("No such file or directory");
        }
    }

    pub fn copy(&mut self, source: VirtualPath, destination: VirtualPath) {
        let state = self.get_state();
        if state.exists(&source) && state.is_directory(&destination) {
            if let Some(destination) = state.get(&destination) {
                let mut add_delta = VirtualDelta::new();
                //TODO : make a Vpath method that is source of truth about the real fs source
                //TODO : append known child (walk) of vfs in the same way that below
                let src = match state.get(&source) {
                    Some(may_source) => match may_source.as_source_path() {
                        Some(owned_source) => owned_source.to_path_buf(),
                        None => source.clone().into_path_buf()
                    },
                    None => source.clone().into_path_buf()
                };
                let dst = destination.as_path().join(source.file_name()).to_path_buf();
                add_delta.attach(
                    VirtualPath::from(dst, Some(src)),
                    state.is_directory(&source)
                );
                self.add = &self.add + &add_delta;
                self.sub = &self.sub - &add_delta;
            }
        } else {
            panic!("Source does not exists, or destination isnt a directory");
        }
    }
}

#[test]
fn vfs_test_assets_ok(){
    let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
    let mut vfs = VirtualFileSystem::new();
    vfs.read(sample_path.as_path());
    vfs.read(sample_path.join(&Path::new("A")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/E")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/G")).as_path());
    let state = vfs.get_state();
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/C")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("F")))));
    assert!(state.is_directory(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))));
}

#[test]
fn vfs_rm(){
    let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
    let mut vfs = VirtualFileSystem::new();
    vfs.read(sample_path.as_path());
    vfs.read(sample_path.join(&Path::new("A")).as_path());
    vfs.read(sample_path.join(&Path::new("B")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/E")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/G")).as_path());
    vfs.rm(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B"))));
    let state = vfs.get_state();
    assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B")))));
    assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
    assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))));
}

#[test]
fn vfs_copy(){
    let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
    let mut vfs = VirtualFileSystem::new();
    vfs.read(sample_path.as_path());
    vfs.read(sample_path.join(&Path::new("A")).as_path());
    vfs.read(sample_path.join(&Path::new("B")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/E")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/G")).as_path());
    vfs.copy(
        VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D"))),
        VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))
    );
    vfs.copy(
        VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D"))),
        VirtualPath::from_path_buf(sample_path.join(&Path::new("B")))
    );
    vfs.read(sample_path.join(&Path::new("A/D")).as_path());
    vfs.read(sample_path.join(&Path::new("A/D/E")).as_path());
    vfs.read(sample_path.join(&Path::new("A/D/G")).as_path());
    let state = vfs.get_state();
    println!("{:#?}", state);
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D/E")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D/G")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
}


/*
Si on fait un ajout, et une suppression sur le même path, qu'est-ce qui va prendre le dessus ? => l'ajout
Idée : utiliser les méthodes union et intersect des hashset ??
ordonner les deux listes add et sub ?
ajouter un champ "priority" ?
*/

//#[test]
//fn vfs_copy_and_rm_same_paths(){
//
//}
