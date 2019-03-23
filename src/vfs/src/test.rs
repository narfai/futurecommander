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

pub fn get_sample_path() -> PathBuf {
    current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples")
}

#[cfg(test)]
mod virtual_path_tests {
    use super::*;

    #[test]
    fn virtual_path_virtually_equal() {
        let vpath1 = VirtualPath::from_str("/intentionally/virtual/full/path");
        let vpath2 = VirtualPath::from_str("/intentionally/virtual/full/path");
        assert_eq!(vpath1, vpath2);
    }

    #[test]
    fn virtual_path_parent_virtually_equal() {
        let parent = VirtualPath::from_str("/intentionally/virtual/full/");
        let child = VirtualPath::from_str("/intentionally/virtual/full/path");
        assert_eq!(parent, VirtualPath::from_path_buf(child.into_parent().unwrap()));
    }

    #[test]
    fn virtual_path_still_equal_with_source_diff() {
        let vpath1 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            None,
            VirtualKind::File
        );
        let vpath2 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            Some(PathBuf::from("/another/source/path")),
            VirtualKind::File
        );
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
        );
        let vpath2 = VirtualPath::from(
            PathBuf::from("/intentionally/virtual/full/path"),
            Some(PathBuf::from("/another/source/path")),
            VirtualKind::File
        );
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
        let path = VirtualPath::from_str("/virtual/path");

        delta.attach(path.as_identity(), None, VirtualKind::Directory);

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
        delta.attach(child, None,VirtualKind::File);

        let parent = Path::new("/virtual");
        delta.attach(parent, None, VirtualKind::Directory);

        let owned_child = delta.children(parent).unwrap().get(&VirtualPath::from_path(child)).unwrap();
        assert_eq!(
            child,
            owned_child.as_identity()
        );
    }

    #[test]
    fn virtual_delta_add_a_delta_to_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(Path::new("/R/to_replace"), None, VirtualKind::Directory);
        delta_r.attach(Path::new("/R/to_not_change"), None,VirtualKind::File);
        delta_r.attach(Path::new("/R/to_complete"), None,VirtualKind::Directory);

        let mut delta_ra = VirtualDelta::new();
        delta_ra.attach(Path::new("/R/to_replace/A"), None, VirtualKind::Directory);
        delta_ra.attach(Path::new("/R/to_not_change"), None, VirtualKind::File);
        delta_ra.attach(Path::new("/R/to_complete/B"), None, VirtualKind::File);

        let delta_r_prime = &delta_r + &delta_ra;
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_replace")));
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_complete")));
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")));
        assert!(delta_r_prime.get(&Path::new("/R/to_replace/A")).is_some());
        assert!(delta_r_prime.get(&Path::new("/R/to_complete/B")).is_some());
    }

    #[test]
    fn virtual_delta_substract_a_delta_from_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(Path::new("/R/to_remove"),  None,VirtualKind::Directory);
        delta_r.attach(Path::new("/R/to_not_change"), None, VirtualKind::File);
        delta_r.attach(Path::new("/R/to_not_change_dir"), None, VirtualKind::Directory);
        delta_r.attach(Path::new("/R/to_not_change_dir/to_remove"), None,VirtualKind::File);

        let mut delta_rs = VirtualDelta::new();
        delta_rs.attach(Path::new("/R/to_remove"), None, VirtualKind::Directory);
        delta_rs.attach(Path::new("/R/to_not_change_dir/to_remove"), None,VirtualKind::File);

        let delta_r_prime = &delta_r - &delta_rs;

        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")));
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_not_change_dir")));
        assert!(!delta_r_prime.get(&Path::new("/R/to_remove")).is_some());
        assert!(!delta_r_prime.get(&Path::new("/R/to_not_change_dir/to_remove")).is_some());
    }

    #[test]
    fn virtual_delta_walk(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/R"), None, VirtualKind::Directory);
        delta.attach(Path::new("/R/to_replace"), None, VirtualKind::Directory);
        delta.attach(Path::new("/R/to_not_change"), None,VirtualKind::File);
        delta.attach(Path::new("/R/to_complete"), None,VirtualKind::Directory);
        delta.attach(Path::new("/R/to_complete/D"), None,VirtualKind::Directory);
        delta.attach(Path::new("/R/to_complete/E"), None,VirtualKind::Directory);

        let mut collection = VirtualChildren::new();
        delta.walk(&mut collection, &Path::new("/R"));
        assert!(collection.contains(&VirtualPath::from_str("/R")));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_replace")));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_not_change")));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete")));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete/D")));
        assert!(collection.contains(&VirtualPath::from_str("/R/to_complete/E")));
    }

    #[test]
    fn virtual_delta_attach_detach_idempotent(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/R"), None, VirtualKind::Directory);
        delta.attach(Path::new("/R/to_replace"), None, VirtualKind::Directory);
        delta.attach(Path::new("/R/to_not_change"), None,VirtualKind::File);
        delta.attach(Path::new("/R/to_complete"), None,VirtualKind::Directory);
        delta.attach(Path::new("/R/to_complete/D"), None,VirtualKind::Directory);
        delta.attach(Path::new("/R/to_complete/E"), None,VirtualKind::Directory);

        delta.detach(&Path::new("/R/to_complete/E"));
        delta.detach(&Path::new("/R/to_complete/D"));
        delta.detach(&Path::new("/R/to_complete"));
        delta.detach(&Path::new("/R/to_not_change"));
        delta.detach(&Path::new("/R/to_replace"));
        delta.detach(&Path::new("/R"));

        assert!(delta.is_empty());
    }

    #[test]
    fn virtual_delta_commute_file_into_dir(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/A"), None, VirtualKind::Directory);
        delta.attach(Path::new("/B"), None, VirtualKind::File);

        assert_eq!(
            delta.get(Path::new("/A")).unwrap().to_kind(),
            VirtualKind::Directory
        );
        assert_eq!(
            delta.get(Path::new("/B")).unwrap().to_kind(),
            VirtualKind::File
        );

        //RENAME Ad to Cd
        //Add a new directory C
        delta.attach(Path::new("/C"), None, VirtualKind::Directory);

        //Delete old dir Af
        delta.detach(Path::new("/A"));

        //RENAME Bf TO Af
        //Add new file A
        delta.attach(Path::new("/A"), None, VirtualKind::File);

        //Delete old file Bf
        delta.detach(Path::new("/B"));

        //RENAME Cd TO Bd
        //Add a new directory Bd
        delta.attach(Path::new("/B"), None, VirtualKind::Directory);

        //Delete old dir Cd
        delta.detach(Path::new("/C"));

        assert_eq!(
            delta.get(Path::new("/A")).unwrap().to_kind(),
            VirtualKind::File
        );

        assert_eq!(
            delta.get(Path::new("/B")).unwrap().to_kind(),
            VirtualKind::Directory
        );

        assert!(delta.get(Path::new("/C")).is_none());
    }

    #[test]
    fn virtual_delta_generate_sub_delta(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/A"), None, VirtualKind::Directory);
        delta.attach(Path::new("/B"), None, VirtualKind::Directory);
        delta.attach(Path::new("/B/C"), None, VirtualKind::Directory);
        delta.attach(Path::new("/B/D"), None, VirtualKind::File);

        let sub_delta = delta.sub_delta(Path::new("/B")).unwrap();

        assert!(sub_delta.get(Path::new("/B/C")).is_some());
        assert_eq!(
            sub_delta.get(Path::new("/B/C")).unwrap().to_kind(),
            VirtualKind::Directory
        );

        assert!(sub_delta.get(Path::new("/B/D")).is_some());
        assert_eq!(
            sub_delta.get(Path::new("/B/D")).unwrap().to_kind(),
            VirtualKind::File
        );

        assert!(sub_delta.get(Path::new("/A")).is_none());
    }
}


