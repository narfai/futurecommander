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

use std::env::current_exe;

use std::path::{ PathBuf, Path };

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::path::{ VirtualPath, VirtualKind };
use crate::delta::{ VirtualDelta };
use crate::file_system::{ VirtualFileSystem };
use crate::children::{VirtualChildren };
use crate::errors::VfsError;

pub fn get_sample_path() -> PathBuf {
    current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("samples")
}

#[cfg(test)]
mod virtual_path_tests {
    use super::*;

    #[test]
    fn virtual_path_virtually_equal() {
        let vpath1 = VirtualPath::from_str("/intentionally/virtual/full/path").unwrap();
        let vpath2 = VirtualPath::from_str("/intentionally/virtual/full/path").unwrap();
        assert_eq!(vpath1, vpath2);
    }

    #[test]
    fn virtual_path_parent_virtually_equal() {
        let parent = VirtualPath::from_str("/intentionally/virtual/full/").unwrap();
        let child = VirtualPath::from_str("/intentionally/virtual/full/path").unwrap();
        assert_eq!(parent, VirtualPath::from_path_buf(child.into_parent().unwrap()).unwrap());
    }

    #[test]
    fn virtual_path_still_equal_with_source_diff() {
        let vpath1 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            None,
            VirtualKind::File
        ).unwrap();
        let vpath2 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            Some(PathBuf::from("/another/source/path")),
            VirtualKind::File
        ).unwrap();
        assert_eq!(vpath1, vpath2);
    }

    #[test]
    fn virtual_path_hash_with_source_equal() {
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        let vpath1 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            None,
            VirtualKind::File
        ).unwrap();
        let vpath2 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            Some(PathBuf::from("/another/source/path")),
            VirtualKind::File
        ).unwrap();
        assert_eq!(
            calculate_hash(&vpath1),
            calculate_hash(&vpath2)
        );
    }
}


#[cfg(test)]
mod virtual_delta_tests {
    use super::*;

    #[test]
    fn virtual_delta_attach_child_to_root_then_find_it_in_children() {
        let mut delta = VirtualDelta::new();
        let path = VirtualPath::from_str("/virtual/path").unwrap();

        delta.attach(path.as_identity(), None, VirtualKind::Directory).unwrap();

        let children= delta.children(&Path::new("/virtual")).unwrap();
        assert_eq!(
            &path,
            children.get(&path).unwrap()
        );
    }


    #[test]
    fn virtual_delta_is_consistent_over_async() {
        let mut delta = VirtualDelta::new();

        let child = Path::new("/virtual/path");
        delta.attach(child, None,VirtualKind::File).unwrap();

        let parent = Path::new("/virtual");
        delta.attach(parent, None, VirtualKind::Directory).unwrap();

        let owned_child = delta.children(parent)
            .unwrap()
            .get(&VirtualPath::from_path(child).unwrap()).unwrap();
        assert_eq!(
            child,
            owned_child.as_identity()
        );
    }

    #[test]
    fn virtual_delta_add_a_delta_to_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(Path::new("/R/to_replace"), None, VirtualKind::Directory).unwrap();
        delta_r.attach(Path::new("/R/to_not_change"), None,VirtualKind::File).unwrap();
        delta_r.attach(Path::new("/R/to_complete"), None,VirtualKind::Directory).unwrap();

        let mut delta_ra = VirtualDelta::new();
        delta_ra.attach(Path::new("/R/to_replace/A"), None, VirtualKind::Directory).unwrap();
        delta_ra.attach(Path::new("/R/to_not_change"), None, VirtualKind::File).unwrap();
        delta_ra.attach(Path::new("/R/to_complete/B"), None, VirtualKind::File).unwrap();

