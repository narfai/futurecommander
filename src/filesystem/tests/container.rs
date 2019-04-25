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

extern crate futurecommander_filesystem;

#[cfg_attr(tarpaulin, skip)]
mod container_integration {
    use std::{
        path:: { Path }
    };

    use futurecommander_filesystem::{
        Container,
        sample::Samples,
        Kind,
        CopyEvent,
        ReadableFileSystem,
        RemoveEvent,
        Listener,
        Delayer,
        VirtualState,
        Entry
    };

    #[test]
    pub fn no_dangling() {
        let mut fs = Container::new();
        _no_dangling(
            &mut fs,
            Samples::init_advanced_chroot("hybrid_no_dangling").as_path()
        );
    }

    /*
    dumb operations which should be resolved
    cp A APRIME
    rm A

    cp APRIME A
    rm APRIME
    */
    pub fn _no_dangling(fs: &mut Container, chroot: &Path) {
        let cp_a_aprime = CopyEvent::new(
            chroot.join("A").as_path(),
            chroot.join("APRIME").as_path(),
            true,
            false
        );

        fs.emit(&cp_a_aprime).unwrap();
        fs.delay(Box::new(cp_a_aprime));

        let rm_a = RemoveEvent::new(
            chroot.join("A").as_path(),
            true
        );

        fs.emit(&rm_a).unwrap();
        fs.delay(Box::new(rm_a));

        let cp_aprime_chroot = CopyEvent::new(
            chroot.join("APRIME").as_path(),
            chroot.join("A").as_path(),
            true,
            false
        );

        fs.emit(&cp_aprime_chroot).unwrap();
        fs.delay(Box::new(cp_aprime_chroot));

        let rm_aprime = RemoveEvent::new(
            chroot.join("APRIME").as_path(),
            true
        );

        fs.emit(&rm_aprime).unwrap();
        fs.delay(Box::new(rm_aprime));

        let stated_a = fs.status(chroot.join("A").as_path())
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

        let stated_aprime = fs.status(chroot.join("APRIME").as_path())
            .unwrap();

        assert!(!stated_aprime.exists());
    }

    #[test]
    pub fn copy_file_dir_interversion() {
        let mut fs = Container::new();
        _copy_file_dir_interversion(
            &mut fs,
            Samples::init_advanced_chroot("hybrid_file_dir_interversion").as_path()
        );
    }


    pub fn _copy_file_dir_interversion(fs: &mut Container, chroot: &Path) {
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

        let cp_ac_chroot = CopyEvent::new(
            chroot.join("A/C").as_path(),
            chroot.join("C").as_path(),
            true,
            false
        );

        fs.emit(&cp_ac_chroot).unwrap();
        fs.delay(Box::new(cp_ac_chroot));

        let rm_ac = RemoveEvent::new(
            chroot.join("A/C").as_path(),
            true
        );

        fs.emit(&rm_ac).unwrap();
        fs.delay(Box::new(rm_ac));

        let cp_c_z = CopyEvent::new(
            chroot.join("C").as_path(),
            chroot.join("Z").as_path(),
            true,
            false
        );

        fs.emit(&cp_c_z).unwrap();
        fs.delay(Box::new(cp_c_z));


        let rm_c = RemoveEvent::new(
            chroot.join("C").as_path(),
            true
        );

        fs.emit(&rm_c).unwrap();
        fs.delay(Box::new(rm_c));

        let cp_b_c = CopyEvent::new(
            chroot.join("B").as_path(),
            chroot.join("C").as_path(),
            true,
            false
        );

        fs.emit(&cp_b_c).unwrap();
        fs.delay(Box::new(cp_b_c));

        let rm_b = RemoveEvent::new(
            chroot.join("B").as_path(),
            true
        );

        fs.emit(&rm_b).unwrap();
        fs.delay(Box::new(rm_b));

        let cp_z_b = CopyEvent::new(
            chroot.join("Z").as_path(),
            chroot.join("B").as_path(),
            true,
            false
        );

        fs.emit(&cp_z_b).unwrap();
        fs.delay(Box::new(cp_z_b));


        let rm_z = RemoveEvent::new(
            chroot.join("Z").as_path(),
            true
        );

        fs.emit(&rm_z).unwrap();
        fs.delay(Box::new(rm_z));


        let stated_b = fs.status(chroot.join("B").as_path())
            .unwrap();

        assert!(stated_b.exists());
        assert!(stated_b.is_file());
        assert_eq!(stated_b.as_inner().as_virtual().as_identity(), chroot.join("B"));
        assert_eq!(stated_b.as_inner().as_virtual().to_kind(), Kind::File);
        assert_eq!(stated_b.as_inner().as_virtual().as_source().unwrap(), chroot.join("A/C"));

        let stated_c = fs.status(chroot.join("C").as_path())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_c.as_identity(), chroot.join("C"));
        assert_eq!(stated_c.to_kind(), Kind::Directory);
        assert_eq!(stated_c.as_source().unwrap(), chroot.join("B"));

        let stated_z = fs.status(chroot.join("Z").as_path())
            .unwrap();

        assert!(!stated_z.exists());
    }

    /*
        nesting
        cp C A/
        cp A/C/D A/
        rm A/D/G //<- should no appear
    */
    pub fn _some_nesting(fs: &mut Container, chroot: &Path) {
        let cp_c_a = CopyEvent::new(
            chroot.join("C").as_path(),
            chroot.join("A").join("C").as_path(),
            true,
            false
        );

        fs.emit(&cp_c_a).unwrap();
        fs.delay(Box::new(cp_c_a));

        let cp_acd_a = CopyEvent::new(
            chroot.join("A/C/D").as_path(),
            chroot.join("A").join("D").as_path(),
            true,
            false
        );

        fs.emit(&cp_acd_a).unwrap();
        fs.delay(Box::new(cp_acd_a));

        let rm_adg = RemoveEvent::new(
            chroot.join("A/D/G").as_path(),
            true
        );
        fs.emit(&rm_adg).unwrap();
        fs.delay(Box::new(rm_adg));

        let stated_ad = fs.status(chroot.join("A/D").as_path())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_ad.as_identity(), chroot.join("A/D"));
        assert_eq!(stated_ad.to_kind(), Kind::Directory);
        assert_eq!(stated_ad.as_source().unwrap(), chroot.join("B/D"));

        let stated_adg = fs.status(chroot.join("A/D/G").as_path())
            .unwrap();

        assert!(!stated_adg.exists());
    }

    #[test]
    pub fn apply_a_vfs_to_real_fs() {
        let chroot = Samples::init_advanced_chroot("hybrid_apply_a_vfs_to_real_fs");
        let mut fs = Container::new();

        _no_dangling(&mut fs, chroot.as_path());
        _copy_file_dir_interversion(&mut fs, chroot.as_path());
        _some_nesting(&mut fs, chroot.as_path());

        fs.apply().unwrap();

        let c = chroot.join("C");
        assert!(c.exists());
        assert!(c.is_dir());

        let b = chroot.join("B");
        assert!(b.exists());
        assert!(b.is_file());

        let ac = chroot.join("A/C");
        assert!(ac.exists());
        assert!(ac.is_dir());

        let ad = chroot.join("A/D");
        assert!(ad.exists());
        assert!(ad.is_dir());

        let ad = chroot.join("A/D/G");
        assert!(!ad.exists());
    }
}
