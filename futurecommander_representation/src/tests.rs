// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod representation_integration {
    #[allow(unused_imports)]
    use std::{
        str::FromStr
    };

    use std::{
        path::{ Path }
    };

    use crate::{
        Kind,
        VirtualPath,
        VirtualDelta
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
}
