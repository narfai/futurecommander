use itertools::Itertools;
use crate::path::{ VirtualPath, VirtualKind };
use std::collections::{ BTreeMap, HashSet };
use std::collections::hash_set::Iter as HashSetIter;
use std::collections::hash_set::IntoIter as HashSetIntoIter;
use std::path::{ PathBuf, Path };
use std::ops::{ Add, Sub };

#[derive(Debug, Clone)]
pub struct VirtualChildren {
    set: HashSet<VirtualPath>
}

impl VirtualChildren {
    pub fn new() -> VirtualChildren {
        VirtualChildren {
            set: HashSet::new()
        }
    }

    pub fn insert(&mut self, virtual_identity: VirtualPath) -> bool {
        self.set.insert(virtual_identity)
    }

    pub fn replace(&mut self, virtual_identity: VirtualPath) -> Option<VirtualPath> {
        self.set.replace(virtual_identity)
    }

    pub fn remove(&mut self, virtual_identity: &VirtualPath) -> bool {
        self.set.remove(virtual_identity)
    }

    pub fn get(&self, virtual_identity: &VirtualPath) -> Option<&VirtualPath> {
        self.set.get(&virtual_identity)
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn contains(&self, virtual_identity: &VirtualPath) -> bool {
        self.set.contains(virtual_identity)
    }

    pub fn iter <'a>(&self) -> VirtualChildrenIterator {
        VirtualChildrenIterator::new(self.set.iter())
    }
}

#[derive(Debug, Clone)]
pub struct VirtualChildrenIterator<'a> {
    iter: HashSetIter<'a, VirtualPath>
}

impl <'a>VirtualChildrenIterator<'a> {
    pub fn new(iter: HashSetIter<'a, VirtualPath>) -> VirtualChildrenIterator {
        VirtualChildrenIterator {
            iter
        }
    }
}

impl <'a>Iterator for VirtualChildrenIterator<'a> {
    type Item = &'a VirtualPath;

    fn next(&mut self) -> Option<&'a VirtualPath> {
        self.iter.next()
    }
}

impl IntoIterator for VirtualChildren {
    type Item = VirtualPath;
    type IntoIter = HashSetIntoIter<VirtualPath>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<PathBuf, VirtualChildren> //TODO transform HashSet into children collection
}

//IDEA : Iterator with next() as get_state for perform writes from a buffer ( over the delta )
impl VirtualDelta {
    pub fn new() -> VirtualDelta {
        VirtualDelta {
            hierarchy: BTreeMap::new()
        }
    }

    pub fn attach_virtual(&mut self, virtual_path: &VirtualPath, is_directory: bool) {
        self.attach(virtual_path.as_identity(), virtual_path.as_source(), is_directory)
    }

    pub fn exp_attach(&mut self, identity: &Path, source: Option<&Path>, is_directory: bool) {
       match self.exists(identity) {
            true => { panic!("ATTACH {:?} already exists", identity) },
            false => {

                let parent = self.get_parent_or_root(identity);

                if !self.hierarchy.contains_key(parent.as_path()) {
                    self.hierarchy.insert(parent.to_path_buf(), VirtualChildren::new());
                } else if self.exp_is_file(parent.as_path()).unwrap() {
                    panic!("ATTACH {:?}/{:?} virtual parent is a file", identity, parent);
                }

                self.hierarchy
                    .get_mut(parent.as_path())
                    .unwrap()
                    .insert(
                        VirtualPath::from_path(identity)
                            .with_source(source)
                            .with_kind(match is_directory {
                                true => VirtualKind::Directory,
                                false => VirtualKind::File
                            })
                    );

            }
        }
    }

    pub fn exp_detach(&mut self, identity: &Path) {
        match self.exists(identity) {
            true => {
                let parent = self.get_parent_or_root(identity);

                self.hierarchy.get_mut(&parent)
                    .unwrap()
                    .remove(&VirtualPath::from_path(identity));

                match self.is_directory_empty(parent.as_path()) {
                    true => { self.hierarchy.remove(&parent); },
                    false => {}
                }

                if self.hierarchy.contains_key(&identity.to_path_buf()) {
                    self.hierarchy.remove(identity);
                }
            },
            false => { panic!("DETACH {:?} does not exists", identity)}
        }
    }

