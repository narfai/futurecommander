use crate::path::VirtualPath;
use std::collections::{ BTreeMap, HashSet };
use std::path::{ PathBuf, Path };
use std::ops::{ Add, Sub };

#[derive(Debug, Clone)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<PathBuf, HashSet<VirtualPath>>
}

//TODO custom arborescent debug
impl VirtualDelta {
    pub fn new() -> VirtualDelta {
        VirtualDelta {
            hierarchy: BTreeMap::new()
        }
    }

    //TODO -> Result
    pub fn attach(&mut self, path: PathBuf, source: Option<PathBuf>, is_directory: bool) {
        let is_already_directory = self.is_directory(path.as_path());
        if  is_already_directory && !is_directory {
            self.hierarchy.remove(path.as_path());
        } else if !is_already_directory && is_directory {
            self.hierarchy.insert(path.to_path_buf(), HashSet::new());
        }

        if let Some(parent) = path.parent() {
            let virtual_path = VirtualPath::from(path.to_path_buf(), source);
            match self.hierarchy.get_mut(parent) {
                Some(children) =>{
                    match children.get(&virtual_path) {
                        Some(_) => {  children.replace(virtual_path); },
                        None => { children.insert(virtual_path); }
                    }
                },
                None => {
                    let mut children: HashSet<VirtualPath> = HashSet::new();
                    children.insert(virtual_path);
                    self.hierarchy.insert(parent.to_path_buf(), children);
                }
            }
        }
    }

    //TODO -> Result
    pub fn detach(&mut self, path: &Path) {
        let virtual_path = VirtualPath::from_path(path);
        if let Some(parent) = virtual_path.parent() {
            match self.hierarchy.get_mut(parent) {
                Some(children) => {
                    match children.get(&virtual_path) {
                        Some(_) => { children.remove(&virtual_path); },
                        None => {/*TODO should log debug : there is no such file defined in btree childs*/}
                    }
                },
                None => {/*TODO should log debug : the parent dir is empty*/}
            }
        }

        if self.is_directory(path) {
            self.hierarchy.remove(path);
        }
    }

    pub fn get(&self, path: &Path) -> Option<&VirtualPath> {
        match path.parent() {
            Some(parent) => match self.hierarchy.get(parent) {
                Some(children) => {
                    match children.get(&VirtualPath::from_path(path)) {
                        Some(child) => {
                            Some(&child)
                        },
                        None => None
                    }
                }
                None => None
            },
            None => None
        }
    }

    pub fn walk(&self, parent: &Path) -> HashSet<&Path>{
        let mut result: HashSet<&Path> = HashSet::new();
        if let Some(children) = self.children(parent) {
            for child in children {
                result.insert(child.as_path());
                let subset = self.walk(child.as_path());
                for sub_child in subset {
                    result.insert(sub_child);
                }
            }
        }
        result
    }

    pub fn children(&self, parent: &Path) -> Option<&HashSet<VirtualPath>> {
        match self.hierarchy.get(parent) {
            Some(children) => {
                Some(&children)
            }
            None => None
        }
    }

    pub fn is_directory(&self, path: &Path) -> bool {
        self.hierarchy.contains_key(path)
    }

    pub fn is_file(&self, vpath: &Path) -> bool {
        match self.get(vpath) {
            Some(_) => !self.is_directory(vpath),
            None => false
        }
    }

    pub fn exists(&self, path: &Path) -> bool {
        self.is_directory(path) || self.is_file(path)
    }
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn add(self, right_vfs: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_vfs.hierarchy {
            for child in children {
                result.attach(child.get_path(), child.get_source(), right_vfs.is_directory(child.as_path()));
            }
        }
        result
    }
}

impl <'a, 'b> Sub<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn sub(self, right_vfs: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_vfs.hierarchy {
            for child in children {
                result.detach(child.as_path());
            }
        }
        result
    }
}

#[cfg(test)]
mod virtual_delta_tests {
    use super::*;

    #[test]
    fn virtual_delta_attach_child_to_root_then_find_it_in_children() {
        let mut delta = VirtualDelta::new();
        let path = PathBuf::from("/virtual/path");

        delta.attach(path.to_path_buf(), None, true);

        let children= delta.children(&Path::new("/virtual")).unwrap();
        let same_vpath = children.get(&VirtualPath::from_path(path.as_path())).unwrap().to_path_buf();
        assert_eq!(path.as_path(), same_vpath);
    }


    #[test]
    fn virtual_delta_is_consistent_over_async() {
        let mut delta = VirtualDelta::new();

        let child = PathBuf::from("/virtual/path");
        delta.attach(child.to_path_buf(), None,false);

        let parent = PathBuf::from("/virtual");
        delta.attach(parent.to_path_buf(), None, true);

        let owned_child = delta.children(parent.as_path()).unwrap().get(&VirtualPath::from_path(child.as_path())).unwrap();
        assert_eq!(child, owned_child.to_path_buf());
    }

    #[test]
    fn virtual_delta_add_a_delta_to_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(PathBuf::from("/R/to_replace"), None, false);
        delta_r.attach(PathBuf::from("/R/to_not_change"), None,false);
        delta_r.attach(PathBuf::from("/R/to_complete"), None,true);

        let mut delta_ra = VirtualDelta::new();
        delta_ra.attach(PathBuf::from("/R/to_replace/A"), None, true);
        delta_ra.attach(PathBuf::from("/R/to_not_change"), None, false);
        delta_ra.attach(PathBuf::from("/R/to_complete/B"), None, false);

        let delta_r_prime = &delta_r + &delta_ra;
        assert!(delta_r_prime.is_directory(&Path::new("/R")));
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_replace")));
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_complete")));
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")));
        assert!(delta_r_prime.exists(&Path::new("/R/to_replace/A")));
        assert!(delta_r_prime.exists(&Path::new("/R/to_complete/B")));
    }

    #[test]
    fn virtual_delta_substract_a_delta_from_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(PathBuf::from("/R/to_remove"),  None,true);
        delta_r.attach(PathBuf::from("/R/to_not_change"), None, false);
        delta_r.attach(PathBuf::from("/R/to_not_change_dir/to_remove"), None,false);

        let mut delta_rs = VirtualDelta::new();
        delta_rs.attach(PathBuf::from("/R/to_remove"), None, true);
        delta_rs.attach(PathBuf::from("/R/to_not_change_dir/to_remove"), None,false);

        let delta_r_prime = &delta_r - &delta_rs;
        assert!(delta_r_prime.is_directory(&Path::new("/R")));
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")));
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_not_change_dir")));
        assert!(!delta_r_prime.exists(&Path::new("/R/to_remove")));
        assert!(!delta_r_prime.exists(&Path::new("/R/to_not_change_dir/to_remove")));
    }
}
