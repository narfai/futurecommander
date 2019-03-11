use std::env::current_exe;

use std::path::{ PathBuf, Path };

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::path::{ VirtualPath, VirtualKind };
use crate::delta::{ VirtualDelta };
use crate::file_system::{ VirtualFileSystem };
use crate::children::{VirtualChildren };

use crate::operation::ls::ls;
use crate::operation::rm::rm;
use crate::operation::cp::cp;
use crate::operation::mv::mv;
use crate::operation::mkdir::mkdir;
use crate::operation::touch::touch;

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
        let vpath1 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"), None, VirtualKind::File);
        let vpath2 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"),Some(PathBuf::from("/another/source/path")), VirtualKind::File);
        assert_eq!(vpath1, vpath2);
    }

    #[test]
    fn virtual_path_hash_with_source_equal() {
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        let vpath1 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"), None, VirtualKind::File);
        let vpath2 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"),Some(PathBuf::from("/another/source/path")), VirtualKind::File);
        assert_eq!(calculate_hash(&vpath1), calculate_hash(&vpath2));
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
        delta_r.attach(Path::new("/R/to_replace"), None, true);
        delta_r.attach(Path::new("/R/to_not_change"), None,false);
        delta_r.attach(Path::new("/R/to_complete"), None,true);

        let mut delta_ra = VirtualDelta::new();
        delta_ra.attach(Path::new("/R/to_replace/A"), None, true);
        delta_ra.attach(Path::new("/R/to_not_change"), None, false);
        delta_ra.attach(Path::new("/R/to_complete/B"), None, false);

        let delta_r_prime = &delta_r + &delta_ra;
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_replace")).unwrap());
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_complete")).unwrap());
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")).unwrap());
        assert!(delta_r_prime.exists(&Path::new("/R/to_replace/A")));
        assert!(delta_r_prime.exists(&Path::new("/R/to_complete/B")));
    }

    #[test]
    fn virtual_delta_substract_a_delta_from_another(){
        let mut delta_r = VirtualDelta::new();
        delta_r.attach(Path::new("/R/to_remove"),  None,true);
        delta_r.attach(Path::new("/R/to_not_change"), None, false);
        delta_r.attach(Path::new("/R/to_not_change_dir"), None, true);
        delta_r.attach(Path::new("/R/to_not_change_dir/to_remove"), None,false);

        let mut delta_rs = VirtualDelta::new();
        delta_rs.attach(Path::new("/R/to_remove"), None, true);
        delta_rs.attach(Path::new("/R/to_not_change_dir/to_remove"), None,false);

        let delta_r_prime = &delta_r - &delta_rs;
        println!("{:#?}", delta_r_prime);
        assert!(!delta_r_prime.is_directory(&Path::new("/R/to_not_change")).unwrap());
        assert!(delta_r_prime.is_directory(&Path::new("/R/to_not_change_dir")).unwrap());
        assert!(!delta_r_prime.exists(&Path::new("/R/to_remove")));
        assert!(!delta_r_prime.exists(&Path::new("/R/to_not_change_dir/to_remove")));
    }

    #[test]
    fn virtual_delta_walk(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/R"), None, true);
        delta.attach(Path::new("/R/to_replace"), None, true);
        delta.attach(Path::new("/R/to_not_change"), None,false);
        delta.attach(Path::new("/R/to_complete"), None,true);
        delta.attach(Path::new("/R/to_complete/D"), None,true);
        delta.attach(Path::new("/R/to_complete/E"), None,true);

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
        delta.attach(Path::new("/R"), None, true);
        delta.attach(Path::new("/R/to_replace"), None, true);
        delta.attach(Path::new("/R/to_not_change"), None,false);
        delta.attach(Path::new("/R/to_complete"), None,true);
        delta.attach(Path::new("/R/to_complete/D"), None,true);
        delta.attach(Path::new("/R/to_complete/E"), None,true);

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
        delta.attach(Path::new("/A"), None, true);
        delta.attach(Path::new("/B"), None, false);

        assert_eq!(delta.get(Path::new("/A")).unwrap().to_kind(), VirtualKind::Directory);
        assert_eq!(delta.get(Path::new("/B")).unwrap().to_kind(), VirtualKind::File);

        //RENAME Ad to Cd
        //Add a new directory C
        delta.attach(Path::new("/C"), None, true);

        //Delete old dir Af
        delta.detach(Path::new("/A"));

        //RENAME Bf TO Af
        //Add new file A
        delta.attach(Path::new("/A"), None, false);

        //Delete old file Bf
        delta.detach(Path::new("/B"));

        //RENAME Cd TO Bd
        //Add a new directory Bd
        delta.attach(Path::new("/B"), None, true);

        //Delete old dir Cd
        delta.detach(Path::new("/C"));

        assert_eq!(delta.get(Path::new("/A")).unwrap().to_kind(), VirtualKind::File);
        assert_eq!(delta.get(Path::new("/B")).unwrap().to_kind(), VirtualKind::Directory);
        assert!(!delta.exists(Path::new("/C")));
    }

    #[test]
    fn virtual_delta_sub_delta(){
        let mut delta = VirtualDelta::new();
        delta.attach(Path::new("/A"), None, true);
        delta.attach(Path::new("/B"), None, true);
        delta.attach(Path::new("/B/C"), None, true);
        delta.attach(Path::new("/B/D"), None, false);

        let sub_delta = delta.sub_delta(Path::new("/B")).unwrap();

        assert!(sub_delta.exists(Path::new("/B/C")));
        assert_eq!(sub_delta.get(Path::new("/B/C")).unwrap().to_kind(), VirtualKind::Directory);
        assert!(sub_delta.exists(Path::new("/B/D")));
        assert_eq!(sub_delta.get(Path::new("/B/D")).unwrap().to_kind(), VirtualKind::File);
        assert!(!sub_delta.exists(Path::new("/A")));
    }
}


