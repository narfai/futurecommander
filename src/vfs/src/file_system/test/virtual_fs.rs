/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::{ Path };
use std::ffi::{ OsString };

use crate::*;
use crate::file_system::test::sample::Samples;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");
        let ab = sample_path.join("A/B");
        let abcdef = sample_path.join("A/B/C/D/E/F");

        Virtual::<CopyOperation>::new(
            b.as_path(),
            a.as_path(),
            None
        ).execute(&mut vfs).unwrap();

        let virtual_state = vfs.virtual_state().unwrap();

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
    fn resolve_through() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");

        let ab = sample_path.join("A/B");
        let bd = sample_path.join("B/D");

        Virtual::<CopyOperation>::new(
            b.as_path(),
            a.as_path(),
            None
        ).execute(&mut vfs).unwrap();

        Virtual::<CopyOperation>::new(
            ab.as_path(),
            bd.as_path(),
            None
        ).execute(&mut vfs).unwrap();

        let virtual_state = vfs.virtual_state().unwrap();

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
    fn stat_none_if_deleted() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        assert!(
            Virtual(StatusQuery::new(a.as_path()))
                .retrieve(&vfs)
                .unwrap()
                .virtual_identity()
                .is_some()
        );

        Virtual::<RemoveOperation>::new(a.as_path())
            .execute(&mut vfs).unwrap();


        assert!(
            Virtual(StatusQuery::new(a.as_path()))
                .retrieve(&vfs)
                .unwrap()
                .virtual_identity()
                .is_none()
        );
    }

    #[test]
    fn stat_virtual() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();
        let z = sample_path.join("Z");

        Virtual::<CreateOperation>::new(
            z.as_path(),
            VirtualKind::Directory
        ).execute(&mut vfs).unwrap();

        let stated = Virtual(StatusQuery::new(z.as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), z);
        assert!(stated.as_source().is_none())
    }

    #[test]
    fn stat_real() {
        let sample_path = Samples::static_samples_path();
        let vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        let stated = Virtual(StatusQuery::new(a.as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), a.as_path());
        assert_eq!(stated.as_source(), Some(a.as_path()))
    }

    #[test]
    fn stat_related() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();
        let abdg = sample_path.join("A/B/D/G");//Note : should exists in samples

        Virtual::<CopyOperation>::new(
            sample_path.join("B").as_path(),
            sample_path.join("A").as_path(),
            None
        ).execute(&mut vfs).unwrap();

        let stated = Virtual(StatusQuery::new(abdg.as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), abdg.as_path());
        assert_eq!(stated.as_source(), Some(sample_path.join("B/D/G").as_path()))
    }

    #[test]
    pub fn no_dangling() {
        let mut vfs = VirtualFileSystem::new();
        _no_dangling(
            &mut vfs,
            Samples::init_virtual_chroot("no_dangling").as_path()
        );
    }

    /*
    dumb operations which should be resolved
    cp A APRIME
    rm A

    cp APRIME A
    rm APRIME
    */
    pub fn _no_dangling(mut vfs: &mut VirtualFileSystem, chroot: &Path) {
        Virtual::<CopyOperation>::new(
            chroot.join("A").as_path(),
            chroot,
            Some(OsString::from("APRIME"))
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            chroot.join("A").as_path()
        ).execute(&mut vfs).unwrap();

        Virtual::<CopyOperation>::new(
            chroot.join("APRIME").as_path(),
            chroot,
            Some(OsString::from("A"))
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            chroot.join("APRIME").as_path()
        ).execute(&mut vfs).unwrap();

        let stated_a = Virtual(StatusQuery::new(chroot.join("A").as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated_a.as_identity(), chroot.join("A"));
        assert_eq!(stated_a.to_kind(), VirtualKind::Directory);
        assert_eq!(stated_a.as_source().unwrap(), chroot.join("A"));

        let stated_aprime = Virtual(StatusQuery::new(chroot.join("APRIME").as_path()))
            .retrieve(&vfs)
            .unwrap();

        assert!(!stated_aprime.exists());
    }

    #[test]
    pub fn file_dir_interversion() {
        let mut vfs = VirtualFileSystem::new();
        _file_dir_interversion(
            &mut vfs,
            Samples::init_virtual_chroot("file_dir_interversion").as_path()
        );
    }


    pub fn _file_dir_interversion(mut vfs: &mut VirtualFileSystem, chroot: &Path) {
        /*
        file dir interversion ( C <-> B )
        cp A/C .
        rm A/C

        cp C Z
        rm C

        cp B C <- C should be dir & still a file, may there is smth to update or not take data from fs but stat insteda
        rm B

        cp Z B
        rm Z
        */

        Virtual::<CopyOperation>::new(
            chroot.join("A/C").as_path(),
            chroot,
            None
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            chroot.join("A/C").as_path()
        ).execute(&mut vfs).unwrap();

        Virtual::<CopyOperation>::new(
            chroot.join("C").as_path(),
            chroot,
            Some(OsString::from("Z"))
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            chroot.join("C").as_path()
        ).execute(&mut vfs).unwrap();

        Virtual::<CopyOperation>::new(
            chroot.join("B").as_path(),
            chroot,
            Some(OsString::from("C"))
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            chroot.join("B").as_path()
        ).execute(&mut vfs).unwrap();

        Virtual::<CopyOperation>::new(
            chroot.join("Z").as_path(),
            chroot,
            Some(OsString::from("B"))
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            chroot.join("Z").as_path()
        ).execute(&mut vfs).unwrap();

        let stated_b = Virtual(StatusQuery::new(chroot.join("B").as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated_b.as_identity(), chroot.join("B"));
        assert_eq!(stated_b.to_kind(), VirtualKind::File);
        assert_eq!(stated_b.as_source().unwrap(), chroot.join("A/C"));

        let stated_c = Virtual(StatusQuery::new(chroot.join("C").as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated_c.as_identity(), chroot.join("C"));
        assert_eq!(stated_c.to_kind(), VirtualKind::Directory);
        assert_eq!(stated_c.as_source().unwrap(), chroot.join("B"));

        let stated_z = Virtual(StatusQuery::new(chroot.join("Z").as_path()))
            .retrieve(&vfs)
            .unwrap();

        assert!(!stated_z.exists());
    }

    /*
        nesting
        cp C A/
        cp A/B/D A/
        rm A/D/G //<- should no appear
    */
    pub fn _some_nesting(mut vfs: &mut VirtualFileSystem, chroot: &Path) {
        Virtual::<CopyOperation>::new(
            chroot.join("C").as_path(),
            chroot.join("A").as_path(),
            None
        ).execute(&mut vfs).unwrap();

        Virtual::<CopyOperation>::new(
            chroot.join("A/C/D").as_path(),
            chroot.join("A").as_path(),
            None
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            chroot.join("A/D/G").as_path()
        ).execute(&mut vfs).unwrap();

        let stated_ad = Virtual(StatusQuery::new(chroot.join("A/D").as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated_ad.as_identity(), chroot.join("A/D"));
        assert_eq!(stated_ad.to_kind(), VirtualKind::Directory);
        assert_eq!(stated_ad.as_source().unwrap(), chroot.join("B/D"));

        let stated_adg = Virtual(StatusQuery::new(chroot.join("A/D/G").as_path()))
            .retrieve(&vfs)
            .unwrap();

        assert!(!stated_adg.exists());
    }


    #[test]
    pub fn apply_a_vfs_to_real_fs() {
        let chroot = Samples::init_virtual_chroot("apply_a_vfs_to_real_fs");
        let mut vfs = VirtualFileSystem::new();
        let _real_fs = RealFileSystem::new(true);


        _no_dangling(&mut vfs, chroot.as_path());
        _file_dir_interversion(&mut vfs, chroot.as_path());
        _some_nesting(&mut vfs, chroot.as_path());

        let _apply: ApplyOperation<Box<WriteOperation<RealFileSystem>>> = ApplyOperation::from_virtual_filesystem(&vfs).unwrap();
//        println!("REAL VERSION : {}", RealVersion::get());
//        apply.execute(&mut real_fs).unwrap();
//        println!("REAL VERSION : {}", RealVersion::get());
//        println!("{:?}", apply)
    }

    //Error testing
    #[test]
    fn copy_or_move_directory_into_itself_must_not_be_allowed() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        let source = sample_path.join("B");
        let destination = sample_path.join("B/D");

        match Virtual::<CopyOperation>::new(
            source.as_path(),
            destination.as_path(),
            None
        ).execute(&mut vfs) {
            Err(VfsError::CopyIntoItSelf(err_source, err_destination)) => {
                assert_eq!(source.as_path(), err_source.as_path());
                assert_eq!(destination.as_path(), err_destination.as_path());
            }
            Err(error) => panic!("{}", error),
            Ok(_) => panic!("Should not be able to copy into itself")
        };
    }

    #[test]
    fn copy_source_does_not_exists() {}

    #[test]
    fn copy_destination_does_not_exists() {}

    #[test]
    fn create_already_exists() {}

    #[test]
    fn remove_does_not_exists() {}

    // No-Backwards tests
    #[test]
    fn reset_empty() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::new();

        Virtual::<CreateOperation>::new(
            sample_path.join("VIRTUALA").as_path(),
            VirtualKind::File
        ).execute(&mut vfs).unwrap();

        Virtual::<CreateOperation>::new(
            sample_path.join("VIRTUALB").as_path(),
            VirtualKind::Directory
        ).execute(&mut vfs).unwrap();

        Virtual::<RemoveOperation>::new(
            sample_path.join("A").as_path()
        ).execute(&mut vfs).unwrap();

        assert!(vfs.has_addition());
        assert!(vfs.has_subtraction());

        vfs.reset();

        assert!(!vfs.has_addition());
        assert!(!vfs.has_subtraction());
    }

    #[test]
    fn status_exists() {}

    #[test]
    fn status_exists_virtually() {}

    #[test]
    fn status_exists_through_virtual_parent() {}

    #[test]
    fn status_exists_not_exists() {}

    #[test]
    fn status_exists_deleted() {}

    #[test]
    fn status_exists_replaced() {}

    #[test]
    fn status_exists_removed_virtually() {}

    #[test]
    fn create() {}

    #[test]
    fn remove() {}

    #[test]
    fn copy() {}

    #[test]
    fn copy_with_rename() {}
}

