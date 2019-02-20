use std::env::current_exe;

use std::path::{ PathBuf, Path };

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::path::VirtualPath;
use crate::delta::VirtualDelta;
use crate::file_system::{VirtualFileSystem, LsItem};

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
        let vpath1 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"), None);
        let vpath2 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"),Some(PathBuf::from("/another/source/path")));
        assert_eq!(vpath1, vpath2);
    }

    #[test]
    fn virtual_path_hash_with_source_equal() {
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        let vpath1 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"), None);
        let vpath2 = VirtualPath::from(PathBuf::from("/intentionally/virtual/full/path"),Some(PathBuf::from("/another/source/path")));
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


#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    #[test]
    fn virtual_file_system_test_assets_ok(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());
        vfs.read_virtual(sample_path.join(&Path::new("A")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/E")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/G")).as_path());
        let state = vfs.get_state();
        assert!(state.exists(sample_path.join(&Path::new("A/C")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("B/D/E")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("B/D/G")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("F")).as_path()));
        assert!(state.is_directory(sample_path.join(&Path::new("A")).as_path()));
    }

    #[test]
    fn virtual_file_system_rm(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());
        vfs.read_virtual(sample_path.join(&Path::new("A")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/E")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/G")).as_path());

        vfs.rm(sample_path.join(&Path::new("B")).as_path());

        let state = vfs.get_state();
        assert!(!state.exists(sample_path.join(&Path::new("B")).as_path()));
        assert!(!state.exists(sample_path.join(&Path::new("B/D/E")).as_path()));
        assert!(!state.exists(sample_path.join(&Path::new("B/D/G")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("A")).as_path()));
    }

    #[test]
    fn virtual_file_system_copy(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();

        vfs.read_virtual(sample_path.as_path());

        vfs.copy(
            sample_path.join(&Path::new("B")).as_path(),
            sample_path.join(&Path::new("A")).as_path()
        );

        match vfs.ls(sample_path.join(&Path::new("A/B/D")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsItem::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/E"))), true)));
                assert!(results.contains(&LsItem::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/G"))), true)));
            },
            None => { panic!("No results") }
        }

    }

    #[test]
    fn virtual_file_system_copy_preserve_source_and_node_kind(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        vfs.copy(
            real_source.as_identity(),
            sample_path.join(&Path::new("A")).as_path()
        );
        vfs.copy(
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        );
        vfs.copy(
            sample_path.join(&Path::new("B/F")).as_path(),
            sample_path.join(&Path::new("B/D/E")).as_path()
        );

        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsItem::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))), false)));
            },
            None => { panic!("No results"); }
        }
    }

    #[test]
    fn virtual_file_system_mv(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        vfs.mv(
            real_source.as_identity(),
            sample_path.join(&Path::new("A")).as_path()
        );
        vfs.mv(
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        );
        vfs.mv(
            sample_path.join(&Path::new("B/F")).as_path(),
            sample_path.join(&Path::new("B/D/E")).as_path()
        );

        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsItem::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))), false)));
            },
            None => { panic!("No results"); }
        }

        assert!(vfs.ls(sample_path.join(&Path::new("F")).as_path()).is_none());
        assert!(vfs.ls(sample_path.join(&Path::new("A/F")).as_path()).is_none());
        assert!(vfs.ls(sample_path.join(&Path::new("B/F")).as_path()).is_none());
    }

    #[test]
    fn virtual_file_system_mkdir(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        vfs.mkdir(sample_path.join(&Path::new("B/D/E/MKDIRED")).as_path());
        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsItem::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/MKDIRED"))), true)));
            },
            None => { panic!("No results"); }
        }
    }

    #[test]
    fn virtual_file_system_touch(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        vfs.touch(sample_path.join(&Path::new("B/D/E/TOUCHED")).as_path());
        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsItem::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/TOUCHED"))), false)));
            },
            None => { panic!("No results"); }
        }
    }
}