#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    #[test]
    fn virtual_file_system_test_assets_ok(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

    }

    #[test]
    fn virtual_file_system_rm(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
    }

    #[test]
    fn virtual_file_system_cp(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        cp(&mut vfs,
            sample_path.join(&Path::new("B")).as_path(),
            sample_path.join(&Path::new("A")).as_path()
        );

        cp(&mut vfs,
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

        cp(&mut vfs,
            real_source.as_identity(),
            sample_path.join(&Path::new("A")).as_path()
        );
        cp(&mut vfs,
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        );
        cp(&mut vfs,
            sample_path.join(&Path::new("B/F")).as_path(),
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

        mv( &mut vfs,
            real_source.as_identity(),
            sample_path.join(&Path::new("A")).as_path()
        );

        mv( &mut vfs,
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        );

        mv( &mut vfs,
            sample_path.join(&Path::new("B/F")).as_path(),
            sample_path.join(&Path::new("B/D/E")).as_path()
        );

        assert!(vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()).unwrap().contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F")))));
        assert!(!vfs.read_dir(sample_path.join(&Path::new("A/")).as_path()).unwrap().contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/F")))));
        assert!(!vfs.read_dir(sample_path.join(&Path::new("B/F")).as_path()).unwrap().contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/F")))));
    }

    #[test]
    fn virtual_file_system_mkdir(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        mkdir(&mut vfs, sample_path.join(&Path::new("B/D/E/MKDIRED")).as_path());
        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/MKDIRED"))));

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn virtual_file_system_touch(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        touch(&mut vfs, sample_path.join(&Path::new("B/D/E/TOUCHED")).as_path());
        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/TOUCHED"))));

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn virtual_file_system_resolve(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join(&Path::new("A"));
        let b = sample_path.join(&Path::new("B"));
        let ab = sample_path.join(&Path::new("A/B"));
        let abcdef = sample_path.join(&Path::new("A/B/C/D/E/F"));

        vfs.add.attach(a.as_path(), None,true);
        vfs.add.attach(b.as_path(), None, true);
        vfs.copy(b.as_path(), a.as_path());

        let virtual_state = vfs.get_virtual_state();

        assert_eq!(b.as_path(), virtual_state.resolve(ab.as_path()));
        assert_eq!(b.join(&Path::new("C/D/E/F")).as_path(), virtual_state.resolve(abcdef.as_path()));

    }
}
