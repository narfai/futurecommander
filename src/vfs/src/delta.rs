/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommanderVfs.
 *
 * FutureCommanderVfs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommanderVfs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommanderVfs.  If not, see <https://www.gnu.org/licenses/>.
 */

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

//TODO check if last modifications are passing tests
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
            virtual_path.to_kind()
        )
    }

    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, kind: VirtualKind) {
       match self.get(identity).is_some() {
            true => { panic!("ATTACH {:?} already exists", identity) },
            false => {
                let parent = VirtualPath::get_parent_or_root(identity);

                if !self.hierarchy.contains_key(parent.as_path()) {
                    self.hierarchy.insert(parent.to_path_buf(), VirtualChildren::new());
                }

                if self.is_file(parent.as_path()) {
                    panic!("ATTACH {:?}/{:?} virtual parent is a file", identity, parent)
                }

                if identity != VirtualPath::root_identity().as_path() {
                    self.hierarchy
                        .get_mut(parent.as_path())
                        .unwrap()
                        .insert(
                            VirtualPath::from_path(identity)
                                .with_source(source)
                                .with_kind(kind)
                        );
                }
            }
        }
    }

    pub fn detach(&mut self, identity: &Path) {
        match self.get(identity).is_some() {
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

    pub fn is_directory(&self, identity: &Path) -> bool {
        if identity == VirtualPath::root_identity().as_path() {
            return true;
        }

        match self.get(identity) {
            Some(virtual_identity) => virtual_identity.kind == VirtualKind::Directory,
            None => false //Do not exists
        }
    }

    pub fn is_file(&self, identity: &Path) -> bool {
        if identity == VirtualPath::root_identity().as_path() {
            return false;
        }

        match self.get(identity) {
            Some(virtual_identity) => virtual_identity.kind == VirtualKind::File,
            None => false //Do not exists
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
                true => self._walk(collection, virtual_identity),
                false => panic!("WALK {:?} is not a directory", identity)
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
        match self.get(identity).is_some() {
            true => {
                let mut collection = VirtualChildren::new();
                self.walk(&mut collection, identity);
                Some(collection.into_delta())
            },
            false => None
        }
    }

    pub fn resolve(&self, path: &Path) -> Option<PathBuf> {
        match self.first_virtual_ancestor(path) {
            Some((depth, ancestor)) =>
                match ancestor.to_source() {
                    Some(source) => Some(
                        source.join(
                            path.strip_prefix(
                                Self::remove_nth_parents(path, depth)
                            ).unwrap()
                        )
                    ),
                    None => None //Has no source
                }
            None => None //Is not virtual
        }
    }

    pub fn remove_nth_parents(path: &Path, depth: usize) -> PathBuf {
        for (index, ancestor) in path.ancestors().enumerate() {
            if index == depth {
                return ancestor.to_path_buf();
            }
        }
        return path.to_path_buf();
    }

    pub fn first_virtual_ancestor(&self, path: &Path) -> Option<(usize, VirtualPath)>{
        for (index, ancestor) in path.ancestors().enumerate() {
            match self.get(ancestor) {
                Some(virtual_identity) => return Some((index, virtual_identity.clone())),
                None => {}
            }
        }
        None
    }

    pub fn is_virtual(&self, path: &Path) -> bool {
        match self.first_virtual_ancestor(path) {
            Some(_) => true,
            None => false
        }
    }
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn add(self, right_delta: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_delta.hierarchy {
            for child in children.iter() {
                if right_delta.get(child.as_identity()).is_some() {
                    if result.get(child.as_identity()).is_some() {
                        result.detach(child.as_identity());
                    }

                    result.attach(
                        child.as_identity(),
                        child.as_source(),
                        child.to_kind()
                    )
                }
            }
        }
        result
    }
}

impl <'a, 'b> Sub<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = VirtualDelta;

    fn sub(self, right_delta: &'b VirtualDelta) -> VirtualDelta {
        let mut result = self.clone();
        for (_parent, children) in &right_delta.hierarchy {
            for child in children.iter() {
                if result.get(child.as_identity()).is_some() {
                    result.detach(child.as_identity());
                }
            }
        }
        result
    }
}

/*
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
*/