    pub fn exp_update(&mut self, virtual_path: &VirtualPath, is_directory: bool) {
        match self.exists(virtual_path.as_identity()) {
            true => {
                let is_already_directory = self.exp_is_directory(virtual_path.as_identity()).unwrap();
                if  is_already_directory && !is_directory {
                    panic!("UPDATE {:?} as transformed into file", virtual_path)
                } else if !is_already_directory && is_directory {
                    panic!("UPDATE {:?} as transformed into directory", virtual_path)
                }

                let root = VirtualPath::root();
                let parent = virtual_path.as_parent().unwrap_or(root.as_identity());
                match self.hierarchy.get_mut(parent) {
                    Some(children) => { children.replace(virtual_path.clone()); },
                    None => { panic!("UPDATE {:?} parent does not exists", virtual_path) }
                }

            },
            false => { { panic!("UPDATE {:?} does not exists", virtual_path)} }
        }
    }

    pub fn exp_is_directory(&self, identity: &Path) -> Option<bool> {
        if identity == VirtualPath::root_identity().as_path() {
            return Some(true);
        }

        match self.get(identity) {
            Some(virtual_identity) => Some(virtual_identity.kind == VirtualKind::Directory),
            None => None //Do not exists
        }
    }

    pub fn exp_is_file(&self, identity: &Path) -> Option<bool> {
        if identity == VirtualPath::root_identity().as_path() {
            return Some(false);
        }

        match self.get(identity) {
            Some(virtual_identity) => Some(virtual_identity.kind == VirtualKind::File),
            None => None //Do not exists
        }
    }

    pub fn exp_children(&self, parent: &Path) -> Option<VirtualChildrenIterator> {
        match self.hierarchy.get(parent) {
            Some(children) => {
                Some(children.iter())
            },
            None => None //No key parent
        }
    }

    pub fn exp_children_owned(&self, parent: &Path) -> Option<VirtualChildren> {
        match self.hierarchy.get(parent) {
            Some(children) => Some(children.clone()),
            None => None //Do not exists
        }
    }

    pub fn exp_walk(&self, collection: &mut VirtualChildren, virtual_identity: &VirtualPath){
        collection.insert(virtual_identity.clone());
        if let Some(children) = self.exp_children(virtual_identity.as_identity()) {
            for child in children {
                self.exp_walk(collection, child);
            }
        };
    }

    pub fn get(&self, identity: &Path) -> Option<&VirtualPath> {
        match self.hierarchy.get(self.get_parent_or_root(identity).as_path()) {
            Some(children) => {
                match children.get(&VirtualPath::from_path(identity)) {
                    Some(child) => {
                        Some(&child)
                    },
                    None => None //No matching child
                }
            }
            None => None //No key parent
        }
    }

    pub fn exists(&self, identity: &Path) -> bool {
        self.get(identity).is_some()
    }

    pub fn is_directory_empty(&self, identity: &Path) -> bool {
        let dir = self.get_parent_or_root(identity);
        match self.children(dir.as_path()) {
            Some(children) => children.len() == 0,
            None => false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hierarchy.len() == 0
    }

    fn get_parent_or_root(&self, identity: &Path) -> PathBuf {
        match identity.parent() {
            Some(parent) => parent.to_path_buf(),
            None => VirtualPath::root_identity()
        }
    }

    //SHOULD BE REMOVED AFTER EXP REFACTO

    //TODO -> Result
    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, is_directory: bool) {
        let is_already_directory = self.is_directory(identity);
        if  is_already_directory && !is_directory {
            self.hierarchy.remove(identity);
        } else if !is_already_directory && is_directory {
            self.hierarchy.insert(identity.to_path_buf(), VirtualChildren::new());
            for ancestor in identity.ancestors().skip(1) {
                self.hierarchy.insert(ancestor.to_path_buf(), VirtualChildren::new());
            }
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
                    let mut children= VirtualChildren::new();
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
                        None => { /*println!("Detach : there is no such file defined in btree childs");*/ }
                    }
                },
                None => { /*println!("Detach : the parent dir is empty");*/ }
            }
        }

        if self.is_directory(identity) {
            self.hierarchy.remove(identity);
        }
    }

    pub fn walk(&self, parent: &Path) -> VirtualChildren {
        let mut result= VirtualChildren::new();
        if let Some(children) = self.children(parent) {
            for child in children.iter() {
                result.insert(child.clone());
                let subset = self.walk(child.as_identity());
                for sub_child in subset.iter() { //TODO add an iterator or use recursion elseway
                    result.insert(sub_child.clone());
                }
            }
        }
        result
    }

    pub fn children(&self, parent_identity: &Path) -> Option<&VirtualChildren> {
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
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn add(self, right_vfs: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_vfs.hierarchy {
            for child in children.iter() {
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
            for child in children.iter() {
                result.detach(child.as_identity());
            }
        }
        result
    }
}
