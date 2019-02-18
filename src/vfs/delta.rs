use crate::path::VirtualPath;
use std::collections::{ BTreeMap, HashSet };
use std::ops::{ Add, Sub };

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
                        Some(_) => {  children.replace(vpath); },
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
                Some(children) => {
                    match children.get(vpath) {
                        Some(_) => { children.remove(vpath); },
                        None => {/*TODO should log debug : there is no such file defined in btree childs*/}
                    }
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
                Some(children) => {
                    for child in children {
                        if child == path {
                            return Some(&child);
                        }
                    }
                    return None;
                }
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
                for child in children {
                    if child == vpath {
                        return !self.is_directory(&vpath);
                    }
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
        for (_parent, children) in &right_vfs.hierarchy {
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
        for (_parent, children) in &right_vfs.hierarchy {
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
