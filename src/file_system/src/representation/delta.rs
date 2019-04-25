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

use std::{
    collections::{
        BTreeMap
    },
    path::{ PathBuf, Path },
    ops::{ Add, Sub }
};

use crate::{
    Kind,
    representation::{
        errors::RepresentationError,
        VirtualChildren,
        VirtualPath
    }
};
use std::clone::Clone;

#[derive(Debug, Clone, Default)]
pub struct VirtualDelta {
    pub hierarchy: BTreeMap<PathBuf, VirtualChildren>,
    pub detached: Vec<VirtualPath>
}

impl VirtualDelta {
    pub fn attach_virtual(&mut self, virtual_path: &VirtualPath) -> Result<(), RepresentationError>{
        self.attach(
            virtual_path.as_identity(),
            virtual_path.as_source(),
            virtual_path.to_kind()
        )
    }

    pub fn attach(&mut self, identity: &Path, source: Option<&Path>, kind: Kind) -> Result<(), RepresentationError> {
       if self.get(identity)?.is_some() { Err(RepresentationError::AlreadyExists(identity.to_path_buf())) }
       else {
            let parent = crate::path_helper::get_parent_or_root(identity);

            if !self.hierarchy.contains_key(parent.as_path()) {
                self.hierarchy.insert(parent.to_path_buf(), VirtualChildren::default());
            }

            if self.is_file(parent.as_path())? {
                return Err(RepresentationError::VirtualParentIsAFile(identity.to_path_buf()));
            }

            if identity != crate::path_helper::root_identity().as_path() {
                self.hierarchy
                    .get_mut(parent.as_path())
                    .unwrap() //Assumed
                    .insert(
                        VirtualPath::from_path(identity)?
                            .with_source(source)
                            .with_kind(kind)
                    );
            }

            Ok(())
        }
    }

    pub fn detach(&mut self, identity: &Path) -> Result<(), RepresentationError> {
        if self.get(identity)?.is_some() {
            let parent = crate::path_helper::get_parent_or_root(identity);

            self.hierarchy.get_mut(&parent)
                .unwrap()//TODO Assumed ? self.get has not the same behavior as hierarchy.get_mut
                .remove(&VirtualPath::from_path(identity)?);


            if self.is_directory_empty(parent.as_path()) {
                self.hierarchy.remove(&parent);
            }

            if self.hierarchy.contains_key(&identity.to_path_buf()) {
                self.hierarchy.remove(identity);
            }
            Ok(())
        } else { Err(RepresentationError::DoesNotExists(identity.to_path_buf())) }
    }

    pub fn is_directory(&self, identity: &Path) -> Result<bool, RepresentationError> {
        if identity == crate::path_helper::root_identity().as_path() {
            return Ok(true);
        }

        match self.get(identity)? {
            Some(virtual_identity) => Ok(virtual_identity.kind == Kind::Directory),
            None => Ok(false) //Do not exists
        }
    }

    pub fn is_file(&self, identity: &Path) -> Result<bool, RepresentationError> {
        if identity == crate::path_helper::root_identity().as_path() {
            return Ok(false);
        }

        match self.get(identity)? {
            Some(virtual_identity) => Ok(virtual_identity.kind == Kind::File),
            None => Ok(false) //Do not exists
        }
    }

    pub fn children(&self, parent: &Path) -> Option<&VirtualChildren> {
        match self.hierarchy.get(parent) {
            Some(children) => Some(&children),
            None => None //No key parent
        }
    }

    //TODO unused but could be usefull
    pub fn walk(&self, collection: &mut VirtualChildren, identity: &Path) -> Result<(), RepresentationError>{
        match self.get(identity)? {
            Some(virtual_identity) => match self.is_directory(identity) {
                Ok(true)   => { self._walk(collection, virtual_identity); Ok(()) },
                Ok(false)  => Err(RepresentationError::IsNotADirectory(identity.to_path_buf())),
                Err(error) => Err(error)
            },
            None => Err(RepresentationError::DoesNotExists(identity.to_path_buf()))
        }
    }

    //TODO unused but could be usefull
    fn _walk(&self, collection: &mut VirtualChildren, virtual_identity: &VirtualPath){
        collection.insert(virtual_identity.clone());
        if let Some(children) = self.children(virtual_identity.as_identity()) {
            for child in children.iter() {
                self._walk(collection, &child);
            }
        };
    }

    pub fn get(&self, identity: &Path) -> Result<Option<&VirtualPath>, RepresentationError> {
        match self.hierarchy.get(crate::path_helper::get_parent_or_root(identity).as_path()) {
            Some(children) => {
                match children.get(&VirtualPath::from_path(identity)?) {
                    Some(child) => Ok(Some(&child)),
                    None => Ok(None) //No matching child
                }
            }
            None => Ok(None) //No key parent
        }
    }

