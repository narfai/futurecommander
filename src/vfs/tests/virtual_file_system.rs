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

extern crate futurecommander_vfs;

#[cfg_attr(tarpaulin, skip)]
mod vfs_integration {
    use std::{
        path::{ Path }
    };

    use futurecommander_vfs::{
        VirtualFileSystem,
        operation::{
            Operation,
            CopyOperation,
            RemoveOperation,
            CreateOperation
        },
        query::{
            Query,
            Entry,
            StatusQuery
        },
        representation::{
            VirtualState
        },
        Samples,
        Kind
    };

    #[test]
    pub fn no_dangling() {
        let mut vfs = VirtualFileSystem::default();
        _no_dangling(
            &mut vfs,
            Samples::init_advanced_chroot("no_dangling").as_path()
        );
    }

    /*
        dumb operations which should be resolved
        cp A APRIME
        rm A

        cp APRIME A
        rm APRIME
        */
    pub fn _no_dangling(vfs: &mut VirtualFileSystem, chroot: &Path) {
        CopyOperation::new(
            chroot.join("A").as_path(),
            chroot.join("APRIME").as_path()
        ).execute(vfs).unwrap();

        RemoveOperation::new(
            chroot.join("A").as_path()
        ).execute(vfs).unwrap();

        CopyOperation::new(
            chroot.join("APRIME").as_path(),
            chroot.join("A").as_path()
        ).execute(vfs).unwrap();

        RemoveOperation::new(
            chroot.join("APRIME").as_path()
        ).execute(vfs).unwrap();

        let stated_a = StatusQuery::new(chroot.join("A").as_path())
            .retrieve(&vfs)
            .unwrap()
            .into_inner();

        assert!(match stated_a.state() {
            VirtualState::Exists => true,
            _ => false
        });

        let virtual_identity = stated_a.into_existing_virtual().unwrap();

        assert_eq!(virtual_identity.as_identity(), chroot.join("A"));
        assert_eq!(virtual_identity.to_kind(), Kind::Directory);
        assert_eq!(virtual_identity.as_source().unwrap(), chroot.join("A"));

        let stated_aprime = StatusQuery::new(chroot.join("APRIME").as_path())
            .retrieve(&vfs)
            .unwrap();

        assert!(!stated_aprime.exists());
    }

    #[test]
    pub fn file_dir_interversion() {
        let mut vfs = VirtualFileSystem::default();
        _file_dir_interversion(
            &mut vfs,
            Samples::init_advanced_chroot("file_dir_interversion").as_path()
        );
    }


    pub fn _file_dir_interversion(vfs: &mut VirtualFileSystem, chroot: &Path) {
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

        CopyOperation::new(
            chroot.join("A/C").as_path(),
            chroot.join("C").as_path()
        ).execute(vfs).unwrap();

        RemoveOperation::new(
            chroot.join("A/C").as_path()
        ).execute(vfs).unwrap();

        CopyOperation::new(
            chroot.join("C").as_path(),
            chroot.join("Z").as_path()
        ).execute(vfs).unwrap();

        RemoveOperation::new(
            chroot.join("C").as_path()
        ).execute(vfs).unwrap();

        CopyOperation::new(
            chroot.join("B").as_path(),
            chroot.join("C").as_path(),
        ).execute(vfs).unwrap();

        RemoveOperation::new(
            chroot.join("B").as_path()
        ).execute(vfs).unwrap();

        CopyOperation::new(
            chroot.join("Z").as_path(),
            chroot.join("B").as_path(),
        ).execute(vfs).unwrap();

        RemoveOperation::new(
            chroot.join("Z").as_path()
        ).execute(vfs).unwrap();

        let stated_b = StatusQuery::new(chroot.join("B").as_path())
            .retrieve(&vfs)
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_b.as_identity(), chroot.join("B"));
        assert_eq!(stated_b.to_kind(), Kind::File);
        assert_eq!(stated_b.as_source().unwrap(), chroot.join("A/C"));

        let stated_c = StatusQuery::new(chroot.join("C").as_path())
            .retrieve(&vfs)
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_c.as_identity(), chroot.join("C"));
        assert_eq!(stated_c.to_kind(), Kind::Directory);
        assert_eq!(stated_c.as_source().unwrap(), chroot.join("B"));

        let stated_z = StatusQuery::new(chroot.join("Z").as_path())
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
    pub fn _some_nesting(vfs: &mut VirtualFileSystem, chroot: &Path) {
        CopyOperation::new(
            chroot.join("C").as_path(),
            chroot.join("A/C").as_path(),
        ).execute(vfs).unwrap();

        CopyOperation::new(
            chroot.join("A/C/D").as_path(),
            chroot.join("A/D").as_path()
        ).execute(vfs).unwrap();

        RemoveOperation::new(
            chroot.join("A/D/G").as_path()
        ).execute(vfs).unwrap();

        let stated_ad = StatusQuery::new(chroot.join("A/D").as_path())
            .retrieve(&vfs)
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_ad.as_identity(), chroot.join("A/D"));
        assert_eq!(stated_ad.to_kind(), Kind::Directory);
        assert_eq!(stated_ad.as_source().unwrap(), chroot.join("B/D"));

        let stated_adg = StatusQuery::new(chroot.join("A/D/G").as_path())
            .retrieve(&vfs)
            .unwrap();

        assert!(!stated_adg.exists());
    }


    // No-Backwards tests
    #[test]
    fn reset_empty() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();

        CreateOperation::new(
            sample_path.join("VIRTUALA").as_path(),
            Kind::File
        ).execute(&mut vfs).unwrap();

        CreateOperation::new(
            sample_path.join("VIRTUALB").as_path(),
            Kind::Directory
        ).execute(&mut vfs).unwrap();

        RemoveOperation::new(
            sample_path.join("A").as_path()
        ).execute(&mut vfs).unwrap();

        assert!(vfs.has_addition());
        assert!(vfs.has_subtraction());

        vfs.reset();

        assert!(!vfs.has_addition());
        assert!(!vfs.has_subtraction());
    }
}
