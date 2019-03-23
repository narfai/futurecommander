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
    fn virtual_delta_update_file_dir_commutation(){
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
        delta.attach(Path::new("/A"), None, VirtualKind::Directory);

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
    fn virtual_delta_sub_delta(){
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
    fn virtual_file_system_resolve(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join(&Path::new("A"));
        let b = sample_path.join(&Path::new("B"));
        let ab = sample_path.join(&Path::new("A/B"));
        let abcdef = sample_path.join(&Path::new("A/B/C/D/E/F"));

        vfs.add.attach(a.as_path(), None,VirtualKind::Directory);
        vfs.add.attach(b.as_path(), None, VirtualKind::Directory);
        vfs.copy(b.as_path(), a.as_path()).unwrap();

        let virtual_state = vfs.get_virtual_state();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path())
        );
        assert_eq!(
            b.join(&Path::new("C/D/E/F")).as_path(),
            virtual_state.resolve(abcdef.as_path())
        );

    }

    #[test]
    fn virtual_file_system_test_samples_ok(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
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
    fn virtual_file_system_rm(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        let b_path = sample_path.join(&Path::new("B"));

        vfs.remove(b_path.as_path());

        assert!(vfs.stat(b_path.as_path()).is_none());
    }

    #[test]
    fn virtual_file_system_cp(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        vfs.copy(
            sample_path.join(&Path::new("B")).as_path(),
           sample_path.join(&Path::new("A")).as_path()
        );

        vfs.copy(
            sample_path.join(&Path::new("A/B")).as_path(),
           sample_path.join(&Path::new("A/B/D")).as_path()
        );

        match vfs.read_dir(sample_path.join(&Path::new("A/B/D")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/E"))));
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/G"))));
            },
            Err(error) => panic!("Error : {}", error)
        }

    }

    #[test]
    fn virtual_file_system_cp_preserve_source_and_node_kind(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        vfs.copy(
           real_source.as_identity(),
           sample_path.join(&Path::new("A")).as_path()
        );
        vfs.copy(sample_path.join(&Path::new("A/F")).as_path(),
           sample_path.join(&Path::new("B")).as_path()
        );
        vfs.copy(sample_path.join(&Path::new("B/F")).as_path(),
           sample_path.join(&Path::new("B/D/E")).as_path()
        );

        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))));

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn virtual_file_system_mv(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        vfs.mv(
            real_source.as_identity(),
            sample_path.join(&Path::new("A")).as_path()
        );

        vfs.mv(
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        );

        vfs.mv( sample_path.join(&Path::new("B/F")).as_path(),
            sample_path.join(&Path::new("B/D/E")).as_path()
        );

        assert!(
            vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))))
        );
        assert!(
            !vfs.read_dir(sample_path.join(&Path::new("A/")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/F")))) //Parent
        );
        assert!(
            !vfs.read_dir(sample_path.join(&Path::new("B")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/F"))))
        );
    }

    #[test]
    fn virtual_file_system_mkdir(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        vfs.mkdir(sample_path.join(&Path::new("B/D/E/MKDIRED")).as_path());
        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                assert!(
                    virtual_children.contains(
                        &VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/MKDIRED")))
                    )
                );
            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn virtual_file_system_touch(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        vfs.touch(sample_path.join(&Path::new("B/D/E/TOUCHED")).as_path());
        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                assert!(
                    virtual_children.contains(
                        &VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/TOUCHED")))
                    )
                );

            },
            Err(error) => panic!("Error : {}", error)
        }
    }
}
