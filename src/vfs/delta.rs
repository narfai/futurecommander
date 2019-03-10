use crate::{ VirtualPath, VirtualKind };
use crate::VirtualChildren;
use std::collections::{ BTreeMap };

use std::path::{ PathBuf, Path };
use std::ops::{ Add, Sub };


//impl PartialEq for VirtualDelta {
//    fn eq(&self, other: &VirtualDelta) -> bool {}
//}

#[derive(Debug, Clone)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<PathBuf, VirtualChildren>
}

//IDEA : Iterator with next() as get_state for perform writes from a buffer ( over the delta )
impl VirtualDelta {
    pub fn new() -> VirtualDelta {
        VirtualDelta {
            hierarchy: BTreeMap::new()
        }
    }

    pub fn attach_virtual(&mut self, virtual_path: &VirtualPath) {
        self.attach(
            virtual_path.as_identity(),
            virtual_path.as_source(),
            virtual_path.to_kind() == VirtualKind::Directory
        )
    }

    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, is_directory: bool) {
       match self.exists(identity) {
            true => { panic!("ATTACH {:?} already exists", identity) },
            false => {

                let parent = VirtualPath::get_parent_or_root(identity);

                if !self.hierarchy.contains_key(parent.as_path()) {
                    self.hierarchy.insert(parent.to_path_buf(), VirtualChildren::new());
                }

                match self.is_file(parent.as_path()) {
                    Some(true) => panic!("ATTACH {:?}/{:?} virtual parent is a file", identity, parent),
                    _ => {}
                }

                if identity != VirtualPath::root_identity().as_path() {
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
    }

    pub fn detach(&mut self, identity: &Path) {
        match self.exists(identity) {
            true => {
                let parent = VirtualPath::get_parent_or_root(identity);

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

    pub fn update(&mut self, virtual_path: &VirtualPath, is_directory: bool) {
        match self.exists(virtual_path.as_identity()) {
            true => {
                let is_already_directory = self.is_directory(virtual_path.as_identity()).unwrap();
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

    pub fn is_directory(&self, identity: &Path) -> Option<bool> {
        if identity == VirtualPath::root_identity().as_path() {
            return Some(true);
        }

        match self.get(identity) {
            Some(virtual_identity) => Some(virtual_identity.kind == VirtualKind::Directory),
            None => None //Do not exists
        }
    }

    pub fn is_file(&self, identity: &Path) -> Option<bool> {
        if identity == VirtualPath::root_identity().as_path() {
            return Some(false);
        }

        match self.get(identity) {
            Some(virtual_identity) => Some(virtual_identity.kind == VirtualKind::File),
            None => None //Do not exists
        }
    }

    pub fn children(&self, parent: &Path) -> Option<&VirtualChildren> {
        match self.hierarchy.get(parent) {
            Some(children) => Some(&children),
            None => None //No key parent
        }
    }

    pub fn children_owned(&self, parent: &Path) -> Option<VirtualChildren> {
        match self.hierarchy.get(parent) {
            Some(children) => Some(children.clone()),
            None => None //Do not exists
        }
    }

    pub fn walk(&self, collection: &mut VirtualChildren, identity: &Path){
        match self.get(identity) {
            Some(virtual_identity) => match self.is_directory(identity) {
                Some(true) => self._walk(collection, virtual_identity),
                Some(false) => panic!("WALK {:?} is not a directory", identity),
                None => panic!("WALK {:?} does not exists", identity)
            },
            None => { panic!("WALK {:?} does not exists", identity) }
        }
    }

    fn _walk(&self, collection: &mut VirtualChildren, virtual_identity: &VirtualPath){
        collection.insert(virtual_identity.clone());
        if let Some(children) = self.children(virtual_identity.as_identity()) {
            for child in children.iter() {
                self._walk(collection, &child);
            }
        };
    }

    pub fn get(&self, identity: &Path) -> Option<&VirtualPath> {
        match self.hierarchy.get(VirtualPath::get_parent_or_root(identity).as_path()) {
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
        let dir = VirtualPath::get_parent_or_root(identity);
        match self.children(dir.as_path()) {
            Some(children) => children.len() == 0,
            None => false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hierarchy.len() == 0
    }

    pub fn sub_delta(&self, identity: &Path) -> Option<VirtualDelta> {
        match self.exists(identity) {
            true => {
                let mut collection = VirtualChildren::new();
                self.walk(&mut collection, identity);
                Some(collection.into_delta())
            },
            false => None
        }
    }
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn add(self, right_vfs: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_vfs.hierarchy {
            for child in children.iter() {
                match right_vfs.is_directory(child.as_identity()) {
                    Some(is_directory) => match self.exists(child.as_identity()) {
                        true => result.update(child, is_directory),
                        false =>
                            result.attach(
                                child.as_identity(),
                                child.as_source(),
                                is_directory
                            )
                    },
                    None => {}
                }
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
                if result.exists(child.as_identity()) {
                    result.detach(child.as_identity());
                }
            }
        }
        result
    }
}
