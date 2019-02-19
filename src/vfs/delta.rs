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

    pub fn attach_virtual(&mut self, virtual_path: &VirtualPath, is_directory: bool) {
        self.attach(virtual_path.as_identity(), virtual_path.as_source(), is_directory)
    }

    //TODO -> Result
    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, is_directory: bool) {
        let is_already_directory = self.is_directory(identity);
        if  is_already_directory && !is_directory {
            self.hierarchy.remove(identity);
        } else if !is_already_directory && is_directory {
            self.hierarchy.insert(identity.to_path_buf(), HashSet::new());
        }

        if let Some(parent) = identity.parent() {
            let virtual_path = VirtualPath::from_path(identity).with_source(source);
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
    pub fn detach(&mut self, identity: &Path) {
        let virtual_path = VirtualPath::from_path(identity);
        if let Some(parent) = virtual_path.as_parent() {
            match self.hierarchy.get_mut(parent) {
                Some(children) => {
                    match children.get(&virtual_path) {
                        Some(_) => { children.remove(&virtual_path); },
                        None => { println!("Detach : there is no such file defined in btree childs"); }
                    }
                },
                None => { println!("Detach : the parent dir is empty"); }
            }
        }

        if self.is_directory(identity) {
            self.hierarchy.remove(identity);
        }
    }

    pub fn get(&self, identity: &Path) -> Option<&VirtualPath> {
        match identity.parent() {
            Some(parent) => match self.hierarchy.get(parent) {
                Some(children) => {
                    match children.get(&VirtualPath::from_path(identity)) {
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

    pub fn walk(&self, parent: &Path) -> HashSet<&VirtualPath>{
        let mut result: HashSet<&VirtualPath> = HashSet::new();
        if let Some(children) = self.children(parent) {
            for child in children {
                result.insert(child);
                let subset = self.walk(child.as_identity());
                for sub_child in subset {
                    result.insert(sub_child);
                }
            }
        }
        result
    }

    pub fn children(&self, parent_identity: &Path) -> Option<&HashSet<VirtualPath>> {
        match self.hierarchy.get(parent_identity) {
            Some(children) => {
                Some(&children)
            }
            None => None
        }
    }

    pub fn is_directory(&self, identity: &Path) -> bool {
        self.hierarchy.contains_key(identity)
    }

    pub fn is_file(&self, identity: &Path) -> bool {
        match self.get(identity) {
            Some(_) => !self.is_directory(identity),
            None => false
        }
    }

    pub fn exists(&self, identity: &Path) -> bool {
        self.is_directory(identity) || self.is_file(identity)
    }
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn add(self, right_vfs: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_vfs.hierarchy {
            for child in children {
                result.attach(child.as_identity(), child.as_source(), right_vfs.is_directory(child.as_identity()));
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
                result.detach(child.as_identity());
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
        let path = VirtualPath::from_str("/virtual/path");

        delta.attach(path.as_identity(), None, true);

        let children= delta.children(&Path::new("/virtual")).unwrap();
        assert_eq!(&path, children.get(&path).unwrap());
    }


    #[test]
    fn virtual_delta_is_consistent_over_async() {
        let mut delta = VirtualDelta::new();

        let child = Path::new("/virtual/path");
        delta.attach(child, None,false);

        let parent = Path::new("/virtual");
        delta.attach(parent, None, true);

        let owned_child = delta.children(parent).unwrap().get(&VirtualPath::from_path(child)).unwrap();
        assert_eq!(child, owned_child.as_identity());
    }

    #[test]
    fn virtual_delta_add_a_delta_to_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(Path::new("/R/to_replace"), None, false);
        delta_r.attach(Path::new("/R/to_not_change"), None,false);
        delta_r.attach(Path::new("/R/to_complete"), None,true);

        let mut delta_ra = VirtualDelta::new();
        delta_ra.attach(Path::new("/R/to_replace/A"), None, true);
        delta_ra.attach(Path::new("/R/to_not_change"), None, false);
        delta_ra.attach(Path::new("/R/to_complete/B"), None, false);

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
        delta_r.attach(Path::new("/R/to_remove"),  None,true);
        delta_r.attach(Path::new("/R/to_not_change"), None, false);
        delta_r.attach(Path::new("/R/to_not_change_dir/to_remove"), None,false);

        let mut delta_rs = VirtualDelta::new();
        delta_rs.attach(Path::new("/R/to_remove"), None, true);
        delta_rs.attach(Path::new("/R/to_not_change_dir/to_remove"), None,false);

        let delta_r_prime = &delta_r - &delta_rs;
        assert!(delta_r_prime.is_directory(&Path::new("/R")));
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")));
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_not_change_dir")));
        assert!(!delta_r_prime.exists(&Path::new("/R/to_remove")));
        assert!(!delta_r_prime.exists(&Path::new("/R/to_not_change_dir/to_remove")));
    }
}