    pub fn is_directory_empty(&self, identity: &Path) -> bool {
        match self.children(identity) {
            Some(children) => children.is_empty(),
            None => true
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hierarchy.len() == 0
    }

    //TODO unused yet but seems useful at least for debugging
    pub fn sub_delta(&self, identity: &Path) -> Result<Option<VirtualDelta>, RepresentationError> {
        if self.get(identity)?.is_some() {
            let mut collection = VirtualChildren::default();
            self.walk(&mut collection, identity)?;
            Ok(Some(collection.into_delta()?))
        } else {
            Ok(None)
        }
    }

    pub fn resolve(&self, path: &Path) -> Result<Option<PathBuf>, RepresentationError> {
        match self.first_virtual_ancestor(path)? {
            Some((depth, ancestor)) =>
                match ancestor.to_source() {
                    Some(source) =>
                        Ok(
                            Some(
                                source.join(
                                    path.strip_prefix(
                                        Self::remove_nth_parents(path, depth)
                                    ).unwrap()//Assumed
                                )
                            )
                        ),
                    None => Ok(None) //Has no source
                }
            None => Ok(None) //Is not virtual
        }
    }

    pub fn remove_nth_parents(path: &Path, depth: usize) -> PathBuf {
        for (index, ancestor) in path.ancestors().enumerate() {
            if index == depth {
                return ancestor.to_path_buf();
            }
        }
        path.to_path_buf()
    }

    pub fn first_virtual_ancestor(&self, path: &Path) -> Result<Option<(usize, VirtualPath)>, RepresentationError>{
        for (index, ancestor) in path.ancestors().enumerate() {
            if let Some(virtual_identity) = self.get(ancestor)? {
                return Ok(Some((index, virtual_identity.clone())))
            }
        }
        Ok(None)
    }

    pub fn is_virtual(&self, path: &Path) -> Result<bool, RepresentationError> {
        match self.first_virtual_ancestor(path)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}


impl <'a, 'b> Add<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = Result<VirtualDelta, RepresentationError>;

    fn add(self, right_delta: &'b VirtualDelta) -> Result<VirtualDelta, RepresentationError> {
        let mut result = self.clone();
        for children in right_delta.hierarchy.values() {
            for child in children.iter() {
                if right_delta.get(child.as_identity())?.is_some() {
                    if result.get(child.as_identity())?.is_some() {
                        result.detach(child.as_identity())?;
                    }

                    result.attach(
                        child.as_identity(),
                        child.as_source(),
                        child.to_kind()
                    )?;
                }
            }
        }
        Ok(result)
    }
}

impl <'a, 'b> Sub<&'b VirtualDelta> for &'a VirtualDelta {
    type Output = Result<VirtualDelta, RepresentationError>;