        let delta_r_prime = (&delta_r + &delta_ra).unwrap();
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_replace")).unwrap());
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_complete")).unwrap());
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")).unwrap());
        assert!(delta_r_prime.get(&Path::new("/R/to_replace/A")).unwrap().is_some());
        assert!(delta_r_prime.get(&Path::new("/R/to_complete/B")).unwrap().is_some());
    }

    #[test]
    fn virtual_delta_substract_a_delta_from_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(Path::new("/R/to_remove"),  None,VirtualKind::Directory).unwrap();
        delta_r.attach(Path::new("/R/to_not_change"), None, VirtualKind::File).unwrap();
        delta_r.attach(Path::new("/R/to_not_change_dir"), None, VirtualKind::Directory).unwrap();
        delta_r.attach(Path::new("/R/to_not_change_dir/to_remove"), None,VirtualKind::File).unwrap();

        let mut delta_rs = VirtualDelta::new();
        delta_rs.attach(Path::new("/R/to_remove"), None, VirtualKind::Directory).unwrap();
        delta_rs.attach(Path::new("/R/to_not_change_dir/to_remove"), None,VirtualKind::File).unwrap();

        let delta_r_prime = (&delta_r - &delta_rs).unwrap();

        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")).unwrap());
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_not_change_dir")).unwrap());
        assert!(!delta_r_prime.get(&Path::new("/R/to_remove")).unwrap().is_some());
        assert!(!delta_r_prime.get(&Path::new("/R/to_not_change_dir/to_remove")).unwrap().is_some());
    }

    #[test]
    fn virtual_delta_walk(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/R"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_replace"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_not_change"), None,VirtualKind::File).unwrap();
        delta.attach(Path::new("/R/to_complete"), None,VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/D"), None,VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/E"), None,VirtualKind::Directory).unwrap();

        let mut collection = VirtualChildren::new();
        delta.walk(&mut collection, &Path::new("/R")).unwrap();
        assert!(collection.contains(&VirtualPath::from_str("/R").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_replace").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_not_change").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete/D").unwrap()));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete/E").unwrap()));
    }

    #[test]
    fn virtual_delta_attach_detach_idempotent(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/R"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_replace"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_not_change"), None,VirtualKind::File).unwrap();
        delta.attach(Path::new("/R/to_complete"), None,VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/D"), None,VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/R/to_complete/E"), None,VirtualKind::Directory).unwrap();

        delta.detach(&Path::new("/R/to_complete/E")).unwrap();
        delta.detach(&Path::new("/R/to_complete/D")).unwrap();
        delta.detach(&Path::new("/R/to_complete")).unwrap();
        delta.detach(&Path::new("/R/to_not_change")).unwrap();
        delta.detach(&Path::new("/R/to_replace")).unwrap();
        delta.detach(&Path::new("/R")).unwrap();

        assert!(delta.is_empty());
    }

    #[test]
    fn virtual_delta_commute_file_into_dir(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/A"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/B"), None, VirtualKind::File).unwrap();

        assert_eq!(
            delta.get(Path::new("/A")).unwrap().unwrap().to_kind(),
            VirtualKind::Directory
        );
        assert_eq!(
            delta.get(Path::new("/B")).unwrap().unwrap().to_kind(),
            VirtualKind::File
        );

        //RENAME Ad to Cd
        //Add a new directory C
        delta.attach(Path::new("/C"), None, VirtualKind::Directory).unwrap();

        //Delete old dir Af
        delta.detach(Path::new("/A")).unwrap();

        //RENAME Bf TO Af
        //Add new file A
        delta.attach(Path::new("/A"), None, VirtualKind::File).unwrap();

        //Delete old file Bf
        delta.detach(Path::new("/B")).unwrap();

        //RENAME Cd TO Bd
        //Add a new directory Bd
        delta.attach(Path::new("/B"), None, VirtualKind::Directory).unwrap();

        //Delete old dir Cd
        delta.detach(Path::new("/C")).unwrap();

        assert_eq!(
            delta.get(Path::new("/A")).unwrap().unwrap().to_kind(),
            VirtualKind::File
        );

        assert_eq!(
            delta.get(Path::new("/B")).unwrap().unwrap().to_kind(),
            VirtualKind::Directory
        );

        assert!(delta.get(Path::new("/C")).unwrap().is_none());
    }

    #[test]
    fn virtual_delta_generate_sub_delta(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/A"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/B"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/B/C"), None, VirtualKind::Directory).unwrap();
        delta.attach(Path::new("/B/D"), None, VirtualKind::File).unwrap();

        let sub_delta = delta.sub_delta(Path::new("/B")).unwrap().unwrap();

        assert!(sub_delta.get(Path::new("/B/C")).unwrap().is_some());
        assert_eq!(
            sub_delta.get(Path::new("/B/C")).unwrap().unwrap().to_kind(),
            VirtualKind::Directory
        );

        assert!(sub_delta.get(Path::new("/B/D")).unwrap().is_some());
        assert_eq!(
            sub_delta.get(Path::new("/B/D")).unwrap().unwrap().to_kind(),
            VirtualKind::File
        );

        assert!(sub_delta.get(Path::new("/A")).unwrap().is_none());
    }
}