#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    #[test]
    fn virtual_file_system_test_samples_ok(){
        let sample_path = get_sample_path();
        assert!(sample_path.join(Path::new("A")).exists());
        assert!(sample_path.join(Path::new("B")).exists());
        assert!(sample_path.join(Path::new("F")).exists());
        assert!(sample_path.join(Path::new("B/D")).exists());
        assert!(sample_path.join(Path::new("B/D/E")).exists());
        assert!(sample_path.join(Path::new("B/D/G")).exists());

        assert!(sample_path.join(Path::new("A")).is_dir());
        assert!(sample_path.join(Path::new("B")).is_dir());
        assert!(sample_path.join(Path::new("F")).is_file());
        assert!(sample_path.join(Path::new("B/D")).is_dir());
        assert!(sample_path.join(Path::new("B/D/E")).is_dir());
        assert!(sample_path.join(Path::new("B/D/G")).is_dir());
    }

    #[test]
    fn virtual_file_system_resolve(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join(&Path::new("A"));
        let b = sample_path.join(&Path::new("B"));
        let ab = sample_path.join(&Path::new("A/B"));
        let abcdef = sample_path.join(&Path::new("A/B/C/D/E/F"));

        vfs.copy(b.as_path(), a.as_path()).unwrap();

        let virtual_state = vfs.get_virtual_state();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap()
        );
        assert_eq!(
            b.join(&Path::new("C/D/E/F")).as_path(),
            virtual_state.resolve(abcdef.as_path()).unwrap()
        );
    }

    #[test]
    fn virtual_file_system_resolve_through(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join(&Path::new("A"));
        let b = sample_path.join(&Path::new("B"));

        let ab = sample_path.join(&Path::new("A/B"));
        let bd = sample_path.join(&Path::new("B/D"));

        vfs.copy(b.as_path(), a.as_path()).unwrap();
        vfs.copy(ab.as_path(), bd.as_path()).unwrap();

        let virtual_state = vfs.get_virtual_state();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap()
        );

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(bd.join(&Path::new("B")).as_path()).unwrap()
        );
    }

    #[test]
    fn virtual_file_system_resolve_through_inside(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join(&Path::new("A"));
        let b = sample_path.join(&Path::new("B"));

        let ab = sample_path.join(&Path::new("A/B"));
        let abd = sample_path.join(&Path::new("A/B/D"));

        let bd = sample_path.join(&Path::new("B/D"));

        vfs.copy(b.as_path(), a.as_path()).unwrap();
        vfs.copy(ab.as_path(), abd.as_path()).unwrap();

        let virtual_state = vfs.get_virtual_state();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap()
        );

        assert_eq!(
            bd.as_path(),
            virtual_state.resolve(abd.as_path()).unwrap()
        );
    }

    #[test]
    fn virtual_file_system_stat_none_if_deleted(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        assert!(vfs.stat(a.as_path()).is_some());

        vfs.remove(a.as_path()).unwrap();

        assert!(vfs.stat(a.as_path()).is_none())
    }

    #[test]
    fn virtual_file_system_stat_virtual(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let z = sample_path.join(Path::new("Z"));

        vfs.create(z.as_path(),VirtualKind::Directory).unwrap();

        let stated = vfs.stat(z.as_path()).unwrap();
        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), z);
        assert!(stated.as_source().is_none())
    }

    #[test]
    fn virtual_file_system_stat_real(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let a = sample_path.join(Path::new("A"));

        let stated = vfs.stat(a.as_path()).unwrap();
        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), a.as_path());
        assert_eq!(stated.as_source(), Some(a.as_path()))
    }

    #[test]
    fn virtual_file_system_stat_related(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let abdg = sample_path.join(Path::new("A/B/D/G"));//Note : should exists in samples

        vfs.copy(
            sample_path.join(Path::new("B")).as_path(),
            sample_path.join(Path::new("A")).as_path()
        ).unwrap();

        let stated = vfs.stat(abdg.as_path());

        assert!(stated.is_some());

        let stated = stated.unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), abdg.as_path());
        assert_eq!(stated.as_source(), Some(sample_path.join(Path::new("B/D/G")).as_path()))
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
    fn virtual_file_system_create_already_exists(){}

    #[test]
    fn virtual_file_system_remove(){}

    #[test]
    fn virtual_file_system_remove_does_not_exists(){}

    #[test]
    fn virtual_file_system_copy(){}

    #[test]
    fn virtual_file_system_copy_source_does_not(){}

    #[test]
    fn virtual_file_system_copy_destination_does_not(){}

}