    fn sub(self, right_delta: &'b VirtualDelta) -> Result<VirtualDelta, RepresentationError> {
        let mut result = self.clone();
        for children in right_delta.hierarchy.values() {
            for child in children.iter() {
                if result.get(child.as_identity())?.is_some() {
                    result.detach(child.as_identity())?;
                }
            }
        }
        Ok(result)
    }
}


#[cfg_attr(tarpaulin, skip)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use std::{
        str::FromStr
    };

    use std::{
        path::{ Path }
    };

    #[test]
    fn attach_child_to_root_then_find_it_in_children() {
        let mut delta = VirtualDelta::default();
        let path = VirtualPath::from_str("/virtual/path").unwrap();

        delta.attach(path.as_identity(), None, Kind::Directory).unwrap();

        let children= delta.children(&Path::new("/virtual")).unwrap();
        assert_eq!(
            &path,
            children.get(&path).unwrap()
        );
    }


    #[test]
    fn is_consistent_over_async() {
        let mut delta = VirtualDelta::default();

        let child = Path::new("/virtual/path");
        delta.attach(child, None, Kind::File).unwrap();

        let parent = Path::new("/virtual");
        delta.attach(parent, None, Kind::Directory).unwrap();

        let owned_child = delta.children(parent)
            .unwrap()
            .get(&VirtualPath::from_path(child).unwrap()).unwrap();
        assert_eq!(
            child,
            owned_child.as_identity()
        );
    }

    #[test]
    fn add_a_delta_to_another(){
        let mut delta_r = VirtualDelta::default();
        delta_r.attach(Path::new("/R/to_replace"), None, Kind::Directory).unwrap();
        delta_r.attach(Path::new("/R/to_not_change"), None, Kind::File).unwrap();
        delta_r.attach(Path::new("/R/to_complete"), None, Kind::Directory).unwrap();

        let mut delta_ra = VirtualDelta::default();
        delta_ra.attach(Path::new("/R/to_replace/A"), None, Kind::Directory).unwrap();
        delta_ra.attach(Path::new("/R/to_not_change"), None, Kind::File).unwrap();
        delta_ra.attach(Path::new("/R/to_complete/B"), None, Kind::File).unwrap();

        let delta_r_prime = (&delta_r + &delta_ra).unwrap();
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_replace")).unwrap());
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_complete")).unwrap());
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")).unwrap());
        assert!(delta_r_prime.get(&Path::new("/R/to_replace/A")).unwrap().is_some());
        assert!(delta_r_prime.get(&Path::new("/R/to_complete/B")).unwrap().is_some());
    }

    #[test]
    fn substract_a_delta_from_another(){
        let mut delta_r = VirtualDelta::default();
        delta_r.attach(Path::new("/R/to_remove"), None, Kind::Directory).unwrap();
        delta_r.attach(Path::new("/R/to_not_change"), None, Kind::File).unwrap();
        delta_r.attach(Path::new("/R/to_not_change_dir"), None, Kind::Directory).unwrap();
        delta_r.attach(Path::new("/R/to_not_change_dir/to_remove"), None, Kind::File).unwrap();

        let mut delta_rs = VirtualDelta::default();
        delta_rs.attach(Path::new("/R/to_remove"), None, Kind::Directory).unwrap();
        delta_rs.attach(Path::new("/R/to_not_change_dir/to_remove"), None, Kind::File).unwrap();

        let delta_r_prime = (&delta_r - &delta_rs).unwrap();

        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")).unwrap());
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_not_change_dir")).unwrap());
        assert!(!delta_r_prime.get(&Path::new("/R/to_remove")).unwrap().is_some());
        assert!(!delta_r_prime.get(&Path::new("/R/to_not_change_dir/to_remove")).unwrap().is_some());
    }

    #[test]
    fn walk(){
        let mut delta = VirtualDelta::default();
        delta.attach(Path::new("/R"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_replace"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_not_change"), None, Kind::File).unwrap();
        delta.attach(Path::new("/R/to_complete"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/D"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/E"), None, Kind::Directory).unwrap();

        let mut collection = VirtualChildren::default();
        delta.walk(&mut collection, &Path::new("/R")).unwrap();
        assert!(collection.contains(&VirtualPath::from_str("/R").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_replace").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_not_change").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete/D").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete/E").unwrap()));
    }

    #[test]
    fn attach_detach_idempotent(){
        let mut delta = VirtualDelta::default();
        delta.attach(Path::new("/R"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_replace"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_not_change"), None, Kind::File).unwrap();
        delta.attach(Path::new("/R/to_complete"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/D"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/E"), None, Kind::Directory).unwrap();

        delta.detach(&Path::new("/R/to_complete/E")).unwrap();
        delta.detach(&Path::new("/R/to_complete/D")).unwrap();
        delta.detach(&Path::new("/R/to_complete")).unwrap();
        delta.detach(&Path::new("/R/to_not_change")).unwrap();
        delta.detach(&Path::new("/R/to_replace")).unwrap();
        delta.detach(&Path::new("/R")).unwrap();

        assert!(delta.is_empty());
    }

    #[test]
    fn commute_file_into_dir(){
        let mut delta = VirtualDelta::default();
        delta.attach(Path::new("/A"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/B"), None, Kind::File).unwrap();

        assert_eq!(
            delta.get(Path::new("/A")).unwrap().unwrap().to_kind(),
            Kind::Directory
        );
        assert_eq!(
            delta.get(Path::new("/B")).unwrap().unwrap().to_kind(),
            Kind::File
        );

        //RENAME Ad to Cd
        //Add a new directory C
        delta.attach(Path::new("/C"), None, Kind::Directory).unwrap();

        //Delete old dir Af
        delta.detach(Path::new("/A")).unwrap();

        //RENAME Bf TO Af
        //Add new file A
        delta.attach(Path::new("/A"), None, Kind::File).unwrap();

        //Delete old file Bf
        delta.detach(Path::new("/B")).unwrap();

        //RENAME Cd TO Bd
        //Add a new directory Bd
        delta.attach(Path::new("/B"), None, Kind::Directory).unwrap();

        //Delete old dir Cd
        delta.detach(Path::new("/C")).unwrap();

        assert_eq!(
            delta.get(Path::new("/A")).unwrap().unwrap().to_kind(),
            Kind::File
        );

        assert_eq!(
            delta.get(Path::new("/B")).unwrap().unwrap().to_kind(),
            Kind::Directory
        );

        assert!(delta.get(Path::new("/C")).unwrap().is_none());
    }

    #[test]
    fn generate_sub_delta(){
        let mut delta = VirtualDelta::default();
        delta.attach(Path::new("/A"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/B"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/B/C"), None, Kind::Directory).unwrap();
        delta.attach(Path::new("/B/D"), None, Kind::File).unwrap();

        let sub_delta = delta.sub_delta(Path::new("/B")).unwrap().unwrap();

        assert!(sub_delta.get(Path::new("/B/C")).unwrap().is_some());
        assert_eq!(
            sub_delta.get(Path::new("/B/C")).unwrap().unwrap().to_kind(),
            Kind::Directory
        );

        assert!(sub_delta.get(Path::new("/B/D")).unwrap().is_some());
        assert_eq!(
            sub_delta.get(Path::new("/B/D")).unwrap().unwrap().to_kind(),
            Kind::File
        );

        assert!(sub_delta.get(Path::new("/A")).unwrap().is_none());
    }
}