#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    #[test]
    fn virtual_file_system_test_samples_ok(){
        let sample_path = get_sample_path();
        assert!(sample_path.join("A").exists());
        assert!(sample_path.join("B").exists());
        assert!(sample_path.join("F").exists());
        assert!(sample_path.join("B/D").exists());
        assert!(sample_path.join("B/D/E").exists());
        assert!(sample_path.join("B/D/G").exists());

        assert!(sample_path.join("A").is_dir());
        assert!(sample_path.join("B").is_dir());
        assert!(sample_path.join("F").is_file());
        assert!(sample_path.join("B/D").is_dir());
        assert!(sample_path.join("B/D/E").is_dir());
        assert!(sample_path.join("B/D/G").is_dir());
    }

    #[test]
    fn virtual_file_system_resolve(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");
        let ab = sample_path.join("A/B");
        let abcdef = sample_path.join("A/B/C/D/E/F");

        vfs.copy(b.as_path(), a.as_path(), None).unwrap();

        let virtual_state = vfs.get_virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );
        assert_eq!(
            b.join("C/D/E/F").as_path(),
            virtual_state.resolve(abcdef.as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn virtual_file_system_resolve_through(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");

        let ab = sample_path.join("A/B");
        let bd = sample_path.join("B/D");

        vfs.copy(b.as_path(), a.as_path(), None).unwrap();
        vfs.copy(ab.as_path(), bd.as_path(), None).unwrap();

        let virtual_state = vfs.get_virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(bd.join("B").as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn virtual_file_system_stat_none_if_deleted(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        assert!(vfs.stat(a.as_path()).unwrap().is_some());

        vfs.remove(a.as_path()).unwrap();

        assert!(vfs.stat(a.as_path()).unwrap().is_none())
    }

    #[test]
    fn virtual_file_system_stat_virtual(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let z = sample_path.join("Z");

        vfs.create(z.as_path(),VirtualKind::Directory).unwrap();

        let stated = vfs.stat(z.as_path()).unwrap().unwrap();
        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), z);
        assert!(stated.as_source().is_none())
    }

    #[test]
    fn virtual_file_system_stat_real(){
        let sample_path = get_sample_path();
        let vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        let stated = vfs.stat(a.as_path()).unwrap().unwrap();
        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), a.as_path());
        assert_eq!(stated.as_source(), Some(a.as_path()))
    }

    #[test]
    fn virtual_file_system_stat_related(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let abdg = sample_path.join("A/B/D/G");//Note : should exists in samples

        vfs.copy(
            sample_path.join("B").as_path(),
            sample_path.join("A").as_path(),
            None
        ).unwrap();

        let stated = vfs.stat(abdg.as_path()).unwrap();

        assert!(stated.is_some());

        let stated = stated.unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), abdg.as_path());
        assert_eq!(stated.as_source(), Some(sample_path.join("B/D/G").as_path()))
    }

    //Error testing

    #[test]
    fn virtual_file_system_copy_or_move_directory_into_itself_must_not_be_allowed(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let source = sample_path.join("B");
        let destination = sample_path.join("B/D");

        match vfs.copy(
            source.as_path(),
            destination.as_path(),
            None
        ) {
            Err(VfsError::CopyIntoItSelft(err_source, err_destination)) => {
                assert_eq!(source.as_path(), err_source.as_path());
                assert_eq!(destination.as_path(), err_destination.as_path());
            }
            Err(error) => panic!("{}", error),
            Ok(_) => panic!("Should not be able to copy into itself")
        };
    }

    #[test]
    fn virtual_file_system_copy_source_does_not_exists(){}

    #[test]
    fn virtual_file_system_copy_destination_does_not_exists(){}

    #[test]
    fn virtual_file_system_create_already_exists(){}

    #[test]
    fn virtual_file_system_remove_does_not_exists(){}

    // No-Backwards tests

    #[test]
    fn virtual_file_system_reset_empty(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        vfs.create(sample_path.join("VIRTUALA").as_path(), VirtualKind::File).unwrap();
        vfs.create(sample_path.join("VIRTUALB").as_path(), VirtualKind::Directory).unwrap();

        assert!(!vfs.is_empty());

        vfs.reset();

        assert!(vfs.is_empty());
    }

    #[test]
    fn virtual_file_system_status_exists(){}

    #[test]
    fn virtual_file_system_status_exists_virtually(){}

    #[test]
    fn virtual_file_system_status_exists_through_virtual_parent(){}

    #[test]
    fn virtual_file_system_status_exists_not_exists(){}

    #[test]
    fn virtual_file_system_status_exists_deleted(){}

    #[test]
    fn virtual_file_system_status_exists_removed_virtually(){}

    #[test]
    fn virtual_file_system_create(){}

    #[test]
    fn virtual_file_system_remove(){}

    #[test]
    fn virtual_file_system_copy(){}

    #[test]
    fn virtual_file_system_copy_with_rename(){

    }
}
